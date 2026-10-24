use super::winnowing::{Fingerprint, winnow_corpus};
use crate::tokenizer::{BpeTrainingResult, SourceFile, Token, Vocabulary, decode_token};
use std::cmp::Reverse;
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DetectionParams {
    /// Shingle size in tokens, rolling hashes are computed on shingles of k tokens.
    pub shingle_size_k: usize,
    /// Window size in shingles, duplicates of at least k + w - 1 tokens are always found.
    pub window_size_w: usize,
    /// Minimum length in tokens of a reported duplicated block.
    pub min_tokens: usize,
    /// Maximum number of occurrences of a shingle to be considered for duplication detection.
    pub max_occurrences: usize,
}

impl Default for DetectionParams {
    fn default() -> Self {
        Self {
            shingle_size_k: 4,
            window_size_w: 4,
            min_tokens: 10,
            max_occurrences: 10,
        }
    }
}

/// A hash shared by two locations: (file, shingle position) on each side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenMatch {
    pub file_a: usize,
    pub pos_a: usize,
    pub file_b: usize,
    pub pos_b: usize,
}

/// A duplicated region expressed as file and token indices (exclusive end).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDuplicate {
    pub file_a: usize,
    pub tokens_a: Range<usize>,
    pub file_b: usize,
    pub tokens_b: Range<usize>,
}

/// A file region: a range of BPE tokens and a character range in the ingested content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub path: PathBuf,
    pub tokens: Range<usize>,
    pub chars: Range<usize>,
}

/// Two regions with identical content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Duplicate {
    pub a: Region,
    pub b: Region,
}

/// Reversed Index: hash -> [(file index, shingle position)].
pub fn build_index(signatures: &[Vec<Fingerprint>]) -> HashMap<u64, Vec<(usize, usize)>> {
    let mut index: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();

    for (file, signature) in signatures.iter().enumerate() {
        for &(hash, pos) in signature {
            index.entry(hash).or_default().push((file, pos));
        }
    }
    index
}

/// All pairs of locations sharing a hash. Hashes found at only one location or at more
/// than `max_occurrences` locations are ignored. In the same file, overlapping shingles
pub fn find_matches(
    index: &HashMap<u64, Vec<(usize, usize)>>,
    k: usize,
    max_occurrences: usize,
) -> Vec<TokenMatch> {
    let mut matches = Vec::new();
    for locations in index.values() {
        if locations.len() < 2 || locations.len() > max_occurrences {
            continue;
        }
        let mut locations = locations.clone();
        locations.sort_unstable();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let (file_a, pos_a) = locations[i];
                let (file_b, pos_b) = locations[j];
                if file_a == file_b && pos_b - pos_a < k {
                    continue;
                }
                matches.push(TokenMatch {
                    file_a,
                    pos_a,
                    file_b,
                    pos_b,
                });
            }
        }
    }
    matches.sort_unstable();
    matches.dedup();
    matches
}

pub fn merge_matches(
    mut matches: Vec<TokenMatch>,
    k: usize,
    w: usize,
    min_tokens: usize,
) -> Vec<RawDuplicate> {
    let diagonal = |m: &TokenMatch| m.pos_b as i64 - m.pos_a as i64;
    matches.sort_unstable_by_key(|m| (m.file_a, m.file_b, diagonal(m), m.pos_a));

    let mut out = Vec::new();
    let mut current: Option<(TokenMatch, TokenMatch)> = None;

    let flush = |run: Option<(TokenMatch, TokenMatch)>, out: &mut Vec<RawDuplicate>| {
        let Some((first, last)) = run else { return };
        let len = last.pos_a - first.pos_a + k;
        if len >= min_tokens {
            out.push(RawDuplicate {
                file_a: first.file_a,
                tokens_a: first.pos_a..last.pos_a + k,
                file_b: first.file_b,
                tokens_b: first.pos_b..last.pos_b + k,
            });
        }
    };

    for m in matches {
        current = match current {
            Some((first, last))
                if (m.file_a, m.file_b, diagonal(&m))
                    == (last.file_a, last.file_b, diagonal(&last))
                    && m.pos_a - last.pos_a <= w =>
            {
                Some((first, m))
            }
            other => {
                flush(other, &mut out);
                Some((m, m))
            }
        };
    }
    flush(current, &mut out);
    out
}

pub fn token_char_offsets(sequence: &[Token], vocabulary: &Vocabulary) -> Vec<usize> {
    let mut offsets = Vec::with_capacity(sequence.len() + 1);
    let mut acc = 0;

    offsets.push(acc);
    for &token in sequence {
        acc += decode_token(token, vocabulary).chars().count();
        offsets.push(acc);
    }
    offsets
}

/// Run duplicate detection on a BPE-encoded corpus.
pub fn detect_duplicates(result: &BpeTrainingResult, params: &DetectionParams) -> Vec<Duplicate> {
    let signatures: Vec<Vec<Fingerprint>> =
        winnow_corpus(result, params.shingle_size_k, params.window_size_w)
            .into_iter()
            .map(|(_, signature)| signature)
            .collect();
    let index = build_index(&signatures);
    let matches = find_matches(&index, params.shingle_size_k, params.max_occurrences);
    let raw = merge_matches(
        matches,
        params.shingle_size_k,
        params.window_size_w,
        params.min_tokens,
    );

    let offsets: Vec<Vec<usize>> = result
        .files
        .iter()
        .map(|f| token_char_offsets(&f.sequence, &result.vocabulary))
        .collect();
    let region = |file: usize, tokens: Range<usize>| Region {
        path: result.files[file].path.clone(),
        chars: offsets[file][tokens.start]..offsets[file][tokens.end],
        tokens,
    };

    raw.into_iter()
        .map(|d| Duplicate {
            a: region(d.file_a, d.tokens_a),
            b: region(d.file_b, d.tokens_b),
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub path: PathBuf,
    pub tokens: Range<usize>,
    pub chars: Range<usize>,
    pub lines: Option<RangeInclusive<usize>>,
}

/// A code block present in at least two locations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateGroup {
    /// Sorted by (file, position). Always contains at least two elements.
    pub instances: Vec<Instance>,
}

impl DuplicateGroup {
    /// Character length of the largest instance.
    pub fn max_chars(&self) -> usize {
        self.instances
            .iter()
            .map(|i| i.chars.end - i.chars.start)
            .max()
            .unwrap_or(0)
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.parent[rb] = ra;
        }
    }
}

/// Maps a character offset to its original line number.
struct LineIndex<'a> {
    newlines: Vec<usize>,
    numbers: &'a [usize],
}

impl<'a> LineIndex<'a> {
    fn new(file: &'a SourceFile) -> Self {
        let newlines = file
            .content
            .chars()
            .enumerate()
            .filter(|&(_, c)| c == '\n')
            .map(|(i, _)| i)
            .collect();
        Self {
            newlines,
            numbers: &file.line_numbers,
        }
    }

    fn line_of(&self, offset: usize) -> Option<usize> {
        let index = self.newlines.partition_point(|&p| p < offset);
        self.numbers.get(index).copied()
    }

    fn lines(&self, chars: &Range<usize>) -> Option<RangeInclusive<usize>> {
        let last = chars.end.saturating_sub(1).max(chars.start);
        Some(self.line_of(chars.start)?..=self.line_of(last)?)
    }
}

fn overlap(a: &Range<usize>, b: &Range<usize>) -> usize {
    a.end.min(b.end).saturating_sub(a.start.max(b.start))
}

/// Groups pairs into sets of duplicate instances.
///
/// 1. Regions in the same file that overlap by at least 50% of the shorter region are
///    considered the same instance (bounds can vary by a few tokens between pairs); the
///    instance covers their union. The two sides of the same pair are never merged.
/// 2. Instances connected by a pair belong to the same group (transitivity: if A≈B and
///    B ≈ C, then {A, B, C}).
///
/// Groups are sorted by descending number of copies, then by size.
pub fn group_duplicates(duplicates: &[Duplicate], files: &[SourceFile]) -> Vec<DuplicateGroup> {
    let region = |node: usize| {
        let d = &duplicates[node / 2];
        if node.is_multiple_of(2) { &d.a } else { &d.b }
    };
    let node_count = duplicates.len() * 2;

    let mut instances_uf = UnionFind::new(node_count);
    let mut by_path: HashMap<&Path, Vec<usize>> = HashMap::new();
    for node in 0..node_count {
        by_path.entry(&region(node).path).or_default().push(node);
    }
    for nodes in by_path.values_mut() {
        nodes.sort_by_key(|&n| (region(n).tokens.start, region(n).tokens.end));
        for (i, &a) in nodes.iter().enumerate() {
            for &b in &nodes[i + 1..] {
                let (ra, rb) = (&region(a).tokens, &region(b).tokens);
                if rb.start >= ra.end {
                    break;
                }
                let shorter = (ra.end - ra.start).min(rb.end - rb.start);
                if a / 2 != b / 2 && overlap(ra, rb) * 2 >= shorter {
                    instances_uf.union(a, b);
                }
            }
        }
    }

    let mut instance_of_root: HashMap<usize, usize> = HashMap::new();
    let mut instances: Vec<Instance> = Vec::new();
    let mut instance_of_node = vec![0; node_count];
    for (node, slot) in instance_of_node.iter_mut().enumerate() {
        let root = instances_uf.find(node);
        let r = region(node);
        let id = *instance_of_root.entry(root).or_insert_with(|| {
            instances.push(Instance {
                path: r.path.clone(),
                tokens: r.tokens.clone(),
                chars: r.chars.clone(),
                lines: None,
            });
            instances.len() - 1
        });
        let inst = &mut instances[id];
        inst.tokens = inst.tokens.start.min(r.tokens.start)..inst.tokens.end.max(r.tokens.end);
        inst.chars = inst.chars.start.min(r.chars.start)..inst.chars.end.max(r.chars.end);
        *slot = id;
    }

    let mut groups_uf = UnionFind::new(instances.len());
    for i in 0..duplicates.len() {
        groups_uf.union(instance_of_node[2 * i], instance_of_node[2 * i + 1]);
    }

    let line_indexes: HashMap<&Path, LineIndex> = files
        .iter()
        .map(|f| (f.path.as_path(), LineIndex::new(f)))
        .collect();
    for inst in &mut instances {
        inst.lines = line_indexes
            .get(inst.path.as_path())
            .and_then(|index| index.lines(&inst.chars));
    }

    let mut groups: HashMap<usize, Vec<Instance>> = HashMap::new();
    for (id, inst) in instances.into_iter().enumerate() {
        groups.entry(groups_uf.find(id)).or_default().push(inst);
    }
    let mut groups: Vec<DuplicateGroup> = groups
        .into_values()
        .filter(|instances| instances.len() >= 2)
        .map(|mut instances| {
            instances.sort_by(|a, b| (&a.path, a.tokens.start).cmp(&(&b.path, b.tokens.start)));
            DuplicateGroup { instances }
        })
        .collect();
    groups.sort_by(|a, b| {
        let key = |g: &DuplicateGroup| {
            let first = &g.instances[0];
            (
                Reverse(g.instances.len()),
                Reverse(g.max_chars()),
                first.path.clone(),
                first.tokens.start,
            )
        };
        key(a).cmp(&key(b))
    });
    groups
}

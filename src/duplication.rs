use crate::tokenizer::{BpeTrainingResult, SourceFile, Token, Vocabulary, decode_token};
use crate::winnowing::{Fingerprint, winnow_corpus};
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
///    B≈C, then {A, B, C}).
///
/// Groups are sorted by descending number of copies, then by size.
pub fn group_duplicates(duplicates: &[Duplicate], files: &[SourceFile]) -> Vec<DuplicateGroup> {
    // Nodes: region a of pair i = 2i, region b = 2i + 1.
    let region = |node: usize| {
        let d = &duplicates[node / 2];
        if node % 2 == 0 { &d.a } else { &d.b }
    };
    let node_count = duplicates.len() * 2;

    // Step 1: merge overlapping regions, file by file.
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

    // Instances are node sets covering the union of their ranges.
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

    // Step 2: instances connected by a pair form a group.
    let mut groups_uf = UnionFind::new(instances.len());
    for i in 0..duplicates.len() {
        groups_uf.union(instance_of_node[2 * i], instance_of_node[2 * i + 1]);
    }

    // Calculate line numbers.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{SourceFile, train_corpus};

    fn sig(entries: &[(u64, usize)]) -> Vec<Fingerprint> {
        entries.to_vec()
    }

    fn tm(file_a: usize, pos_a: usize, file_b: usize, pos_b: usize) -> TokenMatch {
        TokenMatch {
            file_a,
            pos_a,
            file_b,
            pos_b,
        }
    }

    // ---------- inverted index ----------

    #[test]
    fn index_regroupe_les_emplacements_par_hash() {
        let index = build_index(&[sig(&[(1, 0), (2, 5)]), sig(&[(2, 9), (3, 1)])]);
        assert_eq!(index[&1], vec![(0, 0)]);
        assert_eq!(index[&2], vec![(0, 5), (1, 9)]);
        assert_eq!(index[&3], vec![(1, 1)]);
    }

    #[test]
    fn index_vide() {
        assert!(build_index(&[]).is_empty());
        assert!(build_index(&[vec![], vec![]]).is_empty());
    }

    // ---------- matches ----------

    #[test]
    fn hash_unique_ne_donne_aucun_match() {
        let index = build_index(&[sig(&[(1, 0)]), sig(&[(2, 0)])]);
        assert!(find_matches(&index, 3, 50).is_empty());
    }

    #[test]
    fn hash_commun_entre_deux_fichiers() {
        let index = build_index(&[sig(&[(7, 4)]), sig(&[(7, 10)])]);
        assert_eq!(find_matches(&index, 3, 50), vec![tm(0, 4, 1, 10)]);
    }

    #[test]
    fn trois_emplacements_donnent_trois_paires() {
        let index = build_index(&[sig(&[(7, 0)]), sig(&[(7, 0)]), sig(&[(7, 0)])]);
        assert_eq!(
            find_matches(&index, 3, 50),
            vec![tm(0, 0, 1, 0), tm(0, 0, 2, 0), tm(1, 0, 2, 0)]
        );
    }

    #[test]
    fn duplication_dans_un_meme_fichier() {
        let index = build_index(&[sig(&[(7, 2), (7, 20)])]);
        assert_eq!(find_matches(&index, 3, 50), vec![tm(0, 2, 0, 20)]);
    }

    #[test]
    fn shingles_qui_se_chevauchent_dans_un_fichier_sont_ignores() {
        // Distance 2 < k = 3: a periodic pattern, not a true duplication.
        let index = build_index(&[sig(&[(7, 2), (7, 4)])]);
        assert!(find_matches(&index, 3, 50).is_empty());
    }

    #[test]
    fn hash_trop_frequent_est_ignore() {
        let sigs: Vec<_> = (0..5).map(|_| sig(&[(7, 0)])).collect();
        let index = build_index(&sigs);
        assert!(find_matches(&index, 3, 4).is_empty());
        assert_eq!(find_matches(&index, 3, 5).len(), 10);
    }

    // ---------- merging into regions ----------

    #[test]
    fn matches_alignes_fusionnes_en_une_region() {
        // Same diagonal (+6), at most w = 4 tokens apart.
        let m = vec![tm(0, 10, 1, 16), tm(0, 13, 1, 19), tm(0, 17, 1, 23)];
        let d = merge_matches(m, 4, 4, 1);
        assert_eq!(
            d,
            vec![RawDuplicate {
                file_a: 0,
                tokens_a: 10..21,
                file_b: 1,
                tokens_b: 16..27,
            }]
        );
    }

    #[test]
    fn ecart_superieur_a_w_coupe_la_region() {
        let m = vec![tm(0, 10, 1, 10), tm(0, 15, 1, 15)]; // Distance 5 > w = 4.
        let d = merge_matches(m, 4, 4, 1);
        assert_eq!(d.len(), 2);
        assert_eq!(d[0].tokens_a, 10..14);
        assert_eq!(d[1].tokens_a, 15..19);
    }

    #[test]
    fn diagonales_differentes_ne_fusionnent_pas() {
        let m = vec![tm(0, 10, 1, 10), tm(0, 12, 1, 15)];
        assert_eq!(merge_matches(m, 4, 4, 1).len(), 2);
    }

    #[test]
    fn paires_de_fichiers_differentes_ne_fusionnent_pas() {
        let m = vec![tm(0, 10, 1, 10), tm(0, 12, 2, 12)];
        assert_eq!(merge_matches(m, 4, 4, 1).len(), 2);
    }

    #[test]
    fn ordre_des_matches_en_entree_sans_importance() {
        let a = vec![tm(0, 17, 1, 23), tm(0, 10, 1, 16), tm(0, 13, 1, 19)];
        let b = vec![tm(0, 10, 1, 16), tm(0, 13, 1, 19), tm(0, 17, 1, 23)];
        assert_eq!(merge_matches(a, 4, 4, 1), merge_matches(b, 4, 4, 1));
    }

    #[test]
    fn min_tokens_filtre_les_regions_courtes() {
        let m = vec![tm(0, 0, 1, 0)]; // Length = k = 4.
        assert_eq!(merge_matches(m.clone(), 4, 4, 4).len(), 1);
        assert!(merge_matches(m, 4, 4, 5).is_empty());
    }

    #[test]
    fn aucun_match_aucune_region() {
        assert!(merge_matches(vec![], 4, 4, 1).is_empty());
    }

    // ---------- character offsets ----------

    fn corpus(files: &[(&str, &str)], vocab_size: u32) -> BpeTrainingResult {
        let files: Vec<SourceFile> = files
            .iter()
            .map(|(p, c)| SourceFile::new(PathBuf::from(p), c.to_string()))
            .collect();
        train_corpus(&files, vocab_size, 2)
    }

    #[test]
    fn offsets_de_caracteres_couvrent_tout_le_texte() {
        let content =
            "fn add(a: i32, b: i32) -> i32 { a + b } fn add(a: i32, b: i32) -> i32 { a + b }";
        let r = corpus(&[("a.rs", content)], 200);
        let offsets = token_char_offsets(&r.files[0].sequence, &r.vocabulary);
        assert_eq!(offsets.len(), r.files[0].sequence.len() + 1);
        assert_eq!(offsets[0], 0);
        assert_eq!(*offsets.last().unwrap(), content.chars().count());
        assert!(offsets.windows(2).all(|w| w[0] < w[1]));
    }

    // ---------- end-to-end: BPE -> Winnowing -> duplicate detection ----------

    /// Deliberately small vocabulary: few merges, so duplicated blocks remain visible as
    /// multiple tokens (see `e2e_un_bpe_trop_agressif_absorbe_la_duplication`).
    const E2E_VOCAB: u32 = 110;

    const SHARED: &str = "fn compute_total(items: &[Item]) -> u64 { let mut total = 0; \
        for item in items { if item.active { total += item.price * item.qty; } } total }";

    fn slice(content: &str, r: &Range<usize>) -> String {
        content
            .chars()
            .skip(r.start)
            .take(r.end - r.start)
            .collect()
    }

    fn params() -> DetectionParams {
        DetectionParams {
            shingle_size_k: 3,
            window_size_w: 3,
            min_tokens: 10,
            max_occurrences: 50,
        }
    }

    #[test]
    fn e2e_bloc_partage_entre_deux_fichiers() {
        let a = format!(
            "struct Alpha {{ id: u32 }} impl Alpha {{ fn new() -> Self {{ Alpha {{ id: 1 }} }} }} {SHARED}"
        );
        let b = format!(
            "{SHARED} enum Beta {{ One, Two }} fn pick(b: Beta) -> u8 {{ match b {{ Beta::One => 1, Beta::Two => 2 }} }}"
        );
        let r = corpus(&[("a.rs", &a), ("b.rs", &b)], E2E_VOCAB);
        // a.rs also repeats `Alpha { id: `; keep only duplicates across files.
        let dups: Vec<_> = detect_duplicates(&r, &params())
            .into_iter()
            .filter(|d| d.a.path != d.b.path)
            .collect();

        assert_eq!(dups.len(), 1, "{dups:?}");
        let d = &dups[0];
        assert_eq!(d.a.path, PathBuf::from("a.rs"));
        assert_eq!(d.b.path, PathBuf::from("b.rs"));

        // Both regions decode to exactly the same text, within the shared block.
        let text_a = slice(&a, &d.a.chars);
        let text_b = slice(&b, &d.b.chars);
        assert_eq!(text_a, text_b);
        assert!(SHARED.contains(&text_a));
        assert!(text_a.chars().count() >= SHARED.chars().count() / 2);
    }

    #[test]
    fn e2e_fichiers_sans_rapport_aucune_duplication() {
        let a = "struct Alpha { id: u32 } impl Alpha { fn new() -> Self { Alpha { id: 1 } } }";
        let b = "enum Beta { One, Two } fn pick(b: Beta) -> u8 { match b { Beta::One => 1, Beta::Two => 2 } }";
        let r = corpus(&[("a.rs", a), ("b.rs", b)], E2E_VOCAB);
        assert!(detect_duplicates(&r, &params()).is_empty());
    }

    #[test]
    fn e2e_duplication_dans_un_seul_fichier() {
        let a = format!("{SHARED} struct Middle {{ x: i64, y: i64, z: i64 }} {SHARED}");
        let r = corpus(&[("a.rs", &a)], E2E_VOCAB);
        let dups = detect_duplicates(&r, &params());

        assert_eq!(dups.len(), 1, "{dups:?}");
        let d = &dups[0];
        assert_eq!(d.a.path, d.b.path);
        assert!(
            d.a.tokens.end <= d.b.tokens.start,
            "les deux copies ne se chevauchent pas"
        );
        assert_eq!(slice(&a, &d.a.chars), slice(&a, &d.b.chars));
    }

    #[test]
    fn e2e_trois_copies_donnent_trois_paires() {
        let a = format!("struct A {{ a: u8 }} {SHARED}");
        let b = format!("enum B {{ X, Y }} {SHARED}");
        let c = format!("const C: u8 = 3; {SHARED}");
        let r = corpus(&[("a.rs", &a), ("b.rs", &b), ("c.rs", &c)], E2E_VOCAB);
        let dups = detect_duplicates(&r, &params());

        let mut pairs: Vec<_> = dups
            .iter()
            .map(|d| (d.a.path.clone(), d.b.path.clone()))
            .collect();
        pairs.sort();
        pairs.dedup();
        assert_eq!(pairs.len(), 3, "{dups:?}");
    }

    #[test]
    fn e2e_min_tokens_trop_grand_ne_retourne_rien() {
        let a = format!("struct A {{ a: u8 }} {SHARED}");
        let b = format!("enum B {{ X, Y }} {SHARED}");
        let r = corpus(&[("a.rs", &a), ("b.rs", &b)], E2E_VOCAB);
        let p = DetectionParams {
            min_tokens: 10_000,
            ..params()
        };
        assert!(detect_duplicates(&r, &p).is_empty());
    }

    #[test]
    fn e2e_un_bpe_trop_agressif_absorbe_la_duplication() {
        // A block appearing twice has all its pairs at frequency >= 2. With enough merges,
        // BPE reduces it to just a few tokens, leaving Winnowing (k = 3) with no shingle to compare.
        let a = format!(
            "struct Alpha {{ id: u32 }} impl Alpha {{ fn new() -> Self {{ Alpha {{ id: 1 }} }} }} {SHARED}"
        );
        let b = format!(
            "{SHARED} enum Beta {{ One, Two }} fn pick(b: Beta) -> u8 {{ match b {{ Beta::One => 1, Beta::Two => 2 }} }}"
        );
        let r = corpus(&[("a.rs", &a), ("b.rs", &b)], 300);
        assert!(r.encoded_token_count() * 4 < r.initial_token_count);
        assert!(detect_duplicates(&r, &params()).is_empty());
    }

    // ---------- grouping ----------

    fn reg(path: &str, start: usize, end: usize) -> Region {
        Region {
            path: PathBuf::from(path),
            tokens: start..end,
            chars: start * 2..end * 2,
        }
    }

    fn dup(a: Region, b: Region) -> Duplicate {
        Duplicate { a, b }
    }

    fn places(g: &DuplicateGroup) -> Vec<(String, usize)> {
        g.instances
            .iter()
            .map(|i| (i.path.display().to_string(), i.tokens.start))
            .collect()
    }

    #[test]
    fn paires_transitives_forment_un_seul_groupe() {
        let dups = vec![
            dup(reg("x", 0, 10), reg("y", 0, 10)),
            dup(reg("y", 0, 10), reg("z", 0, 10)),
        ];
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups.len(), 1);
        assert_eq!(
            places(&groups[0]),
            vec![("x".into(), 0), ("y".into(), 0), ("z".into(), 0)]
        );
    }

    #[test]
    fn trois_paires_completes_donnent_un_groupe_de_trois() {
        // A≈B, B≈C, A≈C (exactly what `detect_duplicates` produces for 3 copies).
        let dups = vec![
            dup(reg("f", 40, 177), reg("f", 232, 369)),
            dup(reg("f", 232, 369), reg("f", 424, 561)),
            dup(reg("f", 40, 177), reg("f", 424, 561)),
        ];
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups.len(), 1);
        assert_eq!(
            places(&groups[0]),
            vec![("f".into(), 40), ("f".into(), 232), ("f".into(), 424)]
        );
    }

    #[test]
    fn paires_independantes_donnent_deux_groupes() {
        let dups = vec![
            dup(reg("x", 0, 10), reg("y", 0, 10)),
            dup(reg("x", 100, 110), reg("y", 100, 110)),
        ];
        assert_eq!(group_duplicates(&dups, &[]).len(), 2);
    }

    #[test]
    fn bornes_legerement_differentes_sont_la_meme_instance() {
        // The same block in x, with slightly different bounds in two pairs.
        let dups = vec![
            dup(reg("x", 40, 177), reg("y", 40, 177)),
            dup(reg("x", 42, 180), reg("z", 10, 148)),
        ];
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].instances.len(), 3);
        let x = &groups[0].instances[0];
        assert_eq!((x.path.to_str(), x.tokens.clone()), (Some("x"), 40..180));
        assert_eq!(x.chars, 80..360);
    }

    #[test]
    fn recouvrement_faible_reste_deux_instances() {
        // motif périodique : 1469..1652 et 1575..1758 se recouvrent à 42 %
        let dups = vec![dup(reg("f", 1469, 1652), reg("f", 1575, 1758))];
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].instances.len(), 2);
    }

    #[test]
    fn les_deux_cotes_dune_meme_paire_ne_fusionnent_pas() {
        let dups = vec![dup(reg("f", 0, 100), reg("f", 10, 110))]; // 90% overlap.
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups[0].instances.len(), 2);
    }

    #[test]
    fn groupes_tries_du_plus_de_copies_au_moins() {
        let dups = vec![
            dup(reg("a", 0, 500), reg("b", 0, 500)),
            dup(reg("x", 0, 10), reg("y", 0, 10)),
            dup(reg("y", 0, 10), reg("z", 0, 10)),
        ];
        let groups = group_duplicates(&dups, &[]);
        assert_eq!(groups[0].instances.len(), 3);
        assert_eq!(groups[1].instances.len(), 2);
    }

    #[test]
    fn aucune_paire_aucun_groupe() {
        assert!(group_duplicates(&[], &[]).is_empty());
    }

    // ---------- line numbers ----------

    fn file_with_lines(path: &str, content: &str, numbers: &[usize]) -> SourceFile {
        SourceFile {
            path: PathBuf::from(path),
            content: content.to_string(),
            line_numbers: numbers.to_vec(),
        }
    }

    #[test]
    fn offsets_convertis_en_numeros_de_ligne_dorigine() {
        // Content: a\nb\nc\nd\n; original lines 1, 4, 5, 9 (comments were removed in between).
        let f = file_with_lines("f", "a\nb\nc\nd\n", &[1, 4, 5, 9]);
        let index = LineIndex::new(&f);
        assert_eq!(index.lines(&(0..1)), Some(1..=1));
        assert_eq!(index.lines(&(2..4)), Some(4..=4)); // "b\n"
        assert_eq!(index.lines(&(2..6)), Some(4..=5)); // "b\nc\n"
        assert_eq!(index.lines(&(0..8)), Some(1..=9));
        assert_eq!(index.lines(&(6..7)), Some(9..=9));
    }

    #[test]
    fn instance_sans_fichier_fourni_na_pas_de_lignes() {
        let dups = vec![dup(reg("x", 0, 10), reg("y", 0, 10))];
        let groups = group_duplicates(&dups, &[]);
        assert!(groups[0].instances.iter().all(|i| i.lines.is_none()));
    }

    #[test]
    fn group_duplicates_remplit_les_lignes() {
        let x = file_with_lines("x", "a\nb\nc\nd\n", &[1, 4, 5, 9]);
        let y = file_with_lines("y", "a\nb\nc\nd\n", &[2, 3, 4, 5]);
        let dups = vec![dup(
            Region {
                path: "x".into(),
                tokens: 0..3,
                chars: 2..6,
            },
            Region {
                path: "y".into(),
                tokens: 0..3,
                chars: 0..4,
            },
        )];
        let groups = group_duplicates(&dups, &[x, y]);
        assert_eq!(groups[0].instances[0].lines, Some(4..=5));
        assert_eq!(groups[0].instances[1].lines, Some(2..=3));
    }

    // ---------- end-to-end with line numbers ----------

    const FILE_A: &str = "\
// Alpha module
struct Alpha { id: u32 }

fn total(items: &[Item]) -> u64 {
    let mut total = 0;
    for item in items {
        if item.active {
            total += item.price * item.qty;
        }
    }
    total
}
";

    const FILE_B: &str = "\
// Beta module, unrelated header

enum Beta { One, Two }
fn pick(b: Beta) -> u8 { match b { Beta::One => 1, Beta::Two => 2 } }

// the same function, with different comments and blank lines
fn total(items: &[Item]) -> u64 {
    let mut total = 0;
    for item in items {
        // only active items count

        if item.active {
            total += item.price * item.qty;
        }
    }
    total
}
";

    #[test]
    fn e2e_groupe_avec_numeros_de_ligne_dorigine() {
        let files = vec![
            SourceFile::from_text(PathBuf::from("a.rs"), FILE_A),
            SourceFile::from_text(PathBuf::from("b.rs"), FILE_B),
        ];
        let r = train_corpus(&files, E2E_VOCAB, 2);
        let dups = detect_duplicates(&r, &params());
        let groups = group_duplicates(&dups, &files);

        assert_eq!(groups.len(), 1, "{groups:?}");
        let g = &groups[0];
        assert_eq!(g.instances.len(), 2);
        assert_eq!(g.instances[0].path, PathBuf::from("a.rs"));
        assert_eq!(g.instances[1].path, PathBuf::from("b.rs"));

        // `fn total` spans lines 4..=12 in a.rs and 7..=17 in b.rs. The shared text starts
        // at the `}` ending the previous line (lines 2 and 4), which explains the margin.
        let la = g.instances[0].lines.clone().unwrap();
        let lb = g.instances[1].lines.clone().unwrap();
        assert!(*la.start() >= 2 && *la.end() <= 12, "{la:?}");
        assert!(*lb.start() >= 4 && *lb.end() <= 17, "{lb:?}");
        // The region also covers most of the function.
        assert!(la.end() - la.start() >= 6, "{la:?}");
        assert!(lb.end() - lb.start() >= 6, "{lb:?}");
    }
}

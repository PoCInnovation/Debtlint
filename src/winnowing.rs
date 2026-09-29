use crate::tokenizer::{BpeTrainingResult, Token};
use std::collections::VecDeque;
use std::path::PathBuf;

/// Selected fingerprint: (hash value, shingle index in the file).
pub type Fingerprint = (u64, usize);

#[inline]
fn splitmix64(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

pub fn hash_shingle(shingle: &[Token]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;

    for &t in shingle {
        h ^= splitmix64(t as u64 ^ 0x9e37_79b9_7f4a_7c15);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    splitmix64(h)
}

/// First Step: compute the hash of each shingle of size k (m = n - k + 1 values).
pub fn hash_shingles(tokens: &[Token], k: usize) -> Vec<u64> {
    if tokens.len() < k {
        return Vec::new();
    }
    tokens.windows(k).map(hash_shingle).collect()
}

/// Second and Third Steps: sliding window of size w on H, the sequence of shingle hashes
/// (one hash per shingle); select the rightmost minimum in case of equality, without
/// consecutive duplicates. Complexity O(m) via a monotonic deque.
pub fn winnow_hashes(hashes: &[u64], w: usize) -> Vec<Fingerprint> {
    let mut selected: Vec<Fingerprint> = Vec::new();
    let m = hashes.len();
    let w = w.min(m);
    let mut min_candidates: VecDeque<usize> = VecDeque::new();

    if m == 0 {
        return selected;
    }
    for i in 0..m {
        while let Some(&back) = min_candidates.back()
            && hashes[back] >= hashes[i]
        {
            min_candidates.pop_back();
        }
        min_candidates.push_back(i);
        if i + 1 >= w {
            let start = i + 1 - w;
            while let Some(&front) = min_candidates.front()
                && front < start
            {
                min_candidates.pop_front();
            }
            let pos = *min_candidates
                .front()
                .expect("The winnowing deque should never be empty here");
            let fp = (hashes[pos], pos);
            if selected.last() != Some(&fp) {
                selected.push(fp);
            }
        }
    }
    selected
}

pub fn winnow(tokens: &[Token], k: usize, w: usize) -> Vec<Fingerprint> {
    winnow_hashes(&hash_shingles(tokens, k), w)
}

/// Signature of each file in the BPE-encoded corpus.
/// Winnowing is applied file by file: a shingle never crosses a file boundary.
pub fn winnow_corpus(
    result: &BpeTrainingResult,
    k: usize,
    w: usize,
) -> Vec<(PathBuf, Vec<Fingerprint>)> {
    result
        .files
        .iter()
        .map(|f| (f.path.clone(), winnow(&f.sequence, k, w)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_du_cahier_des_charges() {
        let h = [77, 42, 19, 22, 19, 31, 45];
        assert_eq!(winnow_hashes(&h, 3), vec![(19, 2), (19, 4)]);
    }

    #[test]
    fn egalites_prend_le_plus_a_droite() {
        assert_eq!(winnow_hashes(&[5, 5, 5, 5], 3), vec![(5, 2), (5, 3)]);
    }

    #[test]
    fn brute_force_equivalent() {
        let tokens: Vec<u32> = (0..500u32).map(|i| (i * 7919 + i / 3) % 13).collect();
        let (k, w) = (4, 6);
        let h = hash_shingles(&tokens, k);
        let mut expected: Vec<Fingerprint> = Vec::new();
        for s in 0..=(h.len() - w) {
            let win = &h[s..s + w];
            let min = *win.iter().min().unwrap();
            let pos = s + win.iter().rposition(|&x| x == min).unwrap();
            if expected.last() != Some(&(min, pos)) {
                expected.push((min, pos));
            }
        }
        assert_eq!(winnow_hashes(&h, w), expected);
    }

    #[test]
    fn cas_limites() {
        assert!(winnow(&[1, 2], 3, 4).is_empty());
        assert_eq!(winnow_hashes(&[9, 3, 7], 10), vec![(3, 1)]);
    }

    #[test]
    fn code_duplique_partage_des_empreintes() {
        let a: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut b = vec![99, 98, 97];
        b.extend(&a);
        let fa: std::collections::HashSet<u64> = winnow(&a, 3, 4).iter().map(|f| f.0).collect();
        let fb: std::collections::HashSet<u64> = winnow(&b, 3, 4).iter().map(|f| f.0).collect();
        assert!(fa.intersection(&fb).count() > 0);
    }
}

//! Winnowing (Schleimer, Wilkerson, Aiken 2003) appliqué à un flux de tokens BPE.
//! Pipeline : tokens -> shingles (k-grams) -> hachages -> fenêtre glissante (w) -> empreintes.

use crate::tokenizer::{BpeTrainingResult, Token};
use std::collections::VecDeque;
use std::path::PathBuf;

/// Empreinte sélectionnée : (valeur du hash, index du shingle dans le fichier).
pub type Fingerprint = (u64, usize);

/// Finaliseur splitmix64 : casse toute corrélation avec les IDs BPE bruts.
#[inline]
fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Hash déterministe (stable entre exécutions/versions, contrairement à DefaultHasher)
/// d'un shingle : FNV-1a sur chaque token, puis finaliseur pour l'uniformité.
pub fn hash_shingle(shingle: &[Token]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &t in shingle {
        h ^= mix(t as u64 ^ 0x9e37_79b9_7f4a_7c15);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    mix(h)
}

/// Étape 1 : H = hash de chaque shingle de taille k (m = n - k + 1 valeurs).
pub fn hash_shingles(tokens: &[Token], k: usize) -> Vec<u64> {
    assert!(k > 0, "k doit être > 0");
    if tokens.len() < k {
        return Vec::new();
    }
    tokens.windows(k).map(hash_shingle).collect()
}

/// Étapes 2-3 : fenêtre glissante de taille w sur H, minimum le plus à droite en cas
/// d'égalité, sans doublon consécutif. Complexité O(m) via une deque monotone.
pub fn winnow_hashes(hashes: &[u64], w: usize) -> Vec<Fingerprint> {
    assert!(w > 0, "w doit être > 0");
    let m = hashes.len();
    let mut selected: Vec<Fingerprint> = Vec::new();
    if m == 0 {
        return selected;
    }
    // Si m < w, il n'y a aucune fenêtre complète : on traite tout H comme une seule fenêtre.
    let w = w.min(m);

    // Deque d'indices dont les hashes sont strictement croissants ; le front est le minimum.
    // On retire les éléments >= au nouveau : à égalité, le plus à droite survit.
    let mut dq: VecDeque<usize> = VecDeque::new();

    for i in 0..m {
        while let Some(&back) = dq.back() {
            if hashes[back] >= hashes[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);

        // Fenêtre courante : [i+1-w, i] ; on la traite dès qu'elle est complète.
        if i + 1 >= w {
            let start = i + 1 - w;
            while let Some(&front) = dq.front() {
                if front < start {
                    dq.pop_front();
                } else {
                    break;
                }
            }
            let pos = *dq.front().expect("la fenêtre n'est jamais vide");
            let fp = (hashes[pos], pos);
            if selected.last() != Some(&fp) {
                selected.push(fp);
            }
        }
    }
    selected
}

/// Point d'entrée : tokens BPE -> signature du fichier.
pub fn winnow(tokens: &[Token], k: usize, w: usize) -> Vec<Fingerprint> {
    winnow_hashes(&hash_shingles(tokens, k), w)
}

/// Signature de chaque fichier du corpus encodé par le BPE.
/// Le Winnowing est appliqué fichier par fichier : un shingle ne traverse jamais
/// la frontière entre deux fichiers.
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

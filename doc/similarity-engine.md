# Similarity Engine — idée et logique

Ce document décrit la **suite logique après le BPE** : comment passer des merges + occurrences à un rapport de similarités (duplications / near-duplicates).

Voir aussi : [bpe.md](./bpe.md), [hyperparams.md](./hyperparams.md), [bpe-vs-similarity.md](./bpe-vs-similarity.md) (candidats ≠ score).

---

## Objectif

Le BPE répond à : **quels patterns reviennent, et où ?**

Le similarity engine répond à : **ces occurrences sont-elles vraiment des duplications ? à quel point ?**

```
BPE (fait)
  vocabulary + sequences + FileOccurrences
        ↓
Similarity Engine (à faire)
  candidats → fenêtres → score % → rapport
```

---

## Vue d’ensemble du pipeline

```
1. Candidats
   Prendre les merges intéressants (pas les symboles de base)
   Regarder leurs FileOccurrences (path + offsets)

2. Fenêtres
   Autour de chaque offset, découper une fenêtre de tokens
   dans la séquence BPE du fichier

3. Comparaison
   Comparer les fenêtres deux à deux (même merge / proches)

4. Scoring
   Distance sur la fenêtre de tokens (idéalement pondérée par taille) → pourcentage
   (pas un simple ratio de merges BPE — voir bpe-vs-similarity.md)

5. Rapport
   Classer les hits par bande de score + afficher locations
```

---

## 1. Quels tokens on regarde ?

### Pas les symboles de base

Les tokens `0..96` (`a`, `{`, espace, etc.) sont trop génériques.  
Comparer sur `'a'` ou `'('` ne détecte rien d’utile.

### On regarde les **merges**

Un merge = pattern appris, plus riche :

```
Symbol('i'), Symbol('m')  →  trop petit
Merge(pair = (i, m))      →  "im"  (début de pattern)
Merge plus profond         →  blocs plus longs ("fn add", "a + b", …)
```

### Filtre des candidats

On ne garde que les merges qui :

| Critère | Exemple |
|---------|---------|
| Fréquence ≥ seuil | apparaît ≥ 2 fois dans le corpus |
| Occurrences multi-locations | ≥ 2 offsets (même fichier ou fichiers différents) |
| (optionnel) profondeur / taille | merge “assez gros” pour être un vrai bloc |

Les `FileOccurrences` du vocabulaire donnent déjà :

```
Merge {
  pair: (105, 109),
  occurrences: [
    { path: "src/a.rs", offsets: [42, 128] },
    { path: "src/b.rs", offsets: [10] }
  ]
}
```

→ 3 locations candidates pour ce pattern.

---

## 2. Comment on définit la fenêtre ?

Un **offset** pointe sur un token dans la séquence encodée du fichier.  
Seul, c’est trop petit. On élargit autour.

### Fenêtre fixe (MVP)

Hyperparamètre : `window_size` (en **tokens**, pas en caractères).

```
séquence du fichier :
[ t0 t1 t2 t3 t4 t5 t6 t7 t8 t9 t10 ... ]
              ↑
           offset = 5

window_size = 8  →  4 tokens avant + 4 tokens après (ou asymétrique)

fenêtre = tokens[offset - half .. offset + half]
bornée au début / fin du fichier
```

Exemple avec `window_size = 8` :

```
offset = 5, half = 4
start = max(0, 5 - 4) = 1
end   = min(len, 5 + 4) = 9

fenêtre = sequence[1..9]
```

### Pourquoi en tokens BPE ?

- On compare la **structure** déjà compressée par le BPE
- Moins sensible aux petits détails de formatting qu’une comparaison caractère par caractère
- Aligné avec le reste du pipeline Debtlint

### Variantes plus tard (pas MVP)

| Variante | Idée |
|----------|------|
| Fenêtre adaptative | s’étendre jusqu’à un délimiteur générique (`}`, ligne vide…) sans parser le langage |
| Fenêtre asymétrique | plus de tokens après l’offset |
| Fenêtre multi-échelle | tester 16 / 32 / 64 et garder le meilleur score |

Pour le PoC : **fenêtre fixe en tokens** suffit.

---

## 3. Génération des paires à comparer

Pour un merge donné avec N locations :

```
locations = [L1, L2, L3]
paires    = (L1,L2), (L1,L3), (L2,L3)
```

Pour chaque paire :

1. Extraire fenêtre A autour de L1
2. Extraire fenêtre B autour de L2
3. Calculer le score de similarité

On ne fait **pas** “tous les fichiers × toutes les fenêtres”.  
On ne compare que les fenêtres liées au **même merge candidat**.

```
❌ lent :   corpus × corpus × Levenshtein
✅ rapide : merges filtrés → occurrences → paires → Levenshtein
```

---

## 4. Scoring — Levenshtein sur tokens

### Distance

Sur deux séquences de tokens `A` et `B` :

```
distance = nombre min d’éditions (insert / delete / replace)
           pour transformer A en B
```

Exemple :

```
A = [10, 20, 30, 40, 50]
B = [10, 20, 99, 40, 50]

distance = 1  (un replace : 30 → 99)
```

### Pourcentage de similarité

```
score = 1.0 - (distance / max(len(A), len(B)))
```

Exemples :

| A / B | distance | max len | score |
|-------|----------|---------|-------|
| Identiques | 0 | 20 | **100%** |
| 2 tokens différents sur 20 | 2 | 20 | **90%** |
| 6 tokens différents sur 20 | 6 | 20 | **70%** |
| 8 tokens différents sur 20 | 8 | 20 | **60%** |
| Très différents | 18 | 20 | **10%** |

### Structure d’un hit

```rust
struct SimilarityHit {
    left: Location,    // path + offset (+ fenêtre)
    right: Location,
    score: f32,        // 0.0 .. 1.0
    merge_token: u32,  // merge qui a déclenché le candidat
    window_size: usize,
}

struct Location {
    path: PathBuf,
    offset: usize,
}
```

---

## 5. Bandes de score → rapport

Après scoring, on classe les hits :

| Bande | Score | Signification | Action typique |
|-------|-------|---------------|----------------|
| **Critique** | **90–100%** | Duplication quasi exacte | Extraire une fonction / utilitaire commun |
| **Élevée** | **70–90%** | Near-duplicate clair | Refactor probable, vérifier manuellement |
| **Moyenne** | **60–70%** | Similarité structurelle | Signal faible / à confirmer |
| Ignoré | **&lt; 60%** | Trop différent | Hors rapport (bruit) |

Seuil CLI suggéré (aligné README) : `--min-score 0.75`  
→ on n’affiche que **Élevée + Critique** par défaut.

### Exemple de rapport (terminal)

```
Debtlint similarity report
==========================
window_size: 32 tokens
min_score:   0.75
hits:        4

[CRITIQUE] 98%
  src/math.rs:42
  src/utils.rs:10
  merge: #150  window: 32 tokens

[CRITIQUE] 94%
  src/math.rs:128
  src/legacy/math.rs:55
  merge: #150  window: 32 tokens

[ELEVEE] 81%
  src/a.rs:20
  src/b.rs:88
  merge: #203  window: 32 tokens

[ELEVEE] 76%
  src/parser.rs:3
  src/parser.rs:140
  merge: #188  window: 32 tokens
```

### Exemple de rapport (JSON)

```json
{
  "window_size": 32,
  "min_score": 0.75,
  "hits": [
    {
      "band": "critique",
      "score": 0.98,
      "left":  { "path": "src/math.rs", "offset": 42 },
      "right": { "path": "src/utils.rs", "offset": 10 },
      "merge_token": 150
    }
  ]
}
```

---

## 6. Hyperparamètres du engine

| Paramètre | Défaut suggéré | Rôle |
|-----------|----------------|------|
| `window_size` | `32` | Taille de la fenêtre en tokens |
| `min_score` | `0.75` | Seuil d’affichage (ignore &lt; 75%) |
| `min_merge_occurrences` | `2` | Un merge doit apparaître ≥ 2 fois pour être candidat |
| `min_frequency` (BPE) | `2` | Déjà existant — alimente la qualité des merges |

Voir aussi [hyperparams.md](./hyperparams.md) pour `vocab_size` / `min_frequency`.

---

## 7. Exemple concret de bout en bout

Corpus :

```
fichier A:  fn add(a: i32, b: i32) -> i32 { a + b }
fichier B:  fn add(x: i32, y: i32) -> i32 { x + y }
```

1. **BPE** apprend des merges sur la structure commune (`fn`, `add`, `(`, types, `+`, …)
2. Un merge apparaît aux offsets `oA` (fichier A) et `oB` (fichier B)
3. **Fenêtres** de 32 tokens autour de `oA` et `oB`
4. Les fenêtres sont proches : noms `a/b` vs `x/y` changent peu en tokens BPE
5. **Levenshtein** → score ~ **85–95%**
6. **Rapport** → bande Critique ou Élevée, avec les 2 locations

---

## 8. Ordre d’implémentation suggéré

1. Module `engine` : types `Location`, `SimilarityHit`, bandes de score
2. Extraire fenêtres depuis `FileTokens.sequence` + offsets
3. Générer paires de candidats depuis merges filtrés
4. Implémenter Levenshtein sur `&[Token]`
5. Filtrer par `min_score` + classer en bandes
6. Afficher rapport terminal (+ JSON plus tard)
7. Brancher dans le pipeline / CLI (`--window-size`, `--min-score`)

---

## Résumé en une phrase

> On part des **merges BPE** (patterns fréquents + où ils apparaissent), on découpe une **fenêtre de tokens** autour de chaque offset, on compare ces fenêtres avec **Levenshtein**, on obtient un **% de similarité**, puis on publie un **rapport** classé 90–100 / 70–90 / 60–70.

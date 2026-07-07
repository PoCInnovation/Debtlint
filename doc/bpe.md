# BPE — fonctionnement

## Principe

1. Chaque caractère du corpus devient un token initial
2. On compte les paires de tokens adjacents les plus fréquentes
3. La paire la plus fréquente (≥ `min_frequency`) devient un nouveau token (merge)
4. On remplace toutes ses occurrences dans le corpus
5. On répète jusqu'à atteindre `vocab_size` ou qu'il n'y ait plus de paires valides

Résultat : des tokens qui représentent des patterns récurrents

## Entrée / sortie

**Entrée** : `&[SourceFile]` — path + contenu de chaque fichier.

**Sortie** : `BpeTrainingResult`
- `vocabulary` — symboles + merges appris
- `files` — séquences de tokens encodées par fichier
- `merges` — nombre de merges effectués

## Deux modes

### 1. Training (vocab non chargé)

```
SourceFile[] → train_corpus() → BpeTrainingResult
```

Le BPE tourne from scratch. Optionnel : `--save-vocab` pour persister le résultat.

### 2. Vocab pré-chargé

```
SourceFile[] + vocab.json → encode_corpus() → BpeTrainingResult
```

Pas de training. On recharge les merges depuis le JSON et on ré-applique les remplacements sur le corpus.

Utile pour éviter de refaire un BPE coûteux entre deux runs.

## Persistance

Quand on sauve un vocabulaire (`save_vocabulary`), on écrit un fichier JSON.  
Ce n'est **pas** tout le struct `Vocabulary` en mémoire — seulement un sous-ensemble appelé `VocabularyExport`.

### Ce qu'il y a dans le JSON

```json
{
  "format_version": 1,
  "merge_start_id": 97,
  "entries": [
    { "Symbol": "a" },
    { "Symbol": "b" },
    ...
    {
      "Merge": {
        "pair": [105, 109],
        "occurrences": [{ "path": "src/main.rs", "offsets": [42, 128] }]
      }
    }
  ]
}
```

| Champ | Rôle |
|-------|------|
| `format_version` | Version du format de fichier. Permet de lire d'anciens fichiers plus tard si le format évolue. |
| `entries` | **Source de vérité** — liste ordonnée de tous les tokens (symboles + merges). L'id d'un token = son index dans ce tableau (0, 1, 2…). |
| `merge_start_id` | Index où commencent les merges (tout avant = symboles, tout après = merges). |

### Ce qu'on ne sauve **pas** : `char_to_id`

En mémoire, `Vocabulary` ressemble à :

```
Vocabulary {
  entries: Vec<VocabularyEntry>,     ← sauvé dans le JSON
  merge_start_id: Token,             ← sauvé dans le JSON
  char_to_id: HashMap<char, Token>,   ← PAS sauvé
}
```

`char_to_id` est un **cache** : une table `{ 'a' → 0, 'b' → 1, ... }` pour retrouver vite le token d'un caractère lors de l'encodage.

On peut le **recalculer** à partir de `entries` en parcourant la liste :

```
entries[0] = Symbol('a')  →  char_to_id['a'] = 0
entries[1] = Symbol('b')  →  char_to_id['b'] = 1
entries[97] = Merge(...)  →  ignoré (pas un caractère)
```

C'est ce que fait `rebuild_char_index()` au moment du `load_vocabulary`.

**Pourquoi ne pas le sauver ?**
- Redondant → risque d'incohérence si `entries` et `char_to_id` divergent
- Fichier plus gros pour rien
- Recalcul très rapide (quelques centaines d'entrées max)

### Flux save / load

```
SAVE:
  Vocabulary   →  to_export()  →  JSON fichier

LOAD:
  JSON fichier  →  from_export()  →  Vocabulary
                                      └─ rebuild_char_index() recrée char_to_id
```

## Point d'entrée lib

La fonction centrale est `run_bpe` dans `pipeline.rs`.
Elle choisit entre **training** et **vocab pré-chargé** via le 3ᵉ argument : `load_vocab: Option<&Path>`.

## Les 2 chemins possibles

| Valeur | Signification |
|--------|---------------|
| `None` | pas de vocab à charger → on **entraîne** le BPE |
| `Some(path)` | chemin vers un `.vocab.json` → on **charge** le vocab et on encode seulement |

```rust
use debtlint::pipeline::{run_bpe, BpeConfig};

// Mode 1 : training from scratch
let result = run_bpe(
    &files,
    BpeConfig { vocab_size: 1000, min_frequency: 2 },
    None,  // ← pas de fichier vocab → train_corpus()
)?;

// Mode 2 : vocab déjà sauvé
let result = run_bpe(
    &files,
    BpeConfig { vocab_size: 1000, min_frequency: 2 },
    Some(path),  // ← charge le JSON → encode_corpus()
)?;
```

En mode 2, `vocab_size` et `min_frequency` sont ignorés (le vocab est déjà entraîné).

## Architecture des fichiers

```
src/
├── lib.rs           ← expose pipeline, tokenizer, in_out
├── pipeline.rs      ← logique BPE réutilisable (LIB)
├── cli.rs           ← définition des flags terminal (BINARY)
├── debug_run.rs     ← harness de test temporaire (BINARY)
└── main.rs          ← point d'entrée minimal (BINARY)
```

### `pipeline.rs` — le cœur réutilisable

- **Où** : lib (`debtlint::pipeline`)
- **Pour qui** : Loan, tests, n'importe quel caller
- **Fait** : reçoit `&[SourceFile]` + config + `Option<Path>`, retourne `BpeTrainingResult`

C'est **la seule partie obligatoire** à garder meme quand le main de Loan arrivera.

### `cli.rs` — définition des arguments terminal

- **Où** : binaire seulement (pas dans la lib)
- **Fait** : décrit les flags (`--vocab-size`, `--load-vocab`, etc.) via clap

Équivalent de « quels paramètres l'utilisateur peut passer en ligne de commande ».

### `debug_run.rs` — harness de dev temporaire

- **Où** : binaire seulement
- **Fait** :
  1. Lit **un** fichier depuis le disque
  2. Appelle `run_bpe`
  3. Affiche les stats (compression, merges…)
  4. Sauve le vocab si `--save-vocab`
  5. Vérifie que decode(encode(text)) == text
  6. Écrit le JSON des tokens encodés
- **Sera remplacé** par le main de Loan

C'est du **test/debug** — pas de la logique métier.

## Résumé

| Fichier | Couche | Garder avec Loan ? |
|---------|--------|---------------------|
| `pipeline.rs` | Lib — API BPE | **Oui** |
| `tokenizer/*`, `in_out/*` | Lib — algo + I/O | **Oui** |
| `cli.rs` | Binaire — flags | Non (Loan aura son propre CLI) |
| `debug_run.rs` | Binaire — test | Non (remplacé) |
| `main.rs` | Binaire — entry | Non (remplacé) |


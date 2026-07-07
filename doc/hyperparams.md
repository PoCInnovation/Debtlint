# Hyperparamètres BPE

| Paramètre | Défaut | Rôle |
|-----------|--------|------|
| `vocab_size` | `1000` | Taille max du vocabulaire (symboles de base + symboles extra + merges) |
| `min_frequency` | `2` | Fréquence min d'une paire de tokens pour qu'un merge soit créé |

## `vocab_size`

Le vocabulaire contient **97 symboles fixes** (alphabet de base). Les merges s'arrêtent quand `vocabulary.len() >= vocab_size`.

- Trop bas (≤ 97) → aucun merge
- Plus haut → plus de patterns fusionnés, training plus long

## `min_frequency`

Une paire doit apparaître au moins `min_frequency` fois dans le corpus pour être mergée.

- `1` → merge même les paires rares (bruit)
- `2+` → ignore les coïncidences isolées

## API

```rust
run_bpe(&files, BpeConfig { vocab_size, min_frequency }, None)
```

Voir [bpe.md](./bpe.md) pour le flux complet.

# Tester Debtlint en local

## Lancer

```bash
cargo run -- <FILE>
```

Exemples avec les fixtures du repo :

```bash
cargo run -- fixtures/sample.rs
cargo run -- fixtures/unicode.txt
```

## Options

| Flag               | Défaut                | Description                                           |
| --------------------| -----------------------| -------------------------------------------------------|
| `FILE`             | *(obligatoire)*       | Fichier source à encoder                              |
| `--vocab-size`     | `1000`                | Taille max du vocabulaire (base + dynamique + merges) |
| `--min-frequency`  | `2`                   | Fréquence min d'une paire pour déclencher un merge    |
| `--output-encoded` | `<FILE>.encoded.json` | Chemin du JSON de la séquence encodée                 |

## Exemples utiles

**Rust + merges visibles :**

```bash
cargo run -- fixtures/sample.rs --vocab-size 500 --min-frequency 2
```

**Unicode (`é`, emoji) :**

```bash
cargo run -- fixtures/unicode.txt --vocab-size 200
```

**JSON de sortie dans le repo :**

```bash
cargo run -- fixtures/sample.rs --output-encoded fixtures/sample.encoded.json
```

## Sortie attendue

Le terminal affiche notamment :

- `merges performed` — nombre de merges BPE
- `vocabulary size` — `97 fixed + N dynamic + M merged`
- `decoded == content ok: true` — round-trip encode/decode OK
- `encoded sequence written to: ...` — chemin du JSON

Format JSON : `[135, 135, 105, 109, 109, 64]`

## Piège fréquent

`--vocab-size` doit être **> 97** (taille de la base fixe), sinon **aucun merge** ne se lance.

```bash
# ❌ 0 merge
cargo run -- fixtures/sample.rs --vocab-size 50

# ✅ merges OK
cargo run -- fixtures/sample.rs --vocab-size 200
```

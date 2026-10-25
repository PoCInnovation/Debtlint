---
name: debtlint
description: >
  Detect duplicated code with the debtlint CLI, a language-agnostic scanner.
  Use after generating or editing code. Run with
  --report-format json and refactor each reported duplicate group.
license: Apache-2.0
allowed-tools: Bash(cargo install debtlint), Bash(debtlint:*), Bash(cargo run:*), Bash(git add:*)
metadata:
  author: POC Innovation
  version: "1.0.0"
  website: "https://poc-innovation.fr"
---

# Debtlint

Language-agnostic duplicate detection. It does not parse a language: it normalizes text, encodes it with BPE, and finds repeated token spans with winnowing. The same command works for every language in the tree.

## Install

Use `debtlint` when it is on `PATH`. Otherwise:

```sh
cargo install debtlint
```

## Prepare

Run from the project root. There is no path argument. The scan root comes from `debtlint.config.json` in the current directory. If that file is missing, the process panics.

Create it only when it is absent. Do not overwrite an existing one.

```json
{
  "src": ".",
  "excludes": ["target", "node_modules", ".git", "dist", "build", "vendor"]
}
```

- `src`: directory to scan.
- `excludes`: directories skipped during ingestion. Missing entries are ignored.

In a git repository only tracked paths are read (`git ls-files`). Edits to tracked files are visible immediately. A new untracked file is invisible until it is staged. After creating files, stage those paths only:

```sh
git add -- path/to/new_file
```

Do not commit. Outside git, Debtlint walks `src` and skips dot-directories. NOTIFIES THE USER THAT FILES HAVE BEEN ADDED

## Scan

Stdout is the report. Parse stdout only. Exit code 0 with duplicates is a successful scan. A non-zero exit is a crash (missing config, invalid vocabulary JSON, I/O).

Reuse a vocabulary when `.debtlint-vocab.json` exists. `--load-vocab` and `--save-vocab` conflict.

```sh
debtlint --load-vocab .debtlint-vocab.json --report-format json
```

Otherwise train once and save the vocabulary. Treat that file as an opaque cache; do not edit it.

```sh
debtlint --save-vocab .debtlint-vocab.json --report-format json
```

Retrain (the `--save-vocab` command, without `--load-vocab`) only after a large corpus change. Leave `--vocab-size` (default 1000), `--min-frequency` (default 2), `--shingle-size` (default 50), `--window-size` (default 3), and `--min-tokens` (default 20) alone unless the user asks to change them.

## Output

`--report-format json` prints a pretty-printed array. `[]` means no duplicate. Each element is one group of copies of the same block, already sorted with the most repeated and largest groups first. `ranges` always has at least two entries.

```json
[
  {
    "ranges": [
      {
        "source": "src/foo.rs",
        "tokens": { "start": 10, "end": 40 },
        "chars": { "start": 120, "end": 480 },
        "lines": { "start": 12, "end": 28 }
      },
      {
        "source": "src/bar.rs",
        "tokens": { "start": 3, "end": 33 },
        "chars": { "start": 40, "end": 400 },
        "lines": { "start": 5, "end": 21 }
      }
    ]
  }
]
```

- `source`: file path.
- `lines`: 1-based inclusive line numbers in the original file. This is the span to edit. `null` means the line mapping failed; report that range to the user instead of guessing.
- `chars`: offsets in the normalized text, not the raw file.
- `tokens`: BPE token indexes, exclusive end. Ignore them when editing.

## Refactor

After generating or modifying code:

1. Run the scan command above.
2. If the array is non-empty, refactor the groups, largest first. Read every `source` at `lines.start`–`lines.end` and extract one shared function or module. Point every copy at it. A group can list several ranges in the same file.
3. Re-run the same command.
4. Stop when the array is `[]` or when the code cannot be refactored without damage the logic. Then report any remaining groups to the user (file, line span, copy count). Do not keep looping.
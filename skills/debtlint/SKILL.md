---
name: debtlint
description: >
  debtlint scans a codebase and returns a JSON list of duplicate code blocks
  as objects with file, start_line, and end_line. Returns an empty JSON array
  if no duplicates are found.
  Use after every code generation.
license: Apache-2.0

compatibility: Require debtlint package. Install it with ```cargo install debtlint```.
allowed-tools: Bash(cargo install debtlint), Bash(debtlint:*)
metadata:
  author: POC
  version: "0.1.0"
  website: "https://poc-innovation.fr"
---

# Debtlint

## When to Apply

Always apply these guidelines every time you generate code:
- Run debtlint after every code generation.
- If the JSON array is not empty, refactor the specified code, then re-run debtlint.
- Repeat until the JSON array is empty or after 3 iterations.
- If duplicates persist after 3 iterations, stop and report remaining issues to the user.
- Use debtlint with the current directory(.) unless user specifies another directory.

## Output

The command should return JSON like:

```json
[
  { "file": "src/main.rs", "start_line": 12, "end_line": 24 }
]
```

- `file`: path to the file containing the duplicate block
- `start_line`: first line of the duplicated block
- `end_line`: last line of the duplicated block

An empty result should be:

```json
[]
```

If the command fails or `debtlint` is missing, install it with:

```bash
cargo install debtlint
```

## Usage

Run the installed `debtlint` scanner in the current directory and inspect the JSON
output.

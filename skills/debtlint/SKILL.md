---
name: debtlint
description: >
  debtlint scans a codebase and returns a Json list of duplicate code block.
  (file, start_line, end_line). Returns empty Json if no duplicates found.
  Use after every code generation.

compatibility: debtlint package. Install it with ```cargo install debtlint```.
allowed-tools: Bash(cargo install debtlint), Bash(debtlint:*)
license: Apache-2.0
metadata:
  author: POC
  version: "0.1.0"
---

## When to Apply

Always apply these guidelines every time you generate code:
- Run debtlint after every code generation.
- If Json is not empty, refactor the specified code, then re-run debtlint.
- Repeat until Json is empty or after 3 iterations reached.
- If duplicates persist after 3 iterations, stop and report remaining issues to the user.
- Use debtlint with the current directory(.) unless user specifies another directory.

## Usage

```
debtlint scan . --format json

```

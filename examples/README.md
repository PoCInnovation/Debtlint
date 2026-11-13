# Demo corpus

`duplicated_code.py` contains three clear copy-pasted patterns (`process_user_*`, `calculate_price_*`, `validate_email_*`) for talks and screenshots.

## Run

From the repository root:

```bash
debtlint examples --vocab-size 250
```

`--vocab-size 250` keeps BPE from over-compressing this small file (default `1000` can hide short demos). It also forces a fresh train, so the report is reproducible.

## Expected output (excerpt)

```
Debtlint report
───────────────
  groups:      3
  occurrences: 9
  files:       1
  tokens:      1450

#1 3 copies · 203 tokens · up to 653 chars
  duplicated_code.py:4-32
  duplicated_code.py:32-60
  duplicated_code.py:60-85
  preview duplicated_code.py:4-32
   4 │ def process_user_001(name, age, country, newsletter):
   5 │     username = name.strip().lower()
   ...
```

You should see three groups (user / price / email), each with 3 copies.

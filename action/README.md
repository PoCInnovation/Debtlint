# Debtlint Action

GitHub Action that runs [Debtlint](https://github.com/PoCInnovation/Debtlint) on pull requests and comments duplicated or near-duplicate code segments on the PR.

The action metadata lives at the repository root (`action.yml`).

## Requirements

- A `debtlint.config.json` at the root of the checked-out repository (see the [main README](../README.md#quick-start)).
- The workflow must checkout the repository before calling this action.
- `pull_request` events (the action posts review comments on the PR).

## Inputs

- **`vocab-path`** (optional): path to a Debtlint vocabulary JSON. When set, Debtlint runs with `--load-vocab` and skips BPE training.

## Permissions

The default `GITHUB_TOKEN` is enough to post pull request comments. Explicit permissions:

```yaml
permissions:
  contents: read
  pull-requests: write
```

## Example workflow

```yaml
name: Check duplication

on:
  pull_request:
    branches: [main]

jobs:
  debtlint:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write

    steps:
      - uses: actions/checkout@v4

      - name: Run debtlint
        uses: PoCInnovation/Debtlint@v1
        with:
          vocab-path: .debtlint-vocab.json
```

## License

Apache License 2.0 — see [LICENSE](../LICENSE).

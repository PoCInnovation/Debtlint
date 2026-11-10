# Excludes

Debtlint skips directories and file globs listed in `excludes` (plus whatever `.gitignore` already ignores).

Config file: `.debtlint/debtlint.config.json`.

## Important

Setting `"excludes"` in your config **replaces** the built-in list entirely. It does not merge with defaults. Copy the defaults below, then add or remove entries.

## Default excludes

These are the built-in values from `src/config.rs` when no config (or no `excludes` field) is present:

### Directories

| Pattern | Why |
|---------|-----|
| `target` | Rust build output |
| `node_modules` | npm dependencies |
| `vendor` | Vendored deps |
| `bower_components` | Bower deps |
| `jspm_packages` | jspm deps |
| `dist` | Build output |
| `build` | Build output |
| `.debtlint` | Debtlint cache / config |
| `.venv`, `venv` | Python virtualenvs |
| `__pycache__` | Python bytecode cache |
| `.pytest_cache`, `.mypy_cache`, `.ruff_cache` | Python tool caches |
| `.tox`, `.nox` | Python test envs |
| `.next`, `.nuxt`, `.output`, `.svelte-kit`, `.angular` | Frontend framework output |
| `.parcel-cache`, `.turbo`, `.vercel` | Bundler / deploy caches |
| `storybook-static` | Storybook build |
| `.cache` | Generic cache |
| `coverage`, `htmlcov` | Coverage reports |
| `.gradle` | Gradle |
| `Pods`, `Carthage`, `DerivedData` | iOS / Xcode |

### Globs

| Pattern | Why |
|---------|-----|
| `*.json`, `*.jsonl`, `*.yaml`, `*.yml` | Config / data |
| `*.ipynb` | Notebooks (often noisy) |
| `*.csv`, `*.tsv` | Data tables |
| `*.lock` | Lockfiles |
| `*.md`, `*.markdown`, `*.rst`, `*.adoc`, `*.txt` | Documentation / prose |
| `*.min.js`, `*.min.css`, `*.min.html` | Minified assets |
| `*.map` | Source maps |
| `*.log`, `*.pyc`, `*.pyo` | Logs / compiled Python |
| `*.svg`, `*.png`, `*.jpg`, `*.jpeg`, `*.gif`, `*.webp`, `*.ico` | Images |
| `*.pdf`, `*.parquet`, `*.db`, `*.sqlite`, `*.sqlite3` | Binary / datasets |
| `*.wasm`, `*.zip`, `*.gz` | Binaries / archives |

## Minimal override example

Keep most defaults, but **include** `.txt` files in the scan (common surprise: `*.txt` is excluded by default).

Create or edit `.debtlint/debtlint.config.json` and set a full `excludes` list **without** `*.txt`:

```json
{
  "dir": ".",
  "excludes": [
    "target",
    "node_modules",
    "vendor",
    "bower_components",
    "jspm_packages",
    "dist",
    "build",
    ".debtlint",
    ".venv",
    "venv",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".tox",
    ".nox",
    ".next",
    ".nuxt",
    ".output",
    ".svelte-kit",
    ".angular",
    ".parcel-cache",
    ".turbo",
    ".vercel",
    "storybook-static",
    ".cache",
    "coverage",
    "htmlcov",
    ".gradle",
    "Pods",
    "Carthage",
    "DerivedData",
    "*.json",
    "*.jsonl",
    "*.yaml",
    "*.yml",
    "*.ipynb",
    "*.csv",
    "*.tsv",
    "*.lock",
    "*.md",
    "*.markdown",
    "*.rst",
    "*.adoc",
    "*.min.js",
    "*.min.css",
    "*.min.html",
    "*.map",
    "*.log",
    "*.pyc",
    "*.pyo",
    "*.svg",
    "*.png",
    "*.jpg",
    "*.jpeg",
    "*.gif",
    "*.webp",
    "*.ico",
    "*.pdf",
    "*.parquet",
    "*.db",
    "*.sqlite",
    "*.sqlite3",
    "*.wasm",
    "*.zip",
    "*.gz"
  ]
}
```

> If your source of truth lives in `.txt` files, remove `*.txt` from `excludes` (as above). Same idea for `*.md` if you intentionally want Markdown scanned.

## Small custom project

Only skip build noise, scan everything else (including docs):

```json
{
  "dir": ".",
  "excludes": ["target", "node_modules", "dist", "build", ".debtlint"]
}
```

Remember: this **drops** all other default globs (`*.json`, `*.md`, images, …). That may be what you want on a tiny repo; on a large monorepo it can slow scans and add noise.

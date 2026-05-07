<p align="center">
  <img src=".github/assets/logo.png" alt="Debtlint logo" width="360" />
</p>

# Debtlint

**Debtlint** is a technical debt detection tool focused on duplicated logic, built for the era of AI code generation.
It automatically identifies repeated or near-duplicate code segments across a codebase, then provides concrete refactoring guidance.

## Vision

As coding assistants become mainstream, "silent" duplication appears quickly:
- existing utility functions get rewritten,
- business patterns are repeated,
- logic becomes fragmented across multiple files.

Debtlint provides a **programmatic, fast, and cost-efficient** way to prevent this drift and keep architecture healthy over time.

## Objectives

- Detect exact duplications and structural similarities.
- Prioritize technical debt hotspots with a confidence score.
- Provide actionable reports to guide refactoring.
- Integrate easily into team workflows (local, CI, agents).
- Stay as language-agnostic as possible.

## How It Works

The Debtlint pipeline follows four main steps:

1. **Code ingestion**
   Read the target codebase (source files while excluding irrelevant directories).

2. **Sanitization / normalization**
   Clean inputs (comments, surface-level formatting, token normalization) to preserve meaningful logic.

3. **BPE encoding and analysis**
   Use an approach inspired by **Byte Pair Encoding (BPE)** to identify frequent and repetitive patterns.
   The goal is to surface representative segments of recurring code structures at scale.

4. **Scoring and reporting**
   Generate human-readable and machine-readable reports: occurrences, similarity level, potential impact, and refactoring hints.

## Why BPE

BPE is historically a compression and tokenization technique, notably used to segment large amounts of text efficiently.
In Debtlint, this logic helps highlight repetitive structures and grammatical blocks of code.

Core reference: [Byte pair encoding - Wikipedia](https://en.wikipedia.org/wiki/Byte_pair_encoding)

## CLI

Debtlint is designed first as a **CLI**:
- run locally on a codebase,
- simple configuration (includes/excludes, thresholds),
- report output (terminal + CI-friendly formats).

Command example (target spec):

```bash
debtlint scan . --format json --min-score 0.75
```

## GitHub Actions Integration

The project is designed to run automatically in CI pipelines:
- analyze every pull request,
- publish a technical debt report,
- optionally fail the job based on configured thresholds.

Goal: make technical debt visible early, before it spreads into the main branch.

## MCP Integration

Debtlint can be exposed as an MCP tool for AI agents:
- an agent generates code,
- it queries Debtlint after generation,
- it detects existing abstractions or duplicated patterns,
- it updates its proposal to reduce technical debt.

This loop upgrades generation from "best effort" to codebase-aware generation.

## Contributing

Contributions are welcome.
Read the guide: [CONTRIBUTING.md](./CONTRIBUTING.md)

[Project presentation video](https://www.youtube.com/watch?v=-JjcPTZ7Omg&feature=youtu.be)

## Our PoC team ❤️

Developers
| [<img src="https://github.com/EnzoRuffino.png?size=85" width=85><br><sub>Enzo Ruffino</sub>](https://github.com/EnzoRuffino) | [<img src="https://github.com/skl1017.png?size=85" width=85><br><sub>Loan Riyanto</sub>](https://github.com/skl1017) | [<img src="https://github.com/Staney111.png?size=85" width=85><br><sub>Eythan EL QUALI</sub>](https://github.com/Staney111) | [<img src="https://github.com/Rayan-ouer.png?size=85" width=85><br><sub>Rayan</sub>](https://github.com/Rayan-ouer) |
| :---: | :---: | :---: | :---: |

Manager
| [<img src="https://github.com/lg-epitech.png?size=85" width=85><br><sub>Laurent Gonzalez</sub>](https://github.com/lg-epitech)
| :---: |

<h2 align=center>
Organization
</h2>

<p align='center'>
    <a href="https://www.linkedin.com/company/pocinnovation/mycompany/">
        <img src="https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="LinkedIn logo">
    </a>
    <a href="https://www.instagram.com/pocinnovation/">
        <img src="https://img.shields.io/badge/Instagram-E4405F?style=for-the-badge&logo=instagram&logoColor=white" alt="Instagram logo">
    </a>
    <a href="https://twitter.com/PoCInnovation">
        <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white" alt="Twitter logo">
    </a>
    <a href="https://discord.com/invite/Yqq2ADGDS7">
        <img src="https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white" alt="Discord logo">
    </a>
</p>
<p align=center>
    <a href="https://www.poc-innovation.fr/">
        <img src="https://img.shields.io/badge/WebSite-1a2b6d?style=for-the-badge&logo=GitHub Sponsors&logoColor=white" alt="Website logo">
    </a>
</p>

> 🚀 Don't hesitate to follow us on our different networks, and put a star 🌟 on `PoC's` repositories

> Made with ❤️ by PoC
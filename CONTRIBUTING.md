# Contributor Commands

Before each drafting session, run the contributor scripts below from the repository root. These commands are designed to keep prose quality checks and style alignment reminders in the same workflow.

## 1) Pre-draft reminders + quality run

Use this command at the start of each drafting session:

```bash
./scripts/pre-draft-session.sh [chapter-or-directory]
```

- `chapter-or-directory` defaults to `chapters` when omitted.
- The command prints:
  - 5 sampled narrative prose examples from `style-reference`.
  - 5 sampled dialogue examples from `style-reference`.
  - A full quality run across the same target via `quality-orchestrator`.
- The examples support the repo’s paragraph/dialogue rhythm workflow before you write.

Examples:

```bash
# Run all chapters in default style-reference and quality mode
./scripts/pre-draft-session.sh

# Run with one chapter
./scripts/pre-draft-session.sh chapters/chapter-01.md
```

## 2) Quality pipeline only

If you already completed style sampling and only want to run analyzers, use:

```bash
./scripts/run-quality-session.sh <chapter-or-directory> [quality-orchestrator flags...]
```

Examples:

```bash
./scripts/run-quality-session.sh chapters/chapter-01.md
./scripts/run-quality-session.sh chapters --fail-on blocker
./scripts/run-quality-session.sh chapters/chapter-03.md --no-artifact
```

## Notes

- The scripts call the workspace binaries through `cargo run`:
  - `quality-orchestrator` from `rust/crates/quality-orchestrator`
  - `style-reference` from `rust/crates/excerpt-extract`
- For local automation, these scripts require a working Rust toolchain with `cargo`.

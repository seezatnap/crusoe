# Infinite Domain — Chapter & Artifact Delivery Contract

## Scope and precedence

This contract is the authoritative routing rule for all narrative and analytical outputs in this repository.
When it conflicts with another instruction, this contract takes precedence for file-location and retention behavior.

## Chapter delivery contract

1. All chapter drafts must be stored under `chapters/`.
2. Allowed chapter filenames are exact and limited to:
   - `chapters/chapter-01.md` through `chapters/chapter-17.md` (two-digit chapter numbers).
3. Invalid chapter filenames are not permitted, including:
   - `chapter-00.md`, `chapter-18.md`, non-numeric variants, alternate extensions, or any alternate directory.
4. Draft lifecycle:
   - Create one file per chapter at its required final name.
   - Any edits replace the same chapter file in-place.
   - Do not create chapter variants, staging copies, or temporary chapter bundles.
5. One chapter per file:
   - Each draft file must contain one chapter only.
   - No combined multi-chapter compilations are allowed.
6. Canonical delivery:
   - The `chapters/chapter-XX.md` file for that chapter is the only source of canonical text for delivery.

## Analytical/artifact delivery contract

1. All analytical or tooling outputs must live under `writing-artifacts/`.
2. Required artifact subdirectories:
   - `writing-artifacts/drafts/`
   - `writing-artifacts/quality/`
   - `writing-artifacts/postmortems/`
   - `writing-artifacts/reports/`
   - `writing-artifacts/logs/`
3. Forbidden locations:
   - No analytical artifacts may be written outside `writing-artifacts/`.
   - No analytical artifacts may be written to `chapters/`.
4. Naming convention:
   - Artifacts should use `<category>/<chapter-or-topic>/<artifact-type>-<ISO8601-UTC>.md`.
   - Example: `writing-artifacts/reports/chapter-03/report-2026-02-13T12-00-00Z.md`.

## Update rules

1. Any chapter edit to `chapters/chapter-XX.md` must be paired with an artifact update.
2. Minimum required artifact updates after chapter work:
   - one entry in `writing-artifacts/logs/` describing the change event,
   - one `postmortem` or `quality` artifact if review/checks were run,
   - one `drafts/` snapshot when chapter text is produced by an automated pipeline.
3. Automated checks should write deterministic outputs to the corresponding chapter/topic directory.
4. If a required destination is missing, create it before writing outputs.

## Retention policy

1. Canonical chapter text (`chapters/chapter-XX.md`)
   - Keep indefinitely in Git history.
   - No repo-side automatic expiration policy applies to canonical chapter files.
2. Draft and quality artifacts (`writing-artifacts/drafts/`, `writing-artifacts/quality/`)
   - Keep at most the 10 latest versions per chapter/topic.
   - Delete artifacts older than 90 days regardless of count.
3. Postmortem and report artifacts (`writing-artifacts/postmortems/`, `writing-artifacts/reports/`)
   - Keep every completed milestone artifact.
   - Retain at least one final file per chapter/topic indefinitely unless explicitly pruned by governance.
4. Operational logs (`writing-artifacts/logs/`)
   - Keep at most 180 days.


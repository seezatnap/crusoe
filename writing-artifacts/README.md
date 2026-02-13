# Writing-Artifacts Workspace Conventions

This directory stores planning, continuity, and process documents required by the
team before drafting chapter prose.

## Status Enum

- `draft`
- `in_progress`
- `blocked`
- `ready_for_review`
- `revisions_needed`
- `approved`
- `archived`

Use this status enum for chapter-facing metadata and all chapter tracking rows in
`word-count.md`.

## Artifact ID System

- **Format:** `DOC-###` for project artifacts, `LOG-###` for tracking docs.
- IDs are mandatory in filenames and references.
- Reserved ID ranges:
  - `DOC-001` — World Design Document
  - `DOC-002` — Character Bible
  - `DOC-003` — High-Level Plot Outline
  - `DOC-004` — Chapter-by-Chapter Outline
  - `DOC-005` — Mystery Architecture
  - `DOC-006` — Timeline & Continuity Tracker
  - `DOC-007` — Theme & Motif Guide

## File Naming

- **Required pattern for artifacts:** `DOC-###-<slug>.md`
- **Tracking docs:** `LOG-###-<slug>.md`
- `<slug>` is a short kebab-case identifier.
- Examples:
  - `DOC-001-world-design.md`
  - `DOC-002-character-bible.md`
  - `LOG-001-word-count.md`

## Metadata Standard

Artifact files that represent planning documents should include:

```md
---
artifact_id: "DOC-001"
artifact_type: "planning"    # planning | continuity | process
title: "World Design Document"
status: "draft"             # draft | in_progress | blocked | ready_for_review | revisions_needed | approved | archived
depends_on:
  - "DOC-000"               # parent/precedent dependency if any
depends_on_note: "Dependency status and risk notes"
last_updated_utc: "2026-02-13T00:00:00Z"
owner: "Agent Aaron"
---
```

Tracking files (for example word counts, dependency notes, post-mortems, and handoffs)
must include:

```md
---
log_id: "LOG-001"
log_type: "word_count"      # word_count | dependencies | handoff | post_mortem
scope: "project-wide"       # chapter | project-wide
status: "draft"             # draft | in_progress | blocked | ready_for_review | revisions_needed | approved | archived
source_refs:
  - "DOC-001"
  - "DOC-002"
last_updated_utc: "2026-02-13T00:00:00Z"
---
```

## Dependency Notes

- Every planning artifact and chapter must list upstream dependencies.
- If a dependency blocks drafting, record it in `writing-artifacts/dependency-notes.md`.
- If the dependency changes, log the change in the next handoff.

## Recommended Artifact Handoff Order

1. World Design (`DOC-001`)  
2. Character Bible (`DOC-002`)  
3. Plot Outline (`DOC-003`)  
4. Chapter Outline (`DOC-004`)  
5. Mystery Architecture (`DOC-005`)  
6. Timeline (`DOC-006`)  
7. Theme Guide (`DOC-007`)  
8. Chapter Drafts (`CH-###-*`)

## Tracking and Handoff Convention

- Use `writing-artifacts/word-count.md` for chapter length governance.
- Use `writing-artifacts/dependency-notes.md` for cross-document constraints.
- Use `writing-artifacts/handoff-format.md` for every handoff between contributors.

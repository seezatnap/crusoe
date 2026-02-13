# Chapter Workspace Conventions

This directory stores all chapter prose drafts for the novel.

## Chapter ID

- **Format:** `CH-###`  
- **Range:** `CH-001` through `CH-025`
- IDs are immutable and must be referenced in artifacts, logs, and handoffs.

## Chapter File Naming

- **Required pattern:** `CH-###-<slug>.md`
- `###` must match `chapter_id` (zero-padded, 3 digits).
- `<slug>` is a short kebab-case summary (optional for drafts, required for `complete` chapters).
- Examples:
  - `CH-001-the-strange-request.md`
  - `CH-002-river-street-witness.md`

## Required Metadata

Each chapter file must begin with YAML frontmatter:

```md
---
chapter_id: "CH-001"
chapter_number: 1
title: "Chapter 1: The Strange Request"
slug: "the-strange-request"
status: "in_progress"
word_count_target_min: 4000
word_count_target_max: 6000
word_count_current: 0
pov: "Rosa"
primary_location_id: "LOC-001"
dependency_ids:
  - "DOC-001"  # world_design
  - "DOC-002"  # character_bible
  - "DOC-005"  # chapter_outline
depends_on_chapter_ids: []  # no predecessor (first chapter); do not use CH-000
dependencies_note:
  - "Uses Dr. Joe’s clinic setting and chapter-beat requirements."
word_count_updated_utc: "2026-02-13T00:00:00Z"
author: "Agent Aaron"
---
```

The allowed values for `status` are the canonical chapter status enum in
`writing-artifacts/README.md` under [Status Enum](#status-enum), currently:
`draft | blocked | in_progress | ready_for_review | revisions_needed | approved`.

## Content Template

After frontmatter, use this section order:

1. `## Hook`
2. `## Scene 1`
3. `## Scene 2`
4. `## Scene 3` (add as needed)
5. `## Clues & Revelations`
6. `## Continuity Notes`
7. `## Draft Log`

## Word Count Discipline

- Target per chapter: **4,000–6,000 words**.
- Before handoff, compute actual words with:
  - `wc -w chapters/CH-###-*.md`
- Update `word_count_current` and `word_count_updated_utc` to match.

## Chapter Dependency Rule

- A chapter is considered blocked until all listed `dependency_ids` are available and up-to-date.
- Any dependency conflict is recorded in `dependencies_note` and mirrored in
  `writing-artifacts/dependency-notes.md`.

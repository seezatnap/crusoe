# Handoff Format

Use this exact structure for every agent handoff, draft transfer, and milestone update.

## Required Envelope

```
handoff_id: HF-YYYYMMDD-###
timestamp_utc: 2026-02-13T00:00:00Z
from_agent: "Agent Name"
to_agent: "Next owner"
handoff_type: "chapter" | "artifact" | "review" | "release"
scope: "CH-###" | "DOC-###" | "LOG-###"
status: "ready" | "blocked" | "review_needed"
```

## Handoff Body

### 1) Completed Work
- What was changed.
- Files written/updated.
- Assumptions made and why.

### 2) Current Working State
- Current chapter/artifact status.
- Word count summary (if chapter scope):
  - `chapter_id`, `word_count_current`, `target_min`, `target_max`, `delta_to_min`.
- Dependency impact:
  - satisfied dependencies
  - blocked dependencies

### 3) Quality Checks
- Chapter length: pass/fail vs 4,000–6,000 range.
- Continuity check result.
- Lore/POV/stylistic risks identified.

### 4) Immediate Next Actions
- Top 3 tasks for the next owner.
- Exact files to open first.
- Required command checks (if any).

### 5) Risks
- Open blockers and the owner to unblock.
- Any potential timeline conflicts.

## Mandatory Attachment (Optional for chapter drafts)

- Relevant snippets from:
  - `chapters/CH-###-*.md`
  - `writing-artifacts/word-count.md` (row)
  - `writing-artifacts/dependency-notes.md` (if blockers changed)

## Example

```text
handoff_id: HF-20260213-001
timestamp_utc: 2026-02-13T00:00:00Z
from_agent: "Agent Aaron"
to_agent: "Agent Beth"
handoff_type: "chapter"
scope: "CH-001"
status: "ready"
```

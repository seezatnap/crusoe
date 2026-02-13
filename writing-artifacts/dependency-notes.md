# Dependency Notes

Purpose: define and track required dependencies between artifacts and chapter drafting
objects. If any dependency cannot be satisfied, drafting status is blocked.

## Artifact Dependency Matrix

| artifact_id | artifact_name | depends_on | status | dependency_risk | notes |
| --- | --- | --- | --- | --- | --- |
| DOC-001 | World Design Document | none | required | low | Foundation for setting, geography, and atmosphere references. |
| DOC-002 | Character Bible | DOC-001 | required | medium | Includes unresolved character backstory constraints to be resolved before Chapter drafting. |
| DOC-003 | Plot Outline | DOC-001, DOC-002 | required | medium | Governs chapter sequencing and milestone constraints. |
| DOC-004 | Chapter-by-Chapter Outline | DOC-001, DOC-002, DOC-003 | required | medium | Defines scene-by-scene beats and clue placement. |
| DOC-005 | Mystery Architecture | DOC-001, DOC-002, DOC-003, DOC-004 | required | high | Core fair-play and reveal constraints. |
| DOC-006 | Timeline & Continuity Tracker | DOC-001, DOC-002, DOC-003, DOC-004, DOC-005 | required | high | Time/continuity gates must be satisfied before each chapter handoff. |
| DOC-007 | Theme & Motif Guide | DOC-001, DOC-002, DOC-003 | required | low | Tone and motif consistency constraints. |

## Chapter Dependency Baseline (Initial)

- All chapters `CH-001` through `CH-025` are initially blocked until:
  - `DOC-001`, `DOC-002`, and `DOC-004` exist and are approved.
  - `DOC-005` and `DOC-006` are complete before chapters move beyond outline validation.
  - Any chapter-specific dependency is satisfied in its frontmatter (`dependency_ids`).

## Blocked State Policy

When an upstream dependency is missing:
1. Set chapter status to `blocked` in `word-count.md`.
2. Add a concrete reason in chapter frontmatter `dependencies_note`.
3. Add/refresh the blocker entry in this file.
4. Mirror blockers in handoff via `handoff-format.md`.

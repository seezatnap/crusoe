# Postmortem Rubric (Task #9)

## Purpose

This rubric defines the mandatory postmortem checks required after each chapter draft revision cycle.  
A chapter is eligible for release only when it passes all checks or explicitly documents an approved exception.

## Scope

- Applies to all files in `chapters/chapter-XX.md`.
- Uses style requirements from:
  - Scene and POV progression rules (Task #7).
  - Style-and-voice protocol (Task #8).
- Reporting artifacts are written to `writing-artifacts/`.

## Mandatory Gates (all must be completed)

### 1) Word-Count Compliance

- Target: `4,000`–`6,000` words per chapter.
- Method:
  - Compute word count from plain text with Unicode-aware tokenization.
  - Ignore frontmatter and code blocks.
- Failure criteria:
  - `< 4,000` words.
  - `> 6,000` words.
- Required evidence:
  - Exact word total.
  - Required revision action if noncompliant.

### 2) Plot Progression

- Purpose: ensure each chapter advances core narrative logic and planned arc movement.
- Required evidence:
  - State one primary forward-moving plot action in present-Lattice and one in recording-era line (if present in the chapter).
  - Identify which unresolved thread(s) this chapter advances.
+- Must confirm:
  - The chapter reaches a meaningful turning point, complication, or payoff tied to the current act objectives.
  - No major scene ends without an observable consequence in the next chapter scaffold.
- Failure criteria:
  - Zero progression signals for the chapter's assigned beat.
  - Introduced escalation with no narrative payoff path.

### 3) Continuity

- Purpose: preserve cross-chapter logic, character state, lore timing, and relationship trajectories.
- Required checks:
  - Character status and knowledge: verify every returning character remains consistent with latest known facts.
  - Event ordering: verify the chapter timeline order aligns with act/sequence placement.
  - Thread consistency: confirm all referenced unresolved threads are tracked and unchanged unless intentionally resolved.
- Failure criteria:
  - Contradiction in a character's emotional, factual, or relationship state.
  - Timeline violations (chronological ordering breaks expected sequence).
  - Missing or contradictory thread references.

### 4) Cliches

- Purpose: reduce repetition and stale phrasing while preserving character-authentic voice.
- Required checks:
  - Run cliché/repetition scan using the project support tooling when available (Task #12 pipeline).
  - Manually confirm flagged candidates are either eliminated or intentionally retained.
- Failure criteria:
  - Repetition of any cliché marker above medium threshold in prose or dialogue.
  - More than two repeated structural sentence patterns without clear narrative reason.

### 5) Dialogue Realism

- Purpose: ensure dialogue reads as spoken exchange rather than exposition dumps.
- Required evidence:
  - At least one pass for each major speaker with:
    - Distinct tone/word choice.
    - No repeated identical emotional beats in back-to-back lines.
    - At least 80% lines that sound speaker-specific and situational.
  - Dialogue-tag hygiene:
    - Dialogue tags are sparing and meaningful.
    - At least one line per scene shows interruption, overlap, silence, or implication (when appropriate).
- Failure criteria:
  - More than 25% of dialogue lines read as exposition-only.
  - Dominance of one tag style (“said”) without variety where variation is appropriate.
  - Dialogue that advances plot without emotional cost (all exposition or no scene pressure).

### 6) Pacing

- Purpose: preserve progression rhythm from setup to escalation to consequence.
- Required checks:
  - Scene-function map includes at least:
    - Opening anchor or status marker.
    - Escalation turn.
    - Midpoint or turning point.
    - Exit state (setup for next chapter).
  - Scene entropy check:
    - No scene segment should exceed a sustained low-tension drift longer than needed.
    - No action segment should compress into a single undifferentiated paragraph block.
- Failure criteria:
  - Missing turning point or exit state.
  - Multiple adjacent scenes with identical tempo/energy without a planned variation.
  - Chapter ends without a forward-driving closure.

### 7) Tone Stability

- Purpose: keep the book’s tonal architecture coherent across POV and setting shifts.
- Required checks:
  - Verify tone is stable per environment:
    - Present-Lattice: elevated abstraction, low-material sensory load, reflective register.
    - Recording scenes: bodily physicality, grounded sensory texture, historical anchoring.
  - Confirm transitions between tonal modes are either:
    - Clearly marked, or
    - Gradual and scene-justified.
- Failure criteria:
  - Abrupt tonal collapse not called for by scene design.
  - Contradictory emotional registers in same character voice without cause.

### 8) Lore Safety Flags

- Purpose: prevent hard lore violations and protect canon boundaries.
- Required checks:
  - Verify no forbidden lore claim is introduced (e.g., physical-Lattice world existence assumptions, mapping constraints not yet disclosed).
  - Verify all explicit reveal moments align with:
    - Current lore maturity.
    - Established world mechanism constraints.
    - Reveal timing approved by roadmap/reveal matrix.
  - Add red-flag notes for:
    - Missing source in world bible.
    - Contradictions with explicit mechanics (Lattice drift, noise, bleed, Curator constraints).
- Failure criteria:
  - Any unresolved lore contradiction.
  - Any safety-sensitive reveal introduced before required dependency is established.

## Scoring and Gate State

- Each section is scored as:
  - `PASS`
  - `WARN (deferred)` with explicit reason and owner
  - `FAIL (blocker)`
- A chapter can move forward only if:
  - No `FAIL`.
  - At most `1 WARN`, and every `WARN` has a resolved action owner and due date.
- Any `FAIL` in Word-Count, Plot Progression, Continuity, or Lore Safety is an immediate stop to release.

## Postmortem Output Template

For each chapter, create a report with:
- Chapter identifier.
- Timestamp and reviewer.
- Word-count total.
- One row per mandatory check with:
  - status (`PASS`/`WARN`/`FAIL`)
  - findings
  - required action
  - owner
  - target date
- Final gate decision:
  - `READY`
  - `NOT READY`

## Versioning

- This rubric is versioned in-repo as this file.
- Changes must remain backward compatible with previously approved postmortem records unless those records are updated.

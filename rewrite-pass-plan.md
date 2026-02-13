# Infinite Domain — Rewrite Pass Execution Plan

## Overview

This document defines the chapter-by-chapter execution plan for the Infinite Domain rewrite pass. All 17 chapters will be rewritten sequentially from scratch, preserving plot, characters, structure, and reveals while rebuilding prose for cohesion, voice consistency, emotional precision, and pacing.

## Source Artifacts (Read-Only)

These directories and files must **never** be modified during the rewrite pass:

### First draft chapters (`first-draft-codex-chapters/`)
| File | Words | Act |
|---|---|---|
| `chapter-01.md` | 5,873 | 1 |
| `chapter-02.md` | 5,496 | 1 |
| `chapter-03.md` | 5,609 | 1 |
| `chapter-04.md` | 4,367 | 1 |
| `chapter-05.md` | 4,828 | 2 |
| `chapter-06.md` | 5,650 | 2 |
| `chapter-07.md` | 5,585 | 2 |
| `chapter-08.md` | 5,945 | 2 |
| `chapter-09.md` | 5,995 | 2 |
| `chapter-10.md` | 4,468 | 2 |
| `chapter-11.md` | 5,381 | 3 |
| `chapter-12.md` | 4,582 | 3 |
| `chapter-13.md` | 5,937 | 3 |
| `chapter-14.md` | 6,000 | 3 |
| `chapter-15.md` | 4,843 | 4 |
| `chapter-16.md` | 4,739 | 4 |
| `chapter-17.md` | 4,055 | 4 |

### Writing artifacts (`writing-artifacts/`)
| Artifact | Purpose |
|---|---|
| `character-bible.md` | Character definitions, arcs, relationships |
| `setting-bible.md` | World rules, Lattice mechanics, recording tech |
| `style-and-voice-protocol.md` | Prose style, anti-cliche constraints, voice targets |
| `roadmap.md` | Act structure, POV/layer rules, pacing targets, chapter tracker |
| `continuity-reveal-matrices.md` | What is known/revealed per chapter |
| `transition-era-timeline.md` | Historical timeline for recording sequences |
| `unresolved-thread-log.md` | Open threads requiring resolution |
| `postmortems/chapter-NN-postmortem-*.md` | Per-chapter quality findings (chapters 05–17) |

## Output Directory

All rewritten chapters are written to `final-draft-chapters/`:
- `final-draft-chapters/chapter-01.md` through `final-draft-chapters/chapter-17.md`

## Execution Protocol (Per Chapter)

Each chapter rewrite follows this exact sequence:

### Phase 1: Pre-Read (Required Before Writing)

1. **Read governing artifacts** (every chapter):
   - `writing-artifacts/character-bible.md`
   - `writing-artifacts/setting-bible.md`
   - `writing-artifacts/style-and-voice-protocol.md`
   - `writing-artifacts/roadmap.md` (focus on this chapter's row in the chapter tracker and POV matrix)
   - `writing-artifacts/continuity-reveal-matrices.md`
   - `writing-artifacts/transition-era-timeline.md`
   - `writing-artifacts/unresolved-thread-log.md`

2. **Read chapter-specific postmortem** (if available):
   - `writing-artifacts/postmortems/chapter-NN-postmortem-*.md`
   - Postmortems exist for chapters 05–17
   - For chapters 01–04, rely on roadmap and continuity matrices

3. **Read the first draft**:
   - `first-draft-codex-chapters/chapter-NN.md`

4. **Read all prior rewritten chapters** (sequential dependency):
   - `final-draft-chapters/chapter-01.md` through `final-draft-chapters/chapter-(NN-1).md`
   - This ensures voice consistency and continuity with the rewrite so far

### Phase 2: Style Calibration

5. **Run style-reference samples** (per AGENTS.md protocol):
   - `style-reference narrative --count 5`
   - `style-reference dialogue --count 5`

### Phase 3: Drafting

6. **Write the rewritten chapter** to `final-draft-chapters/chapter-NN.md`:
   - Same plot beats, reveals, character arcs as first draft
   - Unified voice consistent with prior rewritten chapters
   - Stronger scene transitions and connective tissue
   - Emotionally precise, anti-cliche prose
   - Natural dialogue
   - Sensory grounding in both Lattice and recording sequences
   - Adherence to POV/layer alternation rules from roadmap
   - **Word count: 4,000–6,000 words**

### Phase 4: Validation

7. **Verify word count** is within 4,000–6,000 range
8. **Run quality pipeline** (if available):
   - `scripts/run-quality-session.sh final-draft-chapters/chapter-NN.md`
9. **Log completion** in `rewrite-pass-status.md` and `writing-artifacts/logs/rewrite-pass-log.md`

## Chapter Sequence and Dependencies

All chapters must be completed in strict sequential order. Each chapter blocks on the prior one.

### Act 1: The Commission (Chapters 01–04)
| Task | Chapter | Blocked By | Postmortem Available | Lead Layer | Key Focus |
|---|---|---|---|---|---|
| #2 | 01 | #1 (this plan) | No | Present-Lattice | Establish emotional equilibrium, Juno, Maren, Lattice present |
| #3 | 02 | #2 | No | Recording | First peripheral jumps; embodied 2030s contrast |
| #4 | 03 | #3 | No | Present-Lattice | Orin's commission; contract tension |
| #5 | 04 | #4 | No | Recording | First transformational Lena jump; Act 1 turning point |

### Act 2: The Descent (Chapters 05–10)
| Task | Chapter | Blocked By | Postmortem Available | Lead Layer | Key Focus |
|---|---|---|---|---|---|
| #6 | 05 | #5 | Yes | Recording | Engram arc; ethical cost deepens |
| #7 | 06 | #6 | Yes | Recording | Sable as pattern; present anxiety sharpens |
| #8 | 07 | #7 | Yes | Present-Lattice | Isolation; interpersonal pressure |
| #9 | 08 | #8 | Yes | Present-Lattice | Maren's lucid warning |
| #10 | 09 | #9 | Yes | Present-Lattice | Restricted archives; active confrontation |
| #11 | 10 | #10 | Yes | Recording | Journal acquisition; living infrastructure reveal |

### Act 3: The Truth (Chapters 11–14)
| Task | Chapter | Blocked By | Postmortem Available | Lead Layer | Key Focus |
|---|---|---|---|---|---|
| #12 | 11 | #11 | Yes | Present-Lattice | Unauthorized deep-jump; bleed risk |
| #13 | 12 | #12 | Yes | Recording | Constraint architecture revealed |
| #14 | 13 | #13 | Yes | Present-Lattice | Institutional enforcement; coercion |
| #15 | 14 | #14 | Yes | Recording | Late Act 3 payoff; irreversible entry |

### Act 4: The Choice (Chapters 15–17)
| Task | Chapter | Blocked By | Postmortem Available | Lead Layer | Key Focus |
|---|---|---|---|---|---|
| #16 | 15 | #15 | Yes | Present-Lattice | Three routes confrontation |
| #17 | 16 | #16 | Yes | Present-Lattice | Public exposure and resistance |
| #18 | 17 | #17 | Yes | Present-Lattice | The Honest Daybreak; resolution |

## Final Consolidation (Task #19)

After all 17 chapters are rewritten:
- Full manuscript-level pass over all `final-draft-chapters/`
- Verify word counts (4,000–6,000 each)
- Continuity cross-check against `continuity-reveal-matrices.md`
- Roadmap pacing/POV adherence verification
- Anti-cliche and repetition diagnostics
- Unresolved-thread closure check
- Compile final handoff report

## Constraints

- **Do not modify** anything in `first-draft-codex-chapters/` or `writing-artifacts/`
- Preserve all plot points, character arcs, reveals, and structural beats
- Follow act structure, POV/layer alternation rules, emotional progression targets
- Obey all anti-cliche and style constraints from the voice protocol
- Target prose style: literary sci-fi — Ted Chiang meets Kazuo Ishiguro

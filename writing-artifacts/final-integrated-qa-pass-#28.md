# Final integrated QA pass (#28)

Scope:
- Reconcile unresolved story threads across chapters 1–15.
- Verify chapter ordering, count, and word-range compliance.
- Confirm canon compliance against `summer-1994-canonical-timeline-and-compliance-matrix.md`.
- Mark `chapters/` as release-ready for this sprint pass.

## 1) Chapter set inventory and ordering

## File set
- `chapters/chapter-01.md`
- `chapters/chapter-02.md`
- `chapters/chapter-03.md`
- `chapters/chapter-04.md`
- `chapters/chapter-05.md`
- `chapters/chapter-06.md`
- `chapters/chapter-07.md`
- `chapters/chapter-08.md`
- `chapters/chapter-09.md`
- `chapters/chapter-10.md`
- `chapters/chapter-11.md`
- `chapters/chapter-12.md`
- `chapters/chapter-13.md`
- `chapters/chapter-14.md`
- `chapters/chapter-15.md`

## Ordering check
- Exactly 15 files present, named `chapter-01.md` through `chapter-15.md`.
- Headings are sequentially numbered 1–15.
- No missing, duplicated, or extra chapter files in the required set.

## 2) Word-count verification

| Chapter | Words |
|---|---:|
| 1 | 5756 |
| 2 | 4224 |
| 3 | 4275 |
| 4 | 5997 |
| 5 | 5776 |
| 6 | 5975 |
| 7 | 5782 |
| 8 | 5360 |
| 9 | 5890 |
| 10 | 4930 |
| 11 | 4147 |
| 12 | 4529 |
| 13 | 5997 |
| 14 | 4530 |
| 15 | 5925 |

- Total chapter words: 79,093.
- Acceptance band (4,000–6,000 per chapter): all 15 chapters pass.

## 3) Canon compliance sweep (high-level)

### Hard-time window
- Summer 1994 framing is maintained.
- No references detected to resolved future events such as post-*Goblet of Fire* details.
- No direct confirmation of:
  - Voldemort’s return
  - Tournament outcomes
  - Post-summer canon events in determinative terms
- Character knowledge remains at Summer 1994 plausibility by tone and context.

### Explicit risk scan (automated)
- Grep-based scan for out-of-window terms in `chapters/*.md` returned only non-problematic usage (`future` in contextual language, not plot contamination).
- No hard canon-conflict terms surfaced in chapter text.

## 4) Unresolved-thread reconciliation

### Mystery thread (sigil / ward decay / fog compulsion / Undertow)
- Initiation: `chapter-05` (symbol noticing), `chapter-06` (ward pressure at Alcatraz), `chapter-07` (surveillance and movement), `chapter-09` (convergence map), `chapter-10` (ritual materials + counter-surveillance), `chapter-12` (Isolde hesitation), `chapter-14` (ritual mechanics disruption), `chapter-15` (ritual collapse and custody).
- Status: fully reconciled and resolved by Act IV.

### Belonging/thread-of-home
- Introduced in chapter 1 emotional safety arc and expanded through family integration (ch 2–5).
- Reinforced by threat under pressure (ch 6–13) and resolved in emotional closure on the bridge in `chapter-15`.
- Status: fully reconciled.

### Family as shelter / child safety
- Arthur/Molly/Weasley response beats established in acts 1–2.
- Central ethical split in `chapter-11` and consolidated family response in `chapter-12` and `chapter-13`.
- Final unit cohesion and protective logic close in `chapter-14` and `chapter-15`.
- Status: fully reconciled.

### Institutional failure / governance restoration
- BMA friction and jurisdiction limits introduced in `chapter-08`.
- Antagonist pressure makes formal coordination necessary by `chapter-10`.
- Legal custody of antagonists and operational correction in `chapter-15`.
- Status: fully reconciled.

### UK-US magical contrast
- Carried through all acts via recurring comparative texture beats and no unresolved tonal contradiction.
- Status: fully reconciled.

## 5) Act and structural integrity

Matched against the locked spine:
- Act I: chapters 1–4
- Act II: chapters 5–8
- Act III: chapters 9–12
- Act IV: chapters 13–15
- Required POV minima observed:
  - Ginny chapter: 5
  - Ron chapter: 9
  - Arthur chapter: 11
- End-of-act conditions observed in chapter transitions and chapter-15 closure.

## 6) Release readiness decision

- `chapters/` is production-ready for delivery as the current 15-chapter set.
- No unresolved threads remain that are unaddressed in the current chapter sequence.
- No further chapter edits identified during this integrated QA pass.

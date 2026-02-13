# Tasks

## Pre-Production & Governance

- [x] (#1) Stand up the rewrite pass workflow on `feature/infinite-domain-rewrite-pass`, verify required source artifacts exist and are untouched (`first-draft-codex-chapters/`, `writing-artifacts/`), create/confirm `final-draft-chapters/`, and define the chapter-by-chapter execution plan with logging and status tracking [5 pts] (A)

## Sequential Chapter Rewrites

- [x] (#2) Rewrite `final-draft-chapters/chapter-01.md` from scratch by first reading `writing-artifacts/character-bible.md`, `writing-artifacts/setting-bible.md`, `writing-artifacts/style-and-voice-protocol.md`, `writing-artifacts/roadmap.md`, `writing-artifacts/continuity-reveal-matrices.md`, `writing-artifacts/transition-era-timeline.md`, `writing-artifacts/unresolved-thread-log.md`, and `writing-artifacts/postmortems/chapter-01-postmortem-*.md`; then read `first-draft-codex-chapters/chapter-01.md`; run targeted `style-reference narrative/dialogue --count 5` samples; preserve all beats, reveals, POV/POV-layer rules, and emotional progression; write a cohesive 4,000–6,000 word chapter in final style [5 pts] (blocked by #1) (A)

- [x] (#3) Rewrite `final-draft-chapters/chapter-02.md` with the same required-reading flow for chapter 02 artifacts, including all prior rewritten chapters (`final-draft-chapters/chapter-01.md`), preserve continuity and transition logic into chapter 02, and produce a 4,000–6,000 word standalone chapter with unified voice and anti-cliche constraints [5 pts] (blocked by #2) (A)

- [x] (#4) Rewrite `final-draft-chapters/chapter-03.md` using the full source stack (`character-bible`, `setting-bible`, `style-and-voice-protocol`, relevant roadmap section, continuity/reveal matrices, timeline, thread log, chapter-03 postmortem), prior rewritten chapters, and first draft baseline; calibrate prose via `style-reference` samples; deliver 4,000–6,000 words with preserved structure and pacing [5 pts] (blocked by #3) (A)

- [x] (#5) Rewrite `final-draft-chapters/chapter-04.md` from scratch with the same evidence-driven process, ensuring chapter-to-chapter connective tissue with chapter 03 is seamless and all planned plot/character arcs are intact while avoiding cliché language and staying within 4,000–6,000 words [5 pts] (blocked by #4) (A)

- [x] (#6) Rewrite `final-draft-chapters/chapter-05.md` after ingesting chapter-05-specific postmortem and all governing bibles/roadmap/continuity artifacts plus prior rewritten chapters and first-draft chapter 05, then produce a polished 4,000–6,000 word chapter aligned to sequence and reveal timing [5 pts] (blocked by #5) (A)

- [x] (#7) Rewrite `final-draft-chapters/chapter-06.md` with full required reads (`character/setting/style/roadmap/continuity/timeline/thread log/postmortem`, prior rewrites, first draft), then rebuild chapter in unified authorial voice with stronger transitions and scene rhythm, enforcing 4,000–6,000 words [5 pts] (blocked by #6) (A)

- [ ] (#8) Rewrite `final-draft-chapters/chapter-07.md` preserving all structural beats and POV/layer alternation rules from the roadmap, using targeted `style-reference` calibration before drafting, and deliver a standalone 4,000–6,000 word rewrite anchored to prior rewritten chapters [5 pts] (blocked by #7)

- [ ] (#9) Rewrite `final-draft-chapters/chapter-08.md` by reconstituting plot flow, emotional arcs, and reveals from first draft under final continuity constraints, including cross-checks against transition-era timeline and unresolved-thread log; maintain 4,000–6,000 words [5 pts] (blocked by #8)

- [ ] (#10) Rewrite `final-draft-chapters/chapter-09.md` with complete pre-read pass and sequential dependency on rewritten prior chapters; improve sensory grounding in both Lattice and recording sequences, and enforce anti-cliche prose targets in 4,000–6,000 words [5 pts] (blocked by #9)

- [ ] (#11) Rewrite `final-draft-chapters/chapter-10.md` from scratch with all governing references and chapter-10 postmortem context, preserving reveal order and emotional trajectory while refining dialogue and transitions; output 4,000–6,000 word standalone chapter [5 pts] (blocked by #10)

- [ ] (#12) Rewrite `final-draft-chapters/chapter-11.md` using full required sources and prior chapter continuity checks, then craft a unified, precise, humane sci-fi prose draft within 4,000–6,000 words and consistent with roadmap pacing [5 pts] (blocked by #11)

- [ ] (#13) Rewrite `final-draft-chapters/chapter-12.md` with complete fidelity to first draft beats but rebuilt prose consistency, incorporating style-reference calibration and all continuity/reveal constraints; deliver 4,000–6,000 words [5 pts] (blocked by #12)

- [ ] (#14) Rewrite `final-draft-chapters/chapter-13.md` based on required bibles, roadmap chapter constraints, continuity/reveal matrices, thread log, prior rewrites, and chapter 13 first draft; ensure scene transitions and chapter linkage are seamless, 4,000–6,000 words [5 pts] (blocked by #13)

- [ ] (#15) Rewrite `final-draft-chapters/chapter-14.md` by first reconciling all lore and timeline constraints with prior chapters, then rewriting chapter 14 in final voice with emotional precision, preserve all twists/reveals, and keep length within 4,000–6,000 words [5 pts] (blocked by #14)

- [ ] (#16) Rewrite `final-draft-chapters/chapter-15.md` after full-reference read + prior rewrite chain validation, then produce a coherent, stylistically aligned 4,000–6,000 word chapter with consistent dialogue cadence and narrative momentum [5 pts] (blocked by #15)

- [ ] (#17) Rewrite `final-draft-chapters/chapter-16.md` with complete source and sequence validation, preserving roadmap-aligned stakes escalation and POV flow, while refining rhythm and sensory detail; output 4,000–6,000 words [5 pts] (blocked by #16)

- [ ] (#18) Rewrite `final-draft-chapters/chapter-17.md` from scratch using chapter-17 postmortem and all governing artifacts plus full prior rewrite context, preserving final structural reveal and emotional resolution while maintaining 4,000–6,000 words and final draft voice coherence [5 pts] (blocked by #17)

## Final Consolidation & Release Readiness

- [ ] (#19) Run a full manuscript-level pass over `final-draft-chapters/chapter-01.md` through `chapter-17.md` to verify word counts (4,000–6,000 each), continuity with `continuity-reveal-matrices.md`, roadmap pacing/POV adherence, anti-cliche and repetition diagnostics, unresolved-thread closure, and unresolved risks; compile final handoff report and status for remaining gaps [5 pts] (blocked by #18)

## Follow-up tasks (from sprint review)
- [x] (#20) Build or stub the `style-reference` CLI tool so that chapter rewrites can run automated style calibration instead of manual calibration (referenced in tasks #3–#18 and noted as missing in the chapter-01 rewrite log) (blocked by #2) (A)

## Follow-up tasks (from sprint review)
- [x] (#21) Address 23 Blocker-level VOICE-STYLE-001 findings in `final-draft-chapters/chapter-03.md` (concentrated in Juno and Orin dialogue lines ~97–193) by reconciling diction and rhythm with established character voice baselines from chapters 01–02 (blocked by #4) (B)
- [x] (#22) Add explicit dialogue tags to the 2 untagged and 3 ambiguous dialogue blocks in `final-draft-chapters/chapter-03.md` flagged by DIAL-TAG-001 and DIAL-AMBIG-001 (lines 45, 49, 91, 119) to resolve speaker-attribution warnings before chapter 04 builds on this text (blocked by #4) (B)
- [x] (#23) Run a continuity-check diagnostic on `final-draft-chapters/chapter-03.md` against `continuity-reveal-matrices.md` and produce an explicit continuity report artifact — the commit claims 0 violations but no continuity report was generated to verify (blocked by #4) (A)

## Follow-up tasks (from sprint review)
- [x] (#24) Run a continuity-check diagnostic on `final-draft-chapters/chapter-04.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapter 03 in task #23 (blocked by #5) (A)
- [x] (#25) Fix the quality-orchestrator target path so it validates `final-draft-chapters/chapter-XX.md` instead of `chapters/chapter-XX.md` — the mismatch caused chapter-04 quality checks to run against a stale file (word count reported 4,367 vs actual 5,213) (B)

## Follow-up tasks (from sprint review)
- [x] (#26) Run quality-orchestrator checks on `final-draft-chapters/chapter-05.md` (not `chapters/chapter-05.md`) — all existing chapter-05 quality reports validated the stale first-draft file (4,828 words, 0 dialogue blocks) instead of the rewritten file (5,259 words), so cliche, voice-consistency, and dialogue-tag results are invalid (blocked by #25) (B)
- [x] (#27) Run a continuity-check diagnostic on `final-draft-chapters/chapter-05.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–04 in tasks #23 and #24 (blocked by #6) (B)
- [x] (#28) Add missing `<!-- pov: <name> -->` markers to `final-draft-chapters/chapter-05.md` — the voice-consistency analyzer flagged 11 POV-MARKER-001 info findings on the old draft, and the rewrite likely inherits or worsens this gap since no POV markers were added during the rewrite (blocked by #6) (A)

## Follow-up tasks (from sprint review)
- [ ] (#29) Run a continuity-check diagnostic on `final-draft-chapters/chapter-06.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–05 in tasks #23, #24, and #27 (blocked by #7)
- [ ] (#30) Run quality-orchestrator on `final-draft-chapters/chapter-06.md` to get a unified quality report against the rewritten file — the existing quality-orchestrator reports for chapter-06 predate the rewrite and validated a stale file (blocked by #7)
- [ ] (#31) Add missing section-level `<!-- pov: <name> -->` markers to `final-draft-chapters/chapter-06.md` at scene/recording transitions — the voice-consistency report flagged multiple POV-DRIFT-001 warnings because only the top-level POV marker exists, the same gap chapter-05 had before task #28 (blocked by #7)
- [ ] (#32) Clean up the 4 duplicate voice-consistency reports for chapter-05 (timestamps 1771003623, 1771003627, 1771003631, 1771003640) generated during task #28 — all 4 are identical and only one should be retained

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

- [x] (#8) Rewrite `final-draft-chapters/chapter-07.md` preserving all structural beats and POV/layer alternation rules from the roadmap, using targeted `style-reference` calibration before drafting, and deliver a standalone 4,000–6,000 word rewrite anchored to prior rewritten chapters [5 pts] (blocked by #7) (A)

- [x] (#9) Rewrite `final-draft-chapters/chapter-08.md` by reconstituting plot flow, emotional arcs, and reveals from first draft under final continuity constraints, including cross-checks against transition-era timeline and unresolved-thread log; maintain 4,000–6,000 words [5 pts] (blocked by #8) (A)

- [x] (#10) Rewrite `final-draft-chapters/chapter-09.md` with complete pre-read pass and sequential dependency on rewritten prior chapters; improve sensory grounding in both Lattice and recording sequences, and enforce anti-cliche prose targets in 4,000–6,000 words [5 pts] (blocked by #9) (C)

- [x] (#11) Rewrite `final-draft-chapters/chapter-10.md` from scratch with all governing references and chapter-10 postmortem context, preserving reveal order and emotional trajectory while refining dialogue and transitions; output 4,000–6,000 word standalone chapter [5 pts] (blocked by #10) (A)

- [x] (#12) Rewrite `final-draft-chapters/chapter-11.md` using full required sources and prior chapter continuity checks, then craft a unified, precise, humane sci-fi prose draft within 4,000–6,000 words and consistent with roadmap pacing [5 pts] (blocked by #11) (A)

- [x] (#13) Rewrite `final-draft-chapters/chapter-12.md` with complete fidelity to first draft beats but rebuilt prose consistency, incorporating style-reference calibration and all continuity/reveal constraints; deliver 4,000–6,000 words [5 pts] (blocked by #12) (A)

- [x] (#14) Rewrite `final-draft-chapters/chapter-13.md` based on required bibles, roadmap chapter constraints, continuity/reveal matrices, thread log, prior rewrites, and chapter 13 first draft; ensure scene transitions and chapter linkage are seamless, 4,000–6,000 words [5 pts] (blocked by #13) (A)

- [x] (#15) Rewrite `final-draft-chapters/chapter-14.md` by first reconciling all lore and timeline constraints with prior chapters, then rewriting chapter 14 in final voice with emotional precision, preserve all twists/reveals, and keep length within 4,000–6,000 words [5 pts] (blocked by #14) (A)

- [x] (#16) Rewrite `final-draft-chapters/chapter-15.md` after full-reference read + prior rewrite chain validation, then produce a coherent, stylistically aligned 4,000–6,000 word chapter with consistent dialogue cadence and narrative momentum [5 pts] (blocked by #15) (A)

- [x] (#17) Rewrite `final-draft-chapters/chapter-16.md` with complete source and sequence validation, preserving roadmap-aligned stakes escalation and POV flow, while refining rhythm and sensory detail; output 4,000–6,000 words [5 pts] (blocked by #16) (A)

- [A] (#18) Rewrite `final-draft-chapters/chapter-17.md` from scratch using chapter-17 postmortem and all governing artifacts plus full prior rewrite context, preserving final structural reveal and emotional resolution while maintaining 4,000–6,000 words and final draft voice coherence [5 pts] (blocked by #17)

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
- [x] (#29) Run a continuity-check diagnostic on `final-draft-chapters/chapter-06.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–05 in tasks #23, #24, and #27 (blocked by #7) (B)
- [x] (#30) Run quality-orchestrator on `final-draft-chapters/chapter-06.md` to get a unified quality report against the rewritten file — the existing quality-orchestrator reports for chapter-06 predate the rewrite and validated a stale file (blocked by #7) (B)
- [x] (#31) Add missing section-level `<!-- pov: <name> -->` markers to `final-draft-chapters/chapter-06.md` at scene/recording transitions — the voice-consistency report flagged multiple POV-DRIFT-001 warnings because only the top-level POV marker exists, the same gap chapter-05 had before task #28 (blocked by #7) (C)
- [x] (#32) Clean up the 4 duplicate voice-consistency reports for chapter-05 (timestamps 1771003623, 1771003627, 1771003631, 1771003640) generated during task #28 — all 4 are identical and only one should be retained (A)

## Follow-up tasks (from sprint review)
- [x] (#33) Add missing section-level `<!-- pov: <name> -->` markers to `final-draft-chapters/chapter-07.md` at scene and recording-fragment transitions — the chapter has only a top-level POV marker but 8 section breaks and a recording fragment, matching the same gap that required follow-up in chapters 05 and 06 (blocked by #8) (B)
- [x] (#34) Run quality-orchestrator on `final-draft-chapters/chapter-07.md` (not `chapters/chapter-07.md`) — all 4 existing chapter-07 quality reports target the stale first-draft path and are invalid for the rewritten file (blocked by #8) (B)
- [x] (#35) Run a continuity-check diagnostic on `final-draft-chapters/chapter-07.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–06 in tasks #23, #24, #27, and #29 (blocked by #8) (B)
- [x] (#36) Fix 6 error-level voice-consistency findings in `final-draft-chapters/chapter-06.md` — 3 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 133, 199, 321 using `you`/`it`/`she` instead of named characters) and 3 corresponding POV-DRIFT-001 errors that persist in the post-rewrite quality report despite task #31 POV marker additions (blocked by #9) (C)

## Follow-up tasks (from sprint review)
- [x] (#37) Add missing section-level `<!-- pov: <name> -->` markers to `final-draft-chapters/chapter-08.md` at scene and recording-fragment transitions — the chapter has only a top-level POV marker but multiple section breaks marked by `---` dividers, matching the same gap that required follow-up in chapters 05, 06, and 07 (blocked by #9) (A)
- [x] (#38) Run a continuity-check diagnostic on `final-draft-chapters/chapter-08.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–07 in tasks #23, #24, #27, #29, and #35 (blocked by #9) (C)
- [x] (#39) Fix 8 error-level DIAL-TAG-001 weak speaker attributions and 8 corresponding POV-DRIFT-001 errors in `final-draft-chapters/chapter-08.md` (lines 99, 145, 177, 265, 325, 329, 365, 371) flagged in quality-orchestrator report 1771005407 (blocked by #9) (A)
- [x] (#40) Fix 6 error-level DIAL-TAG-001 weak speaker attributions and 6 corresponding POV-DRIFT-001 errors in `final-draft-chapters/chapter-07.md` (lines 139, 261, 389, 453, 481, 485) flagged in quality-orchestrator report 1771005163 — same pattern as chapter-06 task #36 (blocked by #9) (B)
- [x] (#41) Clean up the 4 stale quality-orchestrator reports for chapter-07 (timestamps 1770991668, 1770992563, 1770992617, 1770992622) that target `chapters/chapter-07.md` instead of `final-draft-chapters/chapter-07.md` — task #34 generated a valid replacement but the old files remain (B)
- [x] (#42) Clean up the 3 stale quality-orchestrator reports for chapter-08 (timestamps 1770992152, 1770992563, 1770992635) that target `chapters/chapter-08.md` instead of `final-draft-chapters/chapter-08.md` — report 1771005407 is the only valid one (C)

## Follow-up tasks (from sprint review)
- [x] (#43) Run a continuity-check diagnostic on `final-draft-chapters/chapter-09.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–08 in tasks #23, #24, #27, #29, #35, and #38 (blocked by #10) (B)
- [x] (#44) Add missing section-level `<!-- pov: -->` markers to `final-draft-chapters/chapter-09.md` at each of its 10 `---` section dividers — the chapter has only a top-level POV marker, matching the same gap that required follow-up in chapters 05, 06, 07, and 08 (blocked by #10) (B)
- [x] (#45) Run quality-orchestrator on `final-draft-chapters/chapter-08.md` (not `chapters/chapter-08.md`) — no valid quality report exists for the rewritten chapter-08 file; the only existing reports (timestamps 1770992152, 1770992563, 1770992635) all target the stale first-draft path (blocked by #9) (B)
- [x] (#46) Clean up stale quality-orchestrator reports for chapter-09 in `writing-artifacts/quality/chapter-09/` that target `chapters/chapter-09.md` instead of `final-draft-chapters/chapter-09.md` — same pattern as chapter-07 task #41 and chapter-08 task #42 (A)

## Follow-up tasks (from sprint review)
- [x] (#47) Run a continuity-check diagnostic on `final-draft-chapters/chapter-10.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–09 in tasks #23, #24, #27, #29, #35, #38, and #43 (blocked by #11) (C)
- [x] (#48) Clean up the 5 stale quality-orchestrator reports for chapter-10 (timestamps 1770992291, 1770992303, 1770992311, 1770992345, 1770992379) that target `chapters/chapter-10.md` instead of `final-draft-chapters/chapter-10.md` — no valid report exists for the rewritten file (C)
- [x] (#49) Run quality-orchestrator on `final-draft-chapters/chapter-10.md` (not `chapters/chapter-10.md`) — all 5 existing chapter-10 quality reports target the stale first-draft path and are invalid for the rewritten file (blocked by #11) (C)

## Follow-up tasks (from sprint review)
- [x] (#50) Add missing section-level `<!-- pov: -->` markers to `final-draft-chapters/chapter-11.md` at each of its 6 `---` section dividers — the chapter has only a top-level POV marker, matching the same gap that required follow-up in chapters 05–10 (blocked by #12) (A)
- [x] (#51) Run a continuity-check diagnostic on `final-draft-chapters/chapter-11.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–10 in tasks #23, #24, #27, #29, #35, #38, #43, and #47 (blocked by #12) (C)
- [x] (#52) Run quality-orchestrator on `final-draft-chapters/chapter-11.md` (not `chapters/chapter-11.md`) — the 2 existing chapter-11 quality reports (timestamps 1770992711, 1770993219) both target the stale first-draft path and are invalid for the rewritten file (blocked by #12) (C)
- [x] (#53) Clean up the 2 stale quality-orchestrator reports for chapter-11 (timestamps 1770992711, 1770993219) that target `chapters/chapter-11.md` instead of `final-draft-chapters/chapter-11.md` (A)
- [x] (#54) Fix 4 error-level findings in `final-draft-chapters/chapter-10.md` — 2 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 334, 424) and 2 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771008780, which caused the report to fail with 1 blocking report (blocked by #11) (B)
- [x] (#55) Add missing section-level `<!-- pov: -->` markers to `final-draft-chapters/chapter-10.md` at each of its 6 `---` section dividers — the chapter has only a top-level POV marker, matching the same gap that required follow-up in chapters 05–09 (blocked by #11) (B)

## Follow-up tasks (from sprint review)
- [x] (#56) Fix 6 error-level findings in `final-draft-chapters/chapter-11.md` — 3 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 87, 135, 481 using 'i'/'it' instead of named characters) and 3 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771009605, which caused the report to fail with 1 blocking report (blocked by #13) (B)
- [x] (#57) Run quality-orchestrator on `final-draft-chapters/chapter-12.md` (not `chapters/chapter-12.md`) — no valid quality report exists for the rewritten chapter-12 file; all 5 existing reports in `writing-artifacts/quality/chapter-12/` target the stale first-draft path (blocked by #13) (C)
- [x] (#58) Run a continuity-check diagnostic on `final-draft-chapters/chapter-12.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–11 in tasks #23, #24, #27, #29, #35, #38, #43, #47, and #51 (blocked by #13) (B)
- [x] (#59) Clean up the 5 stale quality-orchestrator reports for chapter-12 (timestamps 1770992994, 1770993017, 1770993475, 1770993489, 1770993740) that target `chapters/chapter-12.md` instead of `final-draft-chapters/chapter-12.md` (C)

## Follow-up tasks (from sprint review)
- [x] (#60) Fix 4 error-level findings in `final-draft-chapters/chapter-12.md` — 2 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 103, 143 using 'it' instead of named characters) and 2 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771009897, which caused the report to fail with 2 blocking reports (blocked by #14) (A)
- [x] (#61) Run quality-orchestrator on `final-draft-chapters/chapter-13.md` (not `chapters/chapter-13.md`) — both existing chapter-13 quality reports (timestamps 1770993486, 1770993823) target the stale first-draft path and are invalid for the rewritten file (blocked by #14) (B)
- [x] (#62) Clean up the 2 stale quality-orchestrator reports for chapter-13 (timestamps 1770993486, 1770993823) that target `chapters/chapter-13.md` instead of `final-draft-chapters/chapter-13.md` (C)
- [x] (#63) Run a continuity-check diagnostic on `final-draft-chapters/chapter-13.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–12 in tasks #23, #24, #27, #29, #35, #38, #43, #47, #51, and #58 (blocked by #14) (B)
- [x] (#64) Clean up the stale chapter-11 quality-orchestrator report 1771010017 which shows status: failed — it was generated mid-fix before task #56 completed, and report 1771010043 is the valid passing replacement (C)

## Follow-up tasks (from sprint review)
- [x] (#65) Fix 16 error-level findings in `final-draft-chapters/chapter-13.md` — 8 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 111, 183, 271, 279, 321, 429, 477, 557) and 8 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771011052, which caused the report to fail (blocked by #15) (A)
- [x] (#66) Run quality-orchestrator on `final-draft-chapters/chapter-14.md` (not `chapters/chapter-14.md`) — the only existing chapter-14 quality report (timestamp 1770994354) targets the stale first-draft path and is invalid for the rewritten file (blocked by #15) (B)
- [x] (#67) Run a continuity-check diagnostic on `final-draft-chapters/chapter-14.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–13 in tasks #23, #24, #27, #29, #35, #38, #43, #47, #51, #58, and #63 (blocked by #15) (B)
- [x] (#68) Clean up the stale chapter-14 quality-orchestrator report 1770994354 which targets `chapters/chapter-14.md` instead of `final-draft-chapters/chapter-14.md` (C)
- [x] (#69) Clean up the duplicate voice-consistency report for chapter-12 (timestamps 1771011554 and 1771011559 are near-identical) — retain one and remove the duplicate (C)
- [x] (#70) Clean up the stale chapter-12 quality-orchestrator report 1771009897 which was generated pre-fix before task #60 completed — report 1771011575 is the valid post-fix replacement (A)

## Follow-up tasks (from sprint review)
- [x] (#71) Fix 10 error-level findings in `final-draft-chapters/chapter-14.md` — 5 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 103, 137, 139, 455, 647) and 5 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771012053, which caused the report to fail with 2 blocking reports (blocked by #16) (B)
- [x] (#72) Run quality-orchestrator on `final-draft-chapters/chapter-15.md` (not `chapters/chapter-15.md`) — no valid quality report exists for the rewritten chapter-15 file; the 2 existing reports (timestamps 1770994041, 1770995444) target the stale first-draft path (blocked by #16) (C)
- [x] (#73) Run a continuity-check diagnostic on `final-draft-chapters/chapter-15.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–14 in tasks #23, #24, #27, #29, #35, #38, #43, #47, #51, #58, #63, and #67 (blocked by #16) (C)
- [x] (#74) Clean up the 2 stale chapter-15 quality-orchestrator reports (timestamps 1770994041, 1770995444) that target `chapters/chapter-15.md` instead of `final-draft-chapters/chapter-15.md` (B)

## Follow-up tasks (from sprint review)
- [A] (#75) Fix 12 error-level findings in `final-draft-chapters/chapter-15.md` — 6 DIAL-TAG-001 weak pronoun-based speaker attributions (lines 139, 179, 255, 287, 349, 467) and 6 corresponding POV-DRIFT-001 errors flagged in quality-orchestrator report 1771013005, which caused the report to fail with 1 blocking report (blocked by #17)
- [B] (#76) Run quality-orchestrator on `final-draft-chapters/chapter-16.md` (not `chapters/chapter-16.md`) — all 5 existing chapter-16 quality reports (timestamps 1770994733, 1770995444, 1770996056, 1770996073, 1770996320) target the stale first-draft path and are invalid for the rewritten file (blocked by #17)
- [B] (#77) Run a continuity-check diagnostic on `final-draft-chapters/chapter-16.md` against `continuity-reveal-matrices.md` and produce a manual continuity report artifact, matching the format established for chapters 03–15 in tasks #23, #24, #27, #29, #35, #38, #43, #47, #51, #58, #63, #67, and #73 (blocked by #17)
- [B] (#78) Clean up the 5 stale chapter-16 quality-orchestrator reports (timestamps 1770994733, 1770995444, 1770996056, 1770996073, 1770996320) that target `chapters/chapter-16.md` instead of `final-draft-chapters/chapter-16.md`
- [C] (#79) Clean up the duplicate chapter-14 quality-orchestrator report — reports 1771013092 and 1771013096 were generated 4 seconds apart with near-identical content (both 326 findings, both status: failed); retain one and remove the duplicate
- [C] (#80) Clean up the stale chapter-14 quality-orchestrator report 1771012053 which was generated pre-fix before task #71 completed — reports 1771013092/1771013096 are the post-fix replacements

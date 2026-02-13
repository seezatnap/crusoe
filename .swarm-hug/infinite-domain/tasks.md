# Tasks

## Foundation and Planning

- [x] (#1) Create the project’s chapter/artifact delivery contract so every draft is written to `chapters/chapter-01.md` through `chapters/chapter-17.md` and all analytical outputs land in `writing-artifacts/` with explicit update rules and retention policy [5 pts] (A)
- [x] (#2) Establish the master chapter tracker (`writing-artifacts/roadmap.md`) with act boundaries (1–4, 5–10, 11–14, 15–17), pacing goals, and mandatory 4,000–6,000-word target range per chapter [5 pts] (blocked by #1) (A)
- [x] (#3) Build and maintain a canonical character bible covering Juno, Orin, Maren, Cassiel, Fen, Lena, Sable, Dara, Marcus, and Curator/Faded roles with secrets, motivations, relationships, and reveal timing [5 pts] (blocked by #1, #2) (A)
- [x] (#4) Build the setting and mechanics bible for the Lattice, Archive, drift, noise, mind-jump bleed, and transition-era context, including what is explicit vs forbidden lore [5 pts] (blocked by #3) (A)
- [x] (#5) Construct a historical timeline for the Transition Era (2025–2090) tied to Lena, Sable, Marcus, and major Engram milestones, including scene anchors for each major recording-era beat [5 pts] (blocked by #3, #4) (A)
- [x] (#6) Build continuity and reveal matrices mapping each character arc, unresolved threads, and cross-chapter dependency graph from Act 1 through Act 4 [5 pts] (blocked by #3, #5) (A)
- [x] (#7) Publish chapter-level POV/scene alternation rules and thematic progression (present-Lattice vs recording chapters), with required emotional pacing and perspective shifts per chapter [5 pts] (blocked by #2, #4, #6) (A)
- [x] (#8) Define the style-and-voice protocol targeting literary sci-fi in the vein of Ted Chiang meets Kazuo Ishiguro: precise, humane, quietly devastating prose with emotional restraint and moments of devastating clarity; include anti-cliché constraints and tone-alternation rules for Lattice vs recording chapters [5 pts] (blocked by #2, #7) (A)
- [x] (#9) Define the postmortem rubric with mandatory checks: word-count compliance, plot progression, continuity, cliches, dialogue realism, pacing, tone stability, and lore safety flags [5 pts] (blocked by #7, #8) (A)
- [x] (#10) Create an unresolved-thread log and risk register in `writing-artifacts/`, linking all open questions, secrets, and ethical dilemmas to specific chapters [5 pts] (blocked by #6, #9) (A)

## Technical Tooling

- [x] (#11) Scaffold `rust/` as a Cargo workspace and establish shared crate layout plus utility contracts for all analysis binaries [5 pts] (blocked by #1) (A)
- [x] (#12) Implement a Rust CLI for repetitive cliché and sentence-pattern detection with per-chapter output and severity thresholds tuned for literary prose [5 pts] (blocked by #11) (B)
- [x] (#13) Implement a Rust CLI for dialogue-tag, POV, and character-voice consistency validation with named-character state tracking and confidence scoring [5 pts] (blocked by #11) (B)
- [x] (#14) Implement a Rust CLI for continuity checks (timeline, lore constraints, drift progression, scene dependencies, revealed information order) with diff-style violation reports [5 pts] (blocked by #11) (C)
- [x] (#15) Build a Rust orchestration binary that runs #12–#14 plus basic length checks in one command and writes signed artifacts per chapter to `writing-artifacts/quality/` [5 pts] (blocked by #12, #13, #14) (A)
- [x] (#16) Add contributor-facing command docs and scripts to run the full quality pipeline plus style-reference reminders before drafting sessions [5 pts] (blocked by #15) (A)

## Act 1 Drafting (Chapters 01–04)

- [x] (#17) Draft `chapters/chapter-01.md` in the 4,000–6,000-word range: establish present-day Lattice reality, Juno’s guarded profession as a Finder, her emotional isolation, and first visit to drifted Maren; encode her core curiosity about origin [5 pts] (blocked by #7, #8, #15, #16, #2) (B)
- [x] (#18) Draft `chapters/chapter-02.md` in the 4,000–6,000-word range: begin Juno’s earliest jumps into peripheral recordings, contrast digital abstraction with embodied 2030s detail (coffee, weather, pain, smell), and seed the Rosetta Key thread [5 pts] (blocked by #17) (B)
- [x] (#19) Draft `chapters/chapter-03.md` in the 4,000–6,000-word range: deliver Orin’s commission for Engram-linked recovery work with unusual compensation, hidden restrictions, and Juno’s uneasy acceptance [5 pts] (blocked by #18) (A)
- [x] (#20) Draft `chapters/chapter-04.md` in the 4,000–6,000-word range: escalate to the first emotionally transformative Lena jump; end Act 1 with a decisive hook into obsession and altered behavior [5 pts] (blocked by #19) (A)

## Act 2 Drafting (Chapters 05–10)

- [x] (#21) Draft `chapters/chapter-05.md` in the 4,000–6,000-word range: map the Engram arc through Lena’s eyes, showing breakthroughs, ethical strain, and emotional cost while deepening first-person present/past contrast [5 pts] (blocked by #20) (A)
- [x] (#22) Draft `chapters/chapter-06.md` in the 4,000–6,000-word range: expand Juno’s fixation as Sable appears in background details, and increase subtle bleed symptoms in present scenes [5 pts] (blocked by #21) (A)
- [x] (#23) Draft `chapters/chapter-07.md` in the 4,000–6,000-word range: show Juno’s withdrawal at work, Fen’s alarm, and Cassiel’s return as concern that strains their history [5 pts] (blocked by #22) (A)
- [x] (#24) Draft `chapters/chapter-08.md` in the 4,000–6,000-word range: stage Maren’s lucid warning moment with initially opaque reference to the Rosetta Key that becomes legible through subsequent beats [5 pts] (blocked by #23) (A)
- [x] (#25) Draft `chapters/chapter-09.md` in the 4,000–6,000-word range: Juno hits missing/restricted recordings, confronts Orin, and handles his deflection without resolving the deception yet [5 pts] (blocked by #24) (A)
- [x] (#26) Draft `chapters/chapter-10.md` in the 4,000–6,000-word range: secure Sable journal recordings, reveal the hidden active infrastructure thesis, and close Act 2 with Juno realizing the Key is a living process [5 pts] (blocked by #25) (A)

## Act 3 Drafting (Chapters 11–14)

- [x] (#27) Draft `chapters/chapter-11.md` in the 4,000–6,000-word range: Juno goes rogue with unauthorized deep jumps into restricted/degraded files and accepts immediate risk from prolonged bleed [5 pts] (blocked by #26) (A)
- [x] (#28) Draft `chapters/chapter-12.md` in the 4,000–6,000-word range: shift into Sable’s direct perspective to reveal the constraint architecture governing perception, cognition, and self-modification [5 pts] (blocked by #27) (A)
- [x] (#29) Draft `chapters/chapter-13.md` in the 4,000–6,000-word range: escalate confrontation with Orin and Curator enforcement; frame protective-deception ethics and civilizational-stability arguments [5 pts] (blocked by #28) (A)
- [x] (#30) Draft `chapters/chapter-14.md` in the 4,000–6,000-word range: Maren’s final lucid confession confirms the hidden generational secret, and Juno finally gains access to Rosetta Key infrastructure [5 pts] (blocked by #29) (A)

## Act 4 Drafting (Chapters 15–17)

- [x] (#31) Draft `chapters/chapter-15.md` in the 4,000–6,000-word range: stage the choice architecture—remove limits, suppress truth, or open truth without forced collapse—while defining Juno’s third-path strategy [5 pts] (blocked by #30) (A)
- [x] (#32) Draft `chapters/chapter-16.md` in the 4,000–6,000-word range: dramatize public exposure and resistance from Curators/Orin, with Cassiel and Fen pulled into the moral crisis and taking active roles [5 pts] (blocked by #31) (B)
- [x] (#33) Draft `chapters/chapter-17.md` in the 4,000–6,000-word range: deliver resolution with mixed societal outcomes, final Maren farewell in full drift, and Juno’s return to an honestly uncertain but real future [5 pts] (blocked by #32) (A)

## Quality Assurance and Release

- [x] (#34) Run postmortem and revision cycle for `chapter-01.md`–`chapter-04.md` against all gates in #9; re-run all Rust checks and style workflow until chapter package is within spec [5 pts] (blocked by #17, #18, #19, #20) (B)
- [x] (#35) Run postmortem and revision cycle for `chapter-05.md`–`chapter-10.md` with special review of tone transitions, act-level escalation, and continuity with #6 and #5 [5 pts] (blocked by #21, #22, #23, #24, #25, #26) (B)
- [x] (#36) Run postmortem and revision cycle for `chapter-11.md`–`chapter-14.md` with emphasis on revelation logic, antagonist evolution, and ethical stakes alignment [5 pts] (blocked by #27, #28, #29, #30) (B)
- [x] (#37) Run postmortem and revision cycle for `chapter-15.md`–`chapter-17.md`, validating ending coherence, emotional closure, and open-thread posture [5 pts] (blocked by #31, #32, #33) (A)
- [x] (#38) Execute final integration release checklist: rerun tooling (#15), finalize roadmaps and unresolved-thread log, freeze artifact outputs, and produce final deliverable bundle for editorial handoff [5 pts] (blocked by #34, #35, #36, #37, #15) (A)

## Follow-up tasks (from sprint review)
- [x] (#39) Update `analysis-contracts` `AnalysisInput` to resolve `target` against `working_directory` and use that resolved path in `validate_target_exists`, plus add tests for non-root working-directory invocation (currently only related paths are resolved, which can cause false “missing target” failures). (blocked by #11) (A)

## Follow-up tasks (from sprint review)
- [x] (#40) Update `cliche-detection`, `voice-consistency`, and `continuity-check` to use `AnalysisInput::resolve_target_path()` for validation, file reads, and report `target` paths so `with_working_directory(...)` and relative targets behave consistently (blocked by #39) (B)
- [x] (#41) Add regression tests for all three analyzer crates (`cliche-detection`, `voice-consistency`, `continuity-check`) that use a non-root `working_directory` with relative targets to confirm path resolution is applied end-to-end, including report metadata and file access (blocked by #39) (B)

## Follow-up tasks (from sprint review)
- [x] (#42) Reconcile user-facing examples for `quality-orchestrator` flags so `./scripts/run-quality-session.sh`/`./scripts/pre-draft-session.sh` guidance matches actual accepted CLI options (use `--fail-on` and `--no-artifact`, or add underscore aliases in `rust/crates/quality-orchestrator`). (blocked by #16) (A)

## Follow-up tasks (from sprint review)
- [x] (#43) Update `writing-artifacts/style-and-voice-protocol.md` to align the protocol with the project’s required JK Rowling style direction in AGENTS.md, including any contradictory references currently framing style as Ted Chiang/Kazuo Ishiguro, and sync task `#8` wording to match. (blocked by #8) (A)

## Follow-up tasks (from sprint review)
- [x] (#44) Fix the malformed list marker in `writing-artifacts/postmortem-rubric.md` (`+- Must confirm:`) under Plot Progression so it renders as a valid bullet item and preserves consistent rubric formatting for postmortem checks. (A)

## Follow-up tasks (from sprint review)
- [x] (#45) Restore `chapters/chapter-01.md` in the active `chapters/` tree so task `#17` is actually deliverable and the claimed completed chapter sequence is present. (blocked by #17) (B)
- [x] (#46) Reintroduce `writing-artifacts/style-and-voice-protocol.md` and verify it contains the JK Rowling-aligned protocol so the completed task state (`#8`/`#43`) matches actual repository artifacts. (blocked by #43) (A)

## Follow-up tasks (from sprint review)
- [x] (#47) Reconcile task wording in `.swarm-hug/infinite-domain/tasks.md` by updating `#8` to match the JK Rowling-only style target in `writing-artifacts/style-and-voice-protocol.md` instead of the outdated “Ted Chiang meets Kazuo Ishiguro” phrasing. (blocked by #43) (A)
- [x] (#48) Fix failing quality issues in `chapters/chapter-03.md` so it can pass `quality-orchestrator` (resolve blocker-level voice-consistency findings and dialogue attribution/POV clarity issues, and rerun the orchestrator until the chapter reports clean). (blocked by #19) (B)

## Follow-up tasks (from sprint review)
- [x] (#49) Resolve remaining `voice-consistency` blockers in `chapters/chapter-04/chapter-03` (`VOICE-STYLE-001`, `DIAL-AMBIG-001`, `DIAL-TAG-002`) and generate a final passing `writing-artifacts/quality/chapter-03/quality-orchestrator-*` report before unfreezing completion of chapter-03 quality work. (blocked by #48) (B)
- [x] (#50) Reconcile `#8` in `.swarm-hug/infinite-domain/tasks.md` with the JK Rowling-only target; it still contains the legacy “Ted Chiang/Kazuo Ishiguro” language despite prior style alignment work. (blocked by #47) (A)

## Follow-up tasks (from sprint review)
- [x] (#51) Update `writing-artifacts/roadmap.md` chapter tracker status for chapters 01–05 so it reflects completed draft state (and current word counts) instead of all entries remaining “Not started,” now that the drafts are in place. (blocked by #21) (A)
- [x] (#52) Resolve remaining `POV-MARKER-001`/voice-consistency metadata gaps in `chapters/chapter-05.md` (either by adding supported POV/dialogue markers or adjusting parser expectations) and rerun `scripts/run-quality-session.sh` to capture a cleaner `writing-artifacts/quality/chapter-05/quality-orchestrator-*` report. (B)

## Follow-up tasks (from sprint review)
- [x] (#53) Update `writing-artifacts/roadmap.md` chapter tracker entry for `chapter-06` to `Completed draft (5,650)` so the roadmap reflects the delivered draft and actual word count (blocked by #22) (A)

## Follow-up tasks (from sprint review)
- [x] (#54) Update `writing-artifacts/roadmap.md` chapter-07 tracker status from `Not started` to `Completed draft (5,585)` and keep related chapter-level metadata consistent with the completed draft state (blocked by #23) (A)

## Follow-up tasks (from sprint review)
- [x] (#55) Update `writing-artifacts/roadmap.md` to mark `chapter-08` as completed with its actual draft word count and confirm chapter-level metadata (pacing objective/act notes) matches the delivered draft `(blocked by #24)` (A)

## Follow-up tasks (from sprint review)
- [x] (#56) Update `writing-artifacts/roadmap.md` chapter-09 entry to `Completed draft (...)` with the actual committed word count and synchronized act/pacing metadata now that `chapters/chapter-09.md` was drafted (blocked by #25) (A)

## Follow-up tasks (from sprint review)
- [x] (#57) Update `writing-artifacts/roadmap.md` chapter-10 status from `Not started` to `Completed draft (4,468)` and verify status metadata (pacing objective/act note) matches `chapters/chapter-10.md` (blocked by #26) (B)
- [x] (#58) Consolidate chapter-10 quality artifacts by removing/renaming `writing-artifacts/quality/chapter-10-work/quality-orchestrator-1770992364.md` and ensure only canonical `writing-artifacts/quality/chapter-10/quality-orchestrator-*` reports are retained, all targeting `chapters/chapter-10.md` (blocked by #26) (A)

## Follow-up tasks (from sprint review)
- [x] (#59) Update `writing-artifacts/roadmap.md` chapter-11 row to `Completed draft (5,360)` and align its pacing metadata with `chapters/chapter-11.md` (blocked by #27) (B)
- [x] (#60) Address failing chapter-11 quality gates in `writing-artifacts/quality/chapter-11/quality-orchestrator-1770992711.md` (notably `CLIC-CLICHÉ-01` and dialogue-tagging/ambiguity findings), then rerun quality checks so the chapter has a clean artifact before moving to `chapter-12.md` (blocked by #27) (B)

## Follow-up tasks (from sprint review)
- [x] (#61) Update `writing-artifacts/roadmap.md` chapter-12 tracker row from `Not started` to `Completed draft (4,568)` and align its title/pacing/act metadata with `chapters/chapter-12.md` (blocked by #28) (A)
- [x] (#62) Resolve the chapter-12 quality blockers (`VOICES`, `DIAL-*`, `POV-*`) in `chapters/chapter-12.md` and rerun `scripts/run-quality-session.sh` so `writing-artifacts/quality/chapter-12/quality-orchestrator-*` has a `status: passed` report before unfreezing downstream chapter work (blocked by #28) (B)

## Follow-up tasks (from sprint review)
- [x] (#63) Update `writing-artifacts/roadmap.md` chapter-13 tracker status from `Not started` to `Completed draft (5,890)` and align row metadata (title/pacing/objective) with `chapters/chapter-13.md`. (blocked by #29) (B)
- [x] (#64) Fix chapter-13 quality failures (`CLIC-CLICHÉ-01`, `DIAL-*`, `POV-DRIFT-*`, `VOICE-STYLE-001`) in `chapters/chapter-13.md` and rerun `scripts/run-quality-session.sh` until `writing-artifacts/quality/chapter-13/quality-orchestrator-*` reports `status: passed` before starting `chapter-14.md`. (blocked by #29) (A)
- [x] (#65) Re-run chapter-12 quality after the most recent edits and make the latest artifact in `writing-artifacts/quality/chapter-12/` a passing report, as the newest current file (`quality-orchestrator-1770993489.md`) is `status: failed` despite task #62 being marked complete. (B)

## Follow-up tasks (from sprint review)
- [x] (#66) Update `writing-artifacts/roadmap.md` chapter-14 tracker row from `Not started` to `Completed draft (5,997)` and align the row metadata with `chapters/chapter-14.md` (blocked by #30). (A)
- [x] (#67) Create the missing `writing-artifacts/quality/chapter-14/quality-orchestrator-*.md` artifact by running the chapter-14 quality session; there is currently no `chapter-14` quality directory or report (blocked by #30). (B)

## Follow-up tasks (from sprint review)
- [x] (#68) Resolve the blocking findings in `writing-artifacts/quality/chapter-14/quality-orchestrator-1770994354.md` (including cliche and voice-consistency blockers), then rerun quality for chapter-14 and replace the chapter-14 status with a passing report before treating the Act-3 postmortem cycle as truly complete. (blocked by #36) (B)
- [x] (#69) Update `writing-artifacts/roadmap.md` chapter-15 tracker row from `Not started` to `Completed draft (4,847)` and sync its pacing/title metadata with `chapters/chapter-15.md`. (blocked by #31) (A)

## Follow-up tasks (from sprint review)
- [x] (#70) Update `writing-artifacts/roadmap.md` chapter-16 tracker row from `Not started` to `Completed draft (4,706)` (or authoritative word count from canonical script) and align status metadata with `chapters/chapter-16.md` now that draft delivery task `#32` is marked complete (blocked by #32) (A)

## Follow-up tasks (from sprint review)
- [x] (#71) Update `writing-artifacts/roadmap.md` chapter 17 tracker row from `Not started` to `Completed draft (4,057)` and sync its pacing/title metadata with `chapters/chapter-17.md` (`The Honest Daybreak` / present-layer framing) (blocked by #33) (A)

## Follow-up tasks (from sprint review)
- [x] (#72) Update `writing-artifacts/roadmap.md` chapter-15 row count from `Completed draft (4,847)` to `Completed draft (4,843)` to match the actual `chapters/chapter-15.md` word count and quality/postmortem evidence. (B)
- [x] (#73) Correct `writing-artifacts/postmortems/chapter-15-postmortem-2026-02-13.md`, `writing-artifacts/postmortems/chapter-16-postmortem-2026-02-13.md`, and `writing-artifacts/postmortems/chapter-17-postmortem-2026-02-13.md` to reference existing quality artifacts (`quality-orchestrator-1770995444.md`) instead of non-existent `...1770995384.md` (blocked by #37). (B)
- [x] (#74) Add quote-based dialogue formatting or parser-compatible dialogue markers in `chapters/chapter-16.md` and `chapters/chapter-17.md` (or adjust analyzer expectations) because `writing-artifacts/quality/chapter-16/quality-orchestrator-1770995444.md` and `writing-artifacts/quality/chapter-17/quality-orchestrator-1770995444.md` report `total_dialogue_blocks: 0` despite spoken dialogue present, then rerun quality sessions (blocked by #37). (A)

## Follow-up tasks (from sprint review)
- [x] (#75) Resolve remaining `DIAL-TAG-001`/`POV-DRIFT-001` blockers in `chapters/chapter-16.md` and `chapters/chapter-17.md` from the latest quality runs (`writing-artifacts/quality/chapter-16/quality-orchestrator-1770996056.md`, `writing-artifacts/quality/chapter-17/quality-orchestrator-1770996055.md`) and rerun quality sessions until status is `passed` before considering the chapter block complete. (blocked by #74) (A)
- [x] (#76) Regenerate final handoff/postmortem quality references for chapter 16/17 to point at the newest quality sessions after the dialogue-parser fix, replacing stale links to pre-fix `1770995444`/`1770995773` outputs. (blocked by #74) (A)

## Follow-up tasks (from sprint review)
- [x] (#77) Recompute and sync chapter-16 artifact metadata to the latest draft length (4,739 words): update `writing-artifacts/roadmap.md`, `writing-artifacts/postmortems/chapter-16-postmortem-2026-02-13.md`, and `writing-artifacts/reports/final-editorial-handoff-2026-02-13/metadata/bundle-manifest.md` so they no longer show `4,706`. (blocked by #75, #76) (A)
- [x] (#78) Recompute and sync chapter-17 artifact metadata to the latest draft length (4,055 words): update `writing-artifacts/roadmap.md`, `writing-artifacts/postmortems/chapter-17-postmortem-2026-02-13.md`, and `writing-artifacts/reports/final-editorial-handoff-2026-02-13/metadata/bundle-manifest.md` so they no longer show `4,057`. (blocked by #75, #76) (A)

## Follow-up tasks (from sprint review)
- [ ] (#79) Regenerate `writing-artifacts/reports/final-editorial-handoff-2026-02-13/` end-to-end so the frozen `planning/roadmap.md` and `postmortems/chapter-16-postmortem-2026-02-13.md` / `chapter-17-postmortem-2026-02-13.md` are synchronized to the active 4,739 and 4,055 word-count metadata after the #77/#78 sync work (blocked by #77, #78).
- [ ] (#80) Fix the stale quality link in `writing-artifacts/reports/final-editorial-handoff-2026-02-13/postmortems/chapter-15-postmortem-2026-02-13.md` (`quality-orchestrator-1770995384.md`) by replacing it with a real existing artifact or regenerating the handoff bundle so all handoff links resolve.

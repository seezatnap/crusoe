# AGENTS.md

## Project
- This repository is a **novel writing project**.
- Primary output is long-form fiction delivered in chapter files.
- All chapter drafts should be written to: `chapters/`.
- Analytical or tooling outputs should be written to: `writing-artifacts/`.

## Chapter Length
- Target length for each chapter: **5,000 words**.
- Acceptable range: **4,000 to 6,000 words**.
- During post-mortems, always count the current working chapter's exact word total and expand or contract to keep it in this range.

## Style
- The target prose style is **exactly in the style of J. K. Rowling**.
- Prioritize clear scene momentum, character-forward narrative logic, and rhythmic sentence variation.
- Preserve emotional clarity, purposeful pacing, and narrative consistency across chapters.

## Pre-Writing Style Calibration
- Before writing each chapter draft, sample four style pages:
  - run `style-reference page --count 4`
- Before drafting chapter openings and closings, sample chapter-edge examples:
  - run `style-reference chapter-start --count 2`
  - run `style-reference chapter-end --count 2`
- Use these samples for cadence, pacing, and voice calibration only; do not copy content.
- Avoid overfitting literal structure; prioritize natural scene flow and character logic.

## Style-Reference Access Rules
- Never open source text files directly in full; they are large and token-expensive.
- Only use commands or tools that read targeted subsections (for example line ranges, matched excerpts, or sampled outputs).
- Do not read full corpus files directly from `style-reference/`; use the cached corpus in `.downloads/style-reference/`.
- Prefer `style-reference ...` binary output for examples instead of raw file reads whenever possible.

## Process and Quality
- Use post-mortems after each chapter:
  - verify plot progression and character continuity,
  - check continuity of lore, timeline, and stakes,
  - ensure foreshadowing and escalation are coherent,
  - enforce word-range compliance.
- Perform language-level diagnostics for repetition and cliché patterns where relevant.
- Treat style, structure, and cadence as production quality bars before moving to the next chapter.

## Chapter Outline Artifacts
- For every chapter outline in `writing-artifacts/`, explicitly list:
  - scenes,
  - locations,
  - items.
- Mark whether the chapter is a continuation of the previous chapter.
- If it is not a continuation, explicitly note that it starts a new scene and/or time block.

## Rust tooling (./rust)
- Maintain a **Cargo monorepo** in `./rust` for algorithmic writing support.
- Create Rust binaries as needed to codify algorithmic necessities, including but not limited to:
  - repetitive cliché sentence-structure detection,
  - lexical and syntactic repetition checks,
  - scene and arc inconsistency checks,
  - dialogue-tag and POV consistency validation,
  - readability and rhythm analysis.
- Place Rust crates under `./rust/crates/` and keep shared logic in shared crates where practical.

## “World-class novel” support
- Keep and evolve a writing roadmap in the workspace:
  - character bibles,
  - timeline and geography reference notes,
  - thematic progression checklist,
  - unresolved-thread log.
- Any tooling gaps that materially improve prose quality or consistency should be implemented incrementally in `./rust`.

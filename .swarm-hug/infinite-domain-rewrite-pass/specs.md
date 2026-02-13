# Specifications: infinite-domain-rewrite-pass

# Infinite Domain — Rewrite Pass PRD

## Goal

Rewrite every chapter of _Infinite Domain_ (chapters 01–17) from scratch as a cohesive, polished final draft. The plot, characters, structure, and reveals remain the same; the prose is rebuilt to be more cohesive, more interesting, and more emotionally precise across the full arc.

## Context

The first draft was written by multiple Codex agents across 30 automated sprints. The result is structurally complete but likely inconsistent in voice, pacing, and connective tissue between chapters. This rewrite pass is a single-author (Claude) revision aimed at unifying the novel into a coherent reading experience.

## Source material

All source material lives on the `feature/infinite-domain-rewrite-pass` branch:

- **First draft chapters**: `first-draft-codex-chapters/chapter-01.md` through `first-draft-codex-chapters/chapter-17.md`
- **Character bible**: `writing-artifacts/character-bible.md`
- **Setting/mechanics bible**: `writing-artifacts/setting-bible.md`
- **Style and voice protocol**: `writing-artifacts/style-and-voice-protocol.md`
- **Master roadmap** (act structure, POV rules, pacing targets): `writing-artifacts/roadmap.md`
- **Continuity/reveal matrices**: `writing-artifacts/continuity-reveal-matrices.md`
- **Transition-era timeline**: `writing-artifacts/transition-era-timeline.md`
- **Unresolved thread log**: `writing-artifacts/unresolved-thread-log.md`
- **Postmortems**: `writing-artifacts/postmortems/chapter-*-postmortem-*.md`

## Approach

Each chapter rewrite is one task. Tasks must be done **sequentially** (chapter 01 before 02, etc.) so that each rewrite can build on the voice and continuity established by the prior chapters.

For each chapter, the agent must:

1. **Read the existing artifacts first.** Before writing anything, read the character bible, setting bible, style protocol, roadmap (for that chapter's pacing/POV rules), continuity matrices, and the relevant postmortem. Understand what the chapter must accomplish structurally.
2. **Read the first draft.** Read `first-draft-codex-chapters/chapter-NN.md` to absorb the plot, scenes, reveals, and character beats.
3. **Read all previously rewritten chapters.** Read every `final-draft-chapters/chapter-*.md` file that already exists to maintain voice consistency and continuity with the rewrite so far.
4. **Rewrite from scratch.** Write `final-draft-chapters/chapter-NN.md` as a complete, standalone literary chapter. Same plot beats, same reveals, same character arcs — but rebuilt with:
   - Unified, consistent voice across the whole novel
   - Stronger scene transitions and chapter-to-chapter connective tissue
   - More emotionally precise and interesting prose
   - Better pacing within scenes
   - Adherence to the style-and-voice protocol's anti-cliche constraints
   - Natural, believable dialogue
   - Sensory grounding in both Lattice and recording sequences
5. **Maintain word count target.** Each chapter must be 4,000–6,000 words.

## Output

- All rewritten chapters go in `final-draft-chapters/chapter-01.md` through `final-draft-chapters/chapter-17.md`
- Do not modify anything in `first-draft-codex-chapters/` or `writing-artifacts/`

## Constraints

- Preserve all plot points, character arcs, reveals, and structural beats from the first draft
- Follow the act structure, POV/layer alternation rules, and emotional progression targets in the roadmap
- Obey all anti-cliche and style constraints from the voice protocol
- Each chapter blocks on the previous one (sequential execution required)
- Target: literary sci-fi in the vein of Ted Chiang meets Kazuo Ishiguro — precise, humane, quietly devastating


# Quality Orchestrator Report
- target: chapters/chapter-05.md
- generated_at_unix: 1770990802
- fail_threshold: blocker
- total_findings: 11
- total_blocking_reports: 0
- status: passed

## cliche-detection
- findings: 0
- target: chapters/chapter-05.md
- metadata:
  - cliche_blocker_threshold: 8
  - cliche_error_threshold: 5
  - cliche_warning_threshold: 3
  - duplicate_blocker_threshold: 9
  - duplicate_error_threshold: 6
  - duplicate_warning_threshold: 4
  - min_duplicate_sentence_words: 8
  - opening_pattern_blocker_threshold: 11
  - opening_pattern_error_threshold: 8
  - opening_pattern_warning_threshold: 5
  - opening_word_count: 4
  - sentences: 438
- findings: none

## continuity-check
- findings: 0
- target: chapters/chapter-05.md
- metadata:
  - analyzer: continuity-check
  - scene_count: 0
  - status: no_scenes_found
  - target: chapters/chapter-05.md
- findings: none

## length-check
- findings: 0
- target: chapters/chapter-05.md
- metadata:
  - analyzer: length-check
  - max_words: 6000
  - min_words: 4000
  - target: chapters/chapter-05.md
  - words: 4800
- findings: none

## voice-consistency
- findings: 11
- target: chapters/chapter-05.md
- metadata:
  - ambiguous_dialogue_blocks: 0
  - ambiguous_margin: 0.12
  - analyzer: voice-consistency
  - blocker_confidence: 0.25
  - character_list: 
  - continuation_confidence: 0.72
  - error_confidence: 0.45
  - explicit_name_confidence: 0.95
  - pronoun_tag_confidence: 0.42
  - target: chapters/chapter-05.md
  - total_dialogue_blocks: 0
  - tracked_characters: 0
  - untagged_dialogue_blocks: 0
  - voice_blocker_similarity: 0.25
  - voice_error_similarity: 0.40
  - voice_min_sample_words: 24
  - voice_top_words: 8
  - voice_warning_similarity: 0.55
  - warning_confidence: 0.70
- details:
  - [info] POV-MARKER-001 at chapters/chapter-05.md:82:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:110:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:177:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:229:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:317:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:353:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:355:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:357:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:397:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:616:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.
  - [info] POV-MARKER-001 at chapters/chapter-05.md:618:1
    - First-person narration has no visible POV marker (`<!-- pov: <name> -->`).
    - suggestion: Add a POV marker before this paragraph or confirm expected focalization shift in your chapter metadata.


signature: fnv1a64:54612caf91a31e1d

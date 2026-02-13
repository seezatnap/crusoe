# Quality Orchestrator Report
- target: chapters/chapter-17.md
- generated_at_unix: 1770996073
- fail_threshold: error
- total_findings: 4
- total_blocking_reports: 1
- status: failed

## cliche-detection
- findings: 0
- target: chapters/chapter-17.md
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
  - sentences: 465
- findings: none

## continuity-check
- findings: 0
- target: chapters/chapter-17.md
- metadata:
  - analyzer: continuity-check
  - scene_count: 0
  - status: no_scenes_found
  - target: chapters/chapter-17.md
- findings: none

## length-check
- findings: 0
- target: chapters/chapter-17.md
- metadata:
  - analyzer: length-check
  - max_words: 6000
  - min_words: 4000
  - target: chapters/chapter-17.md
  - words: 4057
- findings: none

## voice-consistency
- findings: 4
- target: chapters/chapter-17.md
- metadata:
  - ambiguous_dialogue_blocks: 0
  - ambiguous_margin: 0.12
  - analyzer: voice-consistency
  - blocker_confidence: 0.25
  - character_list: orin, fen, it, juno, maren, yes juno, the
  - continuation_confidence: 0.72
  - error_confidence: 0.45
  - explicit_name_confidence: 0.95
  - it:avg_confidence: 0.42
  - pronoun_tag_confidence: 0.42
  - target: chapters/chapter-17.md
  - total_dialogue_blocks: 11
  - tracked_characters: 7
  - untagged_dialogue_blocks: 0
  - voice_blocker_similarity: 0.12
  - voice_error_similarity: 0.12
  - voice_min_sample_words: 24
  - voice_top_words: 8
  - voice_warning_similarity: 0.55
  - warning_confidence: 0.70
- details:
  - [error] DIAL-TAG-001 at chapters/chapter-17.md:44:1
    - Speaker attribution is weak (0.42) for 'it'; source is pronoun-based tag.
    - suggestion: Replace weak or pronoun-only tag with a named character tag for stronger continuity tracking.
  - [error] DIAL-TAG-001 at chapters/chapter-17.md:321:1
    - Speaker attribution is weak (0.42) for 'it'; source is pronoun-based tag.
    - suggestion: Replace weak or pronoun-only tag with a named character tag for stronger continuity tracking.
  - [error] POV-DRIFT-001 at chapters/chapter-17.md:44:1
    - Dialogue speaker 'it' does not match active POV marker 'juno'.
    - suggestion: If the POV changed, insert a POV marker before this dialogue block.
  - [error] POV-DRIFT-001 at chapters/chapter-17.md:321:1
    - Dialogue speaker 'it' does not match active POV marker 'juno'.
    - suggestion: If the POV changed, insert a POV marker before this dialogue block.


signature: fnv1a64:076ec6684c8079ec

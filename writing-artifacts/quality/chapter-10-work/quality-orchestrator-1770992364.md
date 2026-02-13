# Quality Orchestrator Report
- target: /tmp/chapter-10-work.md
- generated_at_unix: 1770992364
- fail_threshold: error
- total_findings: 1
- total_blocking_reports: 1
- status: failed

## cliche-detection
- findings: 1
- target: /tmp/chapter-10-work.md
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
  - sentences: 558
- details:
  - [error] CLIC-CLICHÉ-01 at /tmp/chapter-10-work.md:23:1
    - Cliche phrase repeated 5 times: "as if".
    - suggestion: Replace repeated phrase usage with image-grounded, character-specific language.

## continuity-check
- findings: 0
- target: /tmp/chapter-10-work.md
- metadata:
  - analyzer: continuity-check
  - scene_count: 0
  - status: no_scenes_found
  - target: /tmp/chapter-10-work.md
- findings: none

## length-check
- findings: 0
- target: /tmp/chapter-10-work.md
- metadata:
  - analyzer: length-check
  - max_words: 6000
  - min_words: 4000
  - target: /tmp/chapter-10-work.md
  - words: 4466
- findings: none

## voice-consistency
- findings: 0
- target: /tmp/chapter-10-work.md
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
  - target: /tmp/chapter-10-work.md
  - total_dialogue_blocks: 0
  - tracked_characters: 0
  - untagged_dialogue_blocks: 0
  - voice_blocker_similarity: 0.25
  - voice_error_similarity: 0.40
  - voice_min_sample_words: 24
  - voice_top_words: 8
  - voice_warning_similarity: 0.55
  - warning_confidence: 0.70
- findings: none


signature: fnv1a64:2c561f8e650aaec9

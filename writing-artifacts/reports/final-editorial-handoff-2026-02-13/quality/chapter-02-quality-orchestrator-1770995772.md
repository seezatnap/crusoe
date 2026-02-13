# Quality Orchestrator Report
- target: chapters/chapter-02.md
- generated_at_unix: 1770995772
- fail_threshold: info
- total_findings: 6
- total_blocking_reports: 1
- status: failed

## cliche-detection
- findings: 0
- target: chapters/chapter-02.md
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
  - sentences: 373
- findings: none

## continuity-check
- findings: 0
- target: chapters/chapter-02.md
- metadata:
  - analyzer: continuity-check
  - scene_count: 0
  - status: no_scenes_found
  - target: chapters/chapter-02.md
- findings: none

## length-check
- findings: 0
- target: chapters/chapter-02.md
- metadata:
  - analyzer: length-check
  - max_words: 6000
  - min_words: 4000
  - target: chapters/chapter-02.md
  - words: 5496
- findings: none

## voice-consistency
- findings: 6
- target: chapters/chapter-02.md
- metadata:
  - ambiguous_dialogue_blocks: 1
  - ambiguous_margin: 0.12
  - analyzer: voice-consistency
  - blocker_confidence: 0.25
  - character_list: no
  - continuation_confidence: 0.72
  - error_confidence: 0.45
  - explicit_name_confidence: 0.95
  - pronoun_tag_confidence: 0.42
  - target: chapters/chapter-02.md
  - total_dialogue_blocks: 6
  - tracked_characters: 1
  - untagged_dialogue_blocks: 4
  - voice_blocker_similarity: 0.12
  - voice_error_similarity: 0.12
  - voice_min_sample_words: 24
  - voice_top_words: 8
  - voice_warning_similarity: 0.55
  - warning_confidence: 0.70
- details:
  - [info] DIAL-AMBIG-001 at chapters/chapter-02.md:226:1
    - Ambiguous dialogue attribution: multiple candidates around line 226, likely 'no' with confidence 0.95.
    - suggestion: Use a stronger tag so named-character tracking is unambiguous.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:261:1
    - Dialogue uses continuation inference for 'no'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [warning] DIAL-TAG-001 at chapters/chapter-02.md:120:1
    - Dialogue paragraph has no candidate speaker attribution; confidence could not be established.
    - suggestion: Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby.
  - [warning] DIAL-TAG-001 at chapters/chapter-02.md:152:1
    - Dialogue paragraph has no candidate speaker attribution; confidence could not be established.
    - suggestion: Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby.
  - [warning] DIAL-TAG-001 at chapters/chapter-02.md:188:1
    - Dialogue paragraph has no candidate speaker attribution; confidence could not be established.
    - suggestion: Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby.
  - [warning] DIAL-TAG-001 at chapters/chapter-02.md:214:1
    - Dialogue paragraph has no candidate speaker attribution; confidence could not be established.
    - suggestion: Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby.


signature: fnv1a64:0af2337deda16f78

# Quality Orchestrator Report
- target: chapters/chapter-02.md
- generated_at_unix: 1770990829
- fail_threshold: error
- total_findings: 11
- total_blocking_reports: 2
- status: failed

## cliche-detection
- findings: 1
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
- details:
  - [blocker] CLIC-CLICHÉ-01 at chapters/chapter-02.md:31:49
    - Cliche phrase repeated 12 times: "as if".
    - suggestion: Replace repeated phrase usage with image-grounded, character-specific language.

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
- findings: 10
- target: chapters/chapter-02.md
- metadata:
  - ambiguous_dialogue_blocks: 1
  - ambiguous_margin: 0.12
  - analyzer: voice-consistency
  - blocker_confidence: 0.25
  - character_list: no, keep, need, someone
  - continuation_confidence: 0.72
  - error_confidence: 0.45
  - explicit_name_confidence: 0.95
  - no:avg_confidence: 0.80
  - pronoun_tag_confidence: 0.42
  - someone:avg_confidence: 0.78
  - target: chapters/chapter-02.md
  - total_dialogue_blocks: 11
  - tracked_characters: 4
  - untagged_dialogue_blocks: 1
  - voice_blocker_similarity: 0.25
  - voice_error_similarity: 0.40
  - voice_min_sample_words: 24
  - voice_top_words: 8
  - voice_warning_similarity: 0.55
  - warning_confidence: 0.70
- details:
  - [info] DIAL-AMBIG-001 at chapters/chapter-02.md:226:1
    - Ambiguous dialogue attribution: multiple candidates around line 226, likely 'no' with confidence 0.95.
    - suggestion: Use a stronger tag so named-character tracking is unambiguous.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:55:1
    - Dialogue uses continuation inference for 'keep'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:160:1
    - Dialogue uses continuation inference for 'someone'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:216:1
    - Dialogue uses continuation inference for 'someone'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:220:1
    - Dialogue uses continuation inference for 'someone'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:229:1
    - Dialogue uses continuation inference for 'no'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [info] DIAL-TAG-002 at chapters/chapter-02.md:239:1
    - Dialogue uses continuation inference for 'no'; confidence 0.72.
    - suggestion: Add an explicit dialogue tag when the speaker switches or scene changes.
  - [warning] DIAL-TAG-001 at chapters/chapter-02.md:27:1
    - Dialogue paragraph has no candidate speaker attribution; confidence could not be established.
    - suggestion: Add a dialogue tag (named speaker or clear attribution) or keep character state explicit nearby.
  - [warning] VOICE-STYLE-001 at chapters/chapter-02.md:239:1
    - Voice drift risk for 'no' (similarity 0.45, weighted confidence 0.72).
    - suggestion: Reconcile diction and rhythm with prior dialogue from this character.
  - [blocker] VOICE-STYLE-001 at chapters/chapter-02.md:220:1
    - Voice drift risk for 'someone' (similarity 0.25, weighted confidence 0.72).
    - suggestion: Reconcile diction and rhythm with prior dialogue from this character.


signature: fnv1a64:aa662cba09c732da

# Quality Orchestrator Report
- target: chapters/chapter-10.md
- generated_at_unix: 1770992303
- fail_threshold: error
- total_findings: 4
- total_blocking_reports: 4
- status: failed

## cliche-detection
- findings: 1
- target: chapters/chapter-10.md
- metadata:
  - analyzer: cliche-detection
- details:
  - [blocker] ORCH-ERR at chapters/chapter-10.md:1:1
    - cliche-detection failed to analyze 'chapters/chapter-10.md': io error at chapters/chapter-10.md: stream did not contain valid UTF-8
    - suggestion: Re-run the failed analyzer after fixing the source content or path issue.

## continuity-check
- findings: 1
- target: chapters/chapter-10.md
- metadata:
  - analyzer: continuity-check
- details:
  - [blocker] ORCH-ERR at chapters/chapter-10.md:1:1
    - continuity-check failed to analyze 'chapters/chapter-10.md': io error at chapters/chapter-10.md: stream did not contain valid UTF-8
    - suggestion: Re-run the failed analyzer after fixing the source content or path issue.

## length-check
- findings: 1
- target: chapters/chapter-10.md
- metadata:
  - analyzer: length-check
- details:
  - [blocker] ORCH-ERR at chapters/chapter-10.md:1:1
    - length-check failed on 'chapters/chapter-10.md': io error at chapters/chapter-10.md: stream did not contain valid UTF-8
    - suggestion: Re-run the failed analyzer after fixing the source content or path issue.

## voice-consistency
- findings: 1
- target: chapters/chapter-10.md
- metadata:
  - analyzer: voice-consistency
- details:
  - [blocker] ORCH-ERR at chapters/chapter-10.md:1:1
    - voice-consistency failed to analyze 'chapters/chapter-10.md': io error at chapters/chapter-10.md: stream did not contain valid UTF-8
    - suggestion: Re-run the failed analyzer after fixing the source content or path issue.


signature: fnv1a64:c0c1a4c9de14922d

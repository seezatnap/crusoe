//! Literary repetition and cliché detector used by the writing-analysis workspace.

use analysis_contracts::{
    AnalysisBinary, AnalysisError, AnalysisInput, AnalysisReport, Finding, Severity,
    SourceLocation,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Shared phrase patterns for literary cliché matching.
pub const CLICHE_PHRASES: &[&str] = &[
    "at the end of the day",
    "the truth is",
    "in the long run",
    "in a world",
    "as if",
    "to be honest",
    "in a way",
    "the fact that",
    "heart of gold",
    "time will tell",
    "it was as if",
    "in the blink of an eye",
    "off the beaten path",
    "all of the sudden",
    "what happened next",
    "once and for all",
    "at the same time",
];

/// Default thresholds chosen to be forgiving for literary prose.
pub const DEFAULT_DUPLICATE_WARNING: usize = 4;
pub const DEFAULT_DUPLICATE_ERROR: usize = 6;
pub const DEFAULT_DUPLICATE_BLOCKER: usize = 9;
pub const DEFAULT_PATTERN_WARNING: usize = 5;
pub const DEFAULT_PATTERN_ERROR: usize = 8;
pub const DEFAULT_PATTERN_BLOCKER: usize = 11;
pub const DEFAULT_CLICHE_WARNING: usize = 3;
pub const DEFAULT_CLICHE_ERROR: usize = 5;
pub const DEFAULT_CLICHE_BLOCKER: usize = 8;

/// Detect repeated sentence starts using the first N words.
pub const DEFAULT_OPENING_WORDS: usize = 4;
/// Ignore very short exact duplicates.
pub const DEFAULT_MIN_DUPLICATE_WORDS: usize = 8;

#[derive(Debug, Clone)]
pub struct ClicheDetectorConfig {
    pub duplicate_warning_threshold: usize,
    pub duplicate_error_threshold: usize,
    pub duplicate_blocker_threshold: usize,
    pub opening_pattern_warning_threshold: usize,
    pub opening_pattern_error_threshold: usize,
    pub opening_pattern_blocker_threshold: usize,
    pub cliche_warning_threshold: usize,
    pub cliche_error_threshold: usize,
    pub cliche_blocker_threshold: usize,
    pub opening_word_count: usize,
    pub min_duplicate_sentence_words: usize,
}

impl Default for ClicheDetectorConfig {
    fn default() -> Self {
        Self {
            duplicate_warning_threshold: DEFAULT_DUPLICATE_WARNING,
            duplicate_error_threshold: DEFAULT_DUPLICATE_ERROR,
            duplicate_blocker_threshold: DEFAULT_DUPLICATE_BLOCKER,
            opening_pattern_warning_threshold: DEFAULT_PATTERN_WARNING,
            opening_pattern_error_threshold: DEFAULT_PATTERN_ERROR,
            opening_pattern_blocker_threshold: DEFAULT_PATTERN_BLOCKER,
            cliche_warning_threshold: DEFAULT_CLICHE_WARNING,
            cliche_error_threshold: DEFAULT_CLICHE_ERROR,
            cliche_blocker_threshold: DEFAULT_CLICHE_BLOCKER,
            opening_word_count: DEFAULT_OPENING_WORDS,
            min_duplicate_sentence_words: DEFAULT_MIN_DUPLICATE_WORDS,
        }
    }
}

#[derive(Debug, Clone)]
struct SentenceHit {
    line: u32,
    column: u32,
    normalized: String,
    words: usize,
}

#[derive(Debug, Clone)]
struct PhraseHit {
    line: u32,
    column: u32,
}

pub struct ClicheDetector {
    config: ClicheDetectorConfig,
}

impl Default for ClicheDetector {
    fn default() -> Self {
        Self::new(ClicheDetectorConfig::default())
    }
}

impl ClicheDetector {
    pub fn new(config: ClicheDetectorConfig) -> Self {
        Self { config }
    }

    pub fn analyze_path(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        input.validate_target_exists()?;
        let target = input.resolve_target_path();
        let content = fs::read_to_string(&target).map_err(|err| AnalysisError::Io {
            path: target.display().to_string(),
            reason: err.to_string(),
        })?;

        let sentences = extract_sentences(&content);
        let mut report = AnalysisReport::new("cliche-detection", &target)
            .add_metadata("sentences", sentences.len().to_string())
            .add_metadata("duplicate_warning_threshold", self.config.duplicate_warning_threshold.to_string())
            .add_metadata("duplicate_error_threshold", self.config.duplicate_error_threshold.to_string())
            .add_metadata("duplicate_blocker_threshold", self.config.duplicate_blocker_threshold.to_string())
            .add_metadata("opening_pattern_warning_threshold", self.config.opening_pattern_warning_threshold.to_string())
            .add_metadata("opening_pattern_error_threshold", self.config.opening_pattern_error_threshold.to_string())
            .add_metadata("opening_pattern_blocker_threshold", self.config.opening_pattern_blocker_threshold.to_string())
            .add_metadata("cliche_warning_threshold", self.config.cliche_warning_threshold.to_string())
            .add_metadata("cliche_error_threshold", self.config.cliche_error_threshold.to_string())
            .add_metadata("cliche_blocker_threshold", self.config.cliche_blocker_threshold.to_string())
            .add_metadata("opening_word_count", self.config.opening_word_count.to_string())
            .add_metadata("min_duplicate_sentence_words", self.config.min_duplicate_sentence_words.to_string());

        add_exact_duplicate_findings(
            &mut report,
            sentences.as_slice(),
            &self.config,
            &target,
        );
        add_opening_pattern_findings(
            &mut report,
            sentences.as_slice(),
            &self.config,
            &target,
        );
        add_cliche_phrase_findings(&mut report, sentences.as_slice(), &self.config, &target);

        Ok(report)
    }

    fn normalized_threshold(
        &self,
        warning: usize,
        error: usize,
        blocker: usize,
    ) -> (usize, usize, usize) {
        // Keep output deterministic even if callers provide inverted values.
        let warning = warning.min(error).min(blocker);
        let error = error.max(warning).min(blocker);
        let blocker = blocker.max(error);
        (warning, error, blocker)
    }

    fn exact_thresholds(&self) -> (usize, usize, usize) {
        self.normalized_threshold(
            self.config.duplicate_warning_threshold,
            self.config.duplicate_error_threshold,
            self.config.duplicate_blocker_threshold,
        )
    }

    fn opening_thresholds(&self) -> (usize, usize, usize) {
        self.normalized_threshold(
            self.config.opening_pattern_warning_threshold,
            self.config.opening_pattern_error_threshold,
            self.config.opening_pattern_blocker_threshold,
        )
    }

    fn cliche_thresholds(&self) -> (usize, usize, usize) {
        self.normalized_threshold(
            self.config.cliche_warning_threshold,
            self.config.cliche_error_threshold,
            self.config.cliche_blocker_threshold,
        )
    }
}

impl AnalysisBinary for ClicheDetector {
    fn analyzer_name(&self) -> &'static str {
        "cliche-detection"
    }

    fn run(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
        self.analyze_path(input)
    }
}

fn sentence_count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}

fn normalize_sentence(sentence: &str) -> String {
    let mut out = String::new();
    let mut had_space = false;

    for ch in sentence.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() || ch == '\'' || ch == '-' {
            out.push(ch);
            had_space = false;
        } else if ch.is_whitespace() || ch == '"' || ch == '“' || ch == '”' || ch == '‘' || ch == '’' {
            if !had_space && !out.is_empty() {
                out.push(' ');
                had_space = true;
            }
        } else if !had_space && !out.is_empty() {
            out.push(' ');
            had_space = true;
        }
    }

    out.trim().to_string()
}

fn extract_sentences(text: &str) -> Vec<SentenceHit> {
    let normalized_text = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut hits = Vec::new();

    for (line_idx, line) in normalized_text.lines().enumerate() {
        let mut start = 0usize;
        let line_no = (line_idx + 1) as u32;

        let commit_segment = |start: usize, end: usize, hits: &mut Vec<SentenceHit>| {
            if start >= end {
                return;
            }

            let raw = line[start..end].trim();
            if raw.is_empty() {
                return;
            }

            let normalized = normalize_sentence(raw);
            if normalized.is_empty() {
                return;
            }

            let column = line[..start].chars().count() as u32 + 1;
            hits.push(SentenceHit {
                line: line_no,
                column,
                normalized: normalized.clone(),
                words: sentence_count_words(&normalized),
            });
        };

        for (offset, ch) in line.char_indices() {
            if matches!(ch, '.' | '?' | '!') {
                let end = offset + ch.len_utf8();
                commit_segment(start, end, &mut hits);
                start = end;
            }
        }

        if start < line.len() {
            commit_segment(start, line.len(), &mut hits);
        }
    }

    hits
}

fn opening_pattern(sentence: &str, opening_word_count: usize) -> Option<String> {
    if opening_word_count < 2 {
        return None;
    }

    let words: Vec<_> = sentence
        .split_whitespace()
        .filter(|word| !word.is_empty())
        .collect();
    if words.len() < opening_word_count {
        None
    } else {
        Some(words[..opening_word_count].join(" "))
    }
}

fn severity_for_count(count: usize, warning: usize, error: usize, blocker: usize) -> Option<Severity> {
    if count < warning {
        return None;
    }
    if count >= blocker {
        return Some(Severity::Blocker);
    }
    if count >= error {
        return Some(Severity::Error);
    }
    Some(Severity::Warning)
}

fn to_location(path: &Path, line: u32, column: u32) -> SourceLocation {
    SourceLocation::new(path.to_path_buf(), Some(line), Some(column))
}

fn add_exact_duplicate_findings(
    report: &mut AnalysisReport,
    sentences: &[SentenceHit],
    config: &ClicheDetectorConfig,
    target: &Path,
) {
    let (warning, error, blocker) = {
        let detector = ClicheDetector::new(config.clone());
        detector.exact_thresholds()
    };
    let mut counts: HashMap<String, Vec<PhraseHit>> = HashMap::new();

    for sentence in sentences {
        if sentence.words < config.min_duplicate_sentence_words {
            continue;
        }

            counts
                .entry(sentence.normalized.clone())
                .or_default()
                .push(PhraseHit {
                    line: sentence.line,
                    column: sentence.column,
                });
        }

    for (sentence, hits, severity) in counts.into_iter().filter_map(|(sentence, hits)| {
        severity_for_count(hits.len(), warning, error, blocker).map(|severity| (sentence, hits, severity))
    }) {
        if hits.is_empty() {
            continue;
        }
        let first = &hits[0];
        report.findings.push(
            Finding::new(
                "CLIC-EXACT-01".to_string(),
                format!(
                    "Exact sentence repeated {} times. Consider rotating imagery while preserving story function.",
                    hits.len()
                ),
                severity,
            )
            .with_location(to_location(target, first.line, first.column))
            .with_suggestion(format!(
                "Rephrase repeated sentence pattern, for example: \"{}\"",
                sentence.chars().take(120).collect::<String>()
            )),
        );
    }
}

fn add_opening_pattern_findings(
    report: &mut AnalysisReport,
    sentences: &[SentenceHit],
    config: &ClicheDetectorConfig,
    target: &Path,
) {
    let (warning, error, blocker) = {
        let detector = ClicheDetector::new(config.clone());
        detector.opening_thresholds()
    };

    let mut counts: HashMap<String, Vec<PhraseHit>> = HashMap::new();
    for sentence in sentences {
        if let Some(pattern) = opening_pattern(&sentence.normalized, config.opening_word_count) {
            counts.entry(pattern).or_default().push(PhraseHit {
                line: sentence.line,
                column: sentence.column,
            });
        }
    }

    for (pattern, hits, severity) in counts.into_iter().filter_map(|(pattern, hits)| {
        severity_for_count(hits.len(), warning, error, blocker).map(|severity| (pattern, hits, severity))
    }) {
        if hits.is_empty() {
            continue;
        }
        let first = &hits[0];
        report.findings.push(
            Finding::new(
                "CLIC-PATTERN-01".to_string(),
                format!(
                    "Similar sentence start repeated {} times: \"{}\".",
                    hits.len(),
                    pattern
                ),
                severity,
            )
            .with_location(to_location(target, first.line, first.column))
            .with_suggestion(
                "Vary sentence openings while preserving rhythm and narrative momentum.".to_string(),
            )
        );
    }
}

fn add_cliche_phrase_findings(
    report: &mut AnalysisReport,
    sentences: &[SentenceHit],
    config: &ClicheDetectorConfig,
    target: &Path,
) {
    let (warning, error, blocker) = {
        let detector = ClicheDetector::new(config.clone());
        detector.cliche_thresholds()
    };

    let mut counts: HashMap<String, Vec<PhraseHit>> = HashMap::new();

    for sentence in sentences {
        let haystack = sentence.normalized.as_str();
        for phrase in CLICHE_PHRASES {
            if haystack.contains(phrase) {
                counts.entry((*phrase).to_string()).or_default().push(PhraseHit {
                    line: sentence.line,
                    column: sentence.column,
                });
            }
        }
    }

    for (phrase, hits, severity) in counts.into_iter().filter_map(|(phrase, hits)| {
        severity_for_count(hits.len(), warning, error, blocker).map(|severity| (phrase, hits, severity))
    }) {
        if hits.is_empty() {
            continue;
        }
        let first = &hits[0];
        report.findings.push(
            Finding::new(
                "CLIC-CLICHÉ-01".to_string(),
                format!(
                    "Cliche phrase repeated {} times: \"{}\".",
                    hits.len(),
                    phrase
                ),
                severity,
            )
            .with_location(to_location(target, first.line, first.column))
            .with_suggestion(
                "Replace repeated phrase usage with image-grounded, character-specific language.".to_string(),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn write_temp_file(contents: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time drifted")
            .as_nanos();
        path.push(format!("cliche-detection-tests-{nanos}.md"));
        fs::write(&path, contents).unwrap();
        path
    }

    fn run_analysis(contents: &str, config: ClicheDetectorConfig) -> AnalysisReport {
        let detector = ClicheDetector::new(config);
        let input = AnalysisInput::new(write_temp_file(contents))
            .with_working_directory(".")
            .with_option("dummy", "value");

        let report = detector.run(&input).unwrap();
        report
    }

    #[test]
    fn finds_repeated_exact_long_sentence() {
        let cfg = ClicheDetectorConfig {
            duplicate_warning_threshold: 3,
            duplicate_error_threshold: 5,
            duplicate_blocker_threshold: 7,
            ..Default::default()
        };
        let text = "The city tasted of rain and brass, and the sky seemed to lean lower each hour. \
            The city tasted of rain and brass, and the sky seemed to lean lower each hour. \
            The city tasted of rain and brass, and the sky seemed to lean lower each hour. \
            The city tasted of rain and brass, and the sky seemed to lean lower each hour.";

        let report = run_analysis(text, cfg);
        assert!(report.findings.iter().any(|finding| finding.code == "CLIC-EXACT-01"));
        assert!(report.has_blocking_findings(Severity::Warning));
    }

    #[test]
    fn finds_repeated_opening_pattern() {
        let cfg = ClicheDetectorConfig {
            opening_pattern_warning_threshold: 3,
            opening_pattern_error_threshold: 5,
            opening_pattern_blocker_threshold: 7,
            opening_word_count: 5,
            ..Default::default()
        };
        let text = "She stepped into the corridor and reached for the light. \
            She stepped into the corridor and touched the wall. \
            She stepped into the corridor and held her breath. \
            She stepped into the corridor and listened for footsteps.";

        let report = run_analysis(text, cfg);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CLIC-PATTERN-01"));
    }

    #[test]
    fn finds_repeated_cliche_phrase() {
        let cfg = ClicheDetectorConfig {
            cliche_warning_threshold: 3,
            cliche_error_threshold: 5,
            cliche_blocker_threshold: 7,
            ..Default::default()
        };
        let text = "At the end of the day, the truth is not always clear. \
            At the end of the day, the truth is all she could say. \
            At the end of the day, silence broke through the walls. \
            At the end of the day, the truth is what remains.";
        let report = run_analysis(text, cfg);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "CLIC-CLICHÉ-01")
        );
    }

    #[test]
    fn normalize_sentence_collapses_punctuation_and_case() {
        assert_eq!(
            normalize_sentence(" \"At the END...\", he whispered! "),
            "at the end he whispered"
        );
    }

    #[test]
    fn split_sentences_tracks_lines_and_columns() {
        let sentences = extract_sentences("One dawn.\nTwo dusk!\nThree noons? ");
        assert_eq!(sentences.len(), 3);
        assert_eq!((sentences[0].line, sentences[0].column), (1, 1));
        assert_eq!((sentences[1].line, sentences[1].column), (2, 1));
        assert_eq!((sentences[2].line, sentences[2].column), (3, 1));
    }

    #[test]
    fn resolves_relative_target_with_working_directory_and_accesses_content() {
        let mut workdir = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        workdir.push(format!("cliche-detection-tests-{nanos}"));
        let chapter_dir = workdir.join("chapter-files");
        std::fs::create_dir_all(&chapter_dir).unwrap();

        let relative_target = PathBuf::from("chapter-files/chapter-01.md");
        let expected = chapter_dir.join("chapter-01.md");
        std::fs::write(
            &expected,
            "The clock above the door chimed at dawn and the corridor was still. \
            The clock above the door chimed at dawn and the corridor was still. \
            The clock above the door chimed at dawn and the corridor was still. \
            The clock above the door chimed at dawn and the corridor was still.",
        )
        .unwrap();

        let detector = ClicheDetector::new(ClicheDetectorConfig {
            duplicate_warning_threshold: 3,
            duplicate_error_threshold: 4,
            duplicate_blocker_threshold: 8,
            ..Default::default()
        });
        let input = AnalysisInput::new(&relative_target).with_working_directory(&workdir);
        let report = detector
            .run(&input)
            .expect("analysis should resolve working-directory target");

        assert_eq!(report.target, expected);
        assert_eq!(report.metadata.get("sentences"), Some(&"4".to_string()));
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "CLIC-EXACT-01"));

        let _ = std::fs::remove_dir_all(&workdir);
    }
}

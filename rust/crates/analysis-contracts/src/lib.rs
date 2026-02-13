//! Shared contracts and utility types for writing-analysis binaries.
//!
//! These types are intentionally narrow and version-stable so different analysis
//! tools can output compatible artifacts and operate on the same severity and
//! path conventions.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Common severity levels used by all writing-analysis tools.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Severity {
    /// Informational note with no enforcement impact.
    Info,
    /// Style or quality warning worth surfacing to the writer.
    Warning,
    /// A reliability or continuity problem that should block acceptance.
    Error,
    /// Hard failure requiring immediate attention before publication.
    Blocker,
}

impl Severity {
    /// Returns true when this severity is at or above the provided threshold.
    pub fn at_least(self, threshold: Severity) -> bool {
        self >= threshold
    }

    /// Numeric value for quick CLI and report sorting.
    pub fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Blocker => 3,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Blocker => "blocker",
        };

        write!(f, "{label}")
    }
}

/// Optional source location details for a reported issue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceLocation {
    pub path: PathBuf,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

impl SourceLocation {
    /// Creates a source location from a file path.
    pub fn new(path: impl Into<PathBuf>, line: Option<u32>, column: Option<u32>) -> Self {
        Self {
            path: path.into(),
            line,
            column,
        }
    }
}

/// A single unit of analysis output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    /// Short machine-oriented code for grouping and gating rules.
    pub code: String,
    /// Human-friendly summary of the issue.
    pub message: String,
    /// Severity used by all quality-gating steps.
    pub severity: Severity,
    /// Optional evidence location.
    pub location: Option<SourceLocation>,
    /// Optional long-form suggestion for remediation.
    pub suggestion: Option<String>,
}

impl Finding {
    pub fn new(code: impl Into<String>, message: impl Into<String>, severity: Severity) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            severity,
            location: None,
            suggestion: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Input contract used by analysis binaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisInput {
    /// Target chapter or artifact path being analyzed.
    pub target: PathBuf,
    /// Optional supplemental file sources for cross-file checks.
    pub related_paths: Vec<PathBuf>,
    /// Raw configuration map used by concrete analyzers.
    pub options: BTreeMap<String, String>,
    /// Optional working directory for relative artifact writes.
    pub working_directory: Option<PathBuf>,
}

impl AnalysisInput {
    pub fn new(target: impl Into<PathBuf>) -> Self {
        Self {
            target: target.into(),
            related_paths: Vec::new(),
            options: BTreeMap::new(),
            working_directory: None,
        }
    }

    pub fn with_related_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.related_paths.push(path.into());
        self
    }

    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.insert(key.into(), value.into());
        self
    }

    pub fn with_working_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(path.into());
        self
    }

    /// Resolves the configured target path relative to the working directory when present.
    pub fn resolve_target_path(&self) -> PathBuf {
        if let Some(base) = &self.working_directory {
            if self.target.is_relative() {
                base.join(&self.target)
            } else {
                self.target.clone()
            }
        } else {
            self.target.clone()
        }
    }

    /// Returns `Ok(())` when the target exists and is a file or directory.
    pub fn validate_target_exists(&self) -> Result<(), AnalysisError> {
        let resolved_target = self.resolve_target_path();

        if resolved_target.exists() {
            Ok(())
        } else {
            Err(AnalysisError::MissingTarget {
                path: resolved_target.display().to_string(),
            })
        }
    }

    /// Resolves a path relative to the working directory when configured.
    pub fn resolve_related_path(&self, path: &Path) -> PathBuf {
        if let Some(base) = &self.working_directory {
            if path.is_relative() {
                base.join(path)
            } else {
                path.to_path_buf()
            }
        } else {
            path.to_path_buf()
        }
    }
}

/// Canonical report payload all analyzers should return.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisReport {
    pub analyzer: String,
    pub target: PathBuf,
    pub generated_at_epoch_seconds: u64,
    pub findings: Vec<Finding>,
    pub metadata: BTreeMap<String, String>,
}

impl AnalysisReport {
    /// Creates an empty report with a current timestamp.
    pub fn new(analyzer: impl Into<String>, target: impl Into<PathBuf>) -> Self {
        Self {
            analyzer: analyzer.into(),
            target: target.into(),
            generated_at_epoch_seconds: now_epoch_seconds(),
            findings: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Adds a finding and returns the updated report.
    pub fn add_finding(mut self, finding: Finding) -> Self {
        self.findings.push(finding);
        self
    }

    /// Adds arbitrary metadata used for telemetry and deterministic artifact output.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns true when there is at least one finding at or above `threshold`.
    pub fn has_blocking_findings(&self, threshold: Severity) -> bool {
        self.findings.iter().any(|finding| finding.severity.at_least(threshold))
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    pub fn finding_count_by_severity(&self, severity: Severity) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity == severity)
            .count()
    }

    pub fn is_blocking(&self, threshold: Severity) -> bool {
        self.has_blocking_findings(threshold)
    }
}

/// Trait shared by analysis CLI entry points and library entry points.
pub trait AnalysisBinary {
    fn analyzer_name(&self) -> &'static str;

    fn run(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError>;
}

/// Structured errors returned by analysis engines.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnalysisError {
    MissingTarget { path: String },
    Io { path: String, reason: String },
    InvalidInput { message: String },
    Internal { message: String },
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTarget { path } => write!(f, "missing analysis target: {path}"),
            Self::Io { path, reason } => write!(f, "io error at {path}: {reason}"),
            Self::InvalidInput { message } => write!(f, "invalid input: {message}"),
            Self::Internal { message } => write!(f, "internal analysis error: {message}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_compare_and_text() {
        assert!(Severity::Blocker.at_least(Severity::Error));
        assert_eq!(Severity::Warning.to_string(), "warning");
        assert!(Severity::Info.rank() < Severity::Error.rank());
    }

    #[test]
    fn finding_builder_helpers() {
        let location = SourceLocation::new("chapter.md", Some(42), Some(3));
        let finding = Finding::new("LINT001", "flat sentence", Severity::Warning)
            .with_location(location.clone())
            .with_suggestion("Split this sentence into two.");

        assert_eq!(finding.code, "LINT001");
        assert_eq!(finding.location, Some(location));
        assert_eq!(finding.suggestion.as_deref(), Some("Split this sentence into two."));
    }

    #[test]
    fn analysis_input_validation() {
        let input = AnalysisInput::new(".");
        assert!(input.validate_target_exists().is_ok());

        let missing = AnalysisInput::new("./definitely-missing-path.md");
        assert!(missing.validate_target_exists().is_err());
    }

    #[test]
    fn analysis_input_validation_uses_working_directory_for_target() {
        let tmp_root = std::env::temp_dir().join(format!(
            "analysis-contract-target-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        std::fs::create_dir_all(&tmp_root).unwrap();
        let target_path = tmp_root.join("chapter.md");
        std::fs::write(&target_path, "sample").unwrap();

        let input = AnalysisInput::new("chapter.md").with_working_directory(&tmp_root);
        assert!(input.validate_target_exists().is_ok());

        let missing = AnalysisInput::new("missing-chapter.md").with_working_directory(&tmp_root);
        assert!(missing.validate_target_exists().is_err());
        let _ = std::fs::remove_dir_all(&tmp_root);
    }

    #[test]
    fn analysis_input_validation_keeps_absolute_target_absolute() {
        let tmp_root = std::env::temp_dir().join(format!(
            "analysis-contract-target-abs-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        std::fs::create_dir_all(&tmp_root).unwrap();
        let target_path = tmp_root.join("chapter.md");
        std::fs::write(&target_path, "sample").unwrap();

        let input = AnalysisInput::new(&target_path).with_working_directory("/tmp/does-not-exist");
        assert!(input.validate_target_exists().is_ok());
        let _ = std::fs::remove_dir_all(&tmp_root);
    }

    #[test]
    fn report_helpers_track_findings() {
        let report = AnalysisReport::new("sample", "chapter.md")
            .add_finding(Finding::new("A", "a", Severity::Info))
            .add_finding(Finding::new("B", "b", Severity::Error));

        assert_eq!(report.finding_count(), 2);
        assert_eq!(report.finding_count_by_severity(Severity::Info), 1);
        assert!(report.has_blocking_findings(Severity::Error));
        assert_eq!(report.is_blocking(Severity::Blocker), false);
    }

    struct DummyAnalyzer;

    impl AnalysisBinary for DummyAnalyzer {
        fn analyzer_name(&self) -> &'static str {
            "dummy-analyzer"
        }

        fn run(&self, input: &AnalysisInput) -> Result<AnalysisReport, AnalysisError> {
            input.validate_target_exists()?;

            Ok(AnalysisReport::new(self.analyzer_name(), input.target.clone()))
        }
    }

    #[test]
    fn trait_usage_is_stable() {
        let analyzer = DummyAnalyzer;
        let input = AnalysisInput::new(".");
        let report = analyzer.run(&input).expect("dummy analyzer should run");

        assert_eq!(report.analyzer, "dummy-analyzer");
        assert_eq!(report.finding_count(), 0);
    }
}

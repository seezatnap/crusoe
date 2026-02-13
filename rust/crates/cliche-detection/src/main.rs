//! CLI entrypoint for the cliché and sentence-pattern detector.

use analysis_contracts::{AnalysisBinary, AnalysisInput, Severity};
use clap::{Parser, ValueEnum};
use cliche_detection::{
    ClicheDetector, ClicheDetectorConfig, DEFAULT_DUPLICATE_BLOCKER, DEFAULT_DUPLICATE_ERROR,
    DEFAULT_DUPLICATE_WARNING, DEFAULT_CLICHE_BLOCKER, DEFAULT_CLICHE_ERROR, DEFAULT_CLICHE_WARNING,
    DEFAULT_PATTERN_BLOCKER, DEFAULT_PATTERN_ERROR, DEFAULT_PATTERN_WARNING,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "cliche-detection")]
#[command(
    about = "Detect repetitive clichés and sentence patterns in chapter prose with literary thresholds."
)]
struct Cli {
    /// Chapter markdown file, or directory containing chapter files.
    target: PathBuf,

    /// Duplicate exact-sentence warning threshold.
    #[arg(long, default_value_t = DEFAULT_DUPLICATE_WARNING)]
    duplicate_warning: usize,

    /// Duplicate exact-sentence error threshold.
    #[arg(long, default_value_t = DEFAULT_DUPLICATE_ERROR)]
    duplicate_error: usize,

    /// Duplicate exact-sentence blocker threshold.
    #[arg(long, default_value_t = DEFAULT_DUPLICATE_BLOCKER)]
    duplicate_blocker: usize,

    /// Opening-pattern warning threshold.
    #[arg(long, default_value_t = DEFAULT_PATTERN_WARNING)]
    pattern_warning: usize,

    /// Opening-pattern error threshold.
    #[arg(long, default_value_t = DEFAULT_PATTERN_ERROR)]
    pattern_error: usize,

    /// Opening-pattern blocker threshold.
    #[arg(long, default_value_t = DEFAULT_PATTERN_BLOCKER)]
    pattern_blocker: usize,

    /// Cliché phrase warning threshold.
    #[arg(long, default_value_t = DEFAULT_CLICHE_WARNING)]
    cliche_warning: usize,

    /// Cliché phrase error threshold.
    #[arg(long, default_value_t = DEFAULT_CLICHE_ERROR)]
    cliche_error: usize,

    /// Cliché phrase blocker threshold.
    #[arg(long, default_value_t = DEFAULT_CLICHE_BLOCKER)]
    cliche_blocker: usize,

    /// Number of words used to detect sentence-opening pattern repetition.
    #[arg(long, default_value_t = 4)]
    opening_word_count: usize,

    /// Minimum exact duplicate sentence length (words).
    #[arg(long, default_value_t = 8)]
    min_duplicate_words: usize,

    /// Directory for per-chapter reports.
    #[arg(long, default_value = "writing-artifacts/quality")]
    artifact_dir: PathBuf,

    /// Do not write per-chapter reports.
    #[arg(long)]
    no_artifact: bool,

    /// Exit with non-zero if any finding is at least this severity.
    #[arg(long, default_value = "warning")]
    fail_on: CliSeverity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum CliSeverity {
    Info,
    Warning,
    Error,
    Blocker,
}

impl From<CliSeverity> for Severity {
    fn from(value: CliSeverity) -> Self {
        match value {
            CliSeverity::Info => Severity::Info,
            CliSeverity::Warning => Severity::Warning,
            CliSeverity::Error => Severity::Error,
            CliSeverity::Blocker => Severity::Blocker,
        }
    }
}

fn main() {
    let args = Cli::parse();
    let fail_threshold = Severity::from(args.fail_on);

    let config = ClicheDetectorConfig {
        duplicate_warning_threshold: args.duplicate_warning,
        duplicate_error_threshold: args.duplicate_error,
        duplicate_blocker_threshold: args.duplicate_blocker,
        opening_pattern_warning_threshold: args.pattern_warning,
        opening_pattern_error_threshold: args.pattern_error,
        opening_pattern_blocker_threshold: args.pattern_blocker,
        cliche_warning_threshold: args.cliche_warning,
        cliche_error_threshold: args.cliche_error,
        cliche_blocker_threshold: args.cliche_blocker,
        opening_word_count: args.opening_word_count,
        min_duplicate_sentence_words: args.min_duplicate_words,
    };
    let analyzer = ClicheDetector::new(config);

    let targets = match collect_targets(&args.target) {
        Ok(values) => values,
        Err(err) => {
            eprintln!("Unable to find targets from {}: {}", args.target.display(), err);
            process::exit(2);
        }
    };

    if targets.is_empty() {
        eprintln!("No chapter-like files found at {}", args.target.display());
        process::exit(2);
    }

    let mut should_fail = false;

    for target in targets {
        let input = AnalysisInput::new(&target);
        let report = match analyzer.run(&input) {
            Ok(report) => report,
            Err(err) => {
                eprintln!("Failed to analyze {}: {err}", target.display());
                should_fail = true;
                continue;
            }
        };

        print_report(&report);

        if !args.no_artifact {
            if let Err(err) = write_artifact_report(&args.artifact_dir, &report) {
                eprintln!(
                    "Failed writing artifact for {}: {}",
                    report.target.display(),
                    err
                );
                should_fail = true;
            }
        }

        if report.is_blocking(fail_threshold) {
            should_fail = true;
        }
    }

    if should_fail {
        process::exit(1);
    }
}

fn is_markdown(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => matches!(
            ext.to_ascii_lowercase().as_str(),
            "md" | "markdown" | "txt"
        ),
        None => false,
    }
}

fn collect_targets(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if path.is_file() {
        if is_markdown(path) {
            paths.push(path.to_path_buf());
        }
        return Ok(paths);
    }

    if !path.is_dir() {
        return Ok(paths);
    }

    fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_dir() {
                walk(&file_path, out)?;
            } else if is_markdown(&file_path) {
                out.push(file_path);
            }
        }
        Ok(())
    }

    walk(path, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn report_name() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0u64, |duration| duration.as_secs());
    format!("cliche-report-{secs}.md")
}

fn write_artifact_report(artifact_root: &Path, report: &analysis_contracts::AnalysisReport) -> std::io::Result<PathBuf> {
    let chapter_name = report
        .target
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("chapter");

    let mut chapter_dir = artifact_root.to_path_buf();
    chapter_dir.push(chapter_name);
    fs::create_dir_all(&chapter_dir)?;

    let mut output_path = chapter_dir;
    output_path.push(report_name());

    let mut out = String::new();
    out.push_str(&format!("# Cliche Detection Report: {}\n\n", report.target.display()));
    out.push_str("## Summary\n");
    out.push_str(&format!("- Analyzer: {}\n", report.analyzer));
    out.push_str(&format!("- Findings: {}\n", report.findings.len()));

    for (key, value) in report.metadata.iter() {
        out.push_str(&format!("- {key}: {value}\n"));
    }

    out.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No issues detected.\n");
    } else {
        for finding in &report.findings {
            let location = finding
                .location
                .as_ref()
                .and_then(|loc| Some(format!("{}:{}", loc.line.unwrap_or(0), loc.column.unwrap_or(0))))
                .unwrap_or_else(|| "unknown".to_string());

            out.push_str(&format!(
                "- [{}] {} at {location}\n  - code: {}\n  - message: {}\n",
                finding.severity, finding.location.as_ref().and_then(|l| l.path.file_name()).and_then(|n| n.to_str()).unwrap_or("-"), 
                finding.code,
                finding.message
            ));
            if let Some(suggestion) = &finding.suggestion {
                out.push_str(&format!("  - suggestion: {suggestion}\n"));
            }
        }
    }

    fs::write(&output_path, out)?;
    Ok(output_path)
}

fn print_report(report: &analysis_contracts::AnalysisReport) {
    println!("==> {}", report.target.display());
    println!("Findings: {}", report.findings.len());

    if report.findings.is_empty() {
        println!("No issues detected.\n");
        return;
    }

    let mut findings = report.findings.clone();
    findings.sort_by(|left, right| {
        left.severity
            .rank()
            .cmp(&right.severity.rank())
            .then_with(|| left.code.cmp(&right.code))
    });

    for finding in &findings {
        let location = finding
            .location
            .as_ref()
            .map(|loc| format!("{}:{}:{}", loc.path.display(), loc.line.unwrap_or(0), loc.column.unwrap_or(0)))
            .unwrap_or_else(|| "-".to_string());

        println!("{} [{}] {} — {}", location, finding.severity, finding.code, finding.message);
    }
    println!();
}

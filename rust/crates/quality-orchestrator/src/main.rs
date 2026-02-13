//! Orchestrates #12-#14 checks plus basic chapter length validation.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use analysis_contracts::{AnalysisBinary, AnalysisError, AnalysisInput, AnalysisReport, Finding, Severity, SourceLocation};
use clap::{Parser, ValueEnum};
use cliche_detection::{ClicheDetector, ClicheDetectorConfig};
use continuity_check::{run_continuity_check, ContinuityOptions};
use voice_consistency::{VoiceConsistencyAnalyzer, VoiceConsistencyConfig};

#[derive(Parser)]
#[command(name = "quality-orchestrator")]
#[command(about = "Run all core quality analyzers for chapter prose in one command.")]
struct Cli {
    /// Chapter markdown file, or directory containing chapter files.
    target: PathBuf,

    /// Minimum expected chapter word count.
    #[arg(long, default_value_t = 4000)]
    min_words: usize,

    /// Maximum expected chapter word count.
    #[arg(long, default_value_t = 6000)]
    max_words: usize,

    /// Directory for per-chapter quality artifacts.
    #[arg(long, default_value = "writing-artifacts/quality")]
    artifact_dir: PathBuf,

    /// Exit with non-zero if any finding is at least this severity.
    #[arg(long, alias = "fail_on", default_value = "error")]
    fail_on: CliSeverity,

    /// Set this flag to skip artifact output.
    #[arg(long, alias = "no_artifact")]
    no_artifact: bool,
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

#[derive(Clone, Debug)]
struct OrchestratorConfig {
    min_words: usize,
    max_words: usize,
    artifact_dir: PathBuf,
    fail_on: Severity,
    write_artifacts: bool,
}

impl From<Cli> for OrchestratorConfig {
    fn from(cli: Cli) -> Self {
        Self {
            min_words: cli.min_words,
            max_words: cli.max_words,
            artifact_dir: cli.artifact_dir,
            fail_on: Severity::from(cli.fail_on),
            write_artifacts: !cli.no_artifact,
        }
    }
}

fn main() {
    let args = Cli::parse();
    let target = args.target.clone();
    let config: OrchestratorConfig = args.into();

    let targets = match collect_targets(&target) {
        Ok(values) => values,
        Err(err) => {
            eprintln!(
                "Unable to find chapter-like targets under {}: {err}",
                target.display()
            );
            process::exit(2);
        }
    };

    if targets.is_empty() {
        eprintln!("No chapter-like files found at {}", target.display());
        process::exit(2);
    }

    let mut should_fail = false;
    for target in targets {
        let reports = run_quality_suite(&target, config.min_words, config.max_words);

        should_fail = should_fail || report_has_blocking_finding(&reports, config.fail_on);

        println!("==> {}", target.display());
        print_run_summary(&reports, config.fail_on);

        if config.write_artifacts {
            match write_signed_quality_artifact(&config.artifact_dir, &target, &reports, config.fail_on) {
                Ok(path) => {
                    println!("   artifact: {}\n", path.display());
                }
                Err(err) => {
                    eprintln!(
                        "   failed to write quality artifact for {}: {err}",
                        target.display()
                    );
                    should_fail = true;
                }
            }
        }
    }

    if should_fail {
        process::exit(1);
    }
}

fn collect_targets(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut targets = Vec::new();

    if path.is_file() {
        if is_markdown(path) {
            targets.push(path.to_path_buf());
        }
        return Ok(targets);
    }

    if !path.is_dir() {
        return Ok(targets);
    }

    fn walk(current: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                walk(&entry_path, output)?;
            } else if is_markdown(&entry_path) {
                output.push(entry_path);
            }
        }

        Ok(())
    }

    walk(path, &mut targets)?;
    targets.sort();
    Ok(targets)
}

fn is_markdown(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown"),
        None => false,
    }
}

fn run_quality_suite(target: &Path, min_words: usize, max_words: usize) -> Vec<AnalysisReport> {
    let mut reports = Vec::new();
    let input = AnalysisInput::new(target);

    let cliche_detector = ClicheDetector::new(ClicheDetectorConfig::default());
    match cliche_detector.run(&input) {
        Ok(report) => reports.push(report),
        Err(err) => reports.push(orchestrator_failure_report(
            target,
            "cliche-detection",
            &format!("cliche-detection failed to analyze '{}': {err}", target.display()),
        )),
    }

    let voice = VoiceConsistencyAnalyzer::new(VoiceConsistencyConfig::default());
    match voice.run(&input) {
        Ok(report) => reports.push(report),
        Err(err) => reports.push(orchestrator_failure_report(
            target,
            "voice-consistency",
            &format!("voice-consistency failed to analyze '{}': {err}", target.display()),
        )),
    }

    match run_continuity_check(&input, ContinuityOptions::default()) {
        Ok(report) => reports.push(report),
        Err(err) => reports.push(orchestrator_failure_report(
            target,
            "continuity-check",
            &format!("continuity-check failed to analyze '{}': {err}", target.display()),
        )),
    }

    match run_length_check_report(target, min_words, max_words) {
        Ok(report) => reports.push(report),
        Err(err) => reports.push(orchestrator_failure_report(
            target,
            "length-check",
            &format!("length-check failed on '{}': {err}", target.display()),
        )),
    }

    reports
}

fn run_length_check_report(
    target: &Path,
    min_words: usize,
    max_words: usize,
) -> Result<AnalysisReport, AnalysisError> {
    let content = fs::read_to_string(target).map_err(|err| AnalysisError::Io {
        path: target.display().to_string(),
        reason: err.to_string(),
    })?;
    let words = count_words(&content);

    let mut report = AnalysisReport::new("length-check", target)
        .add_metadata("analyzer", "length-check")
        .add_metadata("target", target.display().to_string())
        .add_metadata("words", words.to_string())
        .add_metadata("min_words", min_words.to_string())
        .add_metadata("max_words", max_words.to_string());

    match words {
        0 => {
            report = report.add_finding(
                Finding::new(
                    "LENGTH-000",
                    "Chapter content is empty; expected prose content for this quality run.",
                    Severity::Blocker,
                )
                .with_location(SourceLocation::new(target, Some(1), Some(1)))
                .with_suggestion("Add chapter content before running length checks.".to_string()),
            );
        }
        words if words < min_words => {
            report = report.add_finding(
                Finding::new(
                    "LENGTH-001",
                    format!(
                        "Chapter appears short ({words} words). Minimum expected length is {min_words} words.",
                    ),
                    Severity::Error,
                )
                .with_location(SourceLocation::new(target, Some(1), Some(1)))
                .with_suggestion("Expand prose to the required 4,000–6,000-word window, and keep prose density high.".to_string()),
            );
        }
        words if words > max_words => {
            report = report.add_finding(
                Finding::new(
                    "LENGTH-002",
                    format!(
                        "Chapter is long ({words} words). Maximum expected length is {max_words} words.",
                    ),
                    Severity::Error,
                )
                .with_location(SourceLocation::new(target, Some(1), Some(1)))
                .with_suggestion("Trim excess prose or split planned material into adjacent chapter beats.".to_string()),
            );
        }
        _ => {}
    }

    Ok(report)
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn report_has_blocking_finding(reports: &[AnalysisReport], threshold: Severity) -> bool {
    reports.iter().any(|report| report.has_blocking_findings(threshold))
}

fn orchestrator_failure_report(target: &Path, analyzer: &str, message: &str) -> AnalysisReport {
    AnalysisReport::new(analyzer, target)
        .add_metadata("analyzer", analyzer)
        .add_finding(
            Finding::new("ORCH-ERR", message.to_string(), Severity::Blocker)
                .with_location(SourceLocation::new(target, Some(1), Some(1)))
                .with_suggestion(
                    "Re-run the failed analyzer after fixing the source content or path issue.".to_string(),
                ),
        )
}

fn print_run_summary(reports: &[AnalysisReport], fail_threshold: Severity) {
    let total_findings: usize = reports.iter().map(AnalysisReport::finding_count).sum();
    let passing = !report_has_blocking_finding(reports, fail_threshold);
    if passing {
        println!("   findings: {total_findings} (passes threshold)");
    } else {
        println!("   findings: {total_findings} (failed threshold)");
    }

    for report in reports {
        let blocking = report.finding_count_by_severity(Severity::Error)
            + report.finding_count_by_severity(Severity::Blocker);
        println!(
            "   {:<17} findings: {:<4} blocker+error: {}",
            report.analyzer,
            report.finding_count(),
            blocking
        );
    }
}

fn write_signed_quality_artifact(
    artifact_root: &Path,
    target: &Path,
    reports: &[AnalysisReport],
    fail_threshold: Severity,
) -> std::io::Result<PathBuf> {
    let chapter = chapter_name(target);
    let mut chapter_dir = artifact_root.to_path_buf();
    chapter_dir.push(&chapter);
    fs::create_dir_all(&chapter_dir)?;

    let timestamp = report_timestamp();
    let mut artifact_path = chapter_dir;
    artifact_path.push(format!("quality-orchestrator-{timestamp}.md"));

    let mut body = String::new();
    writeln!(&mut body, "# Quality Orchestrator Report").expect("write header");
    writeln!(&mut body, "- target: {}", target.display()).expect("write target");
    writeln!(&mut body, "- generated_at_unix: {}", timestamp).expect("write timestamp");
    writeln!(&mut body, "- fail_threshold: {}", fail_threshold).expect("write threshold");

    let total_findings: usize = reports.iter().map(AnalysisReport::finding_count).sum();
    let total_blocking = reports
        .iter()
        .filter(|report| report.has_blocking_findings(fail_threshold))
        .count();
    let overall_status = if total_blocking > 0 { "failed" } else { "passed" };
    writeln!(&mut body, "- total_findings: {}", total_findings).expect("write total findings");
    writeln!(&mut body, "- total_blocking_reports: {}", total_blocking).expect("write total blocking");
    writeln!(&mut body, "- status: {}\n", overall_status).expect("write status");

    let mut reports_sorted = reports.to_vec();
    reports_sorted.sort_by(|left, right| left.analyzer.cmp(&right.analyzer));

    for report in &reports_sorted {
        writeln!(&mut body, "## {}", report.analyzer).expect("write analyzer");
        writeln!(&mut body, "- findings: {}", report.finding_count()).expect("write count");
        writeln!(&mut body, "- target: {}", report.target.display()).expect("write report target");
        writeln!(&mut body, "- metadata:").expect("write metadata heading");
        for (name, value) in report.metadata.iter() {
            writeln!(&mut body, "  - {name}: {value}").expect("write metadata");
        }

        if report.findings.is_empty() {
            writeln!(&mut body, "- findings: none\n").expect("write empty findings");
            continue;
        }

        let mut findings = report.findings.clone();
        findings.sort_by(|left, right| {
            left.severity
                .rank()
                .cmp(&right.severity.rank())
                .then_with(|| left.code.cmp(&right.code))
        });

        writeln!(&mut body, "- details:").expect("write details heading");
        for finding in findings {
            let location = finding.location.as_ref().map_or_else(
                || "unknown".to_string(),
                |loc| {
                    let path = loc.path.to_string_lossy();
                    format!("{path}:{}:{}", loc.line.unwrap_or(0), loc.column.unwrap_or(0))
                },
            );

            writeln!(
                &mut body,
                "  - [{}] {} at {location}",
                finding.severity,
                finding.code,
            )
            .expect("write finding header");
            writeln!(&mut body, "    - {}", finding.message).expect("write finding message");

            if let Some(suggestion) = finding.suggestion.as_deref() {
                writeln!(&mut body, "    - suggestion: {suggestion}").expect("write suggestion");
            }
        }

        writeln!(&mut body).expect("write spacing");
    }

    let signature = signature(&body);
    writeln!(&mut body).expect("write separator");
    writeln!(&mut body, "signature: {signature}").expect("write signature");

    fs::write(&artifact_path, body)?;
    Ok(artifact_path)
}

fn signature(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{:016x}", hash)
}

fn report_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

fn chapter_name(target: &Path) -> String {
    target
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("chapter")
        .to_string()
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{
        chapter_name,
        collect_targets,
        count_words,
        run_length_check_report,
        signature,
        write_signed_quality_artifact,
        Cli,
        CliSeverity,
    };
    use analysis_contracts::Severity;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{fs, path::PathBuf};

    fn temp_root(prefix: &str) -> PathBuf {
        let mut root = std::env::temp_dir();
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_micros())
            .to_string();
        root.push(format!("infinite-domain-quality-orchestrator-{prefix}-{micros}"));
        fs::create_dir_all(&root).expect("create temporary root");
        root
    }

    #[test]
    fn collects_only_markdown_targets() {
        let root = temp_root("targets");
        let markdown = root.join("chapter-01.md");
        let other = root.join("notes.txt");
        fs::write(&markdown, "# chapter").expect("write chapter");
        fs::write(&other, "notes").expect("write notes");

        let targets = collect_targets(&root).expect("collect targets");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].file_name().unwrap(), "chapter-01.md");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn count_words_is_whitespace_based() {
        assert_eq!(count_words("one two three"), 3);
        assert_eq!(count_words("multiple\nline\nspaces"), 3);
        assert_eq!(count_words("  punctuation, counts too! "), 3);
    }

    #[test]
    fn length_check_reports_short_and_long_values() {
        let root = temp_root("length");
        let short = root.join("chapter-01.md");
        fs::write(&short, "one two three").expect("write short chapter");

        let short_report = run_length_check_report(&short, 5, 10).expect("run length check");
        assert_eq!(short_report.analyzer, "length-check");
        assert!(short_report.has_blocking_findings(Severity::Error));

        fs::write(&short, &("word ".repeat(20))).expect("write long chapter");
        let long_report = run_length_check_report(&short, 5, 10).expect("run length check");
        assert!(long_report.has_blocking_findings(Severity::Error));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn writes_signed_artifact_and_includes_signature() {
        let root = temp_root("artifact");
        let chapter = root.join("chapter-01.md");
        fs::write(&chapter, &("word ".repeat(20))).expect("write chapter");

        let report = run_length_check_report(&chapter, 5, 10).expect("run length check");
        let artifact_dir = root.join("writing-artifacts").join("quality");
        let path =
            write_signed_quality_artifact(&artifact_dir, &chapter, std::slice::from_ref(&report), Severity::Error)
                .expect("write artifact");

        let content = fs::read_to_string(&path).expect("read artifact");
        assert!(content.contains("signature:"));
        assert!(content.contains("fnv1a64:"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn signature_is_deterministic_for_same_input() {
        let first = signature("stable-text");
        let second = signature("stable-text");
        let third = signature("different-text");
        assert_eq!(first, second);
        assert_ne!(first, third);
    }

    #[test]
    fn cli_parses_hyphenated_quality_orchestrator_flags() {
        let args = Cli::parse_from([
            "quality-orchestrator",
            "final-draft-chapters",
            "--fail-on",
            "blocker",
            "--no-artifact",
        ]);

        assert_eq!(args.fail_on, CliSeverity::Blocker);
        assert!(args.no_artifact);
    }

    #[test]
    fn cli_parses_legacy_underscored_quality_orchestrator_flags() {
        let args = Cli::parse_from([
            "quality-orchestrator",
            "final-draft-chapters",
            "--fail_on",
            "warning",
            "--no_artifact",
        ]);

        assert_eq!(args.fail_on, CliSeverity::Warning);
        assert!(args.no_artifact);
    }

    #[test]
    fn chapter_name_falls_back_to_default_when_missing() {
        let missing = PathBuf::from("");
        let name = chapter_name(&missing);
        assert_eq!(name, "chapter");
    }
}

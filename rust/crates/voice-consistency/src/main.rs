//! CLI entrypoint for dialogue/POV/voice consistency checks.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use analysis_contracts::AnalysisInput;
use analysis_contracts::AnalysisBinary;
use clap::{Parser, ValueEnum};
use voice_consistency::{VoiceConsistencyAnalyzer, VoiceConsistencyConfig};

#[derive(Parser)]
#[command(name = "voice-consistency")]
#[command(
    about = "Validate dialogue tags, POV markers, and character voice continuity with confidence scoring."
)]
struct Cli {
    /// Chapter markdown file, or directory containing chapter files.
    target: PathBuf,

    #[arg(long, default_value = "0.95")]
    explicit_name_confidence: f32,

    #[arg(long, default_value = "0.42")]
    pronoun_tag_confidence: f32,

    #[arg(long, default_value = "0.72")]
    continuation_confidence: f32,

    #[arg(long, default_value = "0.70")]
    warning_confidence: f32,

    #[arg(long, default_value = "0.45")]
    error_confidence: f32,

    #[arg(long, default_value = "0.25")]
    blocker_confidence: f32,

    #[arg(long, default_value = "0.12")]
    ambiguity_margin: f32,

    #[arg(long, default_value = "0.55")]
    voice_warning_similarity: f32,

    #[arg(long, default_value = "0.40")]
    voice_error_similarity: f32,

    #[arg(long, default_value = "0.25")]
    voice_blocker_similarity: f32,

    #[arg(long, default_value_t = 24)]
    voice_min_sample_words: usize,

    #[arg(long, default_value_t = 8)]
    voice_top_words: usize,

    /// Directory for per-chapter reports.
    #[arg(long, default_value = "writing-artifacts/quality")]
    artifact_dir: PathBuf,

    /// Do not write reports.
    #[arg(long)]
    no_artifact: bool,

    /// Exit with non-zero if any finding reaches this severity.
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

impl From<CliSeverity> for analysis_contracts::Severity {
    fn from(value: CliSeverity) -> Self {
        match value {
            CliSeverity::Info => analysis_contracts::Severity::Info,
            CliSeverity::Warning => analysis_contracts::Severity::Warning,
            CliSeverity::Error => analysis_contracts::Severity::Error,
            CliSeverity::Blocker => analysis_contracts::Severity::Blocker,
        }
    }
}

fn main() {
    let args = Cli::parse();
    let fail_threshold = analysis_contracts::Severity::from(args.fail_on);

    let config = VoiceConsistencyConfig {
        explicit_name_confidence: args.explicit_name_confidence,
        pronoun_tag_confidence: args.pronoun_tag_confidence,
        continuation_confidence: args.continuation_confidence,
        warning_confidence: args.warning_confidence,
        error_confidence: args.error_confidence,
        blocker_confidence: args.blocker_confidence,
        ambiguity_margin: args.ambiguity_margin,
        voice_warning_similarity: args.voice_warning_similarity,
        voice_error_similarity: args.voice_error_similarity,
        voice_blocker_similarity: args.voice_blocker_similarity,
        voice_min_sample_words: args.voice_min_sample_words,
        voice_top_words: args.voice_top_words,
        artifact_name_prefix: "voice-consistency",
    };
    let analyzer = VoiceConsistencyAnalyzer::new(config);

    let targets = match collect_targets(&args.target) {
        Ok(values) => values,
        Err(err) => {
            eprintln!("Unable to find targets from {}: {err}", args.target.display());
            std::process::exit(2);
        }
    };

    if targets.is_empty() {
        eprintln!("No chapter-like files found at {}", args.target.display());
        std::process::exit(2);
    }

    let mut fail = false;
    for target in targets {
        let input = AnalysisInput::new(&target);
        let report = match analyzer.run(&input) {
            Ok(report) => report,
            Err(err) => {
                eprintln!("Failed to analyze {}: {err}", target.display());
                fail = true;
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
                fail = true;
            }
        }

        if report.has_blocking_findings(fail_threshold) {
            fail = true;
        }
    }

    if fail {
        std::process::exit(1);
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
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out)?;
            } else if is_markdown(&path) {
                out.push(path);
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
    format!("voice-consistency-report-{secs}.md")
}

fn write_artifact_report(
    artifact_root: &Path,
    report: &analysis_contracts::AnalysisReport,
) -> std::io::Result<PathBuf> {
    let chapter = report
        .target
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("chapter");

    let mut chapter_dir = artifact_root.to_path_buf();
    chapter_dir.push(chapter);
    fs::create_dir_all(&chapter_dir)?;

    let mut path = chapter_dir;
    path.push(report_name());

    let mut out = String::new();
            out.push_str(&format!("# Voice/POV Report: {}\n\n", report.target.display()));
    out.push_str("## Summary\n");
    out.push_str(&format!("- Analyzer: {}\n", report.analyzer));
    out.push_str(&format!("- Findings: {}\n", report.findings.len()));

    for (name, value) in report.metadata.iter() {
        out.push_str(&format!("- {name}: {value}\n"));
    }

    out.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No issues detected.\n");
    } else {
        for finding in &report.findings {
            let location = finding
                .location
                .as_ref()
                .map(|loc| format!("{}:{}", loc.path.display(), loc.line.unwrap_or(0)))
                .unwrap_or_else(|| "-".to_string());
            out.push_str(&format!(
                "- {:?} {}\n  - code: {}\n  - message: {}\n",
                finding.severity, location, finding.code, finding.message
            ));
            if let Some(suggestion) = &finding.suggestion {
                out.push_str(&format!("  - suggestion: {}\n", suggestion));
            }
        }
    }

    fs::write(&path, out)?;
    Ok(path)
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

    for finding in findings {
        let location = finding.location.as_ref().map_or_else(
            || "-".to_string(),
            |loc| format!("{}:{}:{}", loc.path.display(), loc.line.unwrap_or(0), loc.column.unwrap_or(0)),
        );
        println!(
            "{} [{}] {} — {}",
            location, finding.severity, finding.code, finding.message
        );
    }
    println!();
}

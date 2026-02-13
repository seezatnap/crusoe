use std::path::PathBuf;

use clap::ValueEnum;
use clap::Parser;

use analysis_contracts::AnalysisInput;
use continuity_check::{render_report_text_diff, run_continuity_check, ContinuityOptions};

#[derive(Parser)]
#[command(name = "continuity-check")]
#[command(author = "Agent Carlos <agent-C@swarm.local>")]
#[command(version = "0.1.0")]
struct Cli {
    target: PathBuf,

    #[arg(long, default_value_t = 0)]
    max_timeline_backstep: i32,

    #[arg(long, default_value_t = 2)]
    max_drift_jump: i32,

    #[arg(long, default_value_t = 0)]
    max_drift_backstep: i32,

    #[arg(long, value_delimiter = ',', default_value = "unknown,hint,evidence,confirmed,public")]
    lore_state_order: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    reveal_order: Vec<String>,

    #[arg(long, value_enum, default_value_t = SeverityArg::Error)]
    fail_on: SeverityArg,
}

#[derive(Clone, Copy, ValueEnum)]
enum SeverityArg {
    Info,
    Warning,
    Error,
    Blocker,
}

impl From<SeverityArg> for analysis_contracts::Severity {
    fn from(value: SeverityArg) -> Self {
        match value {
            SeverityArg::Info => analysis_contracts::Severity::Info,
            SeverityArg::Warning => analysis_contracts::Severity::Warning,
            SeverityArg::Error => analysis_contracts::Severity::Error,
            SeverityArg::Blocker => analysis_contracts::Severity::Blocker,
        }
    }
}

fn main() {
    let args = Cli::parse();

    let options = ContinuityOptions {
        max_timeline_backstep: args.max_timeline_backstep,
        max_drift_jump: args.max_drift_jump,
        max_drift_backstep: args.max_drift_backstep,
        reveal_order: args.reveal_order,
        lore_states: args.lore_state_order,
    };

    let input = AnalysisInput::new(&args.target);
    let report = run_continuity_check(&input, options);

    match report {
        Ok(report) => {
            print!("{}", render_report_text_diff(&report));
            if report.has_blocking_findings(analysis_contracts::Severity::from(args.fail_on)) {
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Continuity check failed: {err}");
            std::process::exit(2);
        }
    }
}


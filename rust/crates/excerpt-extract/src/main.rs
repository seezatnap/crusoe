use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use excerpt_extract::{pick_examples, collect_examples, ExtractMode};

#[derive(Parser)]
#[command(name = "style-reference")]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(long, default_value_t = 5)]
    count: usize,

    #[arg(value_name = "SOURCE_DIR", default_value = "style-reference")]
    source_dir: PathBuf,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Narrative,
    Dialogue,
}

impl From<Mode> for ExtractMode {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Narrative => ExtractMode::Narrative,
            Mode::Dialogue => ExtractMode::Dialogue,
        }
    }
}

fn main() {
    let args = Cli::parse();

    let mode = ExtractMode::from(args.mode);
    match collect_examples(&args.source_dir, mode) {
        Ok(examples) => {
            let selected = pick_examples(examples, args.count);
            match serde_json::to_string_pretty(&selected) {
                Ok(json) => println!("{json}"),
                Err(err) => {
                    eprintln!("Failed to encode JSON: {err}");
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

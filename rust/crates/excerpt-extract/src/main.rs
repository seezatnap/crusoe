use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use excerpt_extract::{pick_examples, collect_cached_examples, ExtractMode};

#[derive(Parser)]
#[command(name = "style-reference")]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(long, default_value_t = 5)]
    count: usize,
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
    let source_dir = PathBuf::from(".downloads/style-reference");

    let mode = ExtractMode::from(args.mode);
    match collect_cached_examples(&source_dir, mode) {
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

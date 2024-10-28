use std::{fs, path::PathBuf};

use clap::{command, Parser};
use pmc_interpreter::{interactive::InteractiveState, parser::parse, processor::PROC};

/// A simple PMC
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Code filepath
    filepath: PathBuf,

    /// Enable interactive mode
    #[arg(short, long)]
    interactive: bool,
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.filepath)
        .expect("Expected sucessfull file read");

    let mut mem = parse(&content);
    let mut proc = PROC::new();
    // mem.print_range(0..30);
    let interactive_state = if cli.interactive {
        InteractiveState::ENABLED
    } else {
        InteractiveState::DISABLED
    };
    proc.run(&mut mem, interactive_state);
    //mem.dump_all();
}

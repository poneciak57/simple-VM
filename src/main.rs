use std::{fs, path::PathBuf};

use clap::{command, Parser};
use pmc_interpreter::{parser::parse, processor::PROC};

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
    proc.run(&mut mem, cli.interactive);
    mem.dump_all(Some("mem.out".to_string()));
    //mem.dump_all();
}

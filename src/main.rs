// src/main.rs
/*
 * Main executable for ArtificialEraCreator
 */

use clap::Parser;
use artificialeracreator::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialEraCreator - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

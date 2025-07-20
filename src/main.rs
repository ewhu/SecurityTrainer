// src/main.rs
/*
 * Main executable for SecurityTrainer
 */

use clap::Parser;
use securitytrainer::{Result, run};

#[derive(Parser)]
#[command(version, about = "SecurityTrainer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

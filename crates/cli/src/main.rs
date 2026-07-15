use clap::{Parser};

use crate::commands::Command::Analysis;

mod commands;
mod utils;

#[derive(Parser)]
#[command(
    name = "Mantis",
    version, 
    about = "An all in one tool to help score synchronization video makers.", 
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: commands::Command
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Analysis(a) => commands::analysis::handle(a),
    }
}
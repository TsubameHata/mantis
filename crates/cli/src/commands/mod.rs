use clap::Subcommand;

pub mod analysis;

#[derive(Subcommand)]
pub enum Command {
    /// Analyze score pages to generate split lines, which should be further inspected manually
    Analysis(analysis::Args)
}

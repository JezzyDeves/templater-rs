use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.0.1")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create(CreateArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    path: PathBuf,
    #[arg(short, long)]
    verbose: bool,
}

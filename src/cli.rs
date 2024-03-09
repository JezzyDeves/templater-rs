use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.0.1")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create(CreateArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    pub from_path: PathBuf,
    pub dest_path: PathBuf,
    #[arg(short, long)]
    pub verbose: bool,
}

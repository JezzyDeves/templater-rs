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
    /// Clones files to dest_path
    Create(CreateArgs),
    /// Adds a new template to the registry
    Add(AddArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    pub from_path: PathBuf,
    pub dest_path: PathBuf,
    #[arg(short, long)]
    pub verbose: bool,
}
#[derive(Args)]
pub struct AddArgs {
    pub from_path: PathBuf,
    pub name: String,
    #[arg(short, long)]
    pub verbose: bool,
}

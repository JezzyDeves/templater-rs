use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version = "a0.1.0")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clones files to dest_path
    Create,
    /// Adds a new template to the registry
    Add(AddArgs),
    /// Lists the available templates
    List,
}
#[derive(Args)]
pub struct AddArgs {
    pub from_path: PathBuf,
    pub name: String,
    #[arg(short, long)]
    pub verbose: bool,
}

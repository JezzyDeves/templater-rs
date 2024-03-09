use std::path::Path;

use clap::Parser;
use cli::Cli;
use fs_extra::{copy_items, dir::CopyOptions};
use registry::{create_new_registry, list_all_templates, write_to_registry};

mod cli;
mod constants;
mod registry;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Create(args) => {
            let options = CopyOptions::new();

            let args_path = args.from_path.to_str().expect("Something went wrong");
            if args.verbose {
                println!(
                    "Copying from directory {} to {}",
                    args_path,
                    args.dest_path.to_str().expect("Something went wrong")
                );
            }
            let from_path = vec![args_path];

            let _ = copy_items(&from_path, args.dest_path, &options);
        }
        cli::Commands::Add(args) => {
            let registry_path = Path::new("./registry.json");

            if registry_path.exists() {
                write_to_registry(registry_path, args);
            } else {
                create_new_registry(args);
            }
        }
        cli::Commands::List => {
            list_all_templates();
        }
    }
}

use std::{fs, path::Path, process::exit};

use clap::Parser;
use cli::Cli;
use dialoguer::{Input, Select};
use fs_extra::{copy_items, dir::CopyOptions};
use owo_colors::OwoColorize;
use registry::{create_new_registry, list_all_templates, write_to_registry, Registry};

mod cli;
mod constants;
mod registry;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Create => {
            let registry_path = Path::new("./registry/registry.json");

            if !registry_path.exists() {
                println!("{}", "The registry doesn't exist yet. Run \"templater-rs help add\" for instructions on how to add new templates to the registry".red());
                exit(1);
            }

            let registry_data = fs::read_to_string(registry_path)
                .expect("Something went wrong while attempting to read the registry");

            let registry: Registry = serde_json::from_str(&registry_data)
                .expect("Something went wrong in trying to deserialize the registry");

            if registry.registered_templates.len() == 0 {
                println!("{}", "The registry exists but somehow there are no templates. Run \"templater-rs help add\" for instructions on how to add new templates to the registry".red());
                exit(1);
            }

            let items: Vec<String> = registry
                .registered_templates
                .into_iter()
                .map(|template| template.name)
                .collect();

            let selection = Select::new()
                .with_prompt("What template would you like to use?")
                .items(&items)
                .interact()
                .unwrap();

            let dest_path_input = Input::<String>::new()
                .with_prompt("Where should the new project be placed?")
                .interact_text()
                .unwrap();

            let dest_path = Path::new(&dest_path_input);

            let from_path = vec![format!("./templates/{}", items[selection])];

            let options = CopyOptions::new();

            copy_items(&from_path, dest_path, &options)
                .expect("Something went wrong while copying files");
        }
        cli::Commands::Add(args) => {
            let registry_path = Path::new("./registry/registry.json");

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

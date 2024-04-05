use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use cli::Cli;
use constants::SERIALIZATION_FAILURE_MSG;
use dialoguer::{Input, Select};
use fs_utils::copy_dir;
use registry::{create_new_registry, list_all_templates, validate_registry, write_to_registry};
use walkdir::WalkDir;

mod cli;
mod constants;
mod fs_utils;
mod registry;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Create => {
            let registry = validate_registry();

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

            let from_path = PathBuf::from(format!("./registry/templates/{}", items[selection]));

            let total_files = WalkDir::new(&from_path).into_iter().count() - 1;
            let mut completed = 0;

            copy_dir(&from_path, dest_path, &mut completed, total_files)
                .expect("Something went wrong while copying files")
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
        cli::Commands::Remove => {
            let mut registry = validate_registry();

            let items: &Vec<&String> = &registry
                .registered_templates
                .iter()
                .map(|template| &template.name)
                .collect();

            let selection = Select::new()
                .with_prompt("What template would you like to remove?")
                .items(&items)
                .interact()
                .unwrap();

            let template_path_string = format!("./registry/templates/{}", items[selection]);
            let template_path = Path::new(&template_path_string);

            registry.registered_templates.remove(selection);

            let json_data = serde_json::to_string(&registry).expect(SERIALIZATION_FAILURE_MSG);

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open("./registry/registry.json")
                .expect("Something went wrong while attempting to open the registry file");

            file.write_all(json_data.as_bytes())
                .expect("Failed to write to the registry");

            fs::remove_dir_all(template_path)
                .expect("Something went wrong while trying to remove the template");
        }
    }
}

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use fs_extra::{copy_items, dir::CopyOptions};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{
    cli,
    constants::{REGISTRY_READ_FAILURE_MSG, SERIALIZATION_FAILURE_MSG},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub registered_templates: Vec<RegisteredTemplate>,
}

impl Registry {
    pub fn new(registered_templates: Vec<RegisteredTemplate>) -> Self {
        Self {
            registered_templates,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredTemplate {
    pub name: String,
}

pub fn write_to_registry(registry_path: &Path, args: cli::AddArgs) {
    let registry_data = fs::read_to_string(registry_path)
        .expect("Something went wrong while attempting to read the registry");

    let mut registry: Registry = serde_json::from_str(&registry_data)
        .expect("Something went wrong in trying to deserialize the registry");

    registry.registered_templates.push(RegisteredTemplate {
        name: args.name.clone(),
    });

    let json_data = serde_json::to_string(&registry).expect(SERIALIZATION_FAILURE_MSG);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./registry.json")
        .expect(REGISTRY_READ_FAILURE_MSG);

    file.write_all(json_data.as_bytes())
        .expect("Failed to write to the registry");

    let templates_location = format!("./templates/{}", args.name);

    fs::create_dir_all(&templates_location).expect("Failed to create directory");

    let dest_path: &Path = Path::new(templates_location.as_str());

    copy_items(
        &vec![args
            .from_path
            .to_str()
            .expect("Something went wrong with the file path")],
        dest_path,
        &CopyOptions::new(),
    )
    .expect("Something went wrong while cloning the directory");
}

pub fn create_new_registry(args: cli::AddArgs) {
    let template = RegisteredTemplate {
        name: args.name.clone(),
    };

    let registry = Registry::new(vec![template]);

    let json_data = serde_json::to_string(&registry).expect(SERIALIZATION_FAILURE_MSG);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("./registry.json")
        .expect("Something went wrong while attempting to open the registry file");

    file.write_all(json_data.as_bytes())
        .expect("Failed to write to the registry");

    let templates_location = format!("./templates/{}", args.name);

    fs::create_dir_all(&templates_location).expect("Failed to create directory");

    let dest_path: &Path = Path::new(templates_location.as_str());

    copy_items(
        &vec![args
            .from_path
            .to_str()
            .expect("Something went wrong with the file path")],
        dest_path,
        &CopyOptions::new(),
    )
    .expect("Something went wrong while cloning the directory");
}

pub fn list_all_templates() {
    let registry_path = Path::new("./registry.json");

    if registry_path.exists() {
        let registry = fs::read_to_string(registry_path).expect(REGISTRY_READ_FAILURE_MSG);

        let registry: Registry = serde_json::from_str(&registry)
            .expect("Something went wrong in trying to deserialize the registry");

        println!("{}", "Available Templates:".green());
        for template in registry.registered_templates {
            println!("{}", template.name);
        }
    } else {
        println!(
            "{}",
            "No registry has been setup yet. Run \"templater-rs help add\" for more info on how to add templates"
                .red()
        )
    }
}

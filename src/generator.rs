//! coloritty generator

use std::fs;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

// Reads the contents in 'config_file_path', and parses them into a 'Yaml' instance.
// 
// This function will panic if any of the steps fail.
fn read_config_contents(config_file_path: &Path) -> Yaml {
    // The error_msg to display upon failure.
    let error_msg: &str = format!("Failed to read contents of: {}", config_file_path.to_str().unwrap());

    // Read the file contents and panic if it fails.
    let config_file_contents = fs::read_to_string(config_file_path).expect(error_msg);

    // Parse the file contents into a YAML documents, and panic if it fails.
    let documents = YamlLoader::load_from_str(&config_file_contents).expect(error_msg);

    return documents[0];
}

pub fn list(config_file_path: &Path) {
    let config = read_config_contents(config_file_path);
    if config["colors"
}

pub fn update(config_file_path: &Path, color_scheme: &str) {
}


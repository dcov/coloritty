//! coloritty

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{Arg, App};

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

fn find_config_file_path(home_path: &str) -> Option<PathBuf> {
    // The file paths that when appended to the $HOME directory path, may contain an alacritty
    // config file.
    // 
    // Note: These are the paths, and lookup order used by alacritty.
    let possible_config_file_paths = [
        "/.config/alacritty/alacritty.yml",
        "/.config/alacritty.yml",
        "/.alacritty.yml",
    ];

    // Iterate over the [possible_config_file_paths] until a path that exists is found.
    for possible_path in &possible_config_file_paths {
        let path = PathBuf::from(format!("{}{}", home_path, possible_path).as_str());
        if path.exists() {
            return Some(path);
        };
    };

    // The file path was not found
    None
}

fn read_config_schemes(config_file_path: &PathBuf) -> Vec<String> {
    // The error_msg to display upon failure.
    let error_msg = format!("Failed to read contents of: {}", config_file_path.to_str().unwrap());

    // Read the file contents and panic if it fails.
    let config_file_contents = fs::read_to_string(config_file_path).expect(error_msg.as_str());

    // Parse the file contents into a YAML documents, and panic if it fails.
    let documents = YamlLoader::load_from_str(&config_file_contents).expect(error_msg.as_str());

    let config = &documents[0];
    let mut result = Vec::new();

    if let Yaml::Hash(schemes) = &config["schemes"] {
        for (scheme_name, _) in schemes {
            if let Yaml::String(s) = &scheme_name {
                result.push(s.clone());
            }
        }
    }

    result
}

fn list(config_file_path: PathBuf) {
    let schemes = read_config_schemes(&config_file_path);
    if schemes.is_empty() {
        println!("There are no available <color_scheme> values");
    } else {
        println!("Available <color_scheme> values:");
        for scheme in schemes {
            println!("  -  {}", scheme);
        }
    }
}

fn update(config_file_path: PathBuf, color_scheme: &str) {
    let schemes = read_config_schemes(&config_file_path);
    if !schemes.contains(&String::from(color_scheme)) {
        println!("{} is not a valid <color_scheme> value.\nTo see a list of valid <color_scheme> values run 'coloritty -l'", color_scheme);
    } else {
        let contents = fs::read_to_string(&config_file_path).expect("Failed to read config file contents");
        let mut lines: Vec<String> = contents.lines().map(String::from).collect();

        // Find the line that contains the "colors:" value
        let mut value_index = None;
        for (index, line) in lines.iter().enumerate() {
            if line.starts_with("colors:") {
                value_index = Some(index);
            }
        }

        if let Some(index) = value_index {
            let _ = std::mem::replace(&mut lines[index], format!("colors: *{}", color_scheme));
        } else {
            lines.push(format!("colors: *{}", color_scheme));
        }

        fs::write(config_file_path, lines.join("\n").as_bytes())
            .expect("Failed to write updated config file");
    }
}

fn main() {
    // Get the cmd line arguments
    let matches = App::new("colorrity")
        .version("0.1.0")
        .about("Switch between alacritty color schemes")
        .arg(Arg::with_name("list")
            .short("l")
            .takes_value(false)
            .required(false)
            .help("List the color scheme names available under the 'schemes:' property"))
        .arg(Arg::with_name("color_scheme")
            .required_unless("list")
            .multiple(false)
            .help("The color scheme name to set 'colors:' property to"))
        .get_matches();

    // Get the home dir path
    let home_path = env::var("HOME").expect("HOME is not set.");

    // Find the config file path
    let config_file_path = find_config_file_path(&home_path).expect("config file not found");

    if matches.is_present("list") {
        // Generate and print the list of available color schemes
        list(config_file_path);
    } else {
        // This is safe to unwrap because we marked this value as required when generating the
        // matches, so it will have failed before this if it wasn't provided.
        let color_scheme = matches.value_of("color_scheme").unwrap();


        // Generate an updated coloritty file with the desired [color_scheme]
        update(config_file_path, color_scheme);
    }
}


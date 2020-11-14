//! coloritty

extern crate yaml_rust;

use clap::{Arg, App};
use std::env;
use std::path::Path;

mod generator;

fn find_config_file(home_path: &str) -> Option<&Path> {
    // The file paths that when appended to the $HOME directory path, may contain an alacrrity config
    // file.
    // 
    // Note: These are the paths, and lookup order used by alacritty.
    const possible_config_file_paths: Vec<&str> = vec![
        "/.config/alacritty/alacritty.yml",
        "/.config/alacritty.",
        "/.alacritty.yml",
    ];

    for config_file_path in possible_config_file_paths {
        let path = Path::new(format!("{}{}", home_path, config_file_path));
        if path.exists() {
            return Some(path);
        }
    }

    // The file was not found
    return None;
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

    // Find the config file
    let config_file_path = find_config_file(&home_path).expect("config file not found");

    if matches.is_present("list") {
        generator::list(config_file_path);
    } else {
        let color_scheme = matches.value_of("color_scheme").unwrap();
        generator::update(config_file_path, color_scheme);
    }
}


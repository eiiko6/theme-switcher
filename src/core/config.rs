use dirs::home_dir;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Config {
    pub profiles_path: String,
}

pub fn load_config(path: &str, verbose: bool) -> std::io::Result<Config> {
    let path = expand_tilde(path);

    ensure_config_file_exists(path.as_str(), verbose);

    let mut values = HashMap::new();

    if verbose {
        println!("Loading config: {}", path);
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let (key, value) = match line.split_once('=') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => continue,
        };

        values.insert(key.to_string(), value.to_string());
    }

    let profiles_path = if let Some(path) = values.get("profiles_path") {
        path.to_string()
    } else {
        String::default()
    };

    Ok(Config {
        profiles_path: expand_tilde(profiles_path.as_str()),
    })
}

fn ensure_config_file_exists(path: &str, verbose: bool) {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        if let Some(parent) = path_obj.parent() {
            if verbose {
                println!("Creating config file: {}", path_obj.display());
            }

            fs::create_dir_all(parent).expect("Failed to create config directory");
        }

        fs::File::create(path_obj).expect("Failed to create config file");
    }
}

fn expand_tilde(path: &str) -> String {
    if let Some(home) = home_dir() {
        path.replace("~/", &format!("{}/", home.display()))
    } else {
        panic!("could not determine home directory");
    }
}

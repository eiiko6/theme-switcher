use crate::expand_tilde;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub profiles_path: String,
    pub global_scripts: Vec<String>,
    pub per_profile_scripts: Vec<String>,
}

pub fn load_config(path: &str, verbose: bool) -> std::io::Result<Config> {
    let path = expand_tilde(path)?;

    ensure_config_file_exists(path.as_str(), verbose);

    let mut values: HashMap<String, Vec<String>> = HashMap::new();

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

        values
            .entry(key.to_string())
            .or_default()
            .push(value.to_string());
    }

    let profiles_path = if let Some(path) = values.get("profiles_path").and_then(|v| v.first()) {
        path.to_string()
    } else {
        // Fallback to default
        String::from("~/.config/dotswitch/profiles/")
    };
    let profiles_path = match expand_tilde(profiles_path.as_str()) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Could not determine profiles path: {e}");
            panic!();
        }
    };

    let mut global_scripts = Vec::new();
    if let Some(paths) = values.get("global_script") {
        for path in paths {
            global_scripts.push(path.to_string());
        }
    }

    let mut per_profile_scripts = Vec::new();
    if let Some(paths) = values.get("per_profile_script") {
        for path in paths {
            per_profile_scripts.push(path.to_string());
        }
    }

    Ok(Config {
        profiles_path,
        global_scripts,
        per_profile_scripts,
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

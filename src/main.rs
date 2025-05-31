use core::{ProfileAction, expand_tilde, load_config, proceed};
use std::path::Path;
use walkdir::WalkDir;

use clap::Parser;
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();
    let verbose = args.verbose;

    let config = match load_config("~/.config/dotswitch/dotswitch.conf", verbose) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load config: {e}");
            return;
        }
    };

    // Check if specified profiles directory exists
    if !Path::new(config.profiles_path.as_str()).exists() {
        eprintln!(
            "Failed to load profiles: {} does not exist",
            config.profiles_path
        );
        return;
    }

    match args.action {
        cli::Action::List => {
            for entry in WalkDir::new(config.profiles_path)
                .max_depth(1)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }

        cli::Action::Switch(profile_args) => {
            if let Err(e) = proceed(
                profile_args.name,
                profile_args.modules,
                verbose,
                ProfileAction::Update,
                config.clone(),
            ) {
                eprintln!("Failed to switch: {e}");
            }
        }

        cli::Action::Backup(profile_args) => {
            if let Err(e) = proceed(
                profile_args.name,
                profile_args.modules,
                verbose,
                ProfileAction::Backup,
                config,
            ) {
                eprintln!("Failed to backup: {e}");
            }
        }

        cli::Action::Preview(profile_args) => {
            if let Err(e) = proceed(
                profile_args.name,
                profile_args.modules,
                verbose,
                ProfileAction::Preview,
                config.clone(),
            ) {
                eprintln!("Failed to preview: {e}");
            }
        }
    }
}

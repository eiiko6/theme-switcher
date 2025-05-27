use core::{Host, ProfileAction, load_config, proceed};
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
            let _ = proceed(
                profile_args.name,
                Host::Desktop,
                verbose,
                ProfileAction::Update,
                config,
            );
        }

        cli::Action::Backup(profile_args) => {
            let _ = proceed(
                profile_args.name,
                Host::Desktop,
                verbose,
                ProfileAction::Backup,
                config,
            );
        }
    }
}

use core::load_config;
use walkdir::WalkDir;

use clap::Parser;
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();
    let verbose = args.verbose;

    let config = match load_config("~/.config/profiles/profiles.conf", verbose) {
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
                println!("{}", entry.file_name().display());
            }
        }

        _ => {}
    }
}

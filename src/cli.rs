use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
use std::path::Path;

/// Utility to switch between multiple dotfile profiles
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,

    /// Activate verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// List the available profiles
    List,
    /// Switch to a specified profile
    Switch(ProfileArgs),
    /// Backup the files that would be affected by a specific profile
    Backup(ProfileArgs),
}

#[derive(Args, Debug)]
pub struct ProfileArgs {
    pub name: String,
    // #[arg(
    //     value_parser = dir_exists
    // )]
    // pub path: String,
}

fn dir_exists(path: &str) -> Result<String, String> {
    let expanded = if let Some(home) = home_dir() {
        path.replace("~/", &format!("{}/", home.display()))
    } else {
        return Err("could not determine home directory".to_string());
    };

    if Path::new(&expanded).exists() {
        Ok(expanded)
    } else {
        Err("directory does not exist".to_string())
    }
}

use clap::{Args, Parser, Subcommand};

/// Utility to switch between multiple dotfile profiles
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The action to execute
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

    /// Preview the files that would be affected by a specific profile
    Preview(ProfileArgs),
}

#[derive(Args, Debug)]
pub struct ProfileArgs {
    /// The name of the profile
    pub name: String,

    /// The modules, separated by commas (e.g. 'common,laptop')
    #[arg(required = true, num_args = 1..)]
    pub modules: Vec<String>,
}

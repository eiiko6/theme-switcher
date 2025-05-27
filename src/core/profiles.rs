use chrono::Local;
use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
};
use walkdir::DirEntry;

use crate::core::config::Config;

pub enum Host {
    Laptop,
    Desktop,
    Unknown,
}

pub enum ProfileAction {
    Backup,
    Update,
    List,
}

pub fn proceed(
    profile_name: String,
    host: Host,
    verbose: bool,
    action: ProfileAction,
    config: Config,
) -> io::Result<()> {
    let home_dir = env::var("HOME").expect("HOME not set");
    let config_dir = Path::new(&home_dir).join(".config");
    let ignored_files = HashSet::from_iter(["wallpaper.png", "script.sh"]);
    let base = Path::new(&config.profiles_path).join(profile_name);

    match action {
        ProfileAction::Update => {
            // Common
            update_from_dir(
                &base.join("common"),
                config_dir.clone(),
                ignored_files.clone(),
                verbose,
            )?;

            // Host-specific
            let host_dir_name = match host {
                Host::Laptop => "laptop",
                Host::Desktop => "desktop",
                Host::Unknown => return Ok(()), // skip if unknown
            };

            update_from_dir(
                &base.join(host_dir_name),
                config_dir,
                ignored_files,
                verbose,
            )?;
        }

        ProfileAction::Backup => {
            // Common
            backup_from_dir(
                &base.join("common"),
                config_dir.clone(),
                ignored_files.clone(),
                verbose,
            )?;

            // Host-specific
            let host_dir_name = match host {
                Host::Laptop => "laptop",
                Host::Desktop => "desktop",
                Host::Unknown => return Ok(()), // skip if unknown
            };

            backup_from_dir(
                &base.join(host_dir_name),
                config_dir,
                ignored_files,
                verbose,
            )?;
        }

        ProfileAction::List => {
            todo!();
        }
    }

    Ok(())
}

fn update_from_dir(
    dir: &Path,
    config_dir: PathBuf,
    ignored_files: HashSet<&str>,
    verbose: bool,
) -> io::Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    for (entry, path) in get_entries(dir, ignored_files.clone()) {
        // Skip ignored files
        if ignored_files.iter().any(|f| path.ends_with(f)) {
            continue;
        }

        let target_path = config_dir.join(&path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let _ = fs::remove_file(&target_path); // Remove existing file/symlink
        std::os::unix::fs::symlink(entry.path(), &target_path)?;

        if verbose {
            println!(
                "Created symlink: {} -> {}",
                entry.path().display(),
                target_path.display()
            );
        }
    }
    Ok(())
}

fn backup_from_dir(
    dir: &Path,
    config_dir: PathBuf,
    ignored_files: HashSet<&str>,
    verbose: bool,
) -> io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let backup_dir = dirs::cache_dir()
        .unwrap_or_else(|| config_dir.join(".cache"))
        .join("dotswitch")
        .join(timestamp);

    if !dir.exists() {
        return Ok(());
    }

    for (_, path) in get_entries(dir, ignored_files) {
        let target_path = config_dir.join(&path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let backup_path = backup_dir.join(&path);

        if target_path.exists() {
            if let Some(parent) = backup_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&target_path, &backup_path)?;
            if verbose {
                println!(
                    "Backed up: {} -> {}",
                    target_path.display(),
                    backup_path.display()
                );
            }
        }
    }
    Ok(())
}

fn get_entries(dir: &Path, ignored_files: HashSet<&str>) -> Vec<(DirEntry, PathBuf)> {
    let walker = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file());

    walker
        .filter_map(|entry| {
            let relative_path = entry.path().strip_prefix(dir).ok()?;

            if ignored_files.contains(relative_path.to_str()?) {
                None
            } else {
                Some((entry.clone(), relative_path.to_path_buf()))
            }
        })
        .collect()
}

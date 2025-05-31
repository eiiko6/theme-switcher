use chrono::Local;
use std::process::Command;
use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
};
use walkdir::DirEntry;

use crate::core::config::Config;
use crate::expand_tilde;

pub enum ProfileAction {
    Backup,
    Update,
    Preview,
}

pub fn proceed(
    profile_name: String,
    modules: Vec<String>,
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
            for module in modules {
                if verbose {
                    println!("Updating for module {module}");
                }

                let module_path = base.join(module);
                ensure_exists(module_path.clone())?;

                update_from_dir(
                    &module_path,
                    config_dir.clone(),
                    ignored_files.clone(),
                    verbose,
                )?;
            }

            for script in config.global_scripts {
                match execute_script(script, &base.to_string_lossy()) {
                    Ok(s) => {
                        if verbose {
                            println!("{s}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute global script: {e}");
                        continue;
                    }
                }
            }

            for script in config.per_profile_scripts {
                let script = format!("{}/{script}", base.to_string_lossy());
                match execute_script(script, &base.to_string_lossy()) {
                    Ok(s) => {
                        if verbose {
                            println!("{s}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to execute profile script: {e}");
                        continue;
                    }
                }
            }
        }

        ProfileAction::Backup => {
            for module in modules {
                if verbose {
                    println!("Backing up for module {module}");
                }

                let module_path = base.join(module);
                ensure_exists(module_path.clone())?;

                backup_from_dir(
                    &module_path,
                    config_dir.clone(),
                    ignored_files.clone(),
                    verbose,
                )?;
            }
        }

        ProfileAction::Preview => {
            for module in modules {
                if verbose {
                    println!("Previewing for module {module}");
                }

                let module_path = base.join(module);
                ensure_exists(module_path.clone())?;

                preview_from_dir(&module_path, config_dir.clone(), ignored_files.clone())?;
            }

            for script in config.global_scripts {
                println!("Would run global executable: {script}");
            }

            for script in config.per_profile_scripts {
                let script = format!("{}/{script}", base.to_string_lossy());
                println!("Would run profile executable: {script}");
            }
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

    if verbose {
        println!("Backing up profile in {}", backup_dir.display());
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

fn preview_from_dir(
    dir: &Path,
    config_dir: PathBuf,
    ignored_files: HashSet<&str>,
) -> io::Result<()> {
    if !dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory {:?} does not exist", dir),
        ));
    }

    for (entry, path) in get_entries(dir, ignored_files) {
        let target_path = config_dir.join(&path);

        if target_path.exists() {
            println!(
                "Would create symlink: {} -> {}",
                entry.path().display(),
                target_path.display()
            );
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

fn ensure_exists(module_path: PathBuf) -> io::Result<()> {
    if !module_path.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "module directory '{}' does not exist",
                module_path.to_string_lossy()
            ),
        ))
    } else {
        Ok(())
    }
}

fn execute_script(script: String, profile_path: &str) -> Result<String, String> {
    let script = match expand_tilde(script.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let status = match Command::new(&script)
        .env("profile_path", profile_path)
        .status()
    {
        Ok(s) => s,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    if status.success() {
        Ok(format!("Executed script {}", &script))
    } else {
        Err(format!("script exited with status: {}", status))
    }
}

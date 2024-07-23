use crate::debug;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::Window;
use walkdir::WalkDir;

const CONFIG_NAME: &str = "com.kalsvik.eml";
const CONFIG_NAME_LEG: &str = "com.memer.eml";

pub fn open_path_in_file_manager(path: String) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    Command::new("explorer.exe").arg(path).spawn()?;

    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn()?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path).spawn()?;
    Ok(())
}

pub fn check_old_config_path() -> bool {
    PathBuf::from(CONFIG_NAME_LEG).exists()
}

pub fn migrate_old_config() -> std::io::Result<()> {
    let new_config = PathBuf::from(CONFIG_NAME);
    let old_config = PathBuf::from(CONFIG_NAME_LEG);
    if !old_config.exists() {
        debug::log("Old config does not exist.");
        return Ok(());
    }

    inject_files(&old_config, &new_config)?;

    Ok(())
}

pub fn get_config_path() -> std::io::Result<PathBuf> {
    let config_path = dirs_next::config_dir().expect("could not get config dir");

    let mut path = config_path.clone();
    path.push(CONFIG_NAME);

    let mut legacy_path = config_path.clone();
    legacy_path.push(CONFIG_NAME_LEG);

    if legacy_path.exists() {
        return Ok(legacy_path);
    }

    Ok(path)
}

pub fn handle_error(error: &str, window: &Window) {
    debug::log(&format!("Error: {}", error));
    window.emit("on_rust_error", error).unwrap()
}

pub fn inject_files(source: &PathBuf, _destination: &PathBuf) -> std::io::Result<()> {
    for entry in WalkDir::new(&source) {
        let p = PathBuf::from(entry.unwrap().path());

        if p.is_file() {
            let non_abs = remove_absolute_path(&p, &source);
            let mut destination = _destination.clone();
            destination.push(&non_abs);

            let mut destination_folder = _destination.clone();
            destination_folder.push(non_abs);
            destination_folder.pop();

            if !destination_folder.exists() {
                fs::create_dir_all(&destination_folder)?;
            }

            if p.exists() {
                if destination.exists() {
                    fs::remove_file(&destination)?;
                }

                fs::copy(p, destination)?;
            }
        }
    }

    Ok(())
}

pub fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

pub fn correct_all_slashes(path: String) -> String {
    path.replace(r"\", "/")
}

pub fn remove_absolute_path(path: &PathBuf, _abs_path: &PathBuf) -> PathBuf {
    let path = path.to_str().unwrap().to_string();
    let abs_path = _abs_path.to_str().unwrap().len() + 1;

    return PathBuf::from(path[abs_path..path.len()].to_string());
}
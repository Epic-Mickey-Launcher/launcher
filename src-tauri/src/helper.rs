use std::path::PathBuf;
use std::process::Command;
use std::fs;
use walkdir::WalkDir;
use crate::debug;

const CONFIG_NAME: &str = "com.kalsvik.eml";
const CONFIG_NAME_LEG: &str = "com.memer.eml";

pub fn open_path_in_file_manager(path: String) {
    #[cfg(target_os = "windows")]
    Command::new("explorer.exe")
        .arg(path)
        .spawn()
        .expect("failed to execute process");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("failed to execute process");

    #[cfg(target_os = "linux")]
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .expect("failed to execute process");
}

pub fn get_config_path() -> std::io::Result<PathBuf> {
    let config_path = dirs_next::config_dir().expect("could not get config dir");

    let mut path = config_path.clone();
    path.push(CONFIG_NAME);

    let mut legacy_path = config_path.clone();
    legacy_path.push(CONFIG_NAME_LEG);

    if legacy_path.exists() {
        return Ok(legacy_path)
    }

    Ok(path)
}

pub fn inject_files(source: &PathBuf, _destination: &PathBuf) {
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
                fs::create_dir_all(&destination_folder).expect("Failed to create folders.");
            }

            if p.exists() {
                if destination.exists() {
                    fs::remove_file(&destination).unwrap();
                }

                fs::copy(p, destination).expect("Failed to copy file");
            }
        }
    }
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


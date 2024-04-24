use std::path::PathBuf;
use std::process::Command;

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


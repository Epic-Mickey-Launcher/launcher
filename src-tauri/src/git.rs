use std::{path::PathBuf, process::Command};

pub fn clone(url: &String, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !has_git() {
        return Err("git is not installed.".into());
    }

    if url.contains(" ") {
        return Err("illegal url".into());
    }

    Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(destination)
        .output()?;

    Ok(())
}

pub fn has_git() -> bool {
    let res = Command::new("git").output();
    match res {
        Ok(..) => true,
        Err(..) => false,
    }
}

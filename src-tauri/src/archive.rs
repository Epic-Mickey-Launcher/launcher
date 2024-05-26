use crate::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct SmallArchiveValidationInfo {
    under_limit: bool,
    extension: String,
}

pub fn validate(path: String) -> SmallArchiveValidationInfo {
    let mut validation_info = SmallArchiveValidationInfo {
        under_limit: false,
        extension: "".to_string(),
    };
    let mut f = File::open(&path).expect("Couldn't open archive");
    let size = f.metadata().unwrap().len();

    validation_info.under_limit = size < 100000000;

    let mut buffer = [0; 262];
    f.read(&mut buffer).expect("failed to read archive header");

    validation_info
}

pub fn extract(input_path: String, output_path: &PathBuf) -> std::io::Result<()> {
    debug::log(&format!("Extracting Archive {}", input_path));

    println!("Archive is TAR");
    #[cfg(target_os = "windows")]
    Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    #[cfg(target_os = "linux")]
    Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .output()?;

    #[cfg(target_os = "macos")]
    Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .output();

    fs::remove_file(input_path)?;

    Ok(())    
}

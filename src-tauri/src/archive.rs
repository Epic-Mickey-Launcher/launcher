use crate::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct SmallArchiveValidationInfo {
    pub under_limit: bool,
}

pub fn validate(path: String) -> std::io::Result<SmallArchiveValidationInfo> {
    let mut validation_info = SmallArchiveValidationInfo { under_limit: false };
    let mut f = File::open(&path)?;
    let size = f.metadata().unwrap().len();

    validation_info.under_limit = size < 100000000;

    let mut buffer = [0; 262];
    f.read(&mut buffer)?;

    Ok(validation_info)
}

pub fn extract(
    input_path: String,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    debug::log(&format!("Extracting Archive {}", input_path));

    #[cfg(target_os = "windows")]
    let mut result = Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .output()?;

    #[cfg(target_os = "linux")]
    let mut result = Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .spawn()?;

    #[cfg(target_os = "macos")]
    let mut result = Command::new("tar")
        .arg("-xf")
        .arg(&input_path)
        .arg("-C")
        .arg(&output_path)
        .output();

    let error = result.wait()?;

    if error.code().unwrap() != 0 {
        return Err("Archive did not extract successfully".into());
    }

    Ok(())
}

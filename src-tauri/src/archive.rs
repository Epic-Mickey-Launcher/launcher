use crate::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

    let file = File::open(input_path)?;

    compress_tools::uncompress_archive(file, output_path, compress_tools::Ownership::Preserve)?;

    Ok(())
}

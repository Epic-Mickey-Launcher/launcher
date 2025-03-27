use crate::debug;
use anyhow::Error;
use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct SmallArchiveValidationInfo {
    pub under_limit: bool,
}

pub fn validate(path: String) -> Result<SmallArchiveValidationInfo> {
    let mut validation_info = SmallArchiveValidationInfo { under_limit: false };
    let mut f = File::open(&path)?;
    let size = f.metadata().unwrap().len();

    validation_info.under_limit = size < 100000000;

    let mut buffer = [0; 262];
    f.read(&mut buffer)?;
    Ok(validation_info)
}
pub fn compress(input_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
    let file = File::create(output_path)?;
    let mut encoder = GzEncoder::new(file, Compression::best());
    {
        let mut tar_file = tar::Builder::new(&mut encoder);
        tar_file.append_dir_all(".", input_path)?;
    }
    encoder.finish()?;
    Ok(())
}
pub fn extract(input_path: String, output_path: &PathBuf) -> Result<(), Error> {
    debug::log(&format!("Extracting Archive {}", input_path));

    let file = File::open(input_path)?;

    compress_tools::uncompress_archive(file, output_path, compress_tools::Ownership::Preserve)?;

    Ok(())
}

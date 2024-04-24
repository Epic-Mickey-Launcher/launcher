use std::fs;
use std::process::Command;
use std::io::Cursor;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use crate::debug;
        
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

    if &buffer[0..2] == "PK".as_bytes() {
        println!("Archive is Zip");
        validation_info.extension = "zip".to_string();
    } else if &buffer[0..2] == "7z".as_bytes() {
        println!("Archive is 7Zip");
        validation_info.extension = "7zip".to_string();
    } else if &buffer[257..262] == "ustar".as_bytes() {
        println!("Archive is TAR");
        validation_info.extension = "tar".to_string();
    } else {
        println!("Unknown archive type");
    }
    validation_info
}

pub fn extract(input_path: String, output_path: &PathBuf) -> String {
    debug::log(&format!("Extracting Archive {}", input_path));

    let mut f = File::open(&input_path).expect("Couldn't open archive");
    let mut buffer = [0; 262];

    let mut archive_type = "";

    f.read(&mut buffer).expect("failed to read archive header");

    if &buffer[0..2] == "PK".as_bytes() {
        println!("Archive is Zip");

        archive_type = "zip";

        #[cfg(target_os = "macos")]
        {
            Command::new("ditto")
                .arg("-x")
                .arg("-k")
                .arg(&input_path)
                .arg(&output_path)
                .output()
                .unwrap();
        }
        //prolly a way to combine these
        #[cfg(target_os = "windows")]
        {
            let mut f = File::open(&input_path).expect("Failed to open tmpzip");
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("Failed to read tmpzip");
            zip_extract::extract(Cursor::new(buffer), &output_path, false)
                .expect("failed to extract");
        }
        #[cfg(target_os = "linux")]
        {
            let mut f = File::open(&input_path).expect("Failed to open tmpzip");
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("Failed to read tmpzip");
            zip_extract::extract(Cursor::new(buffer), &output_path, false)
                .expect("failed to extract");
        }
    } else if &buffer[0..2] == "7z".as_bytes() {
        println!("Archive is 7Zip");
        archive_type = "7zip";
        sevenz_rust::decompress_file(&input_path, &output_path).expect("complete");
    } else if &buffer[257..262] == "ustar".as_bytes() {
        println!("Archive is TAR");
        archive_type = "tar";
        #[cfg(target_os = "windows")]
        Command::new("tar")
            .arg("-xf")
            .arg(&input_path)
            .arg("-C")
            .arg(&output_path)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .expect("Tar failed to extract");

        #[cfg(target_os = "linux")]
        Command::new("tar")
            .arg("-xf")
            .arg(&input_path)
            .arg("-C")
            .arg(&output_path)
            .output()
            .expect("Tar failed to extract");

        #[cfg(target_os = "macos")]
        Command::new("tar")
            .arg("-xf")
            .arg(&input_path)
            .arg("-C")
            .arg(&output_path)
            .output()
            .expect("Tar failed to extract");
    } else {
        println!("Unknown archive type");
    }
    fs::remove_file(input_path).expect("Failed to remove tmpzip");
    archive_type.to_string()
}


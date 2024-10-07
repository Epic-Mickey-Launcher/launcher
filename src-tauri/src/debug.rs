use chrono::{Datelike, Local, Timelike};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::helper;

pub fn init() -> std::io::Result<()> {
    let mut path = helper::get_config_path().expect("could not get config dir");
    fs::create_dir_all(&path)?;
    path.push("Log.txt");

    if !Path::exists(&path) {
        File::create("Log.txt")?;
    }

    let now = Local::now();

    fs::write(
        &path,
        format!(
            "EML opened at {}.\n",
            now.year().to_string()
                + "/"
                + &now.month().to_string()
                + "/"
                + &now.day().to_string()
                + ", "
                + &now.hour().to_string()
                + ":"
                + &now.minute().to_string()
                + ":"
                + &now.second().to_string()
        ),
    )?;
    Ok(())
}

pub fn log(output: &str) {
    let mut path = helper::get_config_path().expect("could not get config dir");
    path.push(r"Log.txt");
    let now = Local::now();
    let date = now.year().to_string()
        + "/"
        + &now.month().to_string()
        + "/"
        + &now.day().to_string()
        + ", "
        + &now.hour().to_string()
        + ":"
        + &now.minute().to_string()
        + ":"
        + &now.second().to_string();

    let final_output = format!("[{}]: {}\n", date, output);
    print!("{}", final_output);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    file.write(final_output.as_bytes()).unwrap();
}

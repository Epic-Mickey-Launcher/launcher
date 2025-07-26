use crate::debug;
use crate::helper;
use anyhow::Result;
use anyhow::{anyhow, Error};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub async fn extract(isopath: String, gamename: String, dolphin: String) -> Result<String, Error> {
    debug::log("beginning ISO extraction");
    let mut destination = helper::get_config_path()?;
    destination.push("Games");
    destination.push(gamename);
    println!("pee");
    let mut dolphin_tool = PathBuf::from(dolphin);
    dolphin_tool.pop();

    #[cfg(target_os = "windows")]
    dolphin_tool.push("DolphinTool.exe");
    #[cfg(target_os = "linux")]
    dolphin_tool.push("dolphin-tool");
    #[cfg(target_os = "macos")]
    dolphin_tool.push("dolphin-tool");

    fs::create_dir_all(&destination)?;

    if !dolphin_tool.exists() {
        return Err(anyhow!("dolphin tool does not exist"));
    }

    if destination.exists() {
        fs::remove_dir_all(&destination)?;
    }

    Command::new(dolphin_tool)
        .arg("extract")
        .arg("-i")
        .arg(isopath)
        .arg("-o")
        .arg(&destination)
        .output()?;

    let output = destination.to_str().unwrap();

    Ok(output.to_string())
}

pub fn check(path: String, dolphin: String) -> Result<String, Error> {
    debug::log("Checking Game ID");

    let mut dolphin_tool = PathBuf::from(dolphin);
    dolphin_tool.pop();
    #[cfg(target_os = "windows")]
    dolphin_tool.push("DolphinTool.exe");
    #[cfg(target_os = "linux")]
    dolphin_tool.push("dolphin-tool");
    #[cfg(target_os = "macos")]
    dolphin_tool.push("dolphin-tool");

    let dolphin = Command::new(dolphin_tool)
        .arg("header")
        .arg("-i")
        .arg(path)
        .stdout(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(dolphin.stdout)?;
    let mut s = stdout.split("\n");

    println!("{}", stdout);

    if !stdout.contains("Game ID:") {
        return Err(anyhow!("Dolphin Tool Failed to get Game ID from ROM."));
    }

    let mut id_parse = String::new();

    while id_parse == "" {
        let stline = s.next().unwrap();
        if stline.contains("Game ID: ") {
            id_parse = stline.replace("Game ID: ", "");
        }
    }
    let id = id_parse.trim().to_string();
    debug::log(&format!("ID Check Result: {}", id));
    Ok(id)
}

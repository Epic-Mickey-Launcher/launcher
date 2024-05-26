use std::process::{Command, Stdio};
use std::fs;
use std::path::PathBuf;

use crate::helper;
use crate::debug;

pub async fn extract(isopath: String, gamename: String, dolphin: String) -> String {
    let mut destination = helper::get_config_path().expect("could not get config dir");
    destination.push("Games");
    destination.push(gamename);

    let mut dolphin_tool = PathBuf::from(dolphin);
    dolphin_tool.pop();

    #[cfg(target_os = "windows")]
    dolphin_tool.push("DolphinTool.exe");
    #[cfg(target_os = "linux")]
    dolphin_tool.push("dolphin-tool");
    #[cfg(target_os = "macos")]
    dolphin_tool.push("dolphin-tool");

    fs::create_dir_all(&destination).unwrap();

    if !dolphin_tool.exists() {
        return "err_toolnoexist".to_string();
    }

    Command::new(dolphin_tool)
        .arg("extract")
        .arg(isopath)
        .arg(&destination)
        .output()
        .expect("failed to open dolphin-tool");

    return destination.to_str().unwrap().to_string();
}

pub fn check(path: String, dolphin: String) -> String {

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
    .output()
    .unwrap();
    let stdout = String::from_utf8(dolphin.stdout).unwrap();
    let mut s = stdout.split("\n");
    let mut id_parse = String::new();
    while id_parse == "" {
        let stline = s.next().unwrap();
        if stline.contains("Game ID: "){
            id_parse = stline.replace("Game ID: ", "");
        }
    }
    let id = id_parse.trim().to_string();
    debug::log(&format!("ID Check Result: {}", id));
    id 
}

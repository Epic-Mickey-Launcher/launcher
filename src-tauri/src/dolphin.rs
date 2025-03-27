extern crate dirs_next;
use crate::helper;
use anyhow::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn find_dir(where_in: &PathBuf) -> PathBuf {
    let mut config_path = helper::get_config_path().expect("could not get config dir");
    config_path.push("DolphinConfig");
    config_path.push(where_in);
    config_path
}

pub fn auto_set_custom_textures() {
    let mut buf = find_dir(&PathBuf::from("Config"));
    if !buf.exists() {
        fs::create_dir_all(&buf).unwrap();
    }
    buf.push("GFX.ini");
    if !buf.exists() {
        let mut f = File::create(&buf).unwrap();
        f.write(b"[Settings]\nHiresTextures = True").unwrap();
    } else {
        let mut f = File::open(&buf).unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();
        let mut str = String::from_utf8(b).unwrap();
        str = str.replace("HiresTextures = False", "HiresTextures = True");
        fs::remove_file(&buf).unwrap();
        f = File::create(&buf).unwrap();
        f.write(str.as_bytes()).unwrap();
    }
}

pub fn open(path: String) {
    let mut config_path = helper::get_config_path().expect("could not get config dir");
    config_path.push("DolphinConfig");

    #[cfg(target_os = "windows")]
    Command::new(path).arg("-u").arg(config_path).spawn();
    #[cfg(target_os = "linux")]
    //QT env variable is for wayland functionality
    Command::new(if path == "" { "dolphin-emu" } else { &path })
        .arg("-u")
        .arg(config_path)
        .env("QT_QPA_PLATFORM", "xcb")
        .env("WAYLAND_DISPLAY", "0")
        .spawn()
        .expect("failed to start dolphin");
    #[cfg(target_os = "macos")]
    {
        Command::new("xattr")
            .arg("-d")
            .arg("com.apple.quarantine")
            .arg(&path)
            .spawn()
            .unwrap();

        Command::new("open")
            .arg(path)
            .arg("--args")
            .arg("-u")
            .arg(config_path)
            .spawn()
            .unwrap();
    }
}

pub fn set_override(_path: String) -> Result<()> {
    let mut path = helper::get_config_path()?;
    fs::create_dir_all(&path)?;
    path.push("dolphinoverride");
    let mut f = File::create(&path)?;
    f.write_all(_path.as_bytes())?;
    Ok(())
}

pub fn create_portable(dolphinpath: String) -> Result<()> {
    let mut dolphin_config_path = PathBuf::from(&dolphinpath);
    dolphin_config_path.pop();
    let config_folder_name = if env::consts::OS == "windows" {
        "User"
    } else {
        "user"
    };
    dolphin_config_path.push(config_folder_name);
    let mut path = PathBuf::from(&dolphinpath);
    path.pop();
    path.push("portable.txt");
    if !path.exists() {
        File::create(&path).expect("Failed to create file");
        set_override(dolphin_config_path.clone().to_str().unwrap().to_string())?;
        dolphin_config_path.push("Config");
        fs::create_dir_all(&dolphin_config_path).unwrap();
        dolphin_config_path.push("GFX.ini");
        let mut f = File::create(dolphin_config_path).expect("Failed to create file");
        f.write(b"[Settings]\nHiresTextures = True").unwrap();
    }

    Ok(())
}

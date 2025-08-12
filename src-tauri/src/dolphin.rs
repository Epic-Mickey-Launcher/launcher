extern crate dirs_next;
use crate::download;
use crate::helper;
use anyhow::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tauri::Window;

const DOLPHIN_FLATPAK: &str =
    "https://dl.dolphin-emu.org/releases/2506a/dolphin-2506a-x86_64.flatpak";

pub async fn download_dolphin_flatpak(window: &Window) -> Result<()> {
    let mut config_path = helper::get_config_path().expect("could not get config dir");
    config_path.push("dolphin.flatpak");
    download::file(DOLPHIN_FLATPAK.to_string(), &config_path, false, window).await?;

    Command::new("flatpak")
        .arg("install")
        .arg("--user")
        .arg("--assumeyes")
        .arg(&config_path)
        .output()
        .unwrap();

    fs::remove_file(config_path)?;

    Ok(())
}

pub async fn link_global_dolphin() -> Result<()> {
    let mut config_path = helper::get_config_path().expect("could not get config dir");
    config_path.push("Dolphin");

    if config_path.exists() {
        fs::remove_dir_all(&config_path)?;
    }

    fs::create_dir_all(&config_path)?;

    let mut config_path_dolphin_emu = config_path.clone();
    config_path_dolphin_emu.push("dolphin-emu");

    let mut config_path_dolphin_tool = config_path.clone();
    config_path_dolphin_tool.push("dolphin-tool");

    fs::hard_link("/usr/bin/dolphin-emu", config_path_dolphin_emu)?;
    fs::hard_link("/usr/bin/dolphin-tool", config_path_dolphin_tool)?; // this could turn into a
                                                                       // problem
    Ok(())
}

pub async fn check_dolphin_installed_global() -> bool {
    match fs::exists("/usr/bin/dolphin-emu") {
        Ok(res) => return res,
        Err(_) => return false,
    }
}

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

pub fn flatpak_dolphin_override() -> Result<()> {
    Command::new("flatpak")
        .arg("override")
        .arg("org.DolphinEmu.dolphin-emu")
        .arg("--filesystem")
        .arg("host")
        .arg("--user")
        .spawn()?;

    Ok(())
}

pub fn open(path: String, flatpak: bool) {
    let mut config_path = helper::get_config_path().expect("could not get config dir");
    config_path.push("DolphinConfig");

    #[cfg(target_os = "windows")]
    Command::new(path).arg("-u").arg(config_path).spawn();
    #[cfg(target_os = "linux")]
    {
        if !flatpak {
            //QT env variable is for wayland functionality
            Command::new(if path == "" { "dolphin-emu" } else { &path })
                .arg("-u")
                .arg(config_path)
                .env("QT_QPA_PLATFORM", "xcb")
                .env("WAYLAND_DISPLAY", "0")
                .spawn()
                .expect("failed to start dolphin");
        } else {
            // make sure the flatpak can access eml config
            flatpak_dolphin_override().expect("failed to override flatpak permission");
            Command::new("flatpak")
                .arg("run")
                .arg("--command=dolphin-emu")
                .arg("org.DolphinEmu.dolphin-emu")
                .arg("-u")
                .arg(config_path)
                .env("QT_QPA_PLATFORM", "xcb")
                .env("WAYLAND_DISPLAY", "0")
                .spawn()
                .expect("failed to start dolphin (flatpak)");
        }
    }
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

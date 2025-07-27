use crate::dolphin;
use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn game(dolphin: String, exe: String, flatpak: bool) -> Result<()> {
    let config_path = dolphin::find_dir(&PathBuf::new());

    let os = env::consts::OS;
    if !Path::new(&dolphin).exists() {
        return Err(anyhow!("Dolphin Directory does not exist.",));
    }

    if os == "windows" {
        if dolphin.ends_with(".exe") {
            Command::new(&dolphin)
                .arg("-b")
                .arg("-e")
                .arg(&exe)
                .arg("-u")
                .arg(config_path)
                .spawn()?;
        }
    } else if os == "macos" {
        Command::new("open")
            .arg(&dolphin)
            .arg("--args")
            .arg("-b")
            .arg("-e")
            .arg(&exe)
            .arg("-u")
            .arg(config_path)
            .spawn()?;
    } else if os == "linux" {
        if !flatpak {
            Command::new("chmod").arg("+x").arg(&dolphin).output()?;
            Command::new(dolphin)
                .env("WAYLAND_DISPLAY", "0") // Dolphin doesn't support Wayland yet...
                .arg("-b")
                .arg("-e")
                .arg(&exe)
                .arg("-u")
                .arg(config_path)
                .spawn()?;
        } else {
            // make sure the flatpak can access eml config
            dolphin::flatpak_dolphin_override().expect("failed to override flatpak permission");

            Command::new("flatpak")
                .arg("run")
                .arg("--command=dolphin-emu")
                .arg("org.DolphinEmu.dolphin-emu")
                .arg("-b")
                .arg("-e")
                .arg(&exe)
                .arg("-u")
                .arg(config_path)
                .env("QT_QPA_PLATFORM", "xcb")
                .env("WAYLAND_DISPLAY", "0")
                .spawn()
                .expect("failed to start dolphin");
        }
    }

    Ok(())
}

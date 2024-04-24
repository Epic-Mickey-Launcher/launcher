use std::process::Command;
use std::path::{PathBuf, Path};
use std::env;
use crate::dolphin;

pub fn game(dolphin: String, exe: String) -> i32 {
    let config_path = dolphin::find_dir(&PathBuf::new());

    dolphin::auto_set_custom_textures();

    let os = env::consts::OS;
    if !Path::new(&dolphin).exists() {
        return 1;
    }

    if os == "windows" {
        if dolphin.ends_with(".exe") {
            Command::new(&dolphin)
                .arg("-b")
                .arg("-e")
                .arg(&exe)
                .arg("-u")
                .arg(config_path)
                .spawn()
                .expect("could not open exe");
        }
        return 0;
    } else if os == "macos" {
        Command::new("open")
            .arg(&dolphin)
            .arg("--args")
            .arg("-b")
            .arg("-e")
            .arg(&exe)
            .arg("-u")
            .arg(config_path)
            .spawn()
            .expect("could not open dolphin");
        return 0;
    } else if os == "linux" {
        Command::new("chmod")
            .arg("+x")
            .arg(&dolphin)
            .output()
            .expect("failed to give executable the correct permissions");

        Command::new(dolphin)
            .env("WAYLAND_DISPLAY", "0")
            .arg("-b")
            .arg("-e")
            .arg(&exe)
            .arg("-u")
            .arg(config_path)
            .spawn()
            .expect("could not open dolphin");
        return 0;
    }

    return 0;
}


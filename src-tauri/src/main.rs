//note 2self or whoever. macos directory system uses / and not \

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Window;

extern crate chrono;
extern crate tauri_plugin_deep_link;

pub mod archive;
pub mod debug;
pub mod dolphin;
pub mod download;
pub mod git;
pub mod helper;
pub mod iso_extract;
pub mod mod_info;
pub mod mod_management;
pub mod play;

fn main() {
    debug::init().expect("Failed to initialize Debug.");

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            app.get_webview_window("main")
                .unwrap()
                .set_title(
                    format!(
                        "Epic Mickey Launcher {}",
                        app.package_info().version.to_string()
                    )
                    .as_str(),
                )
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            playgame,
            download_mod,
            play_steam_game,
            change_mod_status,
            delete_mod,
            validate_mod,
            get_os,
            extract_iso,
            delete_mod_cache,
            clean_temp_install_directory,
            get_bootbin_id,
            check_iso,
            open_dolphin,
            open_link,
            download_tool,
            validate_archive,
            set_dolphin_emulator_override,
            delete_docs_folder,
            write_mod_info,
            open_process,
            delete_mod_cache_all,
            create_portable,
            open_path_in_file_manager,
            open_config_folder,
            get_frontend_config_path,
            generate_mod_project,
            init_repository,
            update_repository,
            generate_ssh_key_pair
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();
    windows
        .values()
        .next()
        .expect("primary window not found")
        .set_focus()
        .expect("can't focus primary window");
}
#[tauri::command]
fn play_steam_game(id: String) {
    Command::new("steam")
        .arg(format!("steam://rungameid/{}", id))
        .spawn()
        .unwrap();
}

#[tauri::command]
fn open_dolphin(path: String) {
    dolphin::open(path);
}

#[tauri::command]
fn open_link(url: String, window: Window) {
    open::that(url).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

#[tauri::command]
fn create_portable(dolphinpath: String, window: Window) {
    dolphin::create_portable(dolphinpath).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

#[tauri::command]
fn delete_docs_folder() {
    let mut path = dirs_next::document_dir().expect("could not get documents dir");
    path.push("Epic Mickey Launcher");
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

#[tauri::command]
fn delete_mod_cache_all(window: Window) {
    mod_management::delete_cache_all().unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

#[tauri::command]
fn get_bootbin_id(path: String) -> String {
    let mut f = File::open(path).unwrap();
    let mut id_bytes = [0; 6];
    f.read_exact(&mut id_bytes).unwrap();
    let id = std::str::from_utf8(&id_bytes[0..6]).unwrap().to_uppercase();
    return id;
}

#[tauri::command]
fn generate_ssh_key_pair(path: String, window: Window) {
    git::generate_ssh_key_pair(&path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    })
}

#[tauri::command]
fn write_mod_info(
    path: String,
    files: Vec<String>,
    textures: Vec<String>,
    scripts: Vec<String>,
    window: Window,
) {
    mod_info::write(path, files, textures, scripts).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    })
}

#[tauri::command]
async fn extract_iso(isopath: String, gamename: String, dolphin: String, window: Window) -> String {
    iso_extract::extract(isopath, gamename, dolphin)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
            "".to_string()
        })
}

#[tauri::command]
async fn download_tool(url: String, foldername: String, window: Window) -> PathBuf {
    download::tool(url, foldername, &window)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
            PathBuf::new()
        })
}

#[tauri::command]
fn get_os() -> String {
    debug::log(&format!("Operating System: {}", env::consts::OS));
    env::consts::OS.to_string()
}
#[tauri::command]
fn open_process(path: String, args: String) {
    Command::new(path)
        .arg(args)
        .output()
        .expect("failed to execute process");
}

#[tauri::command]
fn open_path_in_file_manager(path: String, window: Window) {
    helper::open_path_in_file_manager(path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

#[tauri::command]
fn playgame(dolphin: String, exe: String, window: Window) {
    play::game(dolphin, exe).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    })
}

#[tauri::command]
fn check_iso(path: String, dolphin: String, window: Window) -> String {
    iso_extract::check(path, dolphin).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
        "".to_string()
    })
}

#[tauri::command]
fn validate_archive(path: String, window: Window) -> archive::SmallArchiveValidationInfo {
    archive::validate(path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
        archive::SmallArchiveValidationInfo { under_limit: false }
    })
}

#[tauri::command]
fn set_dolphin_emulator_override(_path: String, window: Window) {
    dolphin::set_override(_path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    })
}

#[tauri::command]
fn clean_temp_install_directory(destination: PathBuf, window: Window) {
    mod_management::clean_temp_install_directory(destination).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    })
}

#[tauri::command]
fn open_config_folder(window: Window) {
    let path = helper::get_config_path().unwrap();
    open_path_in_file_manager(path.to_str().unwrap().to_owned(), window)
}

#[tauri::command] // todo: brain death
fn get_frontend_config_path(npath: String) -> String {
    npath + "/"
}

// Mod Commands

#[tauri::command]
async fn download_mod(
    url: String,
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    version: String,
    window: Window,
) {
    mod_management::add(url, dumploc, gameid, modid, platform, version, &window)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
        })
}

#[tauri::command]
async fn change_mod_status(
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    active: bool,
    version: String,
    window: Window,
) {
    mod_management::change_status(dumploc, gameid, modid, platform, active, version, &window)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
        });
}

#[tauri::command]
async fn delete_mod(
    dumploc: String,
    gameid: String,
    platform: String,
    modid: String,
    active: bool,
    window: Window,
) {
    mod_management::delete(dumploc, gameid, platform, modid, active, &window)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
        });
}

#[tauri::command]
fn delete_mod_cache(modid: String, window: Window) {
    mod_management::delete_cache(modid).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

#[tauri::command]
async fn generate_mod_project(game: String, platform: String, path: String, window: Window) {
    mod_management::generate_mod_template(game, platform, path)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
        })
}

#[tauri::command]
async fn validate_mod(
    url: String,
    destination: String,
    mode: String,
    window: Window,
) -> mod_management::ValidationInfo {
    mod_management::validate_mod(url, PathBuf::from(destination), mode, &window)
        .await
        .unwrap_or_else(|error| {
            helper::handle_error(&error.to_string(), &window);
            mod_management::ValidationInfo {
                modicon: "".to_string(),
                modname: "".to_string(),
                result: error.to_string(),
                validated: false,
                data: eml_validate::ModInfo::new(),
            }
        })
}

#[tauri::command]
fn init_repository(path: String, window: Window) {
    git::init_repository(&path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}
#[tauri::command]
fn update_repository(path: String, window: Window) {
    git::update_mod(&path).unwrap_or_else(|error| {
        helper::handle_error(&error.to_string(), &window);
    });
}

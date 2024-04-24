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
use std::str;
use tauri::{Manager, Window};

extern crate chrono;

pub mod play;
pub mod helper;
pub mod download;
pub mod archive;
pub mod dolphin;
pub mod debug;
pub mod iso_extract;
pub mod mod_info;
pub mod mod_management;

fn main() {
   debug::init().expect("Failed to initialize Debug.");

    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            tauri_plugin_deep_link::prepare("com.memer.eml");


            tauri_plugin_deep_link::register(
                "eml",
                move |request| {
                  dbg!(&request);
                  window.emit_all("scheme_request_received", request).unwrap();
                },
              )
              .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            playgame,
            download_mod,
            start_em2_steam,
            change_mod_status,
            delete_mod,
            validate_mod,
            get_os,
            extract_iso,
            delete_mod_cache,
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
            open_config_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_em2_steam() {
    Command::new("steam")
        .arg("steam://rungameid/245300")
        .spawn().unwrap();
}

#[tauri::command]
fn open_dolphin(path: String) {
    dolphin::open(path);
}

#[tauri::command]
fn open_link(url: String) {
    open::that(url).expect("Failed to open URL in default browser");
}

#[tauri::command]
fn create_portable(dolphinpath: String) {
    dolphin::create_portable(dolphinpath);
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
fn delete_mod_cache_all() {
    mod_management::delete_cache_all();
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
fn write_mod_info(path: String, files: Vec<String>, textures: Vec<String>) {
   mod_info::write(&path, files, textures).expect("Failed to write mod data.");
}

#[tauri::command]
async fn extract_iso(isopath: String, gamename: String, dolphin: String) -> String {
    iso_extract::extract(isopath, gamename, dolphin).await
}

#[tauri::command]
async fn download_tool(url: String, foldername: String, window: Window) -> PathBuf {
    download::tool(url, foldername, &window).await
}

#[tauri::command]
fn get_os() -> &'static str {
    debug::log(&format!("Operating System: {}", env::consts::OS));
    env::consts::OS
}
#[tauri::command]
fn open_process(path: String, args: String) {
    Command::new(path)
        .arg(args)
        .output()
        .expect("failed to execute process");
}

#[tauri::command]
fn open_path_in_file_manager(path: String) {
    helper::open_path_in_file_manager(path);
}

#[tauri::command]
fn playgame(dolphin: String, exe: String) -> i32 {
    play::game(dolphin, exe)
}

#[tauri::command]
fn check_iso(path: String, dolphin: String) -> String {
    iso_extract::check(path, dolphin)
}

#[tauri::command]
fn validate_archive(path: String) -> archive::SmallArchiveValidationInfo {
    archive::validate(path)
}


#[tauri::command]
fn set_dolphin_emulator_override(_path: String) {
    dolphin::set_override(_path);
}

#[tauri::command]
fn open_config_folder() {
    let mut path = PathBuf::new();
    path.push(dirs_next::config_dir().unwrap());
    path.push("com.memer.eml");
    open_path_in_file_manager(path.to_str().unwrap().to_owned())
}

// Mod Commands

#[tauri::command]
async fn download_mod(
    url: String,
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    window: Window,
) {
    mod_management::add(url, dumploc, gameid, modid, platform, &window).await;
}

#[tauri::command]
async fn change_mod_status(
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    active: bool,
    window: Window,
) {
    mod_management::change_status(dumploc, gameid, modid, platform, active, &window).await;
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
    mod_management::delete(dumploc, gameid, platform, modid, active, &window).await;
}

#[tauri::command]
fn delete_mod_cache(modid: String) {
    mod_management::delete_cache(modid);
}

#[tauri::command]
async fn validate_mod(url: String, local: bool, window: Window) -> mod_management::ValidationInfo {
    mod_management::validate_mod(url, local, &window).await
}



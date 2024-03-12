//note 2self or whoever. macos directory system uses / and not \

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bytes::buf;
use bytes::Bytes;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::default;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Error;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use walkdir::WalkDir;
extern crate chrono;
extern crate dirs_next;
extern crate fs_extra;
extern crate open;
extern crate reqwest;
extern crate scan_dir;
extern crate sevenz_rust;
extern crate walkdir;
extern crate zip_extract;
use tauri::{Manager, Window};
#[derive(Serialize, Deserialize)]
struct ChangedFiles {
    name: String,
    modid: String,
    active: bool,
    update: i32,
}

#[derive(Serialize, Deserialize)]
struct ModInfo {
    name: String,
    game: String,
    description: String,
    dependencies: Vec<String>,
    custom_textures_path: String,
    custom_game_files_path: String,
    icon_path: String,
}

#[derive(Serialize, Deserialize)]
struct CheckISOResult {
    id: String,
    nkit: bool,
}

#[tauri::command]
fn start_em2_steam() {
    Command::new("steam")
        .arg("steam://rungameid/245300")
        .spawn();
}

#[tauri::command]
fn open_dolphin(path: String) {
    println!("deek");
    let mut config_path = dirs_next::config_dir().expect("could not get config dir");
    config_path.push(r"com.memer.eml");
    config_path.push("DolphinConfig");

    #[cfg(target_os = "windows")]
    Command::new(path).arg("-u").arg(config_path).spawn();
    #[cfg(target_os = "linux")]
    //QT env variable is for wayland functionality
    Command::new(if path == "" { "dolphin-emu" } else { &path })
        .arg("-u")
        .arg(config_path)
        .env("QT_QPA_PLATFORM", "xcb")
        .env("WAYLAND_DISPLAY", "+")
        .spawn()
        .expect("failed to start dolphin");
    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .arg("--args")
        .arg("-u")
        .arg(config_path)
        .spawn();
}

#[tauri::command]
fn open_link(url: String) {
    open::that(url).expect("Failed to open URL in default browser");
}

#[tauri::command]
fn create_portable(dolphinpath: String) {
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

    println!("{}", &path.display());

    if !path.exists() {
        File::create(&path).expect("Failed to create file");
        set_dolphin_emulator_override(dolphin_config_path.clone().to_str().unwrap().to_string());
        dolphin_config_path.push("Config");
        fs::create_dir_all(&dolphin_config_path).unwrap();
        dolphin_config_path.push("GFX.ini");
        let mut f = File::create(dolphin_config_path).expect("Failed to create file");
        f.write(b"[Settings]\nHiresTextures = True").unwrap();
    }
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
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml");
    path.push("cachedMods");

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir(path).unwrap();
}

#[tauri::command]
fn get_bootbin_id(path: String) -> String {
    let mut f = File::open(path).unwrap();
    let mut id_bytes = [0; 6];
    f.read_exact(&mut id_bytes).unwrap();
    let id = std::str::from_utf8(&id_bytes[0..6]).unwrap().to_uppercase();
    return id;
}

const CREATE_NO_WINDOW: u32 = 0x08000000;

struct ModFilesInfo {
    files: Vec<String>,
    textures: Vec<String>,
}

fn parse_mod_info(path: String) -> ModFilesInfo {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buffer: String = default::Default::default();
    file.read_to_string(&mut buffer).unwrap();

    let lines = buffer.split("\n");

    let mut files = Vec::new();
    let mut textures = Vec::new();

    let mut texture_mode = false;

    for line in lines {
        if line == "[Textures]" {
            texture_mode = true;
            continue;
        } else if line == "[Files]" {
            texture_mode = false;
            continue;
        } else if line == "" {
            continue;
        }

        if texture_mode {
            textures.push(line.to_string());
        } else {
            files.push(line.to_string());
        }
    }

    return ModFilesInfo {
        files: files,
        textures: textures,
    };
}

#[tauri::command]
fn write_mod_info(path: String, files: Vec<String>, textures: Vec<String>) {
    let mut file = File::create(path).expect("Failed to create file");

    if files.len() > 0 {
        file.write(b"[Files]\n").unwrap();
    }

    for file_path in files {
        file.write(file_path.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }

    if textures.len() > 0 {
        file.write(b"[Textures]\n").unwrap();
    }

    for file_path in textures {
        file.write(file_path.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}

#[tauri::command]
async fn extract_iso(isopath: String, gamename: String, window: Window, dolphin: String) -> String {
    let mut destination = PathBuf::new();
    destination.push(dirs_next::config_dir().expect("could not get config dir"));
    destination.push("com.memer.eml");
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

    println!("{}", destination.display());

    Command::new(dolphin_tool)
        .arg("extract")
        .arg(isopath)
        .arg(&destination)
        .output()
        .expect("failed to open dolphin-tool");

    return destination.to_str().unwrap().to_string();
}

#[tauri::command]
async fn download_tool(url: String, foldername: String, window: Window) -> PathBuf {
    log(&format!("Beginning download of {}", url));
    let mut to_pathbuf = PathBuf::new();
    to_pathbuf.push(dirs_next::config_dir().expect("could not get config dir"));
    to_pathbuf.push("com.memer.eml");
    to_pathbuf.push(foldername);
    download_zip(url, &to_pathbuf, false, window).await;
    log(&format!("Download Finished"));
    to_pathbuf
}

async fn download_zip(url: String, foldername: &PathBuf, local: bool, window: Window) -> String {
    log(&format!("Downloading Archive {}", url));
    fs::create_dir_all(&foldername).expect("Failed to create");

    let mut temporary_archive_path_buf = foldername.clone();

    temporary_archive_path_buf.push("temp");

    let temporary_archive_path = temporary_archive_path_buf.to_str().unwrap().to_string();

    let mut buffer;

    let mut f = File::create(&temporary_archive_path).expect("Failed to create tmpzip");

    if !local {
        let res = Client::new().get(&url).send().await.unwrap();

        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))
            .unwrap();

        window
            .emit(
                "download-stat",
                ModDownloadStats {
                    Download_Total: total_size.to_string(),
                    Download_Remaining: "0".to_string(),
                },
            )
            .unwrap();

        buffer = reqwest::get(&url).await.unwrap().bytes_stream();

        let mut download_bytes_count = 0;

        while let Some(item) = buffer.next().await {
            let mut success = false;

            let mut buf = &Bytes::new();

            let res = item.as_ref();

            buf = match res {
                Ok(b) => b,
                Err(error) => {
                    buffer = reqwest::get(&url).await.unwrap().bytes_stream();
                    download_bytes_count = 0;
                    fs::remove_file(&temporary_archive_path).expect("failed to remove tmpzip");
                    f = File::create(&temporary_archive_path).expect("Failed to create tmpzip");
                    println!("Download error occured. Restarting Download.");
                    log("Download error occured. Restarting Download.");
                    buf
                }
            };

            if Bytes::is_empty(buf) {
                continue;
            }

            download_bytes_count += buf.len();

            window
                .emit(
                    "download-stat",
                    ModDownloadStats {
                        Download_Total: total_size.to_string(),
                        Download_Remaining: download_bytes_count.to_string(),
                    },
                )
                .unwrap();

            f.write_all(buf).expect("Failed to write to tmpzip");
        }
    } else {
        //horrible solution
        fs::copy(&url, &temporary_archive_path).expect("Failed to copy local file");
    }

    let output = PathBuf::from(&foldername);

    let extension = extract_archive(url, temporary_archive_path, &output);

    log("Finished archive download");

    extension
}

#[derive(Clone, serde::Serialize)]

struct ModDownloadStats {
    Download_Remaining: String,
    Download_Total: String,
}

#[tauri::command]
fn open_config_folder() {
    let mut path = PathBuf::new();

    path.push(dirs_next::config_dir().unwrap());
    path.push("com.memer.eml");

    open_path_in_file_manager(path.to_str().unwrap().to_owned())
}

fn extract_archive(url: String, input_path: String, output_path: &PathBuf) -> String {
    log(&format!("Extracting Archive {}", input_path));

    let mut f = File::open(&input_path).expect("Couldn't open archive");
    let mut buffer = [0; 262];

    let mut archive_type = "";

    f.read(&mut buffer).expect("failed to read archive header");

    if &buffer[0..2] == "PK".as_bytes() {
        println!("Archive is Zip");

        archive_type = "zip";

        let mut f = File::open(&input_path).expect("Failed to open tmpzip");

        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer).expect("Failed to read tmpzip");

        zip_extract::extract(Cursor::new(buffer), &output_path, false).expect("failed to extract");
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

fn main() {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml");
    fs::create_dir_all(&path);
    path.push("Log.txt");

    if !Path::exists(&path) {
        fs::write(&path, [0]);
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
    )
    .unwrap();

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
            linux_check_exist,
            open_path_in_file_manager,
            open_config_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn get_os() -> &'static str {
    log(&format!("Operating System: {}", env::consts::OS));
    env::consts::OS
}
#[tauri::command]
fn open_process(path: String, args: String) {
    Command::new(path)
        .arg(args)
        .output()
        .expect("failed to execute process");
}

fn log(output: &str) {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/Log.txt");
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

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    file.write(final_output.as_bytes()).unwrap();
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
    log(&format!("Attempting to delete mod ({}).", modid));
    let p = PathBuf::from(format!("{}/{}", dumploc, modid));

    if !p.exists() {
        return;
    }

    let data = parse_mod_info(p.to_str().unwrap().to_string());

    let files = data.files;
    let texturefiles = data.textures;

    if active {
        let mut datafiles_path = PathBuf::new();
        datafiles_path.push(&dumploc);
        if platform == "wii" {
            datafiles_path.push("files");
        }

        let mut backup_path = PathBuf::new();
        backup_path.push(&dumploc);
        backup_path.push("backup");

        let mut files_to_remove = files.len() + texturefiles.len();

        for file in files {
            let mut source_path = PathBuf::new();
            source_path.push(&backup_path);
            source_path.push(&file);

            let mut destination_path = PathBuf::new();
            destination_path.push(&datafiles_path);
            destination_path.push(&file);

            if std::path::Path::new(&source_path).exists()
                && std::path::Path::new(&destination_path).exists()
            {
                files_to_remove -= 1;

                window
                    .emit(
                        "change_description_text_delete",
                        format!(
                            "Restoring original files... Remaining files: {}",
                            files_to_remove
                        ),
                    )
                    .unwrap();

                fs::copy(source_path, destination_path).unwrap();
            }
        }

        log("Removed modded files.");

        let mut p = PathBuf::from("Load/Textures/");
        p.push(&gameid);

        let dolphin_path = find_dolphin_dir(&p);

        for file in texturefiles {
            let mut path = PathBuf::new();

            path.push(&dolphin_path);

            let path_final = remove_first(&file).expect("couldn't remove slash from string");

            path.push(path_final);

            if std::path::Path::new(&path).exists() {
                files_to_remove -= 1;

                window
                    .emit(
                        "change_description_text_delete",
                        format!("Deleting Textures... Remaining files: {}", files_to_remove),
                    )
                    .unwrap();

                fs::remove_file(&path).unwrap();
            }
        }
        log("Removed texture files.");
    }
    log("Process ended.");
    println!("Proccess ended");
}

#[tauri::command]
fn open_path_in_file_manager(path: String) {
    #[cfg(target_os = "windows")]
    Command::new("explorer.exe")
        .arg(path)
        .spawn()
        .expect("failed to execute process");

    #[cfg(target_os = "macos")]
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("failed to execute process");

    #[cfg(target_os = "linux")]
    Command::new("dolphin")
        .arg(path)
        .spawn()
        .expect("failed to execute process");
}

#[tauri::command]
fn playgame(dolphin: String, exe: String, id: String) -> i32 {
    let config_path = find_dolphin_dir(&PathBuf::new());

    auto_set_custom_textures();

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

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

#[tauri::command]
fn check_iso(path: String) -> CheckISOResult {
    let mut f = File::open(path).expect("Couldn't open ISO");
    let mut buffer = [0; 1000];
    f.read(&mut buffer).expect("failed to read game id");
    let id = std::str::from_utf8(&buffer[0..6]).unwrap().to_uppercase();
    let nkit = std::str::from_utf8(&buffer[0x200..0x204])
        .unwrap()
        .to_uppercase();
    let is_nkit = if nkit == "NKIT" { true } else { false };
    let res = CheckISOResult {
        id: id.clone(),
        nkit: is_nkit,
    };

    log(&format!("Disc ID: {} | NKit: {}", id, is_nkit));
    res
}

#[tauri::command]
async fn change_mod_status(
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    active: bool,
    modname: String,
    window: Window,
) {
    let active = active;

    let name = modname;

    if active {
        log(&format!("Mod ({}) Enabled", modid));
        //todo: fix this shit
        download_mod(
            "".to_string(),
            name.to_string(),
            dumploc,
            gameid,
            modid,
            platform,
            window,
        )
        .await;
    } else {
        log(&format!("Mod ({}) Disabled.", modid));
        delete_mod(dumploc, gameid, platform, modid, !active, window).await;
    }

    println!("Proccess ended");
}

#[derive(Serialize, Deserialize)]
struct ValidationInfo {
    modname: String,
    modicon: String,
    extension: String,
    validated: bool,
}

#[derive(Serialize, Deserialize)]
struct SmallArchiveValidationInfo {
    under_limit: bool,
    extension: String,
}

#[tauri::command]
fn validate_archive(path: String) -> SmallArchiveValidationInfo {
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

#[tauri::command]
fn delete_mod_cache(modid: String) {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/cachedMods");
    path.push(modid);
    if path.exists() {
        fs::remove_dir_all(path).expect("Could not remove mod cache");
    }
}

#[tauri::command]
fn set_dolphin_emulator_override(_path: String) {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml");

    fs::create_dir_all(&path).unwrap();

    path.push("dolphinoverride");

    let mut f = File::create(&path).expect("Failed to create file");

    f.write_all(_path.as_bytes())
        .expect("Failed to write to file");
}

#[tauri::command]
async fn validate_mod(url: String, local: bool, window: Window) -> ValidationInfo {
    println!("Validating mod");

    let mut path_imgcache = dirs_next::config_dir().expect("could not get config dir");
    path_imgcache.push("cache");

    fs::create_dir_all(&mut path_imgcache).expect("Failed to create folders.");

    path_imgcache.push("temp.png");

    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/TMP");

    let mut json_path = path.clone();
    json_path.push("mod.json");

    let mut icon_path = path.clone();

    let extension = download_zip(url, &path, local, window).await;

    println!("Finished Downloading mod for validation");

    let mut validation = ValidationInfo {
        modname: "".to_string(),
        modicon: "".to_string(),
        extension: extension,
        validated: false,
    };

    if Path::exists(&json_path) {
        let json_string =
            fs::read_to_string(&json_path).expect("mod.json does not exist or could not be read");
        let json_data: ModInfo = serde_json::from_str(&json_string)
            .expect("Mod data either doesn't exist or couldn't be loaded due to formatting error.");
        icon_path.push(json_data.icon_path);

        if Path::exists(&icon_path) {
            fs::copy(icon_path, &path_imgcache).expect("Could not copy icon file to cache");
            validation.validated = true;
            validation.modicon = path_imgcache
                .to_str()
                .expect("Couldn't convert path to string.")
                .to_string();
            validation.modname = json_data.name;
        } else {
            println!("Icon file does not exist");
        }
    } else {
        println!("Mod.json does not exist");
    }
    //fs::remove_dir_all(&path).expect("Couldn't remove temporary directory");
    println!("Finished Validating mod");
    validation
}

fn correct_all_slashes(path: String) -> String {
    path.replace(r"\", "/")
}

#[tauri::command]
async fn download_mod(
    url: String,
    name: String,
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    window: Window,
) {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/cachedMods");

    let mut full_path = path.clone();
    full_path.push(&modid);

    let os = env::consts::OS;

    // download

    let mut mod_json_path_check = full_path.clone();
    mod_json_path_check.push("mod.json");

    if !mod_json_path_check.exists() && !url.is_empty() {
        fs::create_dir_all(&full_path).expect("Couldn't create mod cache folder");

        let is_local = !url.starts_with("http");

        download_zip(url, &full_path, is_local, window).await;
        println!("done downloading");
    }

    let mut path_json = full_path.clone();
    path_json.push("mod.json");

    let json_string =
        fs::read_to_string(path_json).expect("mod.json does not exist or could not be read");

    let json_data: ModInfo = serde_json::from_str(&json_string)
        .expect("Mod data either doesn't exist or couldn't be loaded due to formatting error.");

    //inject files

    let mut path_textures = full_path.clone();
    let mut path_datafiles = full_path.clone();

    path_textures.push(&json_data.custom_textures_path);
    path_datafiles.push(&json_data.custom_game_files_path);

    let mut files_to_restore: Vec<String> = Vec::new();

    //inject DATA files into current dump
    if Path::new(&path_datafiles).exists() {
        let mut path_final_location = PathBuf::new();

        let dumploc_clone = dumploc.clone();

        path_final_location.push(&dumploc);

        if platform == "wii" {
            path_final_location.push("files");
        }

        //backup files
        let mut path_backup = PathBuf::new();

        path_backup.push(dumploc_clone);

        path_backup.push("backup");

        fs::create_dir_all(&path_backup).expect("couldn't create backup folder");

        let path_search_clone = path_datafiles.clone();

        let path_datafiles_clone_str = path_datafiles.clone();

        let path_datafiles_str = correct_all_slashes(
            path_datafiles_clone_str
                .into_os_string()
                .into_string()
                .unwrap(),
        );

        let mut dirs: Vec<String> = Vec::new();

        //we're copying the folders too since you never know if the mod put in an extra

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if !p.path().is_file() {
                let p_str = correct_all_slashes(
                    p.path()
                        .to_str()
                        .expect("Couldn't convert path to string.")
                        .to_string(),
                );

                //HACK: this can probably be done better right?
                let dont_end_with = format!(r"{}{}", "/", json_data.custom_game_files_path);

                if p_str.ends_with(&dont_end_with) {
                    continue;
                }

                let p_str_shortened = p_str.replace(&path_datafiles_str, "");

                let p_str_final =
                    remove_first(&p_str_shortened).expect("couldn't remove slash from string");

                dirs.push(p_str_final.to_string());
            }
        }

        for directory in &dirs {
            let mut dir = PathBuf::new();
            dir.push(&path_backup);
            dir.push(directory);

            fs::create_dir_all(&dir).expect("Failed to create folders.");
        }

        println!("Created Folders");

        let mut files: Vec<String> = Vec::new();

        //backup files

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if p.path().is_file() {
                let p_str = correct_all_slashes(
                    p.path()
                        .to_str()
                        .expect("Couldn't convert path to string.")
                        .to_string(),
                );

                let p_str_shortened = &p_str.replace(&path_datafiles_str, "");

                //get rid of slash

                let p_str_final =
                    remove_first(&p_str_shortened).expect("couldn't remove slash from string");

                files.push(p_str_final.to_string());
            }
        }

        for file in &files {
            let mut source = PathBuf::new();
            source.push(&dumploc);
            if platform == "wii" {
                source.push("files");
            }
            source.push(file);

            let mut destination = PathBuf::new();
            destination.push(&path_backup);
            destination.push(file);

            if std::path::Path::new(&source).exists()
                && !std::path::Path::new(&destination).exists()
            {
                fs::copy(&source, destination).expect("couldn't copy file to backup");
            }

            files_to_restore.push(file.to_string());
        }

        println!("Created Files");

        // copy modded files to the game

        log(&format!(
            "Injecting Game files into: {}",
            &path_final_location.display()
        ));

        inject_files(&path_datafiles, &path_final_location);

        //copy(&path_datafiles, path_final_location, &options).expect("failed to inject mod files");
    }

    let mut texturefiles: Vec<String> = Vec::new();

    let mut p = PathBuf::from("Load/Textures/");
    p.push(&gameid);

    let dolphin_path = find_dolphin_dir(&p);

    fs::create_dir_all(&dolphin_path).expect("Failed to create dolphin folder.");

    //inject texture files into dolphin config
    if Path::new(&path_textures).exists() {
        let path_textures_str = &path_textures
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();

        for entry in WalkDir::new(&path_textures) {
            let p = entry.unwrap();

            if p.path().is_file() {
                let p_str = p.path().to_str().expect("Couldn't convert path to string.");
                let p_str_final = &p_str.replace(path_textures_str, "");
                texturefiles.push(p_str_final.to_string());
            }
        }

        fs::create_dir_all(&path).expect("Failed to create folders.");

        log(&format!(
            "Injecting Texture files into: {}",
            &dolphin_path.display()
        ));

        inject_files(&path_textures, &dolphin_path)

        //copy(&path_textures, &dolphin_path, &options).expect("failed to inject texture files");
    }

    write_mod_info(
        format!("{}/{}", &dumploc, modid),
        files_to_restore,
        texturefiles,
    );

    println!("Process ended successfully");
}

fn inject_files(source: &PathBuf, _destination: &PathBuf) {
    for entry in WalkDir::new(&source) {
        let p = PathBuf::from(entry.unwrap().path());

        if p.is_file() {
            let non_abs = remove_absolute_path(&p, &source);
            let mut destination = _destination.clone();
            destination.push(&non_abs);

            let mut destination_folder = _destination.clone();
            destination_folder.push(non_abs);
            destination_folder.pop();

            if !destination_folder.exists() {
                fs::create_dir_all(&destination_folder).expect("Failed to create folders.");
            }

            if p.exists() {
                if destination.exists() {
                    fs::remove_file(&destination).unwrap();
                }

                fs::copy(p, destination).expect("Failed to copy file");
            }
        }
    }
}

fn remove_absolute_path(path: &PathBuf, _abs_path: &PathBuf) -> PathBuf {
    let path = path.to_str().unwrap().to_string();
    let abs_path = _abs_path.to_str().unwrap().len() + 1;

    return PathBuf::from(path[abs_path..path.len()].to_string());
}

#[tauri::command]
fn linux_check_exist(package: String) -> bool {
    let output = Command::new("ls").arg("/bin").output().unwrap();
    let str_output = String::from_utf8(output.stdout);
    str_output.unwrap().contains(&package)
}

fn auto_set_custom_textures() {
    let mut buf = find_dolphin_dir(&PathBuf::from("Config"));

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

fn find_dolphin_dir(where_in: &PathBuf) -> PathBuf {
    let mut config_path = dirs_next::config_dir().expect("could not get config dir");
    config_path.push(r"com.memer.eml");
    config_path.push("DolphinConfig");
    config_path.push(where_in);
    config_path
}

//note 2self or whoever. macos directory system uses / and not \

/* #![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] */

use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::default;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use walkdir::WalkDir;
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
fn open_link(url: String) {
    open::that(url).expect("Failed to open URL in default browser");
}

#[tauri::command]
fn create_portable(_path: String) {
    let mut dolphin_config_path = PathBuf::from(&_path);

    dolphin_config_path.pop();

    dolphin_config_path.push("User");

    let mut path = PathBuf::from(&_path);
    path.pop();
    path.push("portable.txt");

    if !path.exists() {
        File::create(&path).expect("Failed to create file");
        set_dolphin_emulator_override(dolphin_config_path.to_str().unwrap().to_string());
        Command::new(_path).spawn().unwrap();
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
async fn extract_iso(
    witpath: String,
    nkit: String,
    isopath: String,
    gamename: String,
    is_nkit: bool,
    window: Window,
) -> String {
    let mut extracted_iso_path = PathBuf::new();
    #[cfg(target_os = "windows")]
    extracted_iso_path.push("c:/extractedwii");
    #[cfg(target_os = "linux")]
    {
        extracted_iso_path.push("Z:");
        extracted_iso_path.push(dirs_next::data_local_dir().unwrap());
        extracted_iso_path.push("com.memer.eml");
        extracted_iso_path.push("extractedwii");
    }

    let mut source_path = PathBuf::new();
    source_path.push(&extracted_iso_path);

    /*  extracted_iso_path.push(&witpath);
     extracted_iso_path.pop();
    extracted_iso_path.push("extracted_iso");  */

    if Path::new(&extracted_iso_path).exists() {
        fs::remove_dir_all(&extracted_iso_path).expect("Failed to create temp folder");
    }

    let mut response = "".to_string();
    let mut m_isopath = PathBuf::new();
    m_isopath.push(&isopath);

    let mut remove_nkit_processed = false;
    if is_nkit {
        if nkit != "" {
            window
                .emit("change_iso_extract_msg", "Converting NKit to ISO...")
                .unwrap();

            let mut proc_path = PathBuf::new();
            proc_path.push(&nkit);
            proc_path.push("ConvertToISO.exe");

            #[cfg(target_os = "windows")]
            Command::new(proc_path)
                .arg(&m_isopath)
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .expect("NKit failed to start");

            #[cfg(target_os = "linux")]
            Command::new("wine")
                .arg(proc_path)
                .arg("z:".to_owned() + &m_isopath.to_str().unwrap())
                .output()
                .expect("NKit failed to start");

            source_path.push("DATA");

            //HACK: probably the worst way to do this

            let p = nkit + "/Processed/Wii/";

            let paths = fs::read_dir(p).unwrap();
            let mut foundfirst = false;
            for path in paths {
                if !foundfirst {
                    let binding = path
                        .unwrap()
                        .path()
                        .to_str()
                        .expect("Can't get path")
                        .clone()
                        .to_string();

                    if binding.ends_with(".iso") {
                        m_isopath = PathBuf::new();

                        m_isopath = binding.into();

                        println!("{}", m_isopath.display());

                        foundfirst = true;
                        remove_nkit_processed = true;
                    }
                }
            }

            if !foundfirst {
                return "err_nkit".to_string();
            }
        } else {
            return "err_nkit".to_string();
        }
    }

    window
        .emit("change_iso_extract_msg", "Dumping ISO...")
        .unwrap();

    #[cfg(target_os = "windows")]
    Command::new(&witpath)
        .arg("extract")
        .arg(&m_isopath)
        .arg("-D")
        .arg(extracted_iso_path)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("failed to execute process");

    let p = "Z:".to_owned() + &m_isopath.to_str().unwrap();
    let extracted_p = "Z:".to_owned() + extracted_iso_path.clone().to_str().unwrap();
    #[cfg(target_os = "linux")]
    Command::new("wine")
        .arg(&witpath)
        .arg("extract")
        .arg(p)
        .arg("-D")
        .arg(extracted_p)
        .output()
        .expect("failed to execute process");

    window
        .emit("change_iso_extract_msg", "Cleaning Up...")
        .unwrap();

    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push("com.memer.eml");
    path.push("Games");
    path.push(gamename);

    let without_data = path.clone();

    path.push("DATA");

    if !Path::new(&path).exists() {
        fs::create_dir_all(&path).expect("Couldn't create game folder");
    }

    window
        .emit("change_iso_extract_msg", "Injecting Game Files...")
        .unwrap();

    if source_path.exists() {
        inject_files(&source_path, &path);

        window
            .emit("change_iso_extract_msg", "Cleaning up....")
            .unwrap();

        if remove_nkit_processed && Path::new(&m_isopath).exists() {
            fs::remove_file(m_isopath).expect("failed to remove converted nkit iso");
        }

        response = without_data.display().to_string();

        if Path::new(&extracted_iso_path).exists() {
            fs::remove_dir_all(extracted_iso_path).expect("Failed to remove temp folder");
        }
    } else {
        response = "err_extract".to_string();
    }

    window.emit("change_iso_extract_msg", "Finished!").unwrap();

    return response.to_string();
}

#[tauri::command]
async fn download_tool(url: String, foldername: String, window: Window) -> PathBuf {
    let mut to_pathbuf = PathBuf::new();
    to_pathbuf.push(dirs_next::config_dir().expect("could not get config dir"));
    to_pathbuf.push("com.memer.eml");
    to_pathbuf.push(foldername);
    download_zip(url, &to_pathbuf, false, window).await;
    to_pathbuf
}

async fn download_zip(url: String, foldername: &PathBuf, local: bool, window: Window) -> String {
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
            let buf = item.as_ref().unwrap();
            if (buf.is_empty()) {
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

    extension
}

#[derive(Clone, serde::Serialize)]

struct ModDownloadStats {
    Download_Remaining: String,
    Download_Total: String,
}

fn extract_archive(url: String, input_path: String, output_path: &PathBuf) -> String {
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
    } else {
        println!("Unknown archive type");
    }
    fs::remove_file(input_path).expect("Failed to remove tmpzip");
    archive_type.to_string()
}

fn main() {
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
            change_mod_status,
            delete_mod,
            validate_mod,
            get_os,
            extract_iso,
            delete_mod_cache,
            check_iso,
            open_link,
            download_tool,
            validate_archive,
            set_dolphin_emulator_override,
            delete_docs_folder,
            write_mod_info,
            open_process,
            delete_mod_cache_all,
            create_portable,
            linux_check_exist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn get_os() -> &'static str {
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
fn playgame(dolphin: String, exe: String) -> i32 {
    let os = env::consts::OS;
    if Path::new(&dolphin).exists() {
        if os == "windows" {
            if dolphin.ends_with(".exe") {
                let path = find_dolphin_dir(&PathBuf::from("Config/GFX.ini"));

                if path.exists() {
                    let mut f = File::open(&path).unwrap();

                    let mut path_buffer: String = Default::default();

                    f.read_to_string(&mut path_buffer)
                        .expect("Failed to read file");

                    path_buffer =
                        path_buffer.replace("HiresTextures = False", "HiresTextures = True");

                    let mut new = File::create(&path).unwrap();

                    new.write_all(path_buffer.as_bytes())
                        .expect("Failed to write to file");
                }

                Command::new(&dolphin)
                    .arg(&exe)
                    .spawn()
                    .expect("could not open exe");
            } else if Path::new(&exe).exists() {
                Command::new(&dolphin)
                    .arg(&exe)
                    .spawn()
                    .expect("could not open dolphin");
            }
            return 0;
        } else if os == "macos" {
            Command::new("open")
                .arg("-a")
                .arg(&dolphin)
                .arg(&exe)
                .spawn()
                .expect("could not open dolphin");
            return 0;
        } else {
            Command::new("gtk-launch")
                .arg("dolphin-emu.desktop")
                .arg(&exe)
                .spawn()
                .expect("could not open dolphin");
            return 0;
        }
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
        id: id,
        nkit: is_nkit,
    };
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
        //HACK!!
        delete_mod(dumploc, gameid, platform, modid, !active, window).await;
    }

    println!("Proccess ended");
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

        println!("{}", &dolphin_path.display());

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

fn find_dolphin_dir(where_in: &PathBuf) -> PathBuf {
    let os = env::consts::OS;

    let mut dolphin_path = PathBuf::new();

    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml");
    path.push("dolphinoverride");

    if !path.exists() {
        if os == "macos" {
            dolphin_path = dirs_next::config_dir().expect("Failed to get config path");
            dolphin_path.push(Path::new(r"Dolphin"));
            dolphin_path.push(where_in);
        } else if os == "windows" {
            dolphin_path = dirs_next::document_dir().expect("Failed to get config path");
            dolphin_path.push(Path::new(r"Dolphin Emulator"));
            dolphin_path.push(where_in);

            if dolphin_path.exists() {
                return dolphin_path;
            }

            dolphin_path = dirs_next::config_dir().expect("Failed to get config path");
            dolphin_path.push(Path::new(r"Dolphin Emulator"));
            dolphin_path.push(where_in);

            if dolphin_path.exists() {
                return dolphin_path;
            }
        } else {
            if dolphin_path.exists() {
                return dolphin_path;
            }

            dolphin_path = dirs_next::data_dir().expect("Failed to get config path");
            dolphin_path.push(Path::new(r"dolphin-emu"));
            dolphin_path.push(where_in);

            if dolphin_path.exists() {
                return dolphin_path;
            }
        }
    } else {
        let mut f = File::open(path).unwrap();
        dolphin_path.clear();
        let mut buff = String::new();
        f.read_to_string(&mut buff).expect("Failed to read file");
        dolphin_path.push(buff);
        dolphin_path.push(where_in);
    }

    dolphin_path
}

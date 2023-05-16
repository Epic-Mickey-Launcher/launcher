//note 2self or whoever. macos directory system uses / and not \

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use bytes::{BufMut, BytesMut};
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;
extern crate dirs_next;
extern crate fs_extra;
extern crate reqwest;
extern crate scan_dir;
extern crate walkdir;
extern crate zip_extract;
use fs_extra::dir::copy;

#[derive(Serialize, Deserialize)]
struct ChangedFiles {
    name: String,
    modid: String,
    files: Vec<String>,
    texturefiles: Vec<String>,
    active: bool,
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            playgame,
            download_mod,
            change_mod_status,
            delete_mod,
            validate_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn playgame(dolphin: String, exe: String) -> i32 {
    if Path::new(&dolphin).exists() {

        Command::new(dolphin)
        .arg(&exe)
        .spawn()
        .expect("ls command failed to start");
        return 0
    }
    1
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

#[tauri::command]
async fn change_mod_status(json: String, dumploc: String, gameid: String, modid: String) {
    let data: ChangedFiles = serde_json::from_str(&json).unwrap();

    let texturefiles = data.texturefiles;
    let files = data.files;

    let mut datafiles_path = PathBuf::new();
    datafiles_path.push(&dumploc);
    datafiles_path.push("files");

    let mut backup_path = PathBuf::new();
    backup_path.push(&dumploc);
    backup_path.push("backup");

    let active = data.active;
    let name = data.name;

    if active {
        download_mod("".to_string(), name, dumploc, gameid, modid).await;
    } else {
        //this is identical to delete_mod so combining both into a function would be a good practice

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
                fs::copy(source_path, destination_path).unwrap();
            }
        }

        let mut dolphin_path = find_dolphin_dir(gameid);
    

        for file in texturefiles {
            let mut path = PathBuf::new();

            path.push(&dolphin_path);

            let path_final = remove_first(&file).expect("couldn't remove slash from string");

            path.push(path_final);

            if std::path::Path::new(&path).exists() {
                fs::remove_file(&path).unwrap();
            }
        }
    }

    println!("Proccess ended");
}

#[tauri::command]
async fn delete_mod(json: String, dumploc: String, gameid: String) {
    let data: ChangedFiles = serde_json::from_str(&json).unwrap();

    let files = data.files;
    let texturefiles = data.texturefiles;

    let active = data.active;

    if active {
        let mut datafiles_path = PathBuf::new();
        datafiles_path.push(&dumploc);
        datafiles_path.push("files");

        let mut backup_path = PathBuf::new();
        backup_path.push(&dumploc);
        backup_path.push("backup");

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
                fs::copy(source_path, destination_path).unwrap();
            }
        }

        let mut dolphin_path = find_dolphin_dir(gameid);

        for file in texturefiles {
            let mut path = PathBuf::new();

            path.push(&dolphin_path);

            let path_final = remove_first(&file).expect("couldn't remove slash from string");

            path.push(path_final);

            if std::path::Path::new(&path).exists() {
                fs::remove_file(&path).unwrap();
            }
        }
    }

    println!("Proccess ended");
}

#[derive(Serialize, Deserialize)]
struct ValidationInfo{
    modname: String,
    modicon: String,
    validated: bool
}

#[tauri::command]
async fn validate_mod(url: String, local: bool) -> ValidationInfo {

    let mut path_imgcache = dirs_next::config_dir().expect("could not get config dir");
    path_imgcache.push("cache");

    fs::create_dir_all(&mut path_imgcache).expect("Failed to create folders.");

    path_imgcache.push("temp.png");

    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/TMP");

    let mut json_path = path.clone();
    json_path.push("mod.json");

    let mut icon_path = path.clone();
 

    let buffer;
    if !local {
        buffer = reqwest::get(url)
            .await
            .expect("fail")
            .bytes()
            .await
            .expect("get bytes FAIL");
    } else {
        let f = File::open(url).expect("Failed to open local file");
        let mut reader = BufReader::new(f);
        let mut buff = Vec::new();
        reader
            .read_to_end(&mut buff)
            .expect("Failed to read bytes from local file");
        let mut buffer_to_bytes = BytesMut::new();
        buffer_to_bytes.put(buff.as_slice());
        buffer = buffer_to_bytes.into();
    }

    fs::create_dir_all(&mut path).expect("Failed to create folders.");

    let bytes = buffer;
    zip_extract::extract(Cursor::new(bytes), &path, false).expect("failed to extract");
 
    let mut validation = ValidationInfo {
        modname:"".to_string(),
        modicon:"".to_string(),
        validated: false
    };

    if Path::exists(&json_path){
        
        let json_string = fs::read_to_string(&json_path).expect("mod.json does not exist or could not be read");
        let json_data: ModInfo = serde_json::from_str(&json_string).expect("Mod data either doesn't exist or couldn't be loaded due to formatting error.");
        icon_path.push(json_data.icon_path);
        
        if Path::exists(&icon_path)
        {
            fs::copy(icon_path, &path_imgcache).expect("Could not copy icon file to cache");
            validation.validated = true;
            validation.modicon = path_imgcache.to_str().expect("Couldn't convert path to string.").to_string();
            validation.modname = json_data.name;
            fs::remove_dir_all(&path).expect("Couldn't remove temporary directory");
            validation
        }
        else{
            fs::remove_dir_all(&path).expect("Couldn't remove temporary directory");
            validation
        }
    }
    else{
        fs::remove_dir_all(&path).expect("Couldn't remove temporary directory");
        validation
    }

}

fn correct_all_slashes(path: String) -> String
{
        path.replace(r"\", "/")
}

#[tauri::command]
async fn download_mod(url: String, name: String, dumploc: String, gameid: String, modid: String) -> String {
    let mut path = dirs_next::config_dir().expect("could not get config dir");
    path.push(r"com.memer.eml/cachedMods");

    let mut full_path = path.clone();
    full_path.push(&modid);


    let os = env::consts::OS;

    if !Path::new(&full_path).exists() {
        // download
        println!("started downloading");
        println!("{}", url);

        let buffer;

        if url.starts_with("http") {
            buffer = reqwest::get(url)
                .await
                .expect("fail")
                .bytes()
                .await
                .expect("get bytes FAIL");
        } else {
            let f = File::open(url).expect("Failed to open local file");
            let mut reader = BufReader::new(f);
            let mut buff = Vec::new();
            reader
                .read_to_end(&mut buff)
                .expect("Failed to read bytes from local file");
            let mut buffer_to_bytes = BytesMut::new();
            buffer_to_bytes.put(buff.as_slice());
            buffer = buffer_to_bytes.into();
        }

        let bytes = buffer;

        // install
        fs::create_dir_all(&mut path).expect("Failed to create folders.");

        //cache file for later downloads
        zip_extract::extract(Cursor::new(bytes), &full_path, false).expect("failed to extract");

        println!("done downloading");
    }

    

    let mut path_json = full_path.clone();
    path_json.push("mod.json");

    

    let json_string = fs::read_to_string(path_json).expect("mod.json does not exist or could not be read");

    let json_data: ModInfo = serde_json::from_str(&json_string).expect("Mod data either doesn't exist or couldn't be loaded due to formatting error.");

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

        path_final_location.push("files");

        //backup files
        let mut path_backup = PathBuf::new();

        path_backup.push(dumploc_clone);

        path_backup.push("backup");

        fs::create_dir_all(&path_backup).expect("couldn't create backup folder");

        let path_search_clone = path_datafiles.clone();

        let path_datafiles_clone_str = path_datafiles.clone();

        let path_datafiles_str = correct_all_slashes(path_datafiles_clone_str
            .into_os_string()
            .into_string()
            .unwrap());

        let mut dirs: Vec<String> = Vec::new();

        //we're copying the folders too since you never know if the mod put in an extra

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if !p.path().is_file() {
                let p_str = correct_all_slashes(p.path().to_str().expect("Couldn't convert path to string.").to_string());

                let dont_end_with = format!(r"{}{}", "/", json_data.custom_game_files_path);

                if p_str.ends_with(&dont_end_with) {
                    continue;
                }

                let p_str_shortened = p_str.replace(&path_datafiles_str, "");

                let p_str_final = remove_first(&p_str_shortened).expect("couldn't remove slash from string");

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
                let p_str = p.path().to_str().expect("Couldn't convert path to string.");

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
            source.push("files");
            source.push(file);

            let mut destination = PathBuf::new();
            destination.push(&path_backup);
            destination.push(file);


            if std::path::Path::new(&source).exists() && !std::path::Path::new(&destination).exists()
            {
                fs::copy(&source, destination).expect("couldn't copy file to backup");
            }

            files_to_restore.push(file.to_string());
        }

        println!("Created Files");

        // copy modded files to the game

        let options = CopyOptions {
            depth: 0,
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            content_only: true,
            copy_inside: false,
        };

        copy(&path_datafiles, path_final_location, &options).expect("failed to inject mod files");
    }

    let mut texturefiles: Vec<String> = Vec::new();

    let mut dolphin_path = find_dolphin_dir(gameid);

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

        let options = CopyOptions {
            depth: 0,
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            content_only: true,
            copy_inside: false,
        };

        fs::create_dir_all(&path).expect("Failed to create folders.");

        copy(&path_textures, &dolphin_path, &options).expect("failed to inject texture files");
    }

    let changed_files_json = ChangedFiles {
        name: name,
        files: files_to_restore,
        texturefiles: texturefiles,
        modid: modid,
        active: true,
    };

    let json = serde_json::to_string(&changed_files_json).unwrap();

    println!("Process ended successfully");
    json.into()
}

fn find_dolphin_dir(gameid: String) -> PathBuf
{
    let os = env::consts::OS;

    let mut dolphin_path = dirs_next::document_dir().expect("Failed to get documents path");

    if os == "macos"{
        dolphin_path = dirs_next::config_dir().expect("Failed to get config path");
        dolphin_path.push(Path::new(r"Dolphin/Load/Textures/"));
    }
    else{
        dolphin_path.push(Path::new(r"Dolphin Emulator\Load\Textures\"));
    }
    dolphin_path.push(Path::new(&gameid));

    dolphin_path
}
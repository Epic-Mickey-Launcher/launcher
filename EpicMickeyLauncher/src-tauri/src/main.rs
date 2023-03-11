#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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
    files: Vec<String>,
    texturefiles: Vec<String>,
    active: bool,
}

#[derive(Serialize, Deserialize)]
struct ModInfo {
    name: String,
    data_path: String,
    texture_path: String,
    dependencies: Vec<String>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            playgame,
            download_mod,
            change_mod_status,
            delete_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn playgame(dolphin: String, exe: String) {
    Command::new(dolphin)
        .arg(&exe)
        .spawn()
        .expect("ls command failed to start");
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

#[tauri::command]
async fn change_mod_status(json: String, dumploc: String, gameid: String) {
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
        download_mod("".to_string(), name, dumploc, gameid).await;
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

        let mut dolphin_path = dirs_next::document_dir().expect("Failed to get documents path");
        dolphin_path.push(Path::new(r"Dolphin Emulator\Load\Textures\"));
        dolphin_path.push(Path::new(&gameid));

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

        let mut dolphin_path = dirs_next::document_dir().expect("Failed to get documents path");
        let dolphin_texture = format!(r"Dolphin Emulator\Load\Textures\{}", gameid);
        dolphin_path.push(Path::new(&dolphin_texture));

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
async fn download_mod(url: String, name: String, dumploc: String, gameid: String) -> String {
    let mut path = PathBuf::new();
    path.push("C:/EMLStuff/Data/cachedMods");
    path.push(&name);

    if !Path::new(&path).exists() {
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
        fs::create_dir_all("C:/EMLStuff/Data/cachedMods").expect("Failed to create folders.");

        let mut file =
            File::create("C:/EMLStuff/Data/TEMP.zip").expect("failed to create temp file");
        file.write_all(&bytes).expect("failed to write");

        //cache file for later downloads
        zip_extract::extract(Cursor::new(bytes), &path, false).expect("failed to extract");

        println!("done downloading");
    }

    //inject files

    let mut path_textures = path.clone();
    let mut path_datafiles = path.clone();

    path_textures.push("custtext");
    path_datafiles.push("files/DATA/files");
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

        let path_datafiles_str = path_datafiles_clone_str
            .into_os_string()
            .into_string()
            .unwrap();

        let mut dirs: Vec<String> = Vec::new();

        //we're copying the folders too since you never know if the mod put in an extra

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if !p.path().is_file() {
                let p_str = p.path().to_str().expect("Couldn't convert path to string.");

                if p_str.ends_with(r"/files") {
                    continue;
                }

                let p_str_shortened = &p_str.replace(&path_datafiles_str, "");

                //get rid of slash

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

            if std::path::Path::new(&source).exists()
                && !std::path::Path::new(&destination).exists()
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

    let mut dolphin_path = dirs_next::document_dir().expect("Failed to get documents path");

    dolphin_path.push(Path::new(r"Dolphin Emulator\Load\Textures\"));
    dolphin_path.push(Path::new(&gameid));

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
        active: true,
    };

    let json = serde_json::to_string(&changed_files_json).unwrap();

    println!("Process ended successfully");
    json.into()
}

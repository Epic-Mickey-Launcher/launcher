use tauri::Window;
use serde::{Deserialize, Serialize};
use std::fs;
use walkdir::WalkDir;
use std::path::{PathBuf, Path};
use crate::mod_info;
use crate::dolphin;
use crate::helper;
use crate::debug;
use crate::download;

#[derive(Serialize, Deserialize)]
pub struct ValidationInfo {
    pub modname: String,
    pub modicon: String,
    pub validated: bool,
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

pub async fn add(
    url: String,
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    window: &Window,
) {
    let mut path = helper::get_config_path().expect("could not get config dir");
    path.push(r"cachedMods");
    let mut full_path = path.clone();
    full_path.push(&modid);

    // download

    let mut mod_json_path_check = full_path.clone();
    mod_json_path_check.push("mod.json");

    if !mod_json_path_check.exists() && !url.is_empty() {
        fs::create_dir_all(&full_path).expect("Couldn't create mod cache folder");

        let is_local = !url.starts_with("http");

        download::zip(url, &full_path, is_local, window).await;
        debug::log("done downloading");
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

        let path_datafiles_str = helper::correct_all_slashes(
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
                let p_str = helper::correct_all_slashes(
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
                    helper::remove_first(&p_str_shortened).expect("couldn't remove slash from string");

                dirs.push(p_str_final.to_string());
            }
        }

        for directory in &dirs {
            let mut dir = PathBuf::new();
            dir.push(&path_backup);
            dir.push(directory);

            fs::create_dir_all(&dir).expect("Failed to create folders.");
        }

        debug::log("Created Folders");

        let mut files: Vec<String> = Vec::new();

        //backup files

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if p.path().is_file() {
                let p_str = helper::correct_all_slashes(
                    p.path()
                        .to_str()
                        .expect("Couldn't convert path to string.")
                        .to_string(),
                );

                let p_str_shortened = &p_str.replace(&path_datafiles_str, "");

                let p_str_final =
                    helper::remove_first(&p_str_shortened).expect("couldn't remove slash from string");

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

        debug::log("Created Files");

        debug::log(&format!(
            "Injecting Game files into: {}",
            &path_final_location.display()
        ));

        helper::inject_files(&path_datafiles, &path_final_location);
    }

    let mut texturefiles: Vec<String> = Vec::new();

    let mut p = PathBuf::from("Load/Textures/");
    p.push(&gameid);

    let dolphin_path = dolphin::find_dir(&p);

    fs::create_dir_all(&dolphin_path).expect("Failed to create dolphin folder.");

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

        debug::log(&format!(
            "Injecting Texture files into: {}",
            &dolphin_path.display()
        ));

        helper::inject_files(&path_textures, &dolphin_path)
    }

    mod_info::write(
        &format!("{}/{}", &dumploc, modid),
        files_to_restore,
        texturefiles,
    ).expect("Failed to write mod info file.");

    debug::log("Process ended successfully");
}

pub async fn delete(
    dumploc: String,
    gameid: String,
    platform: String,       
    modid: String,
    active: bool,
    window: &Window,
) {
    debug::log(&format!("Attempting to delete mod ({}).", modid));
    let p = PathBuf::from(format!("{}/{}", dumploc, modid));

    if !p.exists() {
        return;
    }

    let data = mod_info::read(&p.to_str().unwrap().to_string()).unwrap();

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

            if source_path.exists()
                && destination_path.exists()
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
            else if destination_path.exists()
            {
                fs::remove_file(destination_path).expect("Could not delete custom file.");
            }
        }

        debug::log("Removed modded files.");

        let mut p = PathBuf::from("Load/Textures/");
        p.push(&gameid);

        let dolphin_path = dolphin::find_dir(&p);

        for file in texturefiles {
            let mut path = PathBuf::new();

            path.push(&dolphin_path);

            let path_final = helper::remove_first(&file).expect("couldn't remove slash from string");

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
        debug::log("Removed texture files.");
    }
    debug::log("Process ended.");
}

pub fn delete_cache(modid: String) {
    let mut path = helper::get_config_path().expect("could not get config dir");
    path.push(r"cachedMods");
    path.push(modid);
    if path.exists() {
        fs::remove_dir_all(path).expect("Could not remove mod cache");
    }
}

pub fn delete_cache_all() {
    let mut path = helper::get_config_path().expect("could not get config dir");
    path.push("cachedMods");

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir(path).unwrap();
}

pub async fn validate_mod(url: String, local: bool, window: &Window) -> ValidationInfo {
    debug::log("Validating mod");

    let mut path_imgcache = helper::get_config_path().expect("could not get config dir");
    path_imgcache.push("cache");

    fs::create_dir_all(&mut path_imgcache).expect("Failed to create folders.");

    path_imgcache.push("temp.png");

    let mut path = helper::get_config_path().expect("could not get config dir");
    path.push(r"TMP");

    let mut json_path = path.clone();
    json_path.push("mod.json");

    let mut icon_path = path.clone();

    download::zip(url, &path, local, window).await;

    debug::log("Finished Downloading mod for validation");

    let mut validation = ValidationInfo {
        modname: "".to_string(),
        modicon: "".to_string(),
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
            debug::log("Icon file does not exist");
        }
    } else {
        debug::log("Mod.json does not exist");
    }
    
    debug::log("Finished Validating mod");
    validation
}

pub async fn change_status(
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    active: bool,
    window: &Window,
) {
    let active = active;

    if active {
        debug::log(&format!("Mod ({}) Enabled", modid));
        //todo: fix this shit
        add(
            "".to_string(),
            dumploc,
            gameid,
            modid,
            platform,
            window,
        )
        .await;
    } else {
        debug::log(&format!("Mod ({}) Disabled.", modid));
        delete(dumploc, gameid, platform, modid, !active, window).await;
    }

    debug::log("Proccess ended");
}


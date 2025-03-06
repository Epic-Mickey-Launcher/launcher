use crate::{archive, debug, dolphin, download, git, helper, mod_info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use tauri::Emitter;
use tauri::Window;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct ValidationInfo {
    pub modname: String,
    pub modicon: String,
    pub result: String,
    pub validated: bool,
    pub data: eml_validate::ModInfo,
}

pub async fn add(
    url: String,
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    version: String,
    window: &Window,
) -> Result<(), Box<dyn std::error::Error>> {
    debug::log("Begin Adding Mod");
    let mut path = helper::get_config_path()?;
    path.push(r"cachedMods");
    let mut full_path = path.clone();
    full_path.push(&modid);
    let mut version_lock_path = full_path.clone();
    version_lock_path.push("version.lock");
    // download

    let mut mod_json_path_check = full_path.clone();
    mod_json_path_check.push("mod.json");

    let mut cached_outdated = true;
    let mut delete_after = false;

    if version_lock_path.exists() {
        let cached_version = fs::read_to_string(&version_lock_path)?;
        if cached_version == version {
            cached_outdated = false;
        }
    }

    if (!mod_json_path_check.exists() && !url.is_empty()) || cached_outdated {
        let is_local = !url.starts_with("http");

        if !is_local {
            if full_path.exists() {
                fs::remove_dir_all(&full_path)?;
            }

            fs::create_dir_all(&full_path)?;
        }

        let mut path = PathBuf::from(&url);
        if path.is_relative() {
            let config_path = helper::get_config_path()?;
            path = PathBuf::from(config_path);
            path.push(url);
        }

        let path_stringed = path.to_str().unwrap();
        if path.is_dir() {
            full_path = path;
            version_lock_path = full_path.clone();
            version_lock_path.push("version.lock");
            delete_after = true;
            mod_json_path_check = full_path.clone();
            mod_json_path_check.push("mod.json");
        } else {
            download::zip(path_stringed.to_string(), &full_path, is_local, window).await?;
        }

        debug::log("done downloading");
    } else {
        window
        .emit(
            "download-stat",
            download::ModDownloadStats {
                download_total: "".to_string(),
                download_remaining: "".to_string(),
                action: "Installing Cached Copy of".to_string(),
                description: "EML caches previously installed mods for convenience. This shouldn't take long.".to_string()
            },
        )?;
    }

    if version_lock_path.exists() {
        fs::remove_file(&version_lock_path)?;
    }

    let mut version_lock = std::fs::File::create(version_lock_path)?;
    version_lock.write(version.as_bytes())?;

    let json_data: eml_validate::ModInfo = eml_validate::validate(&full_path)?;

    //inject files

    let mut path_textures = full_path.clone();
    let mut path_datafiles = full_path.clone();
    let mut path_scripts = full_path.clone();
    path_scripts.push(&json_data.scripts_path);
    path_textures.push(&json_data.custom_textures_path);
    path_datafiles.push(&json_data.custom_game_files_path);

    let textures_exist = &json_data.custom_textures_path != "";
    let datafiles_exist = &json_data.custom_game_files_path != "";
    let scripts_exist = &json_data.scripts_path != "";

    let mut files_to_restore: Vec<String> = Vec::new();

    //inject DATA files into current dump
    if Path::new(&path_datafiles).exists() && datafiles_exist {
        let mut path_final_location = PathBuf::new();

        let dumploc_clone = dumploc.clone();

        path_final_location.push(&dumploc);

        if platform.to_lowercase() == "wii" {
            path_final_location.push("files");
        } else if platform.to_lowercase() == "pc" && json_data.game.to_lowercase() == "emr" {
            path_final_location.push("recolored/Content");
        }

        //backup files
        let mut path_backup = PathBuf::new();

        path_backup.push(dumploc_clone);

        path_backup.push("backup");

        fs::create_dir_all(&path_backup)?;

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
                let p_str = helper::correct_all_slashes(p.path().to_str().unwrap().to_string());

                //HACK: this can probably be done better right?
                let dont_end_with = format!(r"{}{}", "/", json_data.custom_game_files_path);

                if p_str.ends_with(&dont_end_with) {
                    continue;
                }

                let p_str_shortened = p_str.replace(&path_datafiles_str, "");

                let p_str_final = helper::remove_first(&p_str_shortened).unwrap();

                dirs.push(p_str_final.to_string());
            }
        }

        for directory in &dirs {
            let mut dir = PathBuf::new();
            dir.push(&path_backup);
            dir.push(directory);

            fs::create_dir_all(&dir)?;
        }

        debug::log("Created Folders");

        let mut files: Vec<String> = Vec::new();

        //backup files

        for entry in WalkDir::new(&path_search_clone) {
            let p = entry.unwrap();

            if p.path().is_file() {
                let p_str = helper::correct_all_slashes(p.path().to_str().unwrap().to_string());

                let p_str_shortened = &p_str.replace(&path_datafiles_str, "");

                let p_str_final = helper::remove_first(&p_str_shortened).unwrap();

                files.push(p_str_final.to_string());
            }
        }

        for file in &files {
            let mut source = PathBuf::new();
            source.push(&dumploc);
            if platform.to_lowercase() == "wii" {
                source.push("files");
            }
            source.push(file);

            let mut destination = PathBuf::new();
            destination.push(&path_backup);
            destination.push(file);

            if std::path::Path::new(&source).exists()
                && !std::path::Path::new(&destination).exists()
            {
                fs::copy(&source, &destination)?;
            }
            files_to_restore.push(file.to_string());
        }

        debug::log("Created Files");

        debug::log(&format!(
            "Injecting Game files into: {}",
            &path_final_location.display()
        ));

        helper::inject_files(&path_datafiles, &path_final_location)?;
    }

    let mut scriptfolders: Vec<String> = Vec::new();
    if Path::new(&path_scripts).exists() && scripts_exist {
        debug::log(&format!("Injecting Scripts"));
        let mut path = PathBuf::from(dumploc.clone());
        path.push("recolored/Binaries/Win64/Mods");
        helper::inject_files(&path_scripts, &path)?;

        let path_scripts_str = &path_scripts.clone().into_os_string().into_string().unwrap();

        for entry in WalkDir::new(&path_scripts) {
            let p = entry.unwrap();

            if p.path().is_dir() {
                let p_str = p.path().to_str().expect("Couldn't convert path to string.");
                let p_str_final = &p_str.replace(path_scripts_str, "");
                scriptfolders.push(p_str_final.to_string());
            }
        }
    }

    let mut texturefiles: Vec<String> = Vec::new();

    let mut p = PathBuf::from("Load/Textures/");
    p.push(&gameid);

    let dolphin_path = dolphin::find_dir(&p);

    fs::create_dir_all(&dolphin_path).expect("Failed to create dolphin folder.");

    if Path::new(&path_textures).exists() && textures_exist {
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

        helper::inject_files(&path_textures, &dolphin_path)?;
    }

    mod_info::write(
        format!("{}/{}", dumploc, modid),
        files_to_restore,
        texturefiles,
        scriptfolders,
    )?;

    if delete_after {
        fs::remove_dir_all(full_path)?;
    }

    debug::log("Process ended successfully");

    Ok(())
}

pub async fn delete(
    dumploc: String,
    gameid: String,
    platform: String,
    modid: String,
    active: bool,
    window: &Window,
) -> Result<(), Box<dyn std::error::Error>> {
    debug::log(&format!("Attempting to delete mod ({}).", modid));
    let p = PathBuf::from(format!("{}/{}", dumploc, modid));

    if !p.exists() {
        return Err("Mod does not exist in game.".into());
    }

    let data = mod_info::read(&p.to_str().unwrap().to_string())?;

    fs::remove_file(p)?;

    let files = data.files;
    let texturefiles = data.textures;
    let scriptfolders = data.scripts;

    if active {
        let mut datafiles_path = PathBuf::new();
        datafiles_path.push(&dumploc);
        if platform.to_lowercase() == "wii" {
            datafiles_path.push("files");
        } else if platform.to_lowercase() == "pc" {
            //todo: this fucks em2 pc support replace this
            //immediately
            datafiles_path.push("recolored/Content")
        }

        let mut backup_path = PathBuf::new();
        backup_path.push(&dumploc);
        backup_path.push("backup");

        window.emit(
            "change_description_text_delete",
            format!("Restoring original files..."),
        )?;
        for file in files {
            let mut source_path = PathBuf::new();
            source_path.push(&backup_path);
            source_path.push(&file);

            let mut destination_path = PathBuf::new();
            destination_path.push(&datafiles_path);
            destination_path.push(&file);

            println!("{}", destination_path.display());

            if source_path.exists() && destination_path.exists() {
                fs::copy(source_path, destination_path)?;
            } else if destination_path.exists() {
                fs::remove_file(destination_path)?;
            }
        }

        debug::log("Removed modded files.");

        let mut p = PathBuf::from("Load/Textures/");
        p.push(&gameid);

        let dolphin_path = dolphin::find_dir(&p);

        window.emit(
            "change_description_text_delete",
            format!("Removing Custom Textures..."),
        )?;

        for file in texturefiles {
            let mut path = PathBuf::new();

            path.push(&dolphin_path);

            let path_final =
                helper::remove_first(&file).expect("couldn't remove slash from string");

            path.push(path_final);

            if std::path::Path::new(&path).exists() {
                fs::remove_file(&path)?;
            }
        }

        for folder in scriptfolders {
            let mut path = PathBuf::from(&dumploc);

            path.push("recolored/Binaries/Win64/Mods");

            let path_final =
                helper::remove_first(&folder).expect("couldn't remove slash from string");

            path.push(path_final);

            if std::path::Path::new(&path).exists() {
                fs::remove_dir_all(&path)?;
            }
        }
        debug::log("Removed texture files.");
    }
    debug::log("Process ended.");

    Ok(())
}

pub fn delete_cache(modid: String) -> std::io::Result<()> {
    let mut path = helper::get_config_path()?;
    path.push(r"cachedMods");
    path.push(modid);
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

pub fn delete_cache_all() -> std::io::Result<()> {
    let mut path = helper::get_config_path()?;
    path.push("cachedMods");
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }

    fs::create_dir(path)?;
    Ok(())
}

pub fn clean_temp_install_directory(destination: PathBuf) -> std::io::Result<()> {
    let mut path = helper::get_config_path()?;
    if destination.as_os_str().is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "cleaning directory would cause config to get removed.",
        ));
    }
    path.push(destination);
    fs::remove_dir_all(path)?;
    Ok(())
}

pub async fn validate_mod(
    url: String,
    destination: PathBuf,
    validate_type: String,
    window: &Window,
) -> Result<ValidationInfo, Box<dyn std::error::Error>> {
    debug::log("Validating mod");

    let mut path = PathBuf::new();

    if validate_type == "git" {
        path = helper::get_config_path()?;
        path.push(".tempverify");
        fs::create_dir_all(&path)?;
        git::clone(&url, &path)?;
    } else if validate_type == "extern" {
        if url.starts_with("http") {
            path = helper::get_config_path()?;
            path.push(destination);
            download::zip(url, &path, false, window).await?;
        }
    } else if validate_type == "local" {
        if !path.is_dir() {
            path = helper::get_config_path()?;
            path.push("localmod");

            if (fs::exists(&path))? {
                fs::remove_dir_all(&path)?;
            }

            archive::extract(url, &path)?;
        }
    }

    let mut validation = ValidationInfo {
        modname: "".to_string(),
        validated: true,
        modicon: "".to_string(),
        result: "No Issues.".to_string(),
        data: eml_validate::ModInfo::new(),
    };

    let config = match eml_validate::validate(&path) {
        Ok(config) => config,
        Err(e) => {
            validation.result = e.to_string();
            validation.validated = false;
            eml_validate::ModInfo::new()
        }
    };

    validation.modname = config.name.clone();

    if validation.validated {
        let mut mod_icon_path = path.clone();
        mod_icon_path.push(config.icon_path.clone());
        let image_buffer = std::fs::read(mod_icon_path)?;
        let mut data_url = dataurl::DataUrl::new();
        data_url.set_data(&image_buffer);
        data_url.set_media_type("image/png".to_string().into());
        validation.modicon = data_url.to_string();
        validation.data = config;
    }

    if validate_type == "git" {
        fs::remove_dir_all(path)?;
    }
    debug::log("Finished Validating mod");
    Ok(validation)
}

pub async fn generate_mod_template(
    game: String,
    platform: String,
    path: String,
) -> std::io::Result<()> {
    eml_validate::generate_project(game, platform, path)
}

pub async fn change_status(
    dumploc: String,
    gameid: String,
    modid: String,
    platform: String,
    active: bool,
    version: String,
    window: &Window,
) -> Result<(), Box<dyn std::error::Error>> {
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
            version,
            window,
        )
        .await?;
    } else {
        debug::log(&format!("Mod ({}) Disabled.", modid));
        delete(dumploc, gameid, platform, modid, !active, window).await?;
    }

    debug::log("Proccess ended");
    Ok(())
}

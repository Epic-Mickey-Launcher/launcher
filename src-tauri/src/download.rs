use crate::archive;
use crate::debug;
use crate::helper;
use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tauri::Emitter;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
pub struct ModDownloadStats {
    pub download_remaining: String,
    pub download_total: String,
    pub action: String,
    pub description: String,
}

pub async fn tool(
    url: String,
    foldername: String,
    window: &Window,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    debug::log(&format!("Beginning download of {}", url));
    let mut to_pathbuf = helper::get_config_path()?;
    to_pathbuf.push(foldername);
    fs::create_dir_all(&to_pathbuf)?;
    zip(url, &to_pathbuf, false, window).await?;
    debug::log(&format!("Download Finished"));
    Ok(to_pathbuf)
}

pub async fn zip(
    url: String,
    foldername: &PathBuf,
    local: bool,
    window: &Window,
) -> Result<(), Box<dyn std::error::Error>> {
    debug::log(&format!("Downloading Archive {}", url));
    fs::create_dir_all(&foldername)?;

    let mut temporary_archive_path_buf = foldername.clone();

    temporary_archive_path_buf.push("temp");

    let temporary_archive_path = temporary_archive_path_buf.to_str().unwrap().to_string();

    let mut buffer;

    if !local {
        let res = Client::new().get(&url).send().await?;

        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))
            .unwrap();

        window.emit(
            "download-stat",
            ModDownloadStats {
                download_total: total_size.to_string(),
                download_remaining: "0".to_string(),
                action: "".to_string(),
                description: "".to_string(),
            },
        )?;

        buffer = reqwest::get(&url).await?.bytes_stream();

        let mut download_bytes_count = 0;
        let mut f = File::create(&temporary_archive_path)?;
        let mut next_update_count = 0;

        while let Some(item) = buffer.next().await {
            let mut buf = &Bytes::new();

            let res = item.as_ref();

            buf = match res {
                Ok(b) => b,
                Err(error) => {
                    debug::log(&format!(
                        "Download error occured. Restarting Download: {}",
                        error
                    ));
                    buffer = reqwest::get(&url).await?.bytes_stream();
                    download_bytes_count = 0;
                    fs::remove_file(&temporary_archive_path)?;
                    f = File::create(&temporary_archive_path)?;

                    buf
                }
            };

            if Bytes::is_empty(buf) {
                continue;
            }

            download_bytes_count += buf.len();

            if download_bytes_count > next_update_count {
                window.emit(
                    "download-stat",
                    ModDownloadStats {
                        download_total: total_size.to_string(),
                        download_remaining: download_bytes_count.to_string(),
                        action: "".to_string(),
                        description: "".to_string(),
                    },
                )?;
                next_update_count += (total_size / 256) as usize;
            }

            f.write_all(buf)?;
        }
    } else {
        fs::copy(&url, &temporary_archive_path)?;
    }
    window.emit(
        "download-stat",
        ModDownloadStats {
            download_total: "0".to_string(),
            download_remaining: "0".to_string(),
            action: "Extracting".to_string(),
            description: "This might take a while depending on your drive speed.".to_string(),
        },
    )?;
    archive::extract(temporary_archive_path.clone(), foldername)?;
    window.emit(
        "download-stat",
        ModDownloadStats {
            download_total: "0".to_string(),
            download_remaining: "0".to_string(),
            action: "Cleaning up".to_string(),
            description: "This shouldn't take too long.".to_string(),
        },
    )?;
    fs::remove_file(temporary_archive_path)?;
    debug::log("Finished archive download");
    Ok(())
}

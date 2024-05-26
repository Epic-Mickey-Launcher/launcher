use std::path::PathBuf;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use tauri::Window;
use std::fs;
use futures_util::StreamExt;
use bytes::Bytes;
use crate::debug;
use crate::archive;
use crate::helper;

#[derive(Clone, serde::Serialize)]
struct ModDownloadStats {
    download_remaining: String,
    download_total: String,
}

pub async fn tool(url: String, window: &Window) -> PathBuf {
    debug::log(&format!("Beginning download of {}", url));
    let to_pathbuf = helper::get_config_path().unwrap();
    zip(url, &to_pathbuf, false, window).await;
    debug::log(&format!("Download Finished"));
    to_pathbuf
}

pub async fn zip(url: String, foldername: &PathBuf, local: bool, window: &Window) {
    debug::log(&format!("Downloading Archive {}", url));
    fs::create_dir_all(&foldername).expect("Failed to create");

    let mut temporary_archive_path_buf = foldername.clone();

    temporary_archive_path_buf.push("temp.f");

    let temporary_archive_path = temporary_archive_path_buf.to_str().unwrap().to_string();

    let mut f = File::create(&temporary_archive_path).expect("Failed to create tmpzip");

    let mut buffer;


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
                    download_total: total_size.to_string(),
                    download_remaining: "0".to_string(),
                },
            )
            .unwrap();

        buffer = reqwest::get(&url).await.unwrap().bytes_stream();

        let mut download_bytes_count = 0;

        while let Some(item) = buffer.next().await {
            let mut buf = &Bytes::new();

            let res = item.as_ref();

            buf = match res {
                Ok(b) => b,
                Err(error) => {
                    buffer = reqwest::get(&url).await.unwrap().bytes_stream();
                    download_bytes_count = 0;
                    fs::remove_file(&temporary_archive_path).expect("failed to remove tmpzip");
                    f = File::create(&temporary_archive_path).expect("Failed to create tmpzip");
                    debug::log(&format!("Download error occured. Restarting Download: {}", error));
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
                        download_total: total_size.to_string(),
                        download_remaining: download_bytes_count.to_string(),
                    },
                )
                .unwrap();

            f.write_all(buf).expect("Failed to write to tmpzip");
        }
    } else {
        fs::copy(&url, &temporary_archive_path).expect("Failed to copy local file");
    }

    let output = PathBuf::from(&foldername);

    let extension = archive::extract(temporary_archive_path, &output);

    debug::log("Finished archive download");
}

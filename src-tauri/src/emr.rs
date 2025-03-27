use crate::download;
use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;
use tauri::Window;

pub async fn download_and_inject_ue4ss(
    path: &PathBuf,
    server_link: &String,
    window: &Window,
) -> Result<()> {
    if path.clone().join("ue4ss.lock").exists() {
        return Err(anyhow::anyhow!("UE4ss lock file already exists"));
    }

    let binary_path = path.clone().join("recolored/Binaries/Win64/");
    let url = format!("{}{}", server_link, "tool/download?tool=ue4ss&target=none");
    download::tool(url, binary_path.to_str().unwrap().to_string(), window, true).await?;
    File::create(path.clone().join("ue4ss.lock"))?;
    Ok(())
}

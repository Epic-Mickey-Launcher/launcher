use crate::{debug, helper};
use serde::Serialize;
use std::io::{Read, Seek, Write};

#[derive(Serialize)]
#[allow(non_snake_case)]
struct TunnelInitData {
    TunnelID: String,
    FileSize: u64,
    Chunks: u64,
    ChunkSize: i32,
}

pub async fn upload_chunks(
    input_file: &std::path::PathBuf,
    chunk_size_mb: i32,
    tunnel_id: String,
    server_link: &String,
) -> anyhow::Result<()> {
    debug::log("Tunnel File Upload: Getting File Data");
    let mut file = std::fs::File::open(input_file)?;
    let file_length = file.metadata()?.len();
    let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
    let chunks_count = (file_length as f64 / chunk_size_bytes as f64).ceil() as u64;
    debug::log(&format!(
        "Tunnel File Upload: Required Chunks: {}",
        chunks_count
    ));
    debug::log(&format!(
        "Tunnel File Upload: Chunk Size: {}",
        chunk_size_bytes
    ));
    debug::log(&format!("Tunnel File Upload: Filesize: {}", file_length));

    let tunnel_init_data = TunnelInitData {
        TunnelID: tunnel_id.clone(),
        FileSize: file_length,
        Chunks: chunks_count,
        ChunkSize: chunk_size_bytes,
    };

    debug::log("Tunnel File Upload: Sending Initialize Request to Server");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}tunnel/init", &server_link))
        .json(&tunnel_init_data)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Tunnel File Upload: Failed to init tunnel",));
    }

    debug::log("Tunnel File Upload: Successfully Initialized");
    let config_path = helper::get_config_path()?.join("tunnel_upload_chunks_temp");
    std::fs::create_dir_all(&config_path)?;
    for n in 0..chunks_count {
        let mut chunk_size: u64 = chunk_size_bytes as u64;

        if n == chunks_count - 1 {
            // last chunk
            let stream_position = file.stream_position()?;
            chunk_size = file_length - stream_position; // compensate for file likely not being exactly chunk*chunkcount
        }

        debug::log(&format!("Tunnel File Upload: Slicing File (Chunk #{})", n));
        let mut chunk_buffer = vec![0; chunk_size as usize];
        file.read(&mut chunk_buffer)?;

        let mut chunk_output =
            std::fs::File::create(&config_path.clone().join(format!("chunk{}", n)))?;
        chunk_output.write_all(chunk_buffer.as_slice())?
    }

    debug::log("Tunnel File Upload: File Successfully Sliced");

    for n in 0..chunks_count {
        let path = &config_path.clone().join(format!("chunk{}", n));
        debug::log(&format!("Tunnel File Upload: Uploading Chunk (#{})", n));
        let form = reqwest::multipart::Form::new()
            .text("tunnelid", tunnel_id.clone())
            .file("chunk", path)
            .await?;

        let chunk_response = client
            .post(format!("{}tunnel/chunk", server_link))
            .multipart(form)
            .send()
            .await?;
        std::fs::remove_file(path)?;

        if !chunk_response.status().is_success() {
            return Err(anyhow::anyhow!("Tunnel File Upload: Failed to upload"));
        }
    }
    std::fs::remove_dir_all(&config_path)?;
    Ok(())
}

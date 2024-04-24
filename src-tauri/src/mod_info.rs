use std::fs::File;
use std::io::Write;
use std::io::Read;

pub struct ModFilesInfo {
    pub files: Vec<String>,
    pub textures: Vec<String>,
}

impl ModFilesInfo{
    pub fn empty() -> ModFilesInfo {
        ModFilesInfo{
            files: Vec::new(),
            textures: Vec::new()
        }
    }
}


pub fn write(path: &String, files: Vec<String>, textures: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    if files.len() > 0 {
        file.write(b"[Files]\n")?;
    }

    for file_path in files {
        file.write(file_path.as_bytes())?;
        file.write(b"\n")?;
    }

    if textures.len() > 0 {
        file.write(b"[Textures]\n")?;
    }

    for file_path in textures {
        file.write(file_path.as_bytes())?;
        file.write(b"\n")?;
    }
    Ok(())
}

pub fn read(path: &String) -> std::io::Result<ModFilesInfo> {
    let mut file = File::open(path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

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

    Ok(ModFilesInfo {
        files,
        textures,
    })
}

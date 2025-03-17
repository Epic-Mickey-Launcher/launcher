use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const TEXTURE_MODE: i8 = 0;
const FILE_MODE: i8 = 1;
const SCRIPT_MODE: i8 = 2;

pub struct ModFilesInfo {
    pub files: Vec<String>,
    pub textures: Vec<String>,
    pub scripts: Vec<String>,
}

impl ModFilesInfo {
    pub fn empty() -> ModFilesInfo {
        ModFilesInfo {
            scripts: Vec::new(),
            files: Vec::new(),
            textures: Vec::new(),
        }
    }
}

pub fn write(
    path: String,
    files: Vec<String>,
    textures: Vec<String>,
    scripts: Vec<String>,
) -> Result<()> {
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

    if scripts.len() > 0 {
        file.write(b"[Scripts]\n")?;
    }

    for file_path in scripts {
        file.write(file_path.as_bytes())?;
        file.write(b"\n")?;
    }
    Ok(())
}

pub fn read(path: &String) -> Result<ModFilesInfo> {
    let mut file = File::open(path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    let lines = buffer.split("\n");

    let mut files = Vec::new();
    let mut textures = Vec::new();
    let mut scripts = Vec::new();
    let mut mode = -1;

    for line in lines {
        if line == "[Textures]" {
            mode = TEXTURE_MODE;
            continue;
        } else if line == "[Files]" {
            mode = FILE_MODE;
            continue;
        } else if line == "[Scripts]" {
            mode = SCRIPT_MODE;
            continue;
        } else if line == "" {
            continue;
        }

        if mode == TEXTURE_MODE {
            textures.push(line.to_string());
        } else if mode == FILE_MODE {
            files.push(line.to_string());
        } else if mode == SCRIPT_MODE {
            scripts.push(line.to_string());
        }
    }

    Ok(ModFilesInfo {
        files,
        textures,
        scripts,
    })
}

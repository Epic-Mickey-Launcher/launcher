import {
  BaseDirectory,
  exists,
  writeTextFile,
  readTextFile,
  createDir,
  removeFile
} from "@tauri-apps/api/fs"
import {
  appLocalDataDir
} from '@tauri-apps/api/path';
import { invoke } from "@tauri-apps/api/tauri";

let configPath;

export async function GetPath() {
 configPath = await invoke("get_frontend_config_path", {npath: await appLocalDataDir()})
}

async function DataFolderExists() { 
  GetPath()
  let path = configPath;
  let pathExists = await exists(path);
  if (!pathExists) {
    await createDir(path)
  }
}

export async function WriteToJSON(content, file) {
  DataFolderExists()
  let path = configPath
  await writeTextFile({
    path: path + file,
    contents: content
  })
}

export async function ReadJSON(file) {
  DataFolderExists()
  let path = configPath
  console.log(path)
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function WriteFile(content, file) {
  await writeTextFile({
    path: file,
    contents: content
  })
}

export async function ReadFile(file) {
  let content = await readTextFile(file)
  return content;
}

export async function FileExists(path) {
  return await exists(path)
}

export function GetFullName(name)
{
  switch(name)
  {
    case "EM1":
      return "Epic Mickey 1";

    case "EM2":
      return "Epic Mickey 2";
  }
}

export async function WriteToken(token) {
  await WriteFile(token, configPath + "TOKEN")
}

export async function ReadToken() {
  console.log(configPath)
  let appdir = configPath

  if (await FileExists(appdir + "TOKEN")) {
    let token = await ReadFile(appdir + "TOKEN")
    return token
  } else {
    return await ""
  }
}

export async function DeleteAllConfigFiles()
{
  let appdir = configPath;
  let gamesJsonExists = await exists(appdir + "games.json");
  let confJsonExists = await exists(appdir + "conf.json");

  if (gamesJsonExists) {
    await removeFile(appdir + "games.json");
  }

  if (confJsonExists) {
    await removeFile(appdir + "conf.json");
  }
}

export async function InitConfFiles() {
  let path = configPath
  let gamesJsonExists = await exists(path + "games.json");
  let confJsonExists = await exists(path + "conf.json");

  if (!gamesJsonExists) {
    WriteToJSON("[]", "games.json")
  }

  if (!confJsonExists) {
    WriteToJSON(JSON.stringify({
      dolphinPath: "", WITPath:"", NkitPath:""
    }), "conf.json")
  }
}

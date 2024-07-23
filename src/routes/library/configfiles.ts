import {
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

let configPath: string;

export async function GetPath() {
  configPath = await invoke("get_frontend_config_path", { npath: await appLocalDataDir() })
}

//todo: what kind of fuckin name is this? fix!
async function DataFolderExists() {
  await GetPath()
  let path = configPath;
  let pathExists = await exists(path);
  if (!pathExists) {
    await createDir(path)
  }
}

export async function ReadOneTimeNoticeBlacklist(): Promise<string> {
  await DataFolderExists();
  let appdir = configPath
  return await FileExists(appdir + "otn") ? await readTextFile(appdir + "otn") : ""
}
export async function CheckOneTimeNoticeBlacklist(id: string): Promise<boolean> {
  let buffer = await ReadOneTimeNoticeBlacklist()
  return buffer.includes(id)
}
export async function WriteOneTimeNoticeBlacklist(id: string) {
  await DataFolderExists();
  let appdir = configPath
  let buffer = await ReadOneTimeNoticeBlacklist()
  buffer += id + ","
  await writeTextFile(appdir + "otn", buffer)
}

export async function WriteToJSON(content: string, file: string) {
  await DataFolderExists()
  let path = configPath
  await writeTextFile({
    path: path + file,
    contents: content
  })
}

export async function ReadJSON(file: string): Promise<any> {
  await DataFolderExists()
  let path = configPath
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function WriteFile(content: any, file: string) {
  await writeTextFile({
    path: file,
    contents: content
  })
}

export async function ReadFile(file: string) {
  let content = await readTextFile(file)
  return content;
}

export async function FileExists(path: string) {
  return await exists(path)
}

export function GetFullName(name: string) {
  switch (name) {
    case "EM1":
      return "Epic Mickey 1";

    case "EM2":
      return "Epic Mickey 2";
  }
}

export async function WriteToken(token: string) {
  await WriteFile(token, configPath + "TOKEN")
}

export async function ReadToken(): Promise<string> {
  await DataFolderExists();
  let appdir = configPath

  if (await FileExists(appdir + "TOKEN")) {
    let token = await ReadFile(appdir + "TOKEN")
    return token
  } else {
    return ""
  }
}

export async function DeleteAllConfigFiles() {
  await DataFolderExists()
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
  await DataFolderExists()
  let path = configPath
  let gamesJsonExists = await exists(path + "games.json");
  let confJsonExists = await exists(path + "conf.json");

  if (!gamesJsonExists) {
    WriteToJSON("[]", "games.json")
  }

  if (!confJsonExists) {
    WriteToJSON(JSON.stringify({
      dolphinPath: "", WITPath: "", NkitPath: ""
    }), "conf.json")
  }
}

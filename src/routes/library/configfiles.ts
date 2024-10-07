import {
  exists,
  writeTextFile,
  readTextFile,
  mkdir,
  remove
} from "@tauri-apps/plugin-fs"
import {
  appLocalDataDir
} from '@tauri-apps/api/path';
import { invoke } from "@tauri-apps/api/core";

export let configPath: string;

export async function GetPath(): Promise<string> {
  configPath = await invoke("get_frontend_config_path", { npath: await appLocalDataDir() })
  return configPath
}

//todo: what kind of fuckin name is this? fix!
async function DataFolderExists() {
  await GetPath()
  let path = configPath;
  let pathExists = await exists(path);
  if (!pathExists) {
    await mkdir(path)
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
  await writeTextFile(
    path + file,
    content
  )
}

export async function ReadJSON(file: string): Promise<any> {
  await DataFolderExists()
  let path = configPath
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function WriteFile(content: any, file: string) {
  await writeTextFile(
    file,
    content
  )
}

export async function ReadFile(file: string) {
  let content = await readTextFile(file)
  return content;
}

export async function FileExists(path: string) {
  return await exists(path)
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
  remove(appdir, {
    recursive: true
  })
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

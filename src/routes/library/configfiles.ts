import {
  exists,
  mkdir,
  readFile,
  remove,
  writeFile,
} from "@tauri-apps/plugin-fs";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/core";
import { SaveConfig, SaveGamesConfig } from "./config";
import { RetrieveFileByAlias } from "./filealias";

export let configPath: string;

export async function GetPath(): Promise<string> {
  configPath = await invoke("get_frontend_config_path", {
    npath: await appLocalDataDir(),
  });
  return configPath;
}

//todo: what kind of fuckin name is this? fix!
async function DataFolderExists() {
  await GetPath();
  let path = configPath;
  let pathExists = await exists(path);
  if (!pathExists) {
    await mkdir(path);
  }
}

export async function ReadOneTimeNoticeBlacklist(): Promise<string> {
  await DataFolderExists();
  let path = await RetrieveFileByAlias("eml-one-time-notices", configPath);
  return (await FileExists(path)) ? await ReadFile(path) : "";
}

export async function CheckOneTimeNoticeBlacklist(
  id: string,
): Promise<boolean> {
  let buffer = await ReadOneTimeNoticeBlacklist();
  return buffer.includes(id);
}

export async function WriteOneTimeNoticeBlacklist(id: string) {
  await DataFolderExists();
  let buffer = await ReadOneTimeNoticeBlacklist();
  buffer += id + ",";
  await WriteFile(
    buffer,
    await RetrieveFileByAlias("eml-one-time-notices", configPath),
  );
}

export async function WriteToJSON(content: string, file: string) {
  await DataFolderExists();
  await WriteFile(configPath + file, content);
}

export async function ReadJSON(file: string): Promise<any> {
  await DataFolderExists();
  let content = await ReadFile(configPath + file);
  return JSON.parse(content);
}

export async function WriteFile(content: any, file: string) {
  await writeFile(file, new TextEncoder().encode(content));
}

export async function ReadFile(file: string) {
  //this is dumb
  return new TextDecoder().decode(await readFile(file));
}

export async function ConfigFileExists(path: string) {
  console.log(configPath + path);
  return await exists(configPath + path);
}

export async function FileExists(path: string) {
  return await exists(path);
}

export async function WriteToken(token: string) {
  let appDir = configPath;
  await WriteFile(token, appDir + "TOKEN");
}

export async function ReadToken(): Promise<string> {
  await DataFolderExists();
  let appDir = configPath;

  if (await FileExists(appDir + "TOKEN")) {
    let token = await ReadFile(appDir + "TOKEN");
    return token;
  }
  return "";
}

export async function DeleteAllConfigFiles() {
  await DataFolderExists();
  await remove(configPath, {
    recursive: true,
  });
}

export async function InitConfFiles() {
  await DataFolderExists();

  console.log(configPath);

  let gamesJsonExists = await exists(
    await RetrieveFileByAlias("eml-tracked-games", configPath),
  );
  let confJsonExists = await exists(
    await RetrieveFileByAlias("eml-config", configPath),
  );

  if (!gamesJsonExists) {
    await SaveGamesConfig([]);
  }

  if (!confJsonExists) {
    await SaveConfig({
      dolphinPath: "",
      dolphinConfigPath: "",
      developerMode: false,
    });
  }
}

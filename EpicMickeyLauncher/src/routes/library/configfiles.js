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

async function DataFolderExists() {
  let path = await appLocalDataDir()
  let pathExists = await exists(path);
  if (!pathExists) {
    await createDir(path)
  }
}

export async function WriteToJSON(content, file) {
  DataFolderExists()
  let path = await appLocalDataDir()
  await writeTextFile({
    path: path + file,
    contents: content
  })
}

export async function ReadJSON(file) {
  DataFolderExists()
  let path = await appLocalDataDir()
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function ReadJSONSync(file) {
  DataFolderExists()
  let path = await appLocalDataDir()
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
  await WriteFile(token, await appLocalDataDir() + "TOKEN")
}

export async function ReadToken(token) {
  if (await FileExists(await appLocalDataDir() + "TOKEN")) {
    return await ReadFile(await appLocalDataDir() + "TOKEN")
  } else {
    return await ""
  }
}

export async function DeleteAllConfigFiles()
{

  let appdir = await appLocalDataDir();

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
  let gamesJsonExists = await exists(await appLocalDataDir() + "games.json");

  let confJsonExists = await exists(await appLocalDataDir() + "conf.json");
  if (!gamesJsonExists) {
    WriteToJSON("[]", "games.json")
  }

  if (!confJsonExists) {
    WriteToJSON(JSON.stringify({
      dolphinPath: "", WITPath:"", NkitPath:""
    }), "conf.json")
  }
}

import {BaseDirectory, exists, writeTextFile, readTextFile, createDir} from "@tauri-apps/api/fs"
import { appLocalDataDir }  from '@tauri-apps/api/path';

async function DataFolderExists(){
  let path = await appLocalDataDir()
  let pathExists = await exists(path);
  if(!pathExists)
  {
    await createDir(path)
  }
}

export async function WriteToJSON(content, file)
{
  DataFolderExists()
  let path = await appLocalDataDir()
  await writeTextFile({path: path + file, contents: content})
}

export async function ReadJSON(file)
{
  DataFolderExists()
  let path = await appLocalDataDir()
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function ReadJSONSync(file)
{
  DataFolderExists()
  let path = await appLocalDataDir()
  let content = await readTextFile(path + file)
  return JSON.parse(content);
}

export async function WriteFile(content, file)
{
  await writeTextFile({path: file, contents: content})
}
export async function ReadFile(file)
{
  let content = await readTextFile(file)
  return content;
}
export async function FileExists(path){
  return await exists(path)
}

export async function InitConfFiles()
{
    let gamesJsonExists = await exists(await appLocalDataDir() + "games.json");

    let confJsonExists = await exists(await appLocalDataDir() + "conf.json");
    if(!gamesJsonExists)
    {
       WriteToJSON("[]", "games.json")
    }

    if(!confJsonExists)
    {
       WriteToJSON(JSON.stringify({dolphinPath:""}), "conf.json")
    }
}
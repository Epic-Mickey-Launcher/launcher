import { ConfigFileExists, FileExists, ReadJSON, WriteFile, WriteToJSON } from "./configfiles";
import { ConfigFile, Game, GameConfig, PublisherModData } from "./types";

const MODPUBLISHERCONFNAME = "modpublisherconf.json"
export async function LoadGamesConfig(): Promise<GameConfig[]> {
  let jsonData: GameConfig[] = await ReadJSON("games.json");
  return jsonData;
}

export async function SaveGamesConfig(config: GameConfig[]) {
  let processed = JSON.stringify(config)
  await WriteToJSON(processed, "games.json")
}

export async function LoadConfig(): Promise<ConfigFile> {
  let jsonData: ConfigFile = await ReadJSON("conf.json");
  return jsonData;
}

export async function SaveConfig(config: ConfigFile) {
  let processed = JSON.stringify(config)
  await WriteToJSON(processed, "conf.json")
}

export async function LoadModPublisherConfig(): Promise<PublisherModData[]> {
  let jsonData: PublisherModData[];

  console.log("ass")
  if (!ConfigFileExists(MODPUBLISHERCONFNAME)) {
    console.log("butt")
    jsonData = await ReadJSON(MODPUBLISHERCONFNAME);
  }
  else {
    jsonData = [];
  }
  return jsonData;
}
export async function SaveModPublisherConfig(config: PublisherModData[]) {
  let processed = JSON.stringify(config)
  await WriteToJSON(processed, MODPUBLISHERCONFNAME)
}

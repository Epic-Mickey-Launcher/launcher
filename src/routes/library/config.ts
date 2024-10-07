import { ReadJSON, WriteFile, WriteToJSON } from "./configfiles";
import { ConfigFile, Game, GameConfig } from "./types";

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

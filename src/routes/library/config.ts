import { configPath, ReadFile, WriteFile } from "./configfiles";
import { type ConfigFile, OperatingSystemType } from "./types";
import { RetrieveFileByAlias } from "./filealias";
import { exists } from "@tauri-apps/plugin-fs";
import { GameInstance } from "./instance.svelte";
import { invoke } from "@tauri-apps/api/core";

let loadedInstances: GameInstance[] = [];
export let activeInstance: GameInstance = null;
export let headerInstance: any;
export let currentOperatingSystem: OperatingSystemType;

export async function SetOS() {
  currentOperatingSystem = await invoke("get_os");
}

export function SetHeader(header: any) {
  headerInstance = header;
}

export function SetHeaderVisible(active: boolean) {
  headerInstance.SetVisible(active);
}

export function GetLoadedGameInstances() {
  return loadedInstances;
}

export function SetActiveGameInstance(instance: GameInstance) {
  activeInstance = instance;
}

export function UnsetActiveGameInstance() {
  activeInstance = null;
}

export async function AddTrackedGame(path: string): Promise<GameInstance> {
  let trackedGames = await LoadGamesConfig();
  trackedGames.push(path);
  await SaveGamesConfig(trackedGames);

  let instance = new GameInstance(path);
  await instance.Load();
  loadedInstances.push(instance);

  return instance;
}

export async function RemoveTrackedGame(path: string) {
  let trackedGames = await LoadGamesConfig();
  let index = trackedGames.indexOf(path);
  trackedGames.splice(index, 1);
  await SaveGamesConfig(trackedGames);

  let instance = loadedInstances.find((r) => r.path == path);
  if (instance == null) return;
  index = loadedInstances.indexOf(instance);
  loadedInstances.splice(index, 1);
}

export async function LoadGameInstancesFromTrackingFile() {
  let trackedGames = await LoadGamesConfig();

  for (const trackedGame of trackedGames) {
    let instance = new GameInstance(trackedGame);
    loadedInstances.push(instance);
    await instance.Load();
  }
}

export async function LoadGamesConfig(): Promise<string[]> {
  if (
    !(await exists(await RetrieveFileByAlias("eml-tracked-games", configPath)))
  ) {
    await SaveGamesConfig([]);
    return [];
  }

  return JSON.parse(
    await ReadFile(await RetrieveFileByAlias("eml-tracked-games", configPath)),
  );
}

export async function SaveGamesConfig(config: string[]) {
  let processed = JSON.stringify(config, null, 4);
  await WriteFile(
    processed,
    await RetrieveFileByAlias("eml-tracked-games", configPath),
  );
}

export async function LoadConfig(): Promise<ConfigFile> {
  if (!(await exists(await RetrieveFileByAlias("eml-config", configPath)))) {
    return null;
  }
  return JSON.parse(
    await ReadFile(await RetrieveFileByAlias("eml-config", configPath)),
  );
}

export async function SaveConfig(config: ConfigFile) {
  let processed = JSON.stringify(config, null, 4);
  await WriteFile(
    processed,
    await RetrieveFileByAlias("eml-config", configPath),
  );
}

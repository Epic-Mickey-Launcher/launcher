import { configPath, ReadFile, WriteFile } from "./configfiles";
import { RetrieveFileByAlias } from "./filealias";
import { type GameConfig } from "./types";
import { LoadConfig } from "./config";
import { DownloadDolphin } from "./dolphin";

export async function ConvertModJsonToNew(_path) {
  //OBSOLETE
}

export async function ConvertGamesConfigToTrackedGames() {
  //Check if conversion is necessary
  let gamesConfig = await ReadFile(
    await RetrieveFileByAlias("eml-tracked-games", configPath, false),
  );
  let gamesObject = JSON.parse(gamesConfig);
  let gameConfigs = <GameConfig[]>gamesObject;
  if (gameConfigs.length == 0 || gameConfigs[0].path == null) return; // Conversion not necessary
  let tracked_games = [];
  for (let game of gameConfigs) {
    tracked_games.push(game.path);
  }
  await WriteFile(
    JSON.stringify(tracked_games, null, 4),
    await RetrieveFileByAlias("eml-tracked-games", configPath, true),
  );

  // Check if dolphin is installed on the current config and ask to replace it because it probably wont work.

  let config = await LoadConfig();
  if (config.dolphinPath == "") return;

  let confirmMessage = await confirm(
    "Your dolphin install is likely outdated! Would you like to update to the latest version? EML will probably not work with the currently installed build.",
  );

  if (confirmMessage) {
    await DownloadDolphin();
    await alert("Dolphin installed!");
  }
}

import {configPath, ReadFile, WriteFile} from "./configfiles";
import {RetrieveFileByAlias} from "./filealias";
import {type GameConfig,} from "./types";

export async function ConvertModJsonToNew(_path) {
    //OBSOLETE
}

export async function ConvertGamesConfigToTrackedGames() {
    //Check if conversion is necessary
    let gamesConfig = await ReadFile(await RetrieveFileByAlias("eml-tracked-games", configPath, false))
    let gamesObject = JSON.parse(gamesConfig)
    let gameConfigs = <GameConfig[]>gamesObject
    if (gameConfigs.length == 0 || gameConfigs[0].path == null) return // Conversion not necessary
    let tracked_games = []
    for (let game of gameConfigs) {
        tracked_games.push(game.path)
    }
    await WriteFile(JSON.stringify(tracked_games), await RetrieveFileByAlias("eml-tracked-games", configPath, true))
}
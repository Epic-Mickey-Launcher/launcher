import GamesList from "../data/game.json";
import {
  Game,
  type GameConfig,
  type GameIdentity,
  type GameIdentityRelease,
  type IdentifyGameResult,
  type IdentifyInfo,
  type Mod,
  Platform,
  type PublisherMod,
  Region,
  type UnifiedMod,
} from "./types";
import { FileExists } from "./configfiles";

export async function IdentifyISO(id: string): Promise<IdentifyGameResult> {
  id = id.replace(" ", "");
  return await IdentifyGame({ id: id, path: null });
}

export async function IdentifyGame(
  idInfo: IdentifyInfo,
  quiet: boolean = false,
) {
  let result: IdentifyGameResult = {
    game: Game.None,
    platform: Platform.None,
    region: Region.None,
    identifier: "",
  };

  for await (let game of GamesList) {
    result.game = Game[game.id];
    for await (let release of game.releases) {
      result.platform = Platform[release.platform];
      if (release.regions != null && idInfo.id != null) {
        let region: any = release.regions.find((r: any) => r.id === idInfo.id);

        if (region != null) {
          result.region = Region[region.region];
          // exact query!!
          return result;
        }
      }

      if (release.identifier != null && idInfo.path != null) {
        let identifierExists = await FileExists(
          idInfo.path + "/" + release.identifier,
        );
        if (identifierExists) {
          result.platform = Platform[release.platform];
          result.identifier = release.identifier;
          // exact query!! again!!
          return result;
        }
      }
    }
  }
  if (!quiet) console.log("Game could not be identified.");
  return null;
}

export function GetGameWiiID(game: GameConfig) {
  let gameIdentity = GamesList.find((r: any) => r.id == game.game.toString());
  let gameRelease = gameIdentity.releases.find(
    (r: any) => r.platform.toUpperCase() == game.platform.toString(),
  );
  if (gameRelease.regions == null) {
    console.log(game.game + " is not a Wii game.");
    return "";
  }
  let gameRegion = gameRelease.regions.find(
    (r: any) => Region[r.region.toUpperCase()] == game.region,
  );
  return gameRegion.id;
}

export function GetGameRelease(
  game: Game,
  platform: Platform,
): GameIdentityRelease {
  let gameIdentity = GamesList.find((r: any) => r.id == game.toString());
  let gameRelease = gameIdentity.releases.find(
    (r: any) => r.platform == platform,
  );
  return gameRelease as GameIdentityRelease;
}

export function GetGameIdentity(game: Game): GameIdentity {
  let gameIdentity = GamesList.find(
    (r: GameIdentity) => r.id == game.toString(),
  );
  return gameIdentity as GameIdentity;
}

export function InternetModToUnifiedMod(mod: Mod): UnifiedMod {
  return {
    id: mod.ID,
    name: mod.Name,
    game: mod.Game.toUpperCase() as Game,
    platform: mod.Platform.toUpperCase() as Platform,
    version: mod.Version,
  };
}

export function LocalModToUnifiedMod(mod: PublisherMod): UnifiedMod {
  return {
    id: "",
    name: mod.name,
    game: mod.game.toUpperCase() as Game,
    platform: mod.platform.toUpperCase() as Platform,
    version: 0,
  };
}

import GamesList from "../data/game.json"
import { Game, GameConfig, GameIdentity, Platform, GameIdentityRelease, Region, Mod, UnifiedMod, PublisherMod } from "./types";

export function GetGameWiiID(game: GameConfig) {
  let gameIdentity = GamesList.find((r: any) => r.id == game.game.toString())
  let gameRelease = gameIdentity.releases.find((r: any) => r.platform == game.platform)
  let gameRegion = gameRelease.regions.find((r: any) => Region[r.region] == game.region)
  return gameRegion.id
}

export function GetGameRelease(game: Game, platform: Platform): GameIdentityRelease {
  let gameIdentity: GameIdentity = GamesList.find((r: any) => r.id == game.toString())
  let gameRelease: GameIdentityRelease = gameIdentity.releases.find((r: any) => r.platform == platform)
  return gameRelease
}

export function GetGameIdentity(game: Game): GameIdentity {
  let gameIdentity = GamesList.find((r: GameIdentity) => r.id == game.toString())
  return gameIdentity
}

export function InternetModToUnifiedMod(mod: Mod): UnifiedMod {
  return {
    id: mod.ID,
    name: mod.Name,
    game: mod.Game.toUpperCase() as Game,
    platform: mod.Platform.toUpperCase() as Platform,
    version: mod.Version
  }
}
export function LocalModToUnifiedMod(mod: PublisherMod): UnifiedMod {
  console.log(mod)
  return {
    id: mod.id,
    name: mod.name,
    game: mod.game.toUpperCase() as Game,
    platform: mod.platform.toUpperCase() as Platform,
    version: mod.version
  }
}

import {
  Game,
  type GameConfig,
  type GameIdentity,
  type IdentifyGameResult,
  type InstalledMod,
  ModState,
  OperatingSystemType,
  Platform,
  Region,
  type UnifiedMod
} from "./types";
import { ReadFile, WriteFile } from "./configfiles";
import { invoke } from "@tauri-apps/api/core";
import {
  GetGameIdentity,
  GetGameRelease,
  GetGameWiiID,
  IdentifyGame,
  IdentifyISO,
  LocalModToUnifiedMod
} from "./gameid";
import { exists } from "@tauri-apps/plugin-fs";
import { POST, serverLink } from "./networking";
import { RetrieveFileByAlias } from "./filealias";
import { path } from "@tauri-apps/api";
import { currentOperatingSystem, GetLoadedGameInstances, LoadConfig, RemoveTrackedGame } from "./config";

export function GetAllInstancesWithSameGame(game: Game, platform: Platform) {
  let gameInstances = GetLoadedGameInstances();
  return gameInstances.filter(
    (r) => r.gameConfig.game == game && r.gameConfig.platform == platform
  );
}

export class GameInstance {
  gameConfig: GameConfig;
  gameIdentity: GameIdentity;
  mods: InstalledMod[] = $state();
  uniqueID: string;

  busy: boolean;
  loaded: boolean = false;
  path: string;

  constructor(_path: string) {
    this.path = _path;
  }

  async Load(): Promise<boolean> {
    let gameExists = await exists(this.path);
    if (!gameExists) {
      console.log(`Game at (${this.path}) does not exist.`);
      await RemoveTrackedGame(this.path);
      return;
    }

    let identifyGame: IdentifyGameResult = await IdentifyGame(
      {
        id: null,
        path: this.path
      },
      true
    );

    if (identifyGame == null) {
      // User may have selected the folder outside of DATA which is preventing the files to be found so we try again but with DATA added to the path
      identifyGame = await IdentifyGame(
        {
          id: null,
          path: this.path + "/DATA"
        },
        true
      );

      if (identifyGame == null) {
        alert("Could not find a valid game in this directory.");
        return false;
      } else {
        this.path += "/DATA";
      }
    }

    if (identifyGame.platform == Platform.Wii) {
      let id: string = await invoke("get_bootbin_id", {
        path: this.path + "/" + identifyGame.identifier
      });
      identifyGame = await IdentifyISO(id); // Accounting for multiple regions for wii/gc games
    }

    if (identifyGame.game == Game.EMR) {
      let ue4ssLockExists = await exists(this.path + "/ue4ss.lock");
      if (!ue4ssLockExists) {
        let confirmInstall = await confirm(
          "Epic Mickey Rebrushed on EML requires UE4SS. Press Yes to automatically download & inject it. It will not replace any game files. If you cancel this prompt, mods that require it will not work, and you will be given this same prompt when the game is reinitialized."
        );
        if (confirmInstall) {
          await invoke("inject_ue4ss", {
            path: this.path,
            serverUrl: serverLink
          });
        }
      }
    }

    this.gameConfig = {
      path: this.path,
      game: identifyGame.game,
      region: identifyGame.region,
      platform: identifyGame.platform,
      uniqueID: "",
      steamVersion: this.path.includes("steamapps")
    };

    let uidLockExists = await exists(this.path + "/eml-id.lock");
    if (!uidLockExists)
      await WriteFile(
        this.GetShortName(false).toLowerCase().replaceAll(" ", "_") +
        "_" +
        Date.now().toString(),
        this.path + "/eml-id.lock"
      );
    this.gameConfig.uniqueID = await ReadFile(this.path + "/eml-id.lock");

    this.gameIdentity = GetGameIdentity(identifyGame.game);
    this.mods = await this.ReadModList();

    //Allow EM2 PC Dev Level Load
    if (identifyGame.game == Game.EM2 && identifyGame.platform == Platform.PC) {
      let configFileExists = await exists(this.path + "/ConfigFiles.ini");
      if (configFileExists) {
        let configFileContent = await ReadFile(this.path + "/ConfigFiles.ini");
        configFileContent = configFileContent.replace(
          "ShowDevLevelLoad=false",
          "ShowDevLevelLoad=true"
        );
        await WriteFile(configFileContent, this.path + "/ConfigFiles.ini");
      }
    }

    console.log("Game Instance Loaded:");
    console.log(this.gameConfig);

    this.loaded = true;
    return true;
  }

  GetShortName(fullName: boolean): string {
    return (
      (fullName ? this.gameIdentity.name : this.gameConfig.game) +
      " (" +
      this.gameConfig.platform.toUpperCase() +
      (this.gameConfig.region !== Region.None
        ? ", " + this.gameConfig.region
        : "") +
      ")"
    );
  }

  async Play() {
    let config = await LoadConfig();
    if (this.gameConfig.platform.toUpperCase() == Platform.Wii) {
      if (config.dolphinPath == "") {
        alert("Dolphin is required for this game to work!");
        return;
      }
      let id = GetGameWiiID(this.gameConfig);
      invoke("playgame", {
        dolphin: config.dolphinPath,
        exe: this.gameConfig.path + "/sys/main.dol",
        id: id
      }).then((res) => {
        if (res == 1) {
          alert(
            "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings."
          );
        }
      });
    } else if (this.gameConfig.platform == Platform.PC) {
      if (currentOperatingSystem == OperatingSystemType.Linux) {
        let gameIdentity: GameIdentity = GetGameIdentity(
          this.gameConfig.game
        );
        if (this.gameConfig.steamVersion) {
          let steamID = gameIdentity.steamID;
          await invoke("play_steam_game", { id: steamID });
        }
      } else if (currentOperatingSystem == OperatingSystemType.Windows) {
        let gameRelease = GetGameRelease(
          this.gameConfig.game,
          this.gameConfig.platform
        );
        invoke("playgame", {
          dolphin: await path.join(
            this.gameConfig.path,
            gameRelease.identifier
          ),
          exe: "",
          id: ""
        }).then((res) => {
          if (res == 1) {
            alert("Game failed to open.");
          }
        });
      } else {
        alert("Playing Windows games is not supported on this OS yet.");
      }
    }
  }

  async AddMod(modData: UnifiedMod, url: string = ""): Promise<boolean> {
    if (this.busy) {
      return false;
    }

    let gameID =
      this.gameConfig.platform == Platform.Wii
        ? GetGameWiiID(this.gameConfig)
        : "";

    if (modData == null) {
      let validationInfo: any = await invoke("validate_mod", {
        url: url,
        destination: "",
        mode: "local"
      });

      if (!validationInfo.validated) {
        alert(validationInfo.result);
        return;
      }
      url = "localmod";
      modData = LocalModToUnifiedMod(validationInfo.data);
      modData.id = "local_" + modData.name.toLowerCase().replaceAll(" ", "_");
    }

    let modState = await this.CheckMod(modData);

    if (modState == ModState.Incompatible) {
      console.log("This mod is incompatible with target game.");
      return false;
    }

    if (modState == ModState.UpdateAvailable) {
      let existingMod = this.mods.find((r) => r.modid == modData.id);
      let delete_index = this.mods.indexOf(existingMod);
      await invoke("delete_mod", {
        dumploc: this.gameConfig.path,
        gameid: gameID,
        platform: this.gameConfig.platform,
        modid: modData.id,
        active: existingMod.active
      });

      this.mods.splice(delete_index, 1);
      await invoke("delete_mod_cache", { modid: modData.id });
      await this.WriteModList();
    }

    let local = !url.startsWith("http") && url != "";
    let platform: Platform = modData.platform;
    if (platform == null) {
      platform = Platform.Wii;
    }

    await invoke("download_mod", {
      url: local ? url : serverLink + "mod/download?id=" + modData.id,
      name: modData.name,
      dumploc: this.gameConfig.path,
      modid: modData.id.toString(),
      gameid: gameID,
      platform: platform,
      version: String(modData.version)
    });

    this.mods.push({
      name: modData.name,
      modid: modData.id,
      active: true,
      update: modData.version,
      local: local
    });

    await this.WriteModList();

    if (modData.id.trim() != "" && !local) {
      await POST(
        "mod/download/increment",
        {
          ID: modData.id
        },
        false
      );
    }
    this.busy = false;
  }

  async RemoveMod(id: string) {
    let localMod = this.mods.find((r) => r.modid == id);
    if (localMod == null) return;

    let delete_index = this.mods.indexOf(localMod);
    let gameID =
      this.gameConfig.platform == Platform.PC
        ? ""
        : GetGameWiiID(this.gameConfig);

    if (!localMod.active) {
      this.mods.splice(delete_index, 1);
      await this.WriteModList();
      return;
    }

    await invoke("delete_mod", {
      dumploc: this.gameConfig.path,
      gameid: gameID,
      platform: this.gameConfig.platform,
      modid: localMod.modid,
      active: localMod.active
    });
    this.mods.splice(delete_index, 1);
    await this.WriteModList();
  }

  async CheckMod(mod: UnifiedMod): Promise<ModState> {
    let platform = mod.platform.toUpperCase();
    if (platform == undefined) {
      platform = Platform.Wii;
    }

    if (
      this.gameConfig.platform.toUpperCase() != platform ||
      this.gameConfig.game != mod.game
    ) {
      return ModState.Incompatible;
    }

    if (mod.id == null) {
      return ModState.NotInstalled;
    }

    let localInstalledMod = this.mods.find(
      (r: { modid: any }) => r.modid == mod.id
    );
    if (localInstalledMod != null) {
      if (localInstalledMod.update != mod.version) {
        return ModState.UpdateAvailable;
      } else {
        return ModState.Installed;
      }
    }
    return ModState.NotInstalled;
  }

  async SetModActive(id: string, active: boolean) {
    let mod = this.mods.find((r) => r.modid == id);
    let index = this.mods.indexOf(mod);
    this.mods[index].active = active;
    await invoke("change_mod_status", {
      dumploc: this.gameConfig.path,
      modid: mod.modid,
      gameid: GetGameWiiID(this.gameConfig),
      active: active,
      modname: mod.name,
      platform: this.gameConfig.platform,
      version: mod.local ? "0" : mod.update.toString()
    });
    await this.WriteModList();
  }

  async WriteModList() {
    console.log("Writing Mod List...");
    await WriteFile(
      JSON.stringify(this.mods, null, 4),
      await RetrieveFileByAlias("eml-mods-json", this.gameConfig.path, true)
    );
  }

  async ReadModList() {
    let path = await RetrieveFileByAlias(
      "eml-mods-json",
      this.gameConfig.path,
      true
    );
    let modListExists = await exists(path);
    if (!modListExists) return [];
    let file = await ReadFile(path);
    return JSON.parse(file);
  }
}

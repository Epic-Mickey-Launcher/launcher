<script lang="ts">
  import {
    Game,
    GameConfig,
    IdentifyGameResult,
    IdentifyInfo,
    Platform,
    Region,
  } from "../library/types";
  import GamesList from "../data/game.json";
  import ModInstall from "./ModInstall.svelte";
  import { LoadGamesConfig, SaveGamesConfig } from "../library/config";
  import {
    FileExists,
    ReadFile,
    ReadJSON,
    WriteFile,
  } from "../library/configfiles";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { exists } from "@tauri-apps/plugin-fs";

  async function IdentifyGame(idInfo: IdentifyInfo) {
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
          let region: any = release.regions.find(
            (r: any) => r.id === idInfo.id,
          );
          result.region = Region[region.region];
          // exact query!!
          return result;
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

    console.log("Game could not be identified.");
    return null;
  }

  async function IdentifyISO(id: string): Promise<IdentifyGameResult> {
    id = id.replace(" ", "");
    let res = await IdentifyGame({ id: id, path: null });
    return res;
  }

  export async function AddGame(isImage: boolean) {
    const selectedPath = await open({
      title: "Select folder",
      directory: !isImage,
      multiple: false,
      filters: [
        {
          name: "Wii Images",
          extensions: ["iso", "wbfs", "gcz", "wia", "rvz"],
        },
      ],
    });

    let path = selectedPath.toString();

    if (isImage) {
      let d = await ReadJSON("conf.json");
      await invoke("check_iso", { path: path, dolphin: d.dolphinPath }).then(
        async (res: string) => {
          let result = await IdentifyISO(res);

          if (result.game == Game.None) {
            await alert("This is not an Epic Mickey Image.");
            return;
          }

          let modInstallElement = new ModInstall({
            target: document.body,
          });
          try {
            modInstallElement.action = "Extracting";
            modInstallElement.modIcon =
              result.game == Game.EM1 ? "img/emicon.png" : "img/em2icon.png";
            modInstallElement.description =
              "This might take a really long time.";

            await listen("change_iso_extract_msg", (e: any) => {
              modInstallElement.description = e.payload;
            });

            modInstallElement.modName =
              result.game + " (" + result.platform + ", " + result.region + ")";

            await invoke("extract_iso", {
              isopath: path,
              gamename: res,
              dolphin: d.dolphinPath,
            }).then(async (res: string) => {
              modInstallElement.$destroy();

              switch (res) {
                case "err_toolnoexist":
                  await alert(
                    "dolphin-tool not found! Please re-download Dolphin from the settings tab.",
                  );
                  return;
              }

              path = res;
            });
          } catch (e) {
            await alert(e);
            modInstallElement.$destroy();
          }
        },
      );
    }

    let identifyGame: IdentifyGameResult = await IdentifyGame({
      id: null,
      path: path,
    });

    if (identifyGame == null) {
      // User may have selected the folder outside of DATA which is preventing the files to be found so we try again but with DATA added to the path
      identifyGame = await IdentifyGame({
        id: null,
        path: path + "/DATA",
      });

      if (identifyGame == null) {
        await alert("Could not find a valid game in this directory.");
      } else {
        path += "/DATA";
      }
    }

    if (identifyGame.platform == Platform.Wii) {
      let id: string = await invoke("get_bootbin_id", {
        path: path + "/" + identifyGame.identifier,
      });
      identifyGame = await IdentifyISO(id); // Accounting for multiple regions for wii/gc games
    }

    let gameConfig: GameConfig = {
      path: path,
      region: identifyGame.region,
      platform: identifyGame.platform,
      game: identifyGame.game,
      uniqueID: "",
      steamVersion: false,
    };

    let gameConfigFile: GameConfig[] = await LoadGamesConfig();

    if (
      gameConfigFile.find(
        (r) =>
          r.game == gameConfig.game &&
          r.platform == gameConfig.platform &&
          r.region == gameConfig.region,
      )
    ) {
      await alert("Game of this same type has already been added.");
      return;
    }

    if (path.includes("steamapps")) {
      let conf = await confirm(
        "This looks like a steam game path, did you buy this game through steam? (Saying Yes will make EML boot the game from steam with whatever selected Proton version assigned there.)",
      );
      if (conf) {
        gameConfig.steamVersion = true;
      }
    }

    gameConfigFile.push(gameConfig);

    await SaveGamesConfig(gameConfigFile);

    if (gameConfig.game == Game.EM2 && gameConfig.platform == Platform.PC) {
      let configFileExists = await exists(path + "/ConfigFiles.ini");

      if (configFileExists) {
        let conffilecontent = await ReadFile(path + "/ConfigFiles.ini");
        conffilecontent = conffilecontent.replace(
          "ShowDevLevelLoad=false",
          "ShowDevLevelLoad=true",
        );
        await WriteFile(conffilecontent, path + "/ConfigFiles.ini");
      }
    }

    let modJsonExists = await FileExists(path + "/EMLMods.json");
    if (!modJsonExists) {
      await WriteFile("[]", path + "/EMLMods.json");
    }

    return gameConfig;
  }
</script>

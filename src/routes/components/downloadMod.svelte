<svelte:options accessors={true} />

<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
  import {
    GetImagePath,
    ImageType,
    POST,
    serverLink,
  } from "../library/networking";

  import { GameConfig, InstalledMod, Mod, Platform } from "../library/types";

  import ModInstall from "./ModInstall.svelte";
  import { exists } from "@tauri-apps/api/fs";

  let gamedata: GameConfig;
  let moddata: Mod;

  export let downloadButtonStatus = "";
  export let downloadButtonDisabled = false;
  export let canupdate = false;
  export let updatecb = () => {};
  export let downloading = false;

  export async function Initialize(
    _gamedata: any,
    local: any,
    _moddata: any,
    overrideCheck = false,
  ) {
    moddata = _moddata;
    gamedata = _gamedata;
    if (!overrideCheck) {
      await CheckIfDownloaded();
    }
  }

  async function GetJsonData() {
    let jsonData = await ReadJSON("games.json");
    return jsonData;
  }

  async function CheckIfDownloaded() {
    if (moddata == null) return;
    if (gamedata == null) return;

    let haveGame = false;

    let platform = moddata.Platform.toLowerCase();

    if (platform == undefined) {
      platform = Platform.Wii;
    }

    if (
      gamedata.platform.toLowerCase() == platform &&
      gamedata.game == moddata.Game
    ) {
      haveGame = true;
    }

    if (haveGame) {
      let dataStr = await ReadFile(gamedata.path + "/EMLMods.json");
      let dataJson = JSON.parse(dataStr);
      let json = dataJson.find((r: { modid: any }) => r.modid == moddata.ID);
      downloadButtonStatus = "Download";
      if (json != null) {
        if (json.update != moddata.Version) {
          canupdate = true;
          downloadButtonStatus = "Update Available";
        } else {
          downloadButtonDisabled = true;
          downloadButtonStatus = "Already Installed";
        }
      }
    } else {
      downloadButtonDisabled = true;
      downloadButtonStatus = `${moddata.Game} (${platform}) not installed!`;
    }

    updatecb();
  }

  export async function Download() {
    downloading = true;
    let gameid = gamedata.id;

    let modInstallElement = new ModInstall({
      target: document.body,
    });
    modInstallElement.modIcon = GetImagePath(moddata.ID, ImageType.Mod, false);
    modInstallElement.modName = moddata.Name;
    modInstallElement.showDownloadProgression = true;

    setTimeout(async () => {
      let datastring = await ReadFile(gamedata.path + "/EMLMods.json");
      let data = JSON.parse(datastring);
      let existingmod = data.find((r: { modid: any }) => r.modid == moddata.ID);

      let platform = gamedata.platform;

      if (canupdate) {
        modInstallElement.action = "Updating";
        await invoke("delete_mod", {
          json: JSON.stringify(existingmod),
          dumploc: gamedata.path,
          gameid: gameid,
          platform: platform,
          modid: moddata.ID,
          active: existingmod.active,
        });
        let delete_index = data.indexOf(existingmod);
        data.splice(delete_index, 1);
        await WriteFile(JSON.stringify(data), gamedata.path + "/EMLMods.json");
        await invoke("delete_mod_cache", { modid: moddata.ID });
      }

      if (platform == null) {
        platform = Platform.Wii;
      }
      await invoke("download_mod", {
        url: serverLink + "mod/download?id=" + moddata.ID,
        name: moddata.Name,
        dumploc: gamedata.path,
        modid: moddata.ID.toString(),
        gameid: gameid,
        platform: platform,
        version: String(moddata.Version),
      });
      let json_exists = await exists(gamedata.path + "/EMLMods.json");
      let current_mods: InstalledMod[] = [];
      if (json_exists) {
        current_mods = JSON.parse(
          await ReadFile(gamedata.path + "/EMLMods.json"),
        );
      }

      current_mods.push({
        name: moddata.Name,
        modid: moddata.ID,
        active: true,
        update: moddata.Version,
      });

      await WriteFile(
        JSON.stringify(current_mods),
        gamedata.path + "/EMLMods.json",
      );
      modInstallElement.$destroy();

      if (moddata.ID.trim() != "") {
        await POST(
          "mod/download/increment",
          {
            ID: moddata.ID,
          },
          false,
        );
      }
      CheckIfDownloaded();
      downloading = false;
    }, 15);
  }
</script>

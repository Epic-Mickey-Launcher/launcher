<svelte:options accessors={true} />

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
  import {
    GetImagePath,
    ImageType,
    POST,
    serverLink,
  } from "../library/networking";

  import {
    GameConfig,
    InstalledMod,
    Mod,
    Platform,
    UnifiedMod,
  } from "../library/types";

  import ModInstall from "./ModInstall.svelte";
  import { exists } from "@tauri-apps/plugin-fs";
  import { GetGameWiiID } from "../library/gameid";

  let gamedata: GameConfig;
  let moddata: UnifiedMod;

  export let downloadButtonStatus = "";
  export let downloadButtonDisabled = false;
  export let canupdate = false;
  export let updatecb = () => {};
  export let downloading = false;
  let local = false;
  let installPath = "";
  let imgUrl = "";

  export async function Initialize(
    _gamedata: any,
    _local: boolean,
    _moddata: UnifiedMod,
    _installPath: string = "",
    _imgDataUrl: string = "",
    overrideCheck = false,
  ) {
    local = _local;
    moddata = _moddata;
    installPath = _installPath;
    gamedata = _gamedata;
    imgUrl = _imgDataUrl;

    if (local) {
      moddata.id = moddata.name.replace(" ", "").toLowerCase() + Date.now();
    }

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
    console.log(moddata);
    let haveGame = false;
    let platform = moddata.platform.toUpperCase();
    if (platform == undefined) {
      platform = Platform.Wii;
    }

    if (
      gamedata.platform.toUpperCase() == platform &&
      gamedata.game == moddata.game
    ) {
      haveGame = true;
    }

    if (haveGame) {
      let dataStr = await ReadFile(gamedata.path + "/EMLMods.json");
      let dataJson = JSON.parse(dataStr);

      downloadButtonStatus = "Download";

      if (moddata.id == null) {
        return;
      }

      let json = dataJson.find((r: { modid: any }) => r.modid == moddata.id);
      if (json != null) {
        if (json.update != moddata.version) {
          canupdate = true;
          downloadButtonStatus = "Update Available";
        } else {
          downloadButtonDisabled = true;
          downloadButtonStatus = "Already Installed";
        }
      }
    } else {
      downloadButtonDisabled = true;
      downloadButtonStatus = `${moddata.game} (${platform}) not installed!`;
    }

    updatecb();
  }

  export async function Download() {
    downloading = true;
    let gameID =
      gamedata.platform == Platform.Wii ? GetGameWiiID(gamedata) : "";

    let modInstallElement = new ModInstall({
      target: document.body,
    });
    modInstallElement.modIcon = local
      ? imgUrl
      : GetImagePath(moddata.id, ImageType.Mod, false);
    modInstallElement.modName = moddata.name;
    modInstallElement.showDownloadProgression = true;

    setTimeout(async () => {
      let datastring = await ReadFile(gamedata.path + "/EMLMods.json");
      let data = JSON.parse(datastring);
      let existingmod = data.find((r: { modid: any }) => r.modid == moddata.id);

      let platform = gamedata.platform;

      if (canupdate) {
        modInstallElement.action = "Updating";
        await invoke("delete_mod", {
          json: JSON.stringify(existingmod),
          dumploc: gamedata.path,
          gameid: gameID,
          platform: platform,
          modid: moddata.id,
          active: existingmod.active,
        });
        let delete_index = data.indexOf(existingmod);
        data.splice(delete_index, 1);
        await WriteFile(JSON.stringify(data), gamedata.path + "/EMLMods.json");
        await invoke("delete_mod_cache", { modid: moddata.id });
      }

      if (platform == null) {
        platform = Platform.Wii;
      }

      await invoke("download_mod", {
        url: local ? installPath : serverLink + "mod/download?id=" + moddata.id,
        name: moddata.name,
        dumploc: gamedata.path,
        modid: moddata.id.toString(),
        gameid: gameID,
        platform: platform,
        version: String(moddata.version),
      });
      let json_exists = await exists(gamedata.path + "/EMLMods.json");
      let current_mods: InstalledMod[] = [];
      if (json_exists) {
        current_mods = JSON.parse(
          await ReadFile(gamedata.path + "/EMLMods.json"),
        );
      }

      current_mods.push({
        name: moddata.name,
        modid: moddata.id,
        active: true,
        update: moddata.version,
        local: local,
      });

      await WriteFile(
        JSON.stringify(current_mods),
        gamedata.path + "/EMLMods.json",
      );
      modInstallElement.$destroy();

      if (moddata.id.trim() != "" && !local) {
        await POST(
          "mod/download/increment",
          {
            ID: moddata.id,
          },
          false,
        );
      }
      await CheckIfDownloaded();
      downloading = false;
    }, 15);
  }
</script>

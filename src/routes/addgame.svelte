<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { exists } from "@tauri-apps/api/fs";
  import {
    ReadJSON,
    WriteFile,
    WriteToJSON,
    ReadFile,
  } from "./library/configfiles.js";
  import { invoke } from "@tauri-apps/api/tauri";
  import ModInstall from "./components/ModInstall.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { Game, Platform, Region } from "./library/gameid.js";

  interface IdentifyGameResult {
    game: Game;
    region: Region;
  }

  let addgamedumpDiv: HTMLDivElement;
  let dumpFound: HTMLDivElement;
  let error = "";
  let gametype = Game.None;
  let region = Region.None;
  let id = "";
  let path = "";
  let isobutton: HTMLButtonElement;

  onMount(async () => {
    let d = await ReadJSON("conf.json");
    if (d.dolphinPath == "" || d.dolphinPath == null) {
      isobutton.disabled = true;
    }
  });

  function IdentifyISO(id: string): IdentifyGameResult {
    id = id.replace(" ", "");

    let result: IdentifyGameResult = { game: Game.None, region: Region.None };

    switch (id) {
      //EM1

      case "SEME4Q":
        result.game = Game.EM1;
        result.region = Region.NTSC_U;
        break;

      case "SEMX4Q":
        result.game = Game.EM1;
        result.region = Region.PAL_DE_ES_IT;
        break;

      case "SEMY4Q":
        result.game = Game.EM1;
        result.region = Region.PAL_EN_SE_DK;
        break;

      case "SEMZ4Q":
        result.game = Game.EM1;
        //the BEST version
        result.region = Region.PAL_SE_DK_NO;
        break;

      case "SEMP4Q":
        result.game = Game.EM1;
        result.region = Region.PAL_EN_FR_NL;
        break;

      case "SEMJ01":
        result.game = Game.EM1;
        result.region = Region.NTSC_J;
        break;

      //EM2

      case "SERE4Q":
        result.game = Game.EM2;
        result.region = Region.NTSC_U;
        break;

      case "SERF4Q":
        result.game = Game.EM2;
        result.region = Region.PAL_FR_DE_IT;
        break;

      case "SERJ91":
        result.game = Game.EM2;
        result.region = Region.NTSC_J;
        break;

      case "SERK8M":
        result.game = Game.EM2;
        result.region = Region.NTSC_K;
        break;

      case "SERP4Q":
        result.game = Game.EM2;
        result.region = Region.PAL_EN_FR_ES_NL_PT_TR;
        break;

      case "SERV4Q":
        result.game = Game.EM2;
        result.region = Region.PAL_SE_DK_NO;
        break;
    }

    return result;
  }

  async function AddGameDump(iso: boolean) {
    const selectedPath = await open({
      title: "Select folder",
      directory: !iso,
      multiple: false,
      filters: [
        {
          name: "Wii Images",
          extensions: ["iso", "wbfs", "gcz", "wia", "rvz"],
        },
      ],
    });

    path = selectedPath.toString();

    if (iso) {
      let d = await ReadJSON("conf.json");
      await invoke("check_iso", { path: path, dolphin: d.dolphinPath }).then(
        async (res: string) => {
          let result = IdentifyISO(res);

          if (result.game == Game.None) {
            await alert("This is not an Epic Mickey ISO.");
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
              result.game + " (WII, " + result.region + ")";

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
    let wiiFolderExists = await exists(path + "/DATA");
    let exeExists = await exists(path + "/DEM2.exe");

    dataPath = "";

    if (path.endsWith("/DATA") || path.endsWith("\\DATA")) {
      wiiFolderExists = true;
      dataPath = path;
    } else if (wiiFolderExists) {
      dataPath = path + "/DATA";
    }

    if (wiiFolderExists) {
      let p = dataPath + "/sys/boot.bin";
      id = await invoke("get_bootbin_id", { path: p });
      console.log(id);
      let info = IdentifyISO(id);
      gametype = info.game;
      platform = Platform.Wii;
      region = info.region;
      gamename = info.game.toString() + " (Wii)";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else if (exeExists) {
      gametype = Game.EM2;
      gamename = "Epic Mickey 2 (PC)";
      platform = Platform.PC;
      dataPath = selectedPath;
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else {
      error = "Error: Folder does not contain any version of Epic Mickey!";
    }
  }

  let gamename = "";
  let platform = Platform.None;
  let dataPath: string | string[];
  async function Continue() {
    let jsonData = await ReadJSON("games.json");

    if (
      jsonData.find(
        (r: { game: string; platform: string; region: string }) =>
          r.game == gametype.toString() &&
          r.platform == platform.toString() &&
          r.region == region.toString(),
      ) != null
    ) {
      alert(
        gametype +
          " (" +
          platform +
          ") " +
          " has already been added to your game list. There is no need for two versions of it.",
      );
      window.open("#/", "_self");
      return;
    }

    let newData = {
      path: dataPath,
      game: gametype,
      platform: platform,
      id: id,
      region: region,
    };

    jsonData.push(newData);

    await WriteToJSON(JSON.stringify(jsonData), "games.json");

    let fileExists = await exists(dataPath + "/EMLMods.json");

    if (!fileExists) {
      await WriteFile("[]", dataPath + "/EMLMods.json");
    }

    //theres no reason it shouldnt but better to be safe than sorry
    let conffileexists = await exists(dataPath + "/ConfigFiles.ini");

    //enables level loader inside of em2
    if (conffileexists) {
      let conffilecontent = await ReadFile(dataPath + "/ConfigFiles.ini");
      conffilecontent = conffilecontent.replace(
        "ShowDevLevelLoad=false",
        "ShowDevLevelLoad=true",
      );
      await WriteFile(conffilecontent, dataPath + "/ConfigFiles.ini");
    }

    window.open("#/", "_self");
  }

  function Cancel() {
    window.open("#/", "_self");
  }
</script>

<h1 style="text-align:center">Add Game</h1>
<hr style="width:50%" />

<div id="addgamedump" bind:this={addgamedumpDiv}>
  <p />
  <button on:click={() => AddGameDump(false)} class="addgamebutton"
    >Add Game Dump</button
  >
  <p>
    <button
      bind:this={isobutton}
      on:click={() => AddGameDump(true)}
      class="addgamebutton">Add Game ISO/WBFS</button
    >
  </p>
  <p style="text-align:center">{error}</p>
</div>

<div id="dumpfound" style="display:none" bind:this={dumpFound}>
  <p style="text-align:center">{gamename} Found!</p>
  <button on:click={Continue} class="addgamebutton">Continue</button>
  <p />
  <button on:click={Cancel} class="addgamebutton">Cancel</button>
</div>

<style>
  .addgamebutton {
    margin: 0 auto;
    display: block;
    padding: 10px 50px;
  }
</style>

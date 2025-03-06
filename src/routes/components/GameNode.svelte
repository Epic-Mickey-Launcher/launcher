<svelte:options accessors={true} />

<script lang="ts">
  import { SetData } from "../library/datatransfer";
  import { invoke } from "@tauri-apps/api/core";
  import { ReadFile, ReadJSON } from "../library/configfiles";
  import { onMount } from "svelte";
  import { exists } from "@tauri-apps/plugin-fs";
  import { POST } from "../library/networking";
  import DownloadMod from "./downloadMod.svelte";
  import {
    Game,
    GameConfig,
    GameIdentity,
    Mod,
    Platform,
    Region,
  } from "../library/types";
  import { GetGameIdentity, GetGameWiiID } from "../library/gameid";
  export let imgBackgroundURL = undefined;
  export let imgLogoURL = undefined;
  export let errorMSG = "";
  export let data: GameConfig;
  let updateAvailable = false;
  let outdatedMods = [];
  let reg = "";
  let platformlogo: any;
  async function CheckForUpdate() {
    let mods = [];

    try {
      let jsonExists = await exists(data.path + "/EMLMods.json");
      if (jsonExists) {
        let dataStr = await ReadFile(data.path + "/EMLMods.json");
        let jsonData: any = JSON.parse(dataStr);

        jsonData.forEach(async (r: any) => {
          if (r.modid != "" && !r.local) {
            let latestUpdate = await POST("mod/get", { ID: r.modid });
            if (r.update != latestUpdate.body.Version) {
              updateAvailable = true;
              mods.push(latestUpdate.body);
            }
          }
        });
      }

      outdatedMods = mods;
    } catch {
      console.log("failed to check for updates");
    }
  }

  function Unhover() {
    mainDiv.style.transform = "";
    mainDiv.style.transitionDuration = "1s";
    mainDiv.style.filter = "brightness(100%)";
    node.style.transform = "";
    playButton.style.opacity = "0";
  }

  function Hover(e: { x: number; y: number }) {
    const GAME_NODE_X = 200;
    const GAME_NODE_Y = 300;

    let rect = mainDiv.getBoundingClientRect();

    let centerX = rect.left + GAME_NODE_X / 2;
    let centerY = rect.top + GAME_NODE_Y / 2;

    const MAX_ROTATION = 20;

    let rawX = e.x - centerX;
    let rawY = e.y - centerY;

    let x = (rawX / GAME_NODE_X) * MAX_ROTATION * 2;
    let y = (rawY / GAME_NODE_Y) * MAX_ROTATION * 2;

    console.log();
    let percentage = Math.min(
      Math.max((1 - rawY / GAME_NODE_Y) * 100, 70),
      105,
    );

    mainDiv.style.filter = `brightness(${percentage}%)`;

    playButton.style.opacity = "1";
    mainDiv.style.transitionDuration = "";
    mainDiv.style.transform = `rotateX(${-y}deg) rotateY(${x}deg) perspective(500px)`;
    node.style.transform = `scale(1.1)`;
  }

  async function OpenGame() {
    console.log("poopha");
    let d = await ReadJSON("conf.json");


    if (data.platform.toUpperCase() == Platform.Wii) {
    if (d.dolphinPath == "") {
      alert("Dolphin is required for this game to work!");
      return;
    }
      let id = GetGameWiiID(data);
      console.log(id);
      invoke("playgame", {
        dolphin: d.dolphinPath,
        exe: data.path + "/sys/main.dol",
        id: id,
      }).then((res) => {
        if (res == 1) {
          alert(
            "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings.",
          );
        }
      });
    } else if (data.platform == Platform.PC) {
      invoke("get_os").then(async (_os) => {
        if (_os == "linux") {
          
          let gameIdentity: GameIdentity = GetGameIdentity(data.game);
          if (data.steamVersion) {
            let steamID = gameIdentity.steamID;
            invoke("play_steam_game", { id: steamID });
          }
        } else if (_os == "windows") {
          invoke("playgame", {
            dolphin: data.path + "/DEM2.exe",
            exe: "",
            id: "",
          }).then((res) => {
            if (res == 1) {
              alert("Game failed to open.");
            }
          });
        } else {
          alert("Playing Windows games is not supported on this OS yet.");
        }
      });
    }
  }
  function OpenDirectory() {
    invoke("get_os").then((os) => {
      let p = os == "windows" ? data.path.replace("/", "\\") : data.path;
      invoke("open_path_in_file_manager", { path: p });
    });
  }

  async function UpdateAllMods() {
    let downloadMod = new DownloadMod({ target: mainDiv });

    downloadMod.updatecb = () => {
      updateAvailable = false;
    };

    for await (let r of outdatedMods) {
      await downloadMod.Initialize(data, r.ID == "", r);
      await downloadMod.Download();
    }
  }

  let regionTitle = "";
  let platformTitle = "";

  export async function Init() {
    setTimeout(Hover, 5);
    switch (data.platform) {
      case Platform.Wii:
        platformlogo.src = "img/Wii.svg";
        break;
      case Platform.PC:
        if (data.steamVersion) {
          platformlogo.src = "img/steam.svg";
        } else {
          platformlogo.src = "img/windows.svg";
        }
        break;
    }

    switch (data.region) {
      case Region.NTSC_U:
        regionPath = "img/regions/usa.svg";
        break;

      case Region.PAL_DE_ES_IT:
        regionPath = "img/regions/deites.svg";
        break;

      case Region.PAL_EN_SE_DK:
        regionPath = "img/regions/scandi2.svg";
        break;

      case Region.PAL_SE_DK_NO:
        regionPath = "img/regions/scandi1.svg";
        break;

      case Region.PAL_EN_FR_NL:
        regionPath = "img/regions/frnl.svg";
        break;

      case Region.NTSC_J:
        regionPath = "img/regions/jp.svg";
        break;

      //EM2

      case Region.PAL_FR_DE_IT:
        //todo: change this with actual correct region image
        regionPath = "img/regions/deites.svg";
        break;

      case Region.NTSC_K:
        regionPath = "img/regions/korea.svg";
        break;

      case Region.PAL_EN_FR_ES_NL_PT_TR:
        //every single country here except for turkey is in the eu so i'll just call this the EU version
        regionPath = "img/regions/eu.svg";
        break;
    }

    regionTitle = data.region.toString();
    platformTitle = data.platform.toString();
    if (data.steamVersion) {
      platformTitle = "Steam";
    }
    await CheckForUpdate();
  }

  let regionPath = "";

  let linuxUnsupported = false;
  onMount(async () => {
    let os = await invoke("get_os");
    //band-aid patch for 0.5.1. i don't have time to make a mini-lutris im already way over schedule
    if (os == "linux" && data.game == Game.EMR && !data.steamVersion) {
      linuxUnsupported = true;
      playButton.disabled = true;
      playButton.title =
        "Starting this game from Linux without Steam is not supported yet. Please use solutions like Lutris or Steam.";
    }
  });

  function OpenLevelLoader() {
    SetData("levelloaderdata", data);
    window.open("#/levelloader", "_self");
  }
  export let node: HTMLDivElement;
  let mainDiv: HTMLElement;
  let playButton: HTMLButtonElement;
</script>

<main
  style="display:inline-block;"
  bind:this={mainDiv}
  on:mouseleave={() => Unhover()}
  on:mousemove={Hover}
>
  <div
    class="gamenode"
    bind:this={node}
    style="background-image: url('{imgBackgroundURL}')"
  >
    <div>
      <div style="">
        <img class="gamelogo" src={imgLogoURL} alt="" />
      </div>

      <div
        style="position:absolute;left:0px;pointer-events:none;top:0px;display:flex;justify-content:center;width:100%;height:100%;"
      >
        <button class="playbutton" bind:this={playButton} on:click={OpenGame}>
          <img src="img/play.svg" style="width:30%;" />
        </button>
      </div>

      <div style="">
        <div
          style="display:flex; justify-content:center; width:100%; background-color: rgba(20,20,20,0.5); border-radius:0px 0px 5px 5px; position: absolute; bottom:0px; left:0px;"
        >
          <button
            title="Game Settings"
            on:click={OpenLevelLoader}
            class="gamesettings"
            ><img src="img/settings.svg" style="width:16px;" /></button
          >

          <button
            title="Open Game in Explorer"
            on:click={OpenDirectory}
            class="gamesettings"
            ><img src="img/changelevel.svg" style="width:16px;" /></button
          >
          <button
            title="Open Game in Explorer"
            on:click={OpenDirectory}
            class="gamesettings"
            ><img src="img/openinexplorer.svg" style="width:16px;" /></button
          >
        </div>

        <br />

        <div
          style="position:absolute; bottom:30px;background-color: rgba(0,0,0,0.3);padding:5px;border-radius: 10px;"
        >
          <img
            style="width:20px;padding-top:5px;padding-right:3px;"
            alt="platform"
            bind:this={platformlogo}
            title={platformTitle}
            src="img/Wii.svg"
          />
          <br />
          {#if regionPath != ""}
            <img
              style="height:10px;display:inline;;padding-top:5px;padding-right:2px;"
              src={regionPath}
              title={regionTitle}
              alt=""
            />
          {/if}
        </div>

        {#if updateAvailable}
          <button
            style="background-color: transparent; border:none;"
            on:click={UpdateAllMods}
          >
            <svg
              viewBox="0 0 30 30"
              style="width:25px;fill:lime;padding-top:8px;position:absolute;right:10px;bottom:35px;float:right;"
              ><path
                d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
              /><title>Update all mods.</title><path
                d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z"
              /></svg
            >
          </button>
        {/if}
      </div>

      <plaintext class="error">{errorMSG}</plaintext>
    </div>
  </div>
</main>

<style>
  .nameofbuild {
    pointer-events: none;
    opacity: 0;
    transition-duration: 0.3s;
    bottom: 125px;
    left: 20px;
  }

  .gamenode {
    box-shadow: 2px 2px 10px rgb(0, 0, 0);
    border-radius: 10px;
    width: 200px;
    height: 300px;
    background-size: cover;
    justify-content: center;
    position: relative;
    display: flex;
    margin: auto;
    align-self: center;
    transition: 0.5s;
    opacity: 0;
  }
  .playbutton {
    opacity: 0;
    top: 80px;
    pointer-events: fill;
    position: absolute;
    display: flex;
    justify-content: center;
    border-radius: 100%;
    align-items: center;
    width: 128px;
    height: 128px;
    background-color: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(3px);
    -webkit-backdrop-filter: blur(3px);
    transition-duration: 0.3s;
  }

  .playbutton:hover {
    transform: scale(1.1);
  }

  .playbutton:active {
    background-color: rgba(0, 0, 0, 0.5);
    transform: scale(1);
  }

  .gamenode:hover {
    filter: drop-shadow(10px 10px 7px rgba(0, 0, 0, 0.4));
  }

  .error {
    position: relative;
    left: 520px;
    bottom: 135px;
  }

  .gamesettings {
    border: none;
    background-color: transparent;
    justify-content: center;
    border-right: 1px white;
    text-align: center;
    margin-top: 3px;
  }
  .gamesettings:hover {
    transform: translateY(-3px);
  }
  .gamelogo {
    width: 180px;
    margin-top: 15px;
    filter: drop-shadow(1px 3px 5px rgba(0, 0, 0, 0.877));
    transition-duration: 0.3s;
  }

  .gamelogo:hover {
    transform: scale(1.1);
  }
</style>

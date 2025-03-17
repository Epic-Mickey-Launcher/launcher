<script lang="ts">
  import {invoke} from "@tauri-apps/api/core";
  import {mount, onMount, unmount} from "svelte";
  import {GetImagePath, ImageType, POST} from "../library/networking";
  import {Game, Platform, Region,} from "../library/types";
  import type {GameInstance} from "../library/instance.svelte";
  import {SetActiveGameInstance} from "../library/config";
  import {InternetModToUnifiedMod, LocalModToUnifiedMod} from "../library/gameid";
  import ModInstall from "./ModInstall.svelte";

  let updateAvailable = $state(false);
  let outdatedMods = [];
  let platformLogo: any = $state();
  let {
    imgBackgroundURL="",
    imgLogoURL="",
    errorMSG = "",
    gameInstance,
    node = $bindable(),
  } = $props();

  export {
    node
  }

  let mainDiv: HTMLElement = $state();
  let playButton: HTMLButtonElement = $state();

  async function CheckForUpdate() {
    let mods = [];
    try {
        let instance = gameInstance as GameInstance
        for (const r of instance.mods) {
          if (r.modid != "" && !r.local) {
            let latestUpdate = await POST("mod/get", { ID: r.modid }, true, true);
            if (latestUpdate.error) continue
            if (r.update != latestUpdate.body.Version) {
              updateAvailable = true;
              mods.push(latestUpdate.body);
            }
          }
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
    let instance = gameInstance as GameInstance
    await instance.Play()
  }
  function OpenDirectory() {
    let instance = gameInstance as GameInstance

    invoke("get_os").then((os) => {
      let p = os == "windows" ? gameInstance.path.replace("/", "\\") : instance.gameConfig.path;
      invoke("open_path_in_file_manager", { path: p });
    });
  }

  async function UpdateAllMods() {
    let modInstallElement = mount(ModInstall, {
      target: document.body,
    });
    let instance = gameInstance as GameInstance
    for await (let r of outdatedMods) {
      if (r.local) continue
      await instance.AddMod(InternetModToUnifiedMod(r))
      modInstallElement.modName = r.Name;
      modInstallElement.modIcon = GetImagePath(r.ID, ImageType.Mod);
    }
    await unmount(modInstallElement)
  }

  let regionTitle = $state("");
  let platformTitle = $state("");

  export async function Init() {
    let instance = gameInstance as GameInstance

    switch (instance.gameConfig.platform) {
      case Platform.Wii:
        platformLogo.src = "img/Wii.svg";
        break;
      case Platform.PC:
        if (instance.gameConfig.steamVersion) {
          platformLogo.src = "img/steam.svg";
        } else {
          platformLogo.src = "img/windows.svg";
        }
        break;
    }

    switch (instance.gameConfig.region) {
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

    regionTitle = instance.gameConfig.region.toString();
    platformTitle = instance.gameConfig.platform.toString();
    if (instance.gameConfig.steamVersion) {
      platformTitle = "Steam";
    }
    await CheckForUpdate();
  }

  let regionPath = $state("");

  let linuxUnsupported = false;
  onMount(async () => {
    let instance = gameInstance as GameInstance

    let os = await invoke("get_os");
    //band-aid patch for 0.5.1. i don't have time to make a mini-lutris im already way over schedule
    if (os == "linux" && instance.gameConfig.game == Game.EMR && !instance.gameConfig.steamVersion) {
      linuxUnsupported = true;
      playButton.disabled = true;
      playButton.title =
        "Starting this game from Linux without Steam is not supported yet. Please use solutions like Lutris or Steam.";
    }
    await Init()
  });

  function OpenLevelLoader() {
    let instance = gameInstance as GameInstance
    SetActiveGameInstance(instance)
    window.open("#/levelloader", "_self");
  }

</script>

<main
  style="display:inline-block;"
  bind:this={mainDiv}
  onmouseleave={() => Unhover()}
  onmousemove={Hover}
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
        <button class="playbutton" bind:this={playButton} onclick={OpenGame}>
          <img src="img/play.svg" style="width:30%;" />
        </button>
      </div>

      <div style="">
        <div
          style="display:flex; justify-content:center; width:100%; background-color: rgba(20,20,20,0.5); border-radius:0px 0px 5px 5px; position: absolute; bottom:0px; left:0px;"
        >
          <button
            title="Game Settings"
            onclick={OpenLevelLoader}
            class="gamesettings"
            ><img src="img/settings.svg" style="width:16px;" /></button
          >

          <button
            title="Change Level"
            onclick={OpenDirectory}
            class="gamesettings"
            ><img src="img/changelevel.svg" style="width:16px;" /></button
          >
          <button
            title="Open Game in Explorer"
            onclick={OpenDirectory}
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
            bind:this={platformLogo}
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
            onclick={UpdateAllMods}
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

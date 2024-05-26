<svelte:options accessors={true} />

<script>
  import { SetData, objectbuffer } from "../library/datatransfer.js";
  import { invoke } from "@tauri-apps/api/tauri";
  import { ReadFile, ReadJSON } from "../library/configfiles.js";
  import { onMount } from "svelte";
  import { exists } from "@tauri-apps/api/fs";
  import { POST } from "../library/networking.js";
  import DownloadMod from "./downloadMod.svelte";
  export let game = "";
  export let filepath = "";
  export let platform = "";
  export let region = "";
  export let imgBackgroundURL = undefined;
  export let imgLogoURL = undefined;
  export let errorMSG = "";
  export let data;
  export let mouseEnterCB;
  export let mouseExitCB;
  let updateAvailable = false;
  let outdatedMods = [];

  let reg = "";

  let platformlogo;

  let mouseOver = false;

  async function CheckForUpdate() {
    let mods = [];

    try {
      //wrapping this around to stop EML from freaking out when offline
      let jsonExists = await exists(filepath + "/EMLMods.json");
      if (jsonExists) {
        let dataStr = await ReadFile(filepath + "/EMLMods.json");
        let data = JSON.parse(dataStr);

        data.forEach(async (r) => {
          let latestUpdate = await POST("getmod", { id: r.modid });
          if (r.update != latestUpdate.update) {
            updateAvailable = true;
            mods.push(latestUpdate);
          }
        });
      }

      outdatedMods = mods;
      console.log(outdatedMods);
    } catch {
      console.log("failed to check for updates");
    }
  }

  function Unhover() {
    mainDiv.style.transitionDuration = "0.5s";
    mainDiv.style.transform = "";
  }

  function Hover(e) {
    const GAME_NODE_X = 200;
    const GAME_NODE_Y = 300;

    let rect = mainDiv.getBoundingClientRect();

    let centerX = rect.left + GAME_NODE_X / 2;
    let centerY = rect.top + GAME_NODE_Y / 2;

    const MAX_ROTATION = 15;

    let rawX = e.x - centerX;
    let rawY = e.y - centerY;

    console.log(rawY);

    let x = (rawX / GAME_NODE_X) * MAX_ROTATION * 2;
    let y = (rawY / GAME_NODE_Y) * MAX_ROTATION * 2;

    console.log(y);

    mainDiv.style.transitionDuration = "0.1s";
    mainDiv.style.transform = `rotateX(${-y}deg) rotateY(${x}deg) perspective(500px) scale(1.1)`;
  }

  async function OpenGame() {
    let d = await ReadJSON("conf.json");

    if (d.dolphinPath == "") {
      await alert("Dolphin is required for this game to work!");
      return;
    }

    if (platform == "wii") {
      invoke("playgame", {
        dolphin: d.dolphinPath,
        exe: filepath + "/sys/main.dol",
        id: data.id,
      }).then((res) => {
        if (res == 1) {
          alert(
            "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings.",
          );
        }
      });
    } else {
      invoke("get_os").then((_os) => {
        if (_os == "linux") {
          invoke("start_em2_steam", {});
        } else if (_os == "windows") {
          invoke("playgame", {
            dolphin: filepath + "/DEM2.exe",
            exe: "",
            id: "",
          }).then((res) => {
            if (res == 1) {
              alert("Game failed to open.");
            }
          });
        }
      });
    }
  }

  async function UpdateAllMods() {
    let downloadMod = new DownloadMod({ target: mainDiv });

    downloadMod.updatecb = () => {
      updateAvailable = false;
    };

    for await (let r of outdatedMods) {
      await downloadMod.Initialize(data, false, r);
      await downloadMod.Download();
    }
  }

  export async function Init() {
    setTimeout(Hover, 5);
    switch (platform) {
      case "wii":
        platformlogo.src = "img/Wii.svg";
        break;
      case "pc":
        platformlogo.src = "img/windows.svg";
        break;
    }

    //muterrs
    let result = { game: "", result: "" };

    reg = data.region;
    switch (data.region) {
      case "NTSC-U":
        result.game = "EM1";
        result.region = "NTSC-U";

        regionPath = "img/regions/usa.svg";

        break;

      case "PAL.DE,ES,IT":
        regionPath = "img/regions/deites.svg";
        break;

      case "PAL,EN,SE,DK":
        regionPath = "img/regions/scandi2.svg";
        break;

      case "PAL.SE,DK,NO":
        regionPath = "img/regions/scandi1.svg";
        break;

      case "PAL.EN,FR,NL":
        regionPath = "img/regions/frnl.svg";
        break;

      case "NTSC-J":
        regionPath = "img/regions/jp.svg";
        break;

      //EM2

      case "PAL.FR,DE,IT":
        //todo: change this with actual correct region image
        regionPath = "img/regions/deites.svg";
        break;

      case "NTSC-K":
        regionPath = "img/regions/korea.svg";
        break;

      case "PAL.EN,FR,ES,NL,PT,TR":
        //every single country here except for turkey is in the eu so i'll just call this the EU version

        region = "img/regions/eu.svg";
        break;
    }

    await CheckForUpdate();
  }

  let regionPath = "";
  onMount(async () => {});

  function OpenLevelLoader() {
    SetData("levelloaderdata", data);
    window.open("#/levelloader", "_self");
  }

  let mainDiv;
</script>

<main
  style="transition-duration:0.1s"
  bind:this={mainDiv}
  on:mouseleave={() => Unhover()}
  on:mousemove={Hover}
>
  <div class="gamenode" style="background-image: url('{imgBackgroundURL}')">
    <div>
      <div style="">
        <img
          class="gamelogo"
          on:mouseleave={() => mouseExitCB()}
          on:mouseenter={() => mouseEnterCB(game)}
          src={imgLogoURL}
          alt=""
        />
      </div>

      <div
        style="position:absolute;display:flex;justify-content:center; border-radius:100%;width:128px;height:128px;background-color:rgba(0, 0, 0, 0.2);webkit-backdrop-filter:blur(30px);"
      >
        <img src="img/play.svg" style="width:30%;" />
      </div>

      <span
        style="font-size:10px;position:absolute;text-align:center;bottom:30px;width:100%;left:0;"
        >EpicMickey1</span
      >

      <div style="">
        <div style="display:flex; width:100%;">
          <div style="width:100%;">
            <button on:click={OpenGame} class="gameplaybutton"
              ><img src="img/play.svg" style="width:14px;" /></button
            >
            <button on:click={OpenLevelLoader} class="gamesettings"
              ><img src="img/settings.svg" style="width:16px;" /></button
            >
          </div>
        </div>

        <br />

        <div style="position:absolute; bottom:30px;width:100%;">
          <img
            style="width:20px;padding-top:5px;padding-right:3px;"
            alt="platform"
            bind:this={platformlogo}
            src="img/Wii.svg"
          />
          <br />
          <img
            title={reg}
            style="height:10px;display:inline;;padding-top:5px;padding-right:2px;"
            src={regionPath}
            alt=""
          />
          {#if true}
            <svg
              on:click={UpdateAllMods}
              viewBox="0 0 30 30"
              style="width:25px;fill:lime;padding-top:8px;position:absolute;right:10px;bottom:0px;"
              ><path
                d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
              /><title>Update all mods.</title><path
                d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z"
              /></svg
            >
          {/if}
        </div>
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
    margin-right: auto;
    margin-left: auto;
    width: 200px;
    height: 300px;
    background-size: cover;
    justify-content: center;
    position: relative;
    display: flex;
    transition: 0.3s;
  }

  .gamenode:hover {
    filter: drop-shadow(10px 10px 7px rgba(0, 0, 0, 0.4));
  }

  .error {
    position: relative;
    left: 520px;
    bottom: 135px;
  }
  .gameplaybutton {
    position: absolute;
    display: flex;
    justify-content: center;
    bottom: 0px;
    left: 0px;
    border-radius: 0px 0px 0px 10px;
    transition-duration: 0.2s;
    text-align: center;
    width: 50%;
    border-right: 1px white;
  }

  .gameplaybutton:hover {
  }

  .gamesettings:hover {
  }
  .gamesettings {
    display: flex;
    justify-content: center;
    position: absolute;
    bottom: 0px;
    right: 0px;
    border-right: 1px white;
    text-align: center;
    margin: auto;
    width: 50%;
    border-radius: 0px 0px 10px 0px;
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

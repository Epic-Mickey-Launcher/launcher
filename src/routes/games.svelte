<script lang="ts">
  import GameNode from "./components/GameNode.svelte";
  import { exists, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
  import { appLocalDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import { FileExists, InitConfFiles } from "./library/configfiles.js";
  import { ConvertModJsonToNew } from "./library/legacy";
  import { Game, GameData } from "./library/gameid";
  import Settings from "./settings.svelte";

  let gameNodeDiv: HTMLDivElement;
  let blackoutDiv: HTMLDivElement;
  let bannerDiv: HTMLDivElement;
  let hoveredGame = "EM1";

  onMount(async () => {
    const appLocalDataDirPath = await appLocalDataDir();
    let confExists = await FileExists(appLocalDataDirPath + "conf.json");
    if (!confExists) {
      InitConfFiles();
    }
    let jsonExists = await exists(appLocalDataDirPath + "games.json");
    if (!jsonExists) {
      await writeTextFile({
        path: appLocalDataDirPath + "games.json",
        contents: "[]",
      });
    }
    let t = await readTextFile(appLocalDataDirPath + "games.json");
    let jsonData: GameData[] = JSON.parse(t);
    jsonData.forEach(async (dat: GameData) => {
      await ConvertModJsonToNew(dat.gamePath);
      CreateNode(dat);
    });
  });

  function AddGame() {
    window.open("#/addgame", "_self");
  }
  //todo: remove useless variables game, directory, platform
  function CreateNode(dat: GameData) {
    var element = new GameNode({
      target: gameNodeDiv,
    });

    element.data = dat;
    element.mouseEnterCB = (g: string) => {
      hoveredGame = g;
      blackoutDiv.style.opacity = "0.9";
      bannerDiv.style.opacity = "1";
    };
    element.mouseExitCB = () => {
      blackoutDiv.style.opacity = "0";
      bannerDiv.style.opacity = "0";
    };
    element.Init();

    if (dat.game == Game.EM1.toString()) {
      element.imgLogoURL = "/img/emlogo.png";
      element.imgBackgroundURL = "/img/em1banner.png";
    } else {
      element.imgLogoURL = "/img/em2logo.png";
      element.imgBackgroundURL = "/img/EM2banner.png";
    }
  }
</script>

<div bind:this={blackoutDiv} class="blackout"></div>
<div class="gamebanner" bind:this={bannerDiv}>
  <img
    alt=""
    style="width:65vw;margin:auto;overflow:hidden;"
    src="img/{hoveredGame}bannerfull.png"
  />
</div>

<h1 style="text-align:center;filter:drop-shadow(0 0 4px black)">Games</h1>
<hr style="width:500px" />
<p />
<div style="display:flex;justify-content:center">
  <div bind:this={gameNodeDiv} class="gamegrid" />
</div>
<p style="margin-bottom:50px;" />
<button on:click={AddGame} class="addgamebutton"
  ><span style="position:relative;bottom:8px;">+</span></button
>

<style>
  .addgamebutton {
    margin: 0 auto;
    display: flex;
    justify-content: center;
    text-align: center;
    font-size: 20px;
    border-radius: 10px;
    width: 100px;
    height: 30px;
    border: 1px solid;
    padding: 10px 0px;
    border-color: rgb(138, 138, 138);
    background-color: rgb(82, 82, 82);
    transition-duration: 0.1s;
    margin-bottom: 30px;
  }

  .addgamebutton:hover {
    background-color: rgb(43, 43, 43);
  }

  .gamegrid {
    display: flex;
    justify-items: center;
    gap: 24px;
  }

  .gamebanner {
    overflow: hidden;
    align-items: center;
    position: fixed;
    width: 100vw;
    height: 100vh;
    z-index: -499;
    top: 0;
    mask: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0) 0%,
      rgba(255, 255, 255, 0) 19%,
      rgba(255, 255, 255, 1) 38%,
      rgba(255, 255, 255, 1) 50%,
      rgba(255, 255, 255, 1) 62%,
      rgba(255, 255, 255, 0) 81%,
      rgba(255, 255, 255, 0) 100%
    );
    display: flex;
    justify-content: center;
    overflow: hidden;
    opacity: 0;
    transition: 0.5s;
    filter: brightness(2);
  }

  .blackout {
    overflow: hidden;
    transition: 1s;
    position: fixed;
    width: 100vw;
    height: 100vh;
    background-color: black;
    z-index: -500;
    opacity: 0;
    top: 0;
  }
</style>

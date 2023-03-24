<script>
  import GameNode from "./components/GameNode.svelte";
  import {
    BaseDirectory,
    exists,
    writeTextFile,
    readTextFile,
  } from "@tauri-apps/api/fs";
  import { appLocalDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import { ReadJSON, WriteToJSON, FileExists } from "./library/configfiles.js";

  let gameNodeDiv;

  onMount(async () => {
    const appLocalDataDirPath = await appLocalDataDir();

    let confExists = await FileExists(appLocalDataDirPath + "conf.json")
    if(!confExists) {
       window.open("#/quickstart", "_self")
    }

    let jsonExists = await exists(appLocalDataDirPath + "games.json");

    if (!jsonExists) {
      await writeTextFile({
        path: appLocalDataDirPath + "games.json",
        contents: "[]",
      });
    }

    let t = await readTextFile(appLocalDataDirPath + "games.json");

    let jsonData = JSON.parse(t);

    jsonData.forEach((dat) => {
      CreateNode(dat.game, dat.path);
    });
  });

  function AddGame() {
    window.open("#/addgame", "_self");
  }

  function CreateNode(game, directory) {
    var element = new GameNode({
      target: gameNodeDiv,
    });

    element.filepath = directory;
    element.game = game;

    if (game == "EM1") {
      element.imgLogoURL = "/img/emlogo.png";
      element.imgBackgroundURL = "/img/EM1banner.png";
    } else {
      element.imgLogoURL = "/img/em2logo.png";
      element.imgBackgroundURL = "/img/EM2banner.png";
    }
  }
</script>


<h1 style="text-align:center">Games</h1>
<hr style="width:500px" />
<p />

<button on:click={AddGame} class="addgamebutton">+ Add Game Build</button>

<div bind:this={gameNodeDiv} class="gamegrid" />
<p style="margin-bottom:50px;" />

<style>
  .addgamebutton {
    margin: 0 auto;
    display: block;
    font-size: 15px;
    width: 32%;
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
    display: grid;
    grid-column-start: 1;
    grid-column-end: 1;
    grid-row-gap: 24px;
  }
</style>

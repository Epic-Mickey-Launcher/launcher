<script>
  import { open } from "@tauri-apps/api/dialog";
  import { exists } from "@tauri-apps/api/fs";
  import {
    ReadJSON,
    WriteFile,
    WriteToJSON,
    ReadFile,
    FileExists,
  } from "./library/configfiles.js";
  let addgamedumpDiv;
  let dumpFound;
  let error = "";
  let gametype = "";
  let path;

  async function AddGameDump() {
    const selectedPath = await open({
      title: "Select folder",
      directory: true,
    });

    path = selectedPath;
    let exeExists = await FileExists(path + "/DEM2.exe");
    let wiiFolderExists = await exists(path + "/DATA");
    if (wiiFolderExists) {
      //TODO: find a better identifier for different versions because this Sucks!!!
      let verdat = await FileExists(path + "/DATA/files/VERSIONDATA.TXT");
      if (verdat) {
        //EM1
        gamename = "Epic Mickey 1 (Wii)"
        gametype = "EM1";
      } else {
        //EM2
        gamename = "Epic Mickey 2 (Wii)"
        gametype = "EM2";
      }
      path += "/DATA"
      platform = "wii";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else if (exeExists) {
      gametype = "EM2";
      gamename = "Epic Mickey 2 (PC)"
      platform = "pc";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    }
    else{
      error = "Error: Folder does not contain any version of Epic Mickey!";
    }
  }

  let gamename = "";
  let platform = "";

  async function Continue() {
    let jsonData = await ReadJSON("games.json");

    if (jsonData.find((r) => r.game == gametype && r.platform == platform) != null) {
      alert(
        gametype +
          " has already been added to your game list. There is no need for two versions of it."
      );
      window.open("#/", "_self");
      return;
    }

    let newData = { path: path, game: gametype, platform: platform };

    jsonData.push(newData);

    await WriteToJSON(JSON.stringify(jsonData), "games.json");

    let fileExists = await exists(path + "/EMLMods.json");

    if (!fileExists) {
      await WriteFile("[]", path + "/EMLMods.json");
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
  <button on:click={AddGameDump} class="addgamebutton">Add Game Dump</button>
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

<script>
  import { open } from "@tauri-apps/api/dialog";
  import { exists } from "@tauri-apps/api/fs";
  import { ReadJSON, WriteFile, WriteToJSON } from "./library/configfiles.js";
  let addgamedumpDiv;
  let dumpFound;
  let error = "";

  let path;

  async function AddGameDump() {
    const selectedPath = await open({
      title: "Select folder",
      directory: true,
    });

    path = selectedPath;
    let folderExists = await exists(path + "/DATA");
    if (folderExists) {
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else {
      error = "Error: Folder does not contain any version of Epic Mickey!";
    }
  }

  async function Continue() {
    let jsonData = await ReadJSON("games.json");

    let newData = { path: path + "/DATA", game: "EM1", platform: "wii" }

    jsonData.push(newData);

    await WriteToJSON(JSON.stringify(jsonData), "games.json");
 
    await WriteFile("[]", path + "/DATA/EMLMods.json")

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
  <p style="text-align:center">Epic Mickey (Wii) found!</p>
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

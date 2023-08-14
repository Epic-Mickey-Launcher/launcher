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
  import { invoke } from "@tauri-apps/api/tauri";
  import ModInstall from "./components/ModInstall.svelte";
    import { onMount } from "svelte";
    import { emit, listen } from '@tauri-apps/api/event'
  let addgamedumpDiv;
  let dumpFound;
  let error = "";
  let gametype = "";
  let path;
  let isobutton;

  onMount(async () => {
    let d = await ReadJSON("conf.json");
    if(d.WITPath == "" || d.WITPath == null){
         isobutton.disabled = true;
    }
  })

  async function AddGameDump(iso) {
    const selectedPath = await open({
      title: "Select folder",
      directory: !iso,
      multiple: false,
      filters: [
         {name:"ISO Images", extensions:["iso"]}
        ] 
    });

    path = selectedPath.toString();

    if (iso) {
      let d = await ReadJSON("conf.json");
      await invoke("check_iso", { path: path }).then(async (res) => {
        if (res.id == "SEME4Q" || res.id == "SERE4Q") {
          let modInstallElement = new ModInstall({
            target: document.body,
          });
          try {
            let gamename = res.id == "SEME4Q" ? "Epic Mickey 1" : "Epic Mickey 2";

          modInstallElement.action = "Extracting";
          modInstallElement.modIcon = gamename == "Epic Mickey 1" ? "img/emicon.png" : "img/em2icon.png";
          modInstallElement.description = "This might take a really long time."

          await listen("change_iso_extract_msg", (e) => {
            modInstallElement.description = e.payload;
          });

         

          modInstallElement.modName = gamename;

          //nkit: d.NkitPath, 
          await invoke("extract_iso", { isopath: path, witpath:d.WITPath, nkit:d.NkitPath, gamename:gamename, isNkit: res.nkit}).then(async (res) => {
            console.log(res)
            modInstallElement.$destroy();
              if(res != "err_nkit")
              {
                 console.log("fartlock " + res)
                  path = res;
              }
              else
              {
                //nkit cannot be converted because the toolkit is not installed
                alert("You must install NKit in the Settings tab to extract this iso.");
              }
          })
          console.log(path + " fart lock")
          }
          catch (e) {

            await alert(e)
            modInstallElement.$destroy();
          }
        
        } else {
          error = "Error: This is not an Epic Mickey 1/2 ISO!";
          return;
        }
      });
    }

    console.log(path);
    let exeExists = await FileExists(path + "/DEM2.exe");
    let wiiFolderExists = await exists(path + "/DATA");
    if (wiiFolderExists) {
      //TODO: find a better identifier for different versions because this Sucks!!!
      let verdat = await FileExists(path + "/DATA/files/VERSIONDATA.TXT");
      if (verdat) {
        //EM1
        gamename = "Epic Mickey 1 (Wii)";
        gametype = "EM1";
      } else {
        //EM2
        gamename = "Epic Mickey 2 (Wii)";
        gametype = "EM2";
      }
      path += "/DATA";
      platform = "wii";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else if (exeExists) {
      gametype = "EM2";
      gamename = "Epic Mickey 2 (PC)";
      platform = "pc";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else {
      error = "Error: Folder does not contain any version of Epic Mickey!";
    }
  }

  let gamename = "";
  let platform = "";

  async function Continue() {
    let jsonData = await ReadJSON("games.json");

    if (
      jsonData.find((r) => r.game == gametype && r.platform == platform) != null
    ) {
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

    //theres no reason it shouldnt but better to be safe than sorry
    let conffileexists = await exists(path + "/ConfigFiles.ini");

    //enables level loader inside of em2
    if (conffileexists) {
      let conffilecontent = await ReadFile(path + "/ConfigFiles.ini");
      conffilecontent = conffilecontent.replace(
        "ShowDevLevelLoad=false",
        "ShowDevLevelLoad=true"
      );
      await WriteFile(conffilecontent, path + "/ConfigFiles.ini");
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
    <button bind:this={isobutton} on:click={() => AddGameDump(true)} class="addgamebutton"
      >Add Game ISO</button
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

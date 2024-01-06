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
  import { emit, listen } from "@tauri-apps/api/event";
  let addgamedumpDiv;
  let dumpFound;
  let error = "";
  let gametype = "";
  let region = "";
  let id ="";
  let path;
  let isobutton;

  onMount(async () => {
    let d = await ReadJSON("conf.json");
    if (d.WITPath == "" || d.WITPath == null) {
      isobutton.disabled = true;
    }
  });

  async function IdentifyISO(id)
  {

    let result = {game:"", region:""};

    switch(id)
    {
      //EM1

     case "SEME4Q":
      result.game = "EM1";
      result.region = "NTSC-U";
      break;

      case "SEMX4Q":
      result.game = "EM1";
      result.region = "PAL.DE,ES,IT";
      break;

      case "SEMY4Q":
      result.game = "EM1";
      result.region = "PAL.EN,SE,DK";
      break;

      case "SEMZ4Q":
      result.game = "EM1";
      //the BEST version 
      result.region = "PAL.SE,DK,NO";
      break;

      case "SEMP4Q":
      result.game = "EM1";
      result.region = "PAL.EN,FR,NL";
      break;

      case "SEMJ01":
      result.game = "EM1";
      //the WORST version 
      result.region = "NTSC-J";
      break;

      //EM2

      case "SERE4Q":
      result.game = "EM2";
      result.region = "NTSC-U";
      break;

      case "SERF4Q":
      result.game = "EM2";
      result.region = "PAL.FR,DE,IT";
      break;

      case "SERJ91":
      result.game = "EM2";
      result.region = "NTSC-J";
      break;

      case "SERK8M":
      //didnt even know this version existed
      result.game = "EM2";
      result.region = "NTSC-K";
      break;

     case "SERP4Q":
      result.game = "EM2";
      result.region = "PAL.EN,FR,ES,NL,PT,TR";
      break;

      case "SERV4Q":
      result.game = "EM2";
      result.region = "PAL.SE,DK,NO";
      break;
    
    }

    return result;
  }



  async function AddGameDump(iso) {
    const selectedPath = await open({
      title: "Select folder",
      directory: !iso,
      multiple: false,
      filters: [{ name: "ISO Images", extensions: ["iso"] }],
    });

    console.log(selectedPath);

    path = selectedPath.toString();

    if (iso) {
      let d = await ReadJSON("conf.json");
      await invoke("check_iso", { path: path }).then(async (res) => {

        let result = await IdentifyISO(res.id);
        console.log(result);

        if(result.game == "")
        {
          await alert("This is not an Epic Mickey ISO.");
          return;
        }

        

          let modInstallElement = new ModInstall({
            target: document.body,
          });
          try {
        

            modInstallElement.action = "Extracting";
            modInstallElement.modIcon =
              result.game == "EM1"
                ? "img/emicon.png"
                : "img/em2icon.png";
            modInstallElement.description =
              "This might take a really long time.";

            await listen("change_iso_extract_msg", (e) => {
              modInstallElement.description = e.payload;
            });

            modInstallElement.modName = result.game + " (WII, " + result.region + ")"; 

            //nkit: d.NkitPath,
            await invoke("extract_iso", {
              isopath: path,
              witpath: d.WITPath,
              nkit: d.NkitPath,
              gamename: res.id,
              isNkit: res.nkit,
            }).then(async (res) => {
              console.log(res);
              modInstallElement.$destroy();
              if (res != "err_nkit") {
                console.log("fartlock " + res);
                path = res;
              } else {
                //nkit cannot be converted because the toolkit is not installed
                alert(
                  "You must install NKit in the Settings tab to extract this iso.",
                );
              }
            });
          } catch (e) {
            await alert(e);
            modInstallElement.$destroy();
          }
        
      });
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
      id = await invoke("get_bootbin_id", {path: p});
      console.log(id)
      let info = await IdentifyISO(id);
      gametype = info.game;
      platform = "wii"
      region = info.region;
      gamename = info.game + " (Wii)";
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
     
    } else if (exeExists) {
      gametype = "EM2";
      gamename = "Epic Mickey 2 (PC)";
      platform = "pc";
      dataPath = selectedPath;
      addgamedumpDiv.style.display = "none";
      dumpFound.style.display = "block";
    } else {
      error = "Error: Folder does not contain any version of Epic Mickey!";
    }
  }

  let gamename = "";
  let platform = "";
  let dataPath;
  async function Continue() {
    let jsonData = await ReadJSON("games.json");

    console.log(jsonData);

    if (
      jsonData.find((r) => r.game == gametype && r.platform == platform && r.region == region) != null
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

    let newData = { path: dataPath, game: gametype, platform: platform, id:id, region:region};

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
      class="addgamebutton">Add Game ISO</button
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

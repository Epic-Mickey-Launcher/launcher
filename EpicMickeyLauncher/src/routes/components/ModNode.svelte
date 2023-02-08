<svelte:options accessors={true}/>

<style>
    .modNodeDiv{
        border: 2px solid white;
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
    }
    
    .modNodeImg{
      width: 120px;
      height: 120px;
      bottom: 133px;
      right: 10px;
      float:right;
      border-radius: 10px;
      z-index: -1;
      position: relative;
    }
</style>

<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import { ReadFile, ReadJSON, WriteFile } from '../library/configfiles';
    import ModInstall from './ModInstall.svelte';
    export let modName = "";
    export let description ="";
    export let iconLink = "";
    export let downloadLink = "";
    export let author ="";

    let downloadButton;

    export async function Init() {
        if(description.length > 70)
        {
            let newDesc = description.substring(0, 70)
            newDesc += "..."
            description = newDesc;
        }

        let jsonData = await ReadJSON("games.json");

        let modsData = JSON.parse(await ReadFile(jsonData[0].path + "/EMLMods.json"));

        console.log(modsData)

        if(modsData.find(r => r.name == modName))
        {
             downloadButton.innerHTML = "Already Installed";
             downloadButton.disabled = true;
        }
        else
        {
            downloadButton.disabled = false;
             downloadButton.innerHTML = "Download";
        }
    }

    async function Download() {
        let jsonData = await ReadJSON("games.json");

        let modInstallElement = new ModInstall({
            target: document.body
        })
        modInstallElement.modIcon = iconLink;
        modInstallElement.modName = modName;

        invoke("download_mod", {url: downloadLink, name: modName, dumploc:jsonData[0].path}).then(async (json) => {
           console.log("hey bitch")

           let changedFiles = JSON.parse(json);
               let currentMods = JSON.parse(await ReadFile(jsonData[0].path + "/EMLMods.json"))
               currentMods.push(changedFiles)
               await WriteFile(JSON.stringify(currentMods),  jsonData[0].path + "/EMLMods.json")

               Init()
               modInstallElement.$destroy()
           
        })
    }

</script>

<div class="modNodeDiv">
    <h3>{modName}</h3>
    <h4>Author: {author}</h4>
    <h5>Description: {description}</h5>
    <img class="modNodeImg" alt="" src={iconLink}>
    <button bind:this={downloadButton} on:click={Download}>Download</button> <button>View Page</button>
</div>
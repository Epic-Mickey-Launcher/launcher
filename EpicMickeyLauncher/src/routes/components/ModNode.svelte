<svelte:options accessors={true}/>

<style>
    .modNodeDiv{
        z-index: -1;
        border: 2px solid white;
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
    }
    
    .modNodeImg{
        z-index: -1;
      width: 120px;
      height: 120px;
      bottom: 133px;
      right: 10px;
      float:right;
      border-radius: 10px;
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
    export let gamedata;

    let downloadButton;

    export async function Init() {
        if(description.length > 70)
        {
            let newDesc = description.substring(0, 70)
            newDesc += "..."
            description = newDesc;
        }


        let modsData = JSON.parse(await ReadFile(gamedata.path + "/EMLMods.json"));

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

        let modInstallElement = new ModInstall({
            target: document.body
        })
        modInstallElement.modIcon = iconLink;
        modInstallElement.modName = modName;

       let gameid;
       if(gamedata.game == "EM1")
       {
           gameid = "SEME4Q"
       }
       else{
        gameid = "SERE4Q"
       }

        invoke("download_mod", {url: downloadLink, name: modName, dumploc:gamedata.path, gameid: gameid}).then(async (json) => {

           let changedFiles = JSON.parse(json);
               let currentMods = JSON.parse(await ReadFile(gamedata.path + "/EMLMods.json"))
               currentMods.push(changedFiles)
               await WriteFile(JSON.stringify(currentMods),  gamedata.path + "/EMLMods.json")

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
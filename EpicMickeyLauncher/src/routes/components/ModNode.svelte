<svelte:options accessors={true} />

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
    import { SetData } from "../library/datatransfer";
    import { POST } from "../library/networking";
    import ModInstall from "./ModInstall.svelte";
    export let modName = "";
    export let description = "";
    export let iconLink = "";
    export let downloadLink = "";
    export let author = "";
    export let modid = ""
    let authoraccountexists = true;
    let authorname = "";
    export let gamedata;

    let downloadButton;

    function ViewPage(){
        SetData("modpage_id", modid)
        SetData("modpage_dumploc", gamedata.path)
        window.open("#/modpage", "_self")
    }

    export async function Init() {

        let authorinfo = await POST("getaccount", {id:author})

        if(authorinfo.username == null){
            authoraccountexists = false;
             authorname = "Unknown Account"
        }
        else{
            authorname = authorinfo.username;
        }

        if (description.length > 70) {
            let newDesc = description.substring(0, 70);
            newDesc += "...";
            description = newDesc;
        }

        let modsData = JSON.parse(
            await ReadFile(gamedata.path + "/EMLMods.json")
        );

        if (modsData.find((r) => r.modid == modid)) {
            downloadButton.textContent = "Already Installed";
            downloadButton.disabled = true;
        } else {
            downloadButton.disabled = false;
            downloadButton.textContent = "Download";
        }
    }

    async function OpenProfileOfAuthor(){
        if(!authoraccountexists)return;
        SetData("profile_id", author)
        window.open("#/profilepage", "_self")
    }


    async function Download() {
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = iconLink;
        modInstallElement.modName = modName;

        let gameid;
        if (gamedata.game == "EM1") {
            gameid = "SEME4Q";
        } else {
            gameid = "SERE4Q";
        }

        invoke("download_mod", {
            url: downloadLink,
            name: modName,
            dumploc: gamedata.path,
            modid: modid.toString(),
            gameid: gameid,
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
            let currentMods = JSON.parse(
                await ReadFile(gamedata.path + "/EMLMods.json")
            );
            currentMods.push(changedFiles);
            await WriteFile(
                JSON.stringify(currentMods),
                gamedata.path + "/EMLMods.json"
            );

            Init();
            modInstallElement.$destroy();
        });
    }
</script>

<div class="modNodeDiv">
    <h3>{modName}</h3>
    <h4>Author:<button style="margin-left:5px;" on:click={OpenProfileOfAuthor} class="hyperlinkbutton">{authorname}</button> </h4>
    <h5>Description: {description}</h5>
    <img class="modNodeImg" alt="" src={iconLink} />
    <button bind:this={downloadButton} on:click={Download}>Download</button>
    <button on:click={ViewPage}>View Page</button>
</div>

<style>
    .modNodeDiv {
        z-index: -1;
        border: 2px solid white;
        background-color: rgb(31, 31, 31);
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
    }

    .modNodeImg {
        z-index: 1;
        width: 120px;
        height: 120px;
        bottom: 133px;
        right: 10px;
        float: right;
        border-radius: 10px;
        position: relative;
    }
</style>

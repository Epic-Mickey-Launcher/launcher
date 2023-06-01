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
    export let modid = "";
    export let update = 0;
    export let visible = true;
    let authoraccountexists = true;
    export let authorname = "";
    export let gamedata;
    let downloadStatus = "Download"
    export let json = "";
    let canupdate = false;
    let downloadButton;

    function ViewPage() {
        SetData("modpage_id", modid);
        SetData("modpage_dumploc", gamedata.path);
        window.open("#/modpage", "_self");
    }

    export async function Init() {
        let authorinfo = await POST("getaccount", { id: author });

        if (authorinfo.username == null) {
            authoraccountexists = false;
            authorname = "Unknown Account";
        } else {
            authorname = authorinfo.username;
        }

        if (description.length > 70) {
            let newDesc = description.substring(0, 70);
            newDesc += "...";
            description = newDesc;
        }

        let dataStr = await ReadFile(gamedata.path + "/EMLMods.json");
        let dataJson = JSON.parse(dataStr);
        let json = dataJson.find((r) => r.modid == modid);
        downloadStatus = "Download";
        if (json != null) {
           if(json.update != update){
            canupdate = true;
            downloadStatus = "Update Available";
           }
           else{
            downloadButton.disabled = true;
            downloadStatus = "Already Installed";
           }
        }
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", author);
        window.open("#/profilepage", "_self");
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

        if(canupdate){
            let datastring = await ReadFile(gamedata.path + "/EMLMods.json");
        let data = JSON.parse(datastring);
        let existingmod = data.find(r => r.modid == modid);

            modInstallElement.action = "Updating";
            await invoke("delete_mod", {
            json: JSON.stringify(existingmod),
            dumploc: gamedata.path,
            gameid: gameid,
            platform: gamedata.platform
        })
            let delete_index = data.indexOf(existingmod);
            data.splice(delete_index, 1);
            await WriteFile(JSON.stringify(data), gamedata.path + "/EMLMods.json");
            await invoke("delete_mod_cache", {modid: modid});
        }

        invoke("download_mod", {
            url: downloadLink,
            name: modName,
            dumploc: gamedata.path,
            modid: modid.toString(),
            gameid: gameid,
            platform: gamedata.platform
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
            changedFiles.update = update;
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
{#if visible}
<div class="modNodeDiv">
        <span class="spanHyperLink" on:click={ViewPage} style="font-weight:bold;">{modName}</span>
        <h4>
            Author:<button
                style="margin-left:5px;"
                on:click={OpenProfileOfAuthor}
                class="hyperlinkbutton">{authorname}</button
            >
        </h4>
        <h5>Description: {description}</h5>
        <div class="imgArea">
            <img class="modNodeImg" alt="" src={iconLink} />
            <br>
            <button bind:this={downloadButton} on:click={Download}>{downloadStatus}</button>
            <br>

        </div>

</div>
{/if}
<style>
    .modNodeDiv {
        z-index: -1;
        background-color: rgb(41, 41, 41);
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        height:100px;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
    }
    .imgArea{
        text-align: right;
        bottom: 125px;
        right: 0px;
        float: right;
        position: relative;
    }
    .modNodeImg {
        z-index: 1;
        width: 80px;
        height: 80px;
        border-radius: 10px;
    }
</style>

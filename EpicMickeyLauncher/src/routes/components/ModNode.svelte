<svelte:options accessors={true} />

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
    import { SetData } from "../library/datatransfer";
    import { GetToken, POST, staticAssetsLink } from "../library/networking";
    import ModInstall from "./ModInstall.svelte";
    import { exists } from "@tauri-apps/api/fs";
    export let modName = "";
    export let description = "";
    export let iconLink = "";
    export let downloadLink = "";
    export let author = "";
    export let modid = "";
    export let modplatform = "";
    export let modgame = ""
    export let update = 0;
    export let visible = true;
    let authoraccountexists = true;
    export let authorname = "";
    export let gamedata;
    let downloadStatus = "Download"
    export let json = "";
    let canupdate = false;
    let downloadButton;
    async function SetJsonData() {
        let jsonData = await ReadJSON("games.json");

        return jsonData;
    }
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

        await CheckIfDownloaded();
    }

    async function CheckIfDownloaded() {
        let Gamesjson = await SetJsonData();

        let haveGame = false;

        let platform = modplatform;

        if(modplatform == undefined)
        {
            platform = "wii"
        }

        Gamesjson.forEach((element) => {
            if (element.platform == platform && element.game == gamedata.game) {
                gamedata = element;
                haveGame = true;
            }
        });

        if (haveGame) {
            let dataStr = await ReadFile(gamedata.path + "/EMLMods.json");
            let dataJson = JSON.parse(dataStr);
            let json = dataJson.find((r) => r.modid == modid);
            downloadStatus = "Download";
            if (json != null) {
                if (json.update != update) {
                    canupdate = true;
                    downloadStatus = "Update Available";
                } else {
                    downloadButton.disabled = true;
                    downloadStatus = "Already Installed";
                }
            }  
        } 
        else {
            downloadButton.disabled = true;
            downloadStatus = `${modgame} (${platform}) not installed!`;
        }
    
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", author);
        window.open("#/profilepage", "_self");
    }

    async function Download() {
        let gameid;
        gameid = "SEME4Q";

        if (gamedata.game == "EM2") {
            gameid = "SERE4Q";
        }
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon =  iconLink;
        modInstallElement.modName = modName;
        modInstallElement.showDownloadProgression = true;

        let datastring = await ReadFile(gamedata.path + "/EMLMods.json");
        let data = JSON.parse(datastring);
        let existingmod = data.find((r) => r.modid == modid);

        let platform = gamedata.platform;

        if (update) {
            modInstallElement.action = "Updating";
            await invoke("delete_mod", {
                json: JSON.stringify(existingmod),
                dumploc: gamedata.path,
                gameid: gameid,
                platform: platform,
            });
            let delete_index = data.indexOf(existingmod);
            data.splice(delete_index, 1);
            await WriteFile(JSON.stringify(data), gamedata.path + "/EMLMods.json");
            await invoke("delete_mod_cache", { modid: modid });
        }

        if(platform == null)
        {
            platform = "wii"
        }

        invoke("download_mod", {
            url: downloadLink,
            name: modName,
            dumploc: gamedata.path,
            modid: modid.toString(),
            gameid: gameid,
            platform: platform,
        }).then(async () => {
            let json_exists = await exists(gamedata.path + "/EMLMods.json");
            let current_mods = []
            if (json_exists)
            {
                current_mods = JSON.parse(await ReadFile(gamedata.path + "/EMLMods.json"));
            }


            current_mods.push({
                    name: modName,
                    modid: modid,
                    active: true,
                    update: update,
                })


            
            await WriteFile(
                JSON.stringify(current_mods),
                gamedata.path + "/EMLMods.json"
            );
            modInstallElement.$destroy();
            CheckIfDownloaded();
            let token = await GetToken();
            await POST("addmodimpression", {
                token: token,
                modid: modid,
                impression: { download: true, like: false },
            });
        });
    }
</script>
{#if visible}
<div class="modNodeDiv">

    <div>
        <span class="spanHyperLink" on:click={ViewPage} style="font-weight:bold;">{modName}</span>
        <p>
        <span>
            Author:<button
                style="margin-left:5px;"
                on:click={OpenProfileOfAuthor}
                class="hyperlinkbutton">{authorname}</button
            >
        </span>
        <p>
        <span>{description}</span>
    </div>

        <div class="imgArea">
            <img class="modNodeImg" alt="" src={iconLink} />
            <br>
            <button bind:this={downloadButton} on:click={Download}>{downloadStatus}</button>
            <br>
        </div>
</div>
{/if}
<style>
  .break {
  flex-basis: 100%;
  height: 0;
}

    .modNodeDiv {
        flex-wrap: wrap;
        z-index: -1;
        background-color: rgb(41, 41, 41);
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        height:100px;
        display:flex;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
       transition-duration: 0.1s;
    }

    .modNodeDiv:hover{
        transform: scale(1.01);
    }

    .imgArea{
        display: inline;
        margin-left:auto;
        text-align: right;
        justify-content: right;
    }
    .modNodeImg {
        z-index: 1;
        width: 80px;
        height: 80px;
        border-radius: 10px;
    }
</style>

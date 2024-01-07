<script>
    import { onMount } from "svelte";
    import { GetData, SetData } from "./library/datatransfer";
    import {
        GetId,
        GetToken,
        POST,
        loggedin,
        staticAssetsLink,
    } from "./library/networking";
    import { ReadFile, ReadJSON, WriteFile } from "./library/configfiles";
    import ModInstall from "./components/ModInstall.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import CommentNode from "./components/CommentNode.svelte";
    import { exists } from "@tauri-apps/api/fs";
    import Loading from "./components/loading.svelte";

    let downloads = 0;
    let likes = 0;

    let authoraccountexists = true;
    let modid;
    let authorname = "";
    let dumploc;
    let time = ""
    let gameinfo;
    let modinfo;
    let youtubevideoembed;

    let localid;

    let ownercontrols;

    async function SetJsonData() {
        let jsonData = await ReadJSON("games.json");

        return jsonData;
    }

    async function InstantiateComment(userid, comment, id)
    {
                let commentNode = new CommentNode({
                    target: commentsDiv,
                });

                let commentName = "";

                let userinfo = await POST("getprofileinfo", { id: userid });
                if (userinfo.username == null) {
                    commentName = "Unknown Account";
                } else {
                    commentName = userinfo.username;
                }

                let emblem = userinfo.emblems.sort((a, b) => {
                    return b.weight - a.weight;
                })[0];

                commentNode.InitCommentNode(
                    comment,
                    staticAssetsLink + "img/" + userinfo.pfp,
                    userinfo.username,
                    userinfo.id,
                    id,
                    localid,
                    modid,
                    emblem.color
                );

                commentNode.onDelete = () => {
                    RefreshComments();
                };

                commentNodes.push(commentNode);
    }

    onMount(async () => {
        modid = GetData("modpage_id");
        let id = await GetId();
        localid = id;
        let token = await GetToken();

        modinfo = await POST("getmod", { id: modid, token: token });
        RefreshComments()

        let impressions = await POST("getmodimpressions", {
            token: token,
            mod: modid,
        });
        downloads = impressions.downloads;
        likes = impressions.likes;
        if (impressions.liked) {
            hearticon.style.fill = "red";
        }
        let userinfo = await POST("getaccount", { id: modinfo.author });
        if (userinfo.username == null) {
            authorname = "Unknown Account";
            authoraccountexists = false;
        } else {
            authorname = userinfo.username;
        }

        if (modinfo.youtubevideo != null && modinfo.youtubevideo != "") {
            youtubevideoembed.style.display = "block";
            youtubelink =
                "https://www.youtube.com/embed/" + modinfo.youtubevideo;
        }

        let d = new Date(parseInt(modinfo.id));

        time = d.toLocaleString();


        modPublished = false;

        if (modinfo.published == undefined || modinfo.published === true) {
            modPublished = true;
        }

        if (loggedin) {
            if (modinfo.author == id) {
                ownercontrols.style.display = "block";
            }
        }

        CheckIfDownloaded();
    });

    let commentInput;

    async function PostComment()
    {
        if(commentInput.value.trim().length > 0 && commentInput.value.trim().length < 300) {
            let token = await GetToken();
            let res = await POST("postcomment", { token: token, pageid: modid, comment: commentInput.value.trim() });
            if(res.res == 0)
            {
                RefreshComments()
            }
            commentInput.value = ""
        }
    }

    let commentNodes = []
    let commentsCount = 0

    async function RefreshComments()
    {
        commentNodes.forEach(r => {
            r.$destroy();
        })
        
        commentNodes = [];

        let comments = await POST("getcomments", { pageid: modid });
        if (comments != null) {
            commentsCount = comments.comments.length;
            comments.comments.reverse().forEach(async (r) => {
             InstantiateComment(r.user, r.comment, r.id)
            });
        }
    }

    async function DeleteMod() {
        let confirmed = await confirm("Are you sure?");

        if (confirmed) {
            let id = await GetToken();
            let res = await POST("deletemod", { token: id, id: modid });
            if (res.error === 0) {
                window.open("#/modmarket", "_self");
            }
        }
    }

    function UpdateMod() {
        SetData("modupload_id", modinfo.id);
        window.open("#/uploadmod", "_self");
    }

    async function PublishMod() {
        let token = await GetToken();
        let res = await POST("publishmod", { token: token, id: modid });
        if (res.finished === true) {
            window.open("#/modmarket", "_self");
        }
    }

    async function LikeMod() {
        let token = await GetToken();
        let response = await POST("addmodimpression", {
            token: token,
            modid: modinfo.id,
            impression: { download: false, like: true },
        });
        console.log(response);
        likes += response.likes;
        if (response.likes > 0) {
            hearticon.style.fill = "red";
        } else {
            hearticon.style.fill = "white";
        }
    }
    
    let hearticon;

    async function OnPressDownload()
    {
    if(allRegions.length > 1)
    {
selectRegion = true;
    }
    else{
    Download();
    }
    }

    async function Download(regionoverride = null) {

        if(regionoverride != null)
        {
            selectRegion = false;
            gameinfo = regionoverride;
            dumploc = regionoverride.path;
            id = regionoverride.id;
        }

        let gameid = id;
        
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = staticAssetsLink + modinfo.icon;
        modInstallElement.modName = modinfo.name;
        modInstallElement.showDownloadProgression = true;

        let datastring = await ReadFile(dumploc + "/EMLMods.json");
        let data = JSON.parse(datastring);
        let existingmod = data.find((r) => r.modid == modinfo.id);

        if (update) {
            modInstallElement.action = "Updating";
            await invoke("delete_mod", {
                json: JSON.stringify(existingmod),
                dumploc: dumploc,
                gameid: gameid,
                platform: gameinfo.platform,
            });
            let delete_index = data.indexOf(existingmod);
            data.splice(delete_index, 1);
            await WriteFile(JSON.stringify(data), dumploc + "/EMLMods.json");
            await invoke("delete_mod_cache", { modid: modinfo.id });
        }

        let platform = modinfo.platform;
        if(platform == null)
        {
            platform = "wii"
        }

        invoke("download_mod", {
            url: staticAssetsLink + modinfo.download,
            name: modinfo.name,
            dumploc: dumploc,
            modid: modinfo.id.toString(),
            gameid: gameid,
            platform: platform,
        }).then(async () => {
            let json_exists = await exists(dumploc + "/EMLMods.json");
            let current_mods = []
            if (json_exists)
            {
                current_mods = JSON.parse(await ReadFile(dumploc + "/EMLMods.json"));
            }


            current_mods.push({
                    name: modinfo.name,
                    modid: modinfo.id,
                    active: true,
                    update: modinfo.update,
                })


            
            await WriteFile(
                JSON.stringify(current_mods),
                dumploc + "/EMLMods.json"
            );
            modInstallElement.$destroy();
            let token = await GetToken();
            await POST("addmodimpression", {
                token: token,
                modid: modinfo.id,
                impression: { download: true, like: false },
            });
        });
        CheckIfDownloaded();
    }
    let update = false;
    let downloadButton;
    let id ="";
    let allRegions = []

    async function CheckInstall(path)
    {
        let file_exists = await exists(path + "/EMLMods.json");

if (!file_exists) {
    await WriteFile("[]", path + "/EMLMods.json");
}

let dataStr = await ReadFile(path + "/EMLMods.json");

let dataJson = JSON.parse(dataStr);
let json = dataJson.find((r) => r.modid == modid);
downloadStatus = "Download";
if (json != null) {
    if (json.update != modinfo.update) {
        return "update"
    } else {
        return "installed"
    }
}
return "none"
    }

    async function CheckIfDownloaded() {
        let Gamesjson = await SetJsonData();

        let haveGame = false;

        let platform = modinfo.platform;

        if (modinfo.platform == null) {
         
            platform = "wii";
        }

        console.log("dppf")

           allRegions = Gamesjson.filter(r => r.platform == platform && r.game == modinfo.game);
           console.log(allRegions)

   
            if (allRegions.length == 1) {
                gameinfo = allRegions[0];
                dumploc = allRegions[0].path;
                id = allRegions[0].id;
                haveGame = true;
            }
            else if (allRegions.length > 1)
             {
                haveGame = true;

                for (let i = 0; i < allRegions.length; i++)
                {
                    let res = await CheckInstall(allRegions[i].path);
                    allRegions[i].installed = res;
                }

                let allInstalled = allRegions.filter(r => r.installed == "installed");
                if(allInstalled.length == allRegions.length)
                {
                    downloadButton.disabled = true;
                    downloadStatus = "Already Installed";
                }
             }


        if (haveGame && allRegions.length == 1) {
            let res = await CheckInstall(dumploc);
                if (res == "update") {
                    update = true;
                    downloadStatus = "Update Available";
                } else if (res == "installed"){
                    downloadButton.disabled = true;
                    downloadStatus = "Already Installed";
                }
        
        }
        else if(allRegions.length < 0) {
            downloadButton.disabled = true;
            downloadStatus = `${modinfo.game} (${platform}) not installed!`;
        }
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", modinfo.author);
        window.open("#/profilepage", "_self");
    }

    function CloseRegion()
    {
        selectRegion = false;
    }
    let selectRegion = false;
    let youtubelink;
    let downloadStatus = "Download";
    let commentsDiv;
    let modPublished = true;
</script>

{#if !modinfo == null}

<span style="margin-left:45%;">
    <Loading></Loading>
  </span>

{/if}

{#if modinfo != null}
    <div style="display:flex;width:100%;height:100%;justify-content:center;">
        <img
            src={staticAssetsLink + modinfo.icon}
            alt=""
            style="border-radius:20px;margin-right:20px;width:200px;height:200px;"
        />
        <div>
            <span style="font-size:30px;">{modinfo.name}</span>
            <svg style="width:25px;height:25px;margin-left:10px;fill:white;"
                ><path
                    d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
                /><path
                    d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z"
                /></svg
            > <span style="margin-left: 3px;">{downloads}</span>
            <button
                on:click={LikeMod}
                style="background:transparent;border:none;width:25px;height:25px;margin-left:10px;"
                ><svg
                    bind:this={hearticon}
                    width="25px"
                    height="25px"
                    style="fill:white;"
                    ><path
                        d="M12,23.462l-.866-.612C9.994,22.044,0,14.783,0,8.15A7.036,7.036,0,0,1,6.75.875,6.57,6.57,0,0,1,12,3.582,6.57,6.57,0,0,1,17.25.875,7.036,7.036,0,0,1,24,8.15c0,6.633-9.994,13.894-11.134,14.7ZM6.75,3.875A4.043,4.043,0,0,0,3,8.15c0,3.916,5.863,9.21,9,11.611,3.137-2.4,9-7.695,9-11.611a4.043,4.043,0,0,0-3.75-4.275A4.043,4.043,0,0,0,13.5,8.15h-3A4.043,4.043,0,0,0,6.75,3.875Z"
                    /></svg
                ></button
            ><span style="margin-left: 10px;">{likes}</span>
            <p>
                <button on:click={OpenProfileOfAuthor} class="hyperlinkbutton"
                    >{authorname}</button
                >

                <span>| Published on: {time}</span>
            </p>
            <p>
                <span>{modinfo.description}</span>
            </p>
            <iframe
                style="display:none;"
                title="YouTube Video"
                width="320"
                height="180"
                allow="fullscreen;"
                bind:this={youtubevideoembed}
                src={youtubelink}
            />

            <p>
                <button bind:this={downloadButton} on:click={OnPressDownload}
                    >{downloadStatus}</button
                >
                <button on:click={() => window.open("#/modmarket", "_self")}
                    >Go back to Mod Market</button
                >
            </p>
            <p />
            <div style="display:none;" bind:this={ownercontrols}>
                <button on:click={UpdateMod}>Update Mod</button>
                <button on:click={DeleteMod}>Delete Mod</button>
            </div>
        </div>
    </div>

    {#if !modPublished}
        <span
            style="display:flex;width:100;align-items:center;justify-content:center;"
        >
            <p>
                <span style="color:yellow;"
                    >This mod has not been published yet and is only visible to
                    you.</span
                >
                <button style="margin-left:10px;" on:click={PublishMod}
                    >Publish</button
                >
            </p></span
        >
    {/if}
{/if}

<p style="margin-bottom:50px;" />
<h1>Comments</h1>
<span>{commentsCount} Comments</span>
<hr />
<div style="margin:auto;align-items:center;text-align:center;">
    <div>
        <button on:click={PostComment} style="position:relative;bottom:25px;border:none;padding-left:25px;padding-right:25px;padding-top:10px;padding-bottom:10px;border-radius:6px;">Send</button>
        <textarea placeholder="Comment..."  bind:this={commentInput} style="border:none;font-size:15px;padding:3px;border-radius:5px;height:50px;width:500px;" />
    </div>
    <p />
    <div style="align-items:center;" bind:this={commentsDiv} />
</div>
{#if selectRegion}
<div class="selectregion">
    <span style="position:relative;top:250px;">
        <span>Select region to download to</span>
        <p>
        <div style="background-color: rgb(38 38 38); width:20%; height:30%;  border-radius:10px; margin:auto; filter:drop-shadow(0 0 5px black)">
        {#each allRegions as region}
        {#if region.installed != "installed"}
        <button style="width:100%;" on:click={() => Download(region)}>{region.region}</button>
        {:else}
        <button style="width:100%;" disabled>{region.region}</button>
        {/if}
        <br>
        {/each}
        </div>
        <p>
        <button on:click={() => CloseRegion()} class="hyperlinkbutton">Back</button>
    </span>
</div>
{/if}

<style>
    .selectregion{
        width: 100%;
height: 100%;
top: 0;
bottom: 0;
left: 0;
right: 0;
background-color: rgba(0, 0, 0, 0.3);
border-radius: 5px;
-webkit-backdrop-filter: blur(10px);
position: fixed;
backdrop-filter: blur(10px);
z-index: 500;
align-items: center;
align-self: center;
text-align: center;
    }
</style>
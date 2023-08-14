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

    let downloads = 0;
    let likes = 0;

    let authoraccountexists = true;
    let modid;
    let authorname = "";
    let dumploc;
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

                let userinfo = await POST("getaccount", { id: userid });
                if (userinfo.username == null) {
                    commentName = "Unknown Account";
                } else {
                    commentName = userinfo.username;
                }

                commentNode.InitCommentNode(
                    comment,
                    staticAssetsLink + "img/" + userinfo.pfp,
                    userinfo.username,
                    userinfo.id,
                    id,
                    localid,
                    modid
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

    async function Download() {
        let gameid;
        gameid = "SEME4Q";

        if (modinfo.game == "EM2") {
            gameid = "SERE4Q";
        }
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
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
            changedFiles.update = modinfo.update;
            let currentMods = JSON.parse(
                await ReadFile(dumploc + "/EMLMods.json")
            );
            currentMods.push(changedFiles);
            await WriteFile(
                JSON.stringify(currentMods),
                dumploc + "/EMLMods.json"
            );
            modInstallElement.$destroy();
            CheckIfDownloaded();
            let token = await GetToken();
            await POST("addmodimpression", {
                token: token,
                modid: modinfo.id,
                impression: { download: true, like: false },
            });
        });
    }
    let update = false;
    let downloadButton;
    async function CheckIfDownloaded() {
        let Gamesjson = await SetJsonData();

        let haveGame = false;

        let platform = modinfo.platform;

        if (modinfo.platform == null) {
            platform = "wii";
        }

        Gamesjson.forEach((element) => {
            if (element.platform == platform) {
                gameinfo = element;
                dumploc = element.path;
                haveGame = true;
            }
        });

        if (haveGame) {
            let dataStr = await ReadFile(dumploc + "/EMLMods.json");
            let dataJson = JSON.parse(dataStr);
            let json = dataJson.find((r) => r.modid == modid);
            downloadStatus = "Download";
            if (json != null) {
                if (json.update != modinfo.update) {
                    update = true;
                    downloadStatus = "Update Available";
                } else {
                    downloadButton.disabled = true;
                    downloadStatus = "Already Installed";
                }
            }
        } else {
            downloadButton.disabled = true;
            downloadStatus = `${modinfo.game} (${platform}) not installed!`;
        }
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", modinfo.author);
        window.open("#/profilepage", "_self");
    }

    let youtubelink;
    let downloadStatus = "Download";
    let commentsDiv;
    let modPublished = true;
</script>

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
                <button bind:this={downloadButton} on:click={Download}
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
    <span style="width:50%;height:25px;">
        <input placeholder="Comment..." bind:this={commentInput} type="text" style="border:none;font-size:20px;padding:3px;border-radius:5px;" />
        <p>
        <button on:click={PostComment} style="border:none;padding-left:50px;padding-right:50px;padding-top:10px;padding-bottom:10px;border-radius:6px;">Send</button>
    </span>
    <p />
    <div style="align-items:center;" bind:this={commentsDiv} />
</div>

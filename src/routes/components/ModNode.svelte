<svelte:options accessors={true} />

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
    import { GetData, SetData } from "../library/datatransfer";
    import { GetToken, POST, staticAssetsLink } from "../library/networking";
    import ModInstall from "./ModInstall.svelte";
    import { exists } from "@tauri-apps/api/fs";
    import DownloadMod from "./downloadMod.svelte";
    export let modName = "";
    export let description = "";
    export let iconLink = "";
    export let downloadLink = "";
    export let author = "";
    export let modid = "";
    export let modplatform = "";
    export let modgame = "";
    export let update = 0;
    export let likes = 0;
    export let comments = 0;
    export let downloads = 0;
    export let visible = true;
    let authoraccountexists = true;
    export let authorname = "";
    export let gamedata;
    export let moddata;
    let downloadStatus = "Download";
    let modNodeDiv;
    let downloadMod;
    $: innerWidth = 0;
    let color = "white";
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
        let authorinfo = await POST("getprofileinfo", { id: author });

        let emblem = authorinfo.emblems.sort((a, b) => {
            return b.weight - a.weight;
        })[0];

        color = emblem.color;

        if (authorinfo.username == null) {
            authoraccountexists = false;
            authorname = "Unknown Account";
        } else {
            authorname = authorinfo.username;
        }

        let len = 0;

        for (let i = 0; i < innerWidth; i += 30) {
            len += 1;
        }

        if (description.length > len) {
            let newDesc = description.substring(0, len);
            newDesc += "...";
            description = newDesc;
        }

        downloadMod = new DownloadMod({
            target: modNodeDiv,
        });

        let gameinfo = GetData("gameinfo");
        console.log(gameinfo)

        downloadMod.Initialize(gameinfo, false, moddata);
        downloadMod.updatecb = () => {
            downloadButton.disabled = downloadMod.downloadButtonDisabled;
            downloadStatus = downloadMod.downloadButtonStatus;
        };
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", author);
        window.open("#/profilepage", "_self");
    }

    async function Download() {
        await downloadMod.Download();
    }
</script>

<svelte:window bind:innerWidth />
{#if visible}
    <div bind:this={modNodeDiv} class="modNodeDiv">
        <div>
            <span
                class="spanHyperLink"
                on:click={ViewPage}
                style="font-weight:bold;text-overflow: ellipsis;"
                >{modName}</span
            >
            <p>
                <span>
                    Author:<button
                        style="margin-left:5px;color:{color};text-overflow: ellipsis;"
                        on:click={OpenProfileOfAuthor}
                        class="hyperlinkbutton">{authorname}</button
                    >
                </span>
            </p>
            <p>
                <span style="text-overflow: ellipsis;">{description}</span>
            </p>
        </div>

        <div class="imgArea">
            <img class="modNodeImg" alt="" src={iconLink} />
            <br />
            <span style="font-size:8px;">Likes: {likes}</span>
            <span style="font-size:8px;">Downloads: {downloads}</span>
            <span style="font-size:8px;">Comments: {comments}</span>
            <button bind:this={downloadButton} on:click={Download}
                >{downloadStatus}</button
            >
            <br />
        </div>
    </div>
{/if}

<style>
    .break {
        flex-basis: 100%;
        height: 0;
    }

    .modNodeDiv {
        position: relative;
        flex-wrap: wrap;
        z-index: 1;
        background-color: rgb(41, 41, 41);
        border-radius: 20px;
        padding: 10px 10px;
        width: 50%;
        height: 100px;
        display: flex;
        margin-right: auto;
        margin-left: auto;
        margin-bottom: 20px;
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
        transition-duration: 0.1s;
    }

    .modNodeDiv:hover {
        transform: scale(1.01);
    }

    .imgArea {
        position: absolute;
        right: 5px;
        display: inline;
        margin-left: auto;
        text-align: right;
        justify-content: right;
    }
    .modNodeImg {
        width: 80px;
        height: 80px;
        border-radius: 10px;
    }
</style>

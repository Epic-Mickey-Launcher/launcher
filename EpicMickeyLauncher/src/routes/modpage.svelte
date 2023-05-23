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
    import { ReadFile, WriteFile } from "./library/configfiles";
    import ModInstall from "./components/ModInstall.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { Subscribe } from "./library/callback";

    let authoraccountexists = true;
    let modid;
    let authorname = "";
    let dumploc;
    let modinfo;
    let youtubevideoembed;
    onMount(async () => {
        modid = GetData("modpage_id");
        dumploc = GetData("modpage_dumploc");
        modinfo = await POST("getmod", { id: modid });
        let userinfo = await POST("getaccount", { id: modinfo.author });
        if (userinfo.username == null) {
            authorname = "Unknown Account";
            authoraccountexists = false;
        } else {
            authorname = userinfo.username;
        }

        if(modinfo.youtubevideo != null){
            youtubevideoembed.style.display = "block";
            youtubelink = "https://www.youtube.com/embed/" + modinfo.youtubevideo;
        }

        Subscribe(
            "SignedIn",
            (m) => {
                if (m.error != 1) {
                    if (modinfo.author == m.id) {
                        ownercontrols.style.display = "block";
                    }
                }
            },
            false
        );

        if (loggedin) {
            let id = await GetId();
            if (modinfo.author == id) {
                ownercontrols.style.display = "block";
            }
        }

        CheckIfDownloaded();
    });

    async function DeleteMod() {
        let id = await GetToken();
        let res = await POST("deletemod", { token: id, id: modid });
        if (res.error === 0) {
            window.open("#/modmarket", "_self");
        }
    }

    function UpdateMod() {
        SetData("modupload_id", modinfo.id);
        window.open("#/uploadmod", "_self");
    }

    function Download() {
        let gameid;
        gameid = "SEME4Q";

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = staticAssetsLink + modinfo.icon;
        modInstallElement.modName = modinfo.name;

        invoke("download_mod", {
            url: staticAssetsLink + modinfo.download,
            name: modinfo.name,
            dumploc: dumploc,
            modid: modinfo.id.toString(),
            gameid: gameid,
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
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
        });
    }
    let downloadButton;
    async function CheckIfDownloaded() {
        let dataStr = await ReadFile(dumploc + "/EMLMods.json");
        let dataJson = JSON.parse(dataStr);
        let json = dataJson.find((r) => r.modid == modid);
        downloadStatus = "Download";
        if (json != null) {
            downloadButton.disabled = true;
            downloadStatus = "Already Installed";
        }
    }

    async function OpenProfileOfAuthor() {
        if (!authoraccountexists) return;
        SetData("profile_id", modinfo.author);
        window.open("#/profilepage", "_self");
    }

    let ownercontrols;
    let youtubelink;
    let downloadStatus = "Download";
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

            <button on:click={() => console.log("deception")} style="width:10px;height:10px;border:none;background-color:transparent;">
                <svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 122.88 102.66" style="enable-background:new 0 0 122.88 102.66;fill:white;width:20px;height:20px;" xml:space="preserve"><style type="text/css">.st0{fill-rule:evenodd;clip-rule:evenodd;}</style><g><path class="st0" d="M0,0c10.38,7.43,27.02-0.55,33.56,12.4c1.74,3.43,2.11,8.13,0.55,11.86c-0.63,1.5-1.56,2.84-2.82,3.86 c-0.56,0.45-1.18,0.85-1.87,1.19c-8.54,4.24-17.44-1.69-22.16-8.85C2.91,13.87,1.02,5.64,0,0L0,0z M52.65,56.81 c5.78-4.62,10.27-9.93,13.32-16.02l53.72,50.94c2.81,2.66,4.4,4.91,2.04,8.99c-1.17,1.2-2.41,1.84-3.71,1.93 c-1.3,0.09-2.66-0.38-4.09-1.41L52.65,56.81L52.65,56.81z M33.03,34.05c2.5-1.35,5.94-4.66,6.75-8.27l23.29,12.78 c-3.36,6.69-7.64,12.42-13.51,16.48C43.44,46.82,40,41.86,33.03,34.05L33.03,34.05z"/></g></svg>
            </button>

            <p>
                <button on:click={OpenProfileOfAuthor} class="hyperlinkbutton"
                    >{authorname}</button
                >
            </p>
            <p>
                <span>{modinfo.description}</span>
            </p>
            <iframe style="display:none;" title="YouTube Video" width="320" height="180" allow="fullscreen;" bind:this={youtubevideoembed}
src={youtubelink}>
</iframe>

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
{/if}

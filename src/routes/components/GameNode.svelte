<svelte:options accessors={true} />

<script>
    import { SetData, objectbuffer } from "../library/datatransfer.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import { ReadFile, ReadJSON } from "../library/configfiles.js";
    import { onMount } from "svelte";
    import { exists } from "@tauri-apps/api/fs";
    import { POST } from "../library/networking.js";
    import DownloadMod from "./downloadMod.svelte";
    export let game = "";
    export let filepath = "";
    export let platform = "";
    export let region = "";
    export let imgBackgroundURL = undefined;
    export let imgLogoURL = undefined;
    export let errorMSG = "";
    export let data;
    export let mouseEnterCB;
    export let mouseExitCB;
    let updateAvailable = false;
    let outdatedMods = [];

    let reg = "";

    let platformlogo;

    async function CheckForUpdate() {
        let mods = [];

        try {
            //wrapping this around to stop EML from freaking out when offline
            let jsonExists = await exists(filepath + "/EMLMods.json");
            if (jsonExists) {
                let dataStr = await ReadFile(filepath + "/EMLMods.json");
                let data = JSON.parse(dataStr);

                data.forEach(async (r) => {
                    let latestUpdate = await POST("getmod", { id: r.modid });
                    if (r.update != latestUpdate.update) {
                        updateAvailable = true;
                        mods.push(latestUpdate);
                    }
                });
            }

            outdatedMods = mods;
            console.log(outdatedMods);
        } catch {
            console.log("failed to check for updates");
        }
    }

    async function OpenGame() {
        let d = await ReadJSON("conf.json");

        if (d.dolphinPath == "") {
            await alert("Dolphin is required for this game to work!");
            return;
        }

        if (platform == "wii") {
            invoke("playgame", {
                dolphin: d.dolphinPath,
                exe: filepath + "/sys/main.dol",
                id: data.id,
            }).then((res) => {
                if (res == 1) {
                    alert(
                        "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings.",
                    );
                }
            });
        } else {
            invoke("playgame", {
                dolphin: filepath + "/DEM2.exe",
                exe: "",
                id: "",
            }).then((res) => {
                if (res == 1) {
                    alert("Game failed to open.");
                }
            });
        }
    }

    async function UpdateAllMods() {
        let downloadMod = new DownloadMod({ target: mainDiv });

        downloadMod.updatecb = () => {
            updateAvailable = false;
        };

        for await (let r of outdatedMods) {
            await downloadMod.Initialize(data, false, r);
            await downloadMod.Download();
        }
    }

    export async function Init() {
        switch (platform) {
            case "wii":
                platformlogo.src = "img/Wii.svg";
                break;
            case "pc":
                platformlogo.src = "img/windows.svg";
                break;
        }

        //muterrs
        let result = { game: "", result: "" };

        reg = data.region;
        switch (data.region) {
            case "NTSC-U":
                result.game = "EM1";
                result.region = "NTSC-U";

                regionPath = "img/regions/usa.svg";

                break;

            case "PAL.DE,ES,IT":
                regionPath = "img/regions/deites.svg";
                break;

            case "PAL,EN,SE,DK":
                regionPath = "img/regions/scandi2.svg";
                break;

            case "PAL.SE,DK,NO":
                regionPath = "img/regions/scandi1.svg";
                break;

            case "PAL.EN,FR,NL":
                regionPath = "img/regions/frnl.svg";
                break;

            case "NTSC-J":
                regionPath = "img/regions/jp.svg";
                break;

            //EM2

            case "PAL.FR,DE,IT":
                //todo: change this with actual correct region image
                regionPath = "img/regions/deites.svg";
                break;

            case "NTSC-K":
                regionPath = "img/regions/korea.svg";
                break;

            case "PAL.EN,FR,ES,NL,PT,TR":
                //every single country here except for turkey is in the eu so i'll just call this the EU version

                region = "img/regions/eu.svg";
                break;
        }

        await CheckForUpdate();
    }

    let regionPath = "";
    onMount(async () => {});

    function OpenLevelLoader() {
        SetData("levelloaderdata", data);
        window.open("#/levelloader", "_self");
    }

    let mainDiv;
</script>

<main bind:this={mainDiv}>
    <div class="gamenode" style="background-image: url('{imgBackgroundURL}')">
        <div style="float:right;margin-left:15px;">
            <img
                class="gamelogo"
                on:mouseleave={() => mouseExitCB()}
                on:mouseenter={() => mouseEnterCB(game)}
                src={imgLogoURL}
                alt=""
            />
        </div>

        <div style="margin-left:auto;margin-top:10px;">
            <button on:click={OpenGame} class="gameplaybutton">Play</button>
            <button on:click={OpenLevelLoader} class="gamesettings">...</button>
            <br />
            <img
                style="width:15px;height:15px;float:right;padding-top:5px;padding-right:3px;"
                alt="platform"
                bind:this={platformlogo}
                src="img/Wii.svg"
            />
            <img
                title={reg}
                style="height:15px;display:inline;;padding-top:5px;float:right;padding-right:2px;"
                src={regionPath}
                alt=""
            />
            {#if updateAvailable}
                <svg
                    on:click={UpdateAllMods}
                    viewBox="0 0 30 30"
                    style="width:15px;height:15px;fill:lime;float:right;padding-top:8px;"
                    ><path
                        d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
                    /><title>Update all mods.</title><path
                        d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z"
                    /></svg
                >
            {/if}
        </div>

        <plaintext class="error">{errorMSG}</plaintext>
    </div>
</main>

<style>
    .nameofbuild {
        pointer-events: none;
        opacity: 0;
        transition-duration: 0.3s;
        bottom: 125px;
        left: 20px;
    }

    .gamenode {
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
        border-radius: 10px;
        margin-right: auto;
        margin-left: auto;
        width: 500px;
        height: 80px;
        align-items: center;
        display: flex;
    }

    .error {
        position: relative;
        left: 520px;
        bottom: 135px;
    }

    .gameplaybutton {
        padding: 10px 20px;
        background: rgb(2, 0, 36);
        background: linear-gradient(
            143deg,
            rgba(2, 0, 36, 1) 0%,
            rgba(0, 0, 0, 1) 0%,
            rgba(229, 0, 255, 1) 0%,
            rgba(133, 0, 196, 1) 100%
        );
        border: none;
        border-radius: 10px 0px 0px 10px;
        transition-duration: 0.2s;
    }

    .gameplaybutton:hover {
        background: linear-gradient(
            0deg,
            rgba(2, 0, 36, 1) 0%,
            rgba(0, 0, 0, 1) 0%,
            rgba(229, 0, 255, 1) 0%,
            rgba(133, 0, 196, 1) 100%
        );
    }

    .gamesettings:hover {
        background: linear-gradient(
            0deg,
            rgba(2, 0, 36, 1) 0%,
            rgba(0, 0, 0, 1) 0%,
            rgba(229, 0, 255, 1) 0%,
            rgba(133, 0, 196, 1) 100%
        );
    }
    .gamesettings {
        padding: 10px 5px;
        background: rgb(2, 0, 36);
        background: linear-gradient(
            143deg,
            rgba(2, 0, 36, 1) 0%,
            rgba(0, 0, 0, 1) 0%,
            rgba(229, 0, 255, 1) 0%,
            rgba(133, 0, 196, 1) 100%
        );
        border: none;
        border-radius: 0px 10px 10px 0px;
    }
    .gamelogo {
        width: 200px;
        height: 50px;
        left: 20px;
        bottom: 70px;
        filter: drop-shadow(1px 3px 5px rgba(0, 0, 0, 0.877));
        transition-duration: 0.3s;
    }

    .gamelogo:hover {
        transform: scale(1.1);
    }
</style>

<svelte:options accessors={true} />

<script>
    import { SetData, objectbuffer } from "../library/datatransfer.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import { ReadJSON } from "../library/configfiles.js";
    import { onMount } from "svelte";
    export let game = "";
    export let filepath = "";
    export let platform = "";
    export let imgBackgroundURL = undefined;
    export let imgLogoURL = undefined;
    export let errorMSG = "";
    export let data;
 
    let platformlogo;

    async function OpenGame() {
        let d = await ReadJSON("conf.json");
        if(platform == "wii")
        {
            invoke("playgame", {
            dolphin: d.dolphinPath,
            exe: filepath + "/sys/main.dol",
        }).then((res) => {
            if (res == 1) {
                alert(
                    "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings."
                );
            }
        });
        }
        else{
            invoke("playgame", {
            dolphin: filepath + "/Launch.exe",
            exe: "",
        }).then((res) => {
            if (res == 1) {
                alert(
                    "Game failed to open."
                );
            }
        });
        }
    }

    export function Init(){
        switch(platform) {
            case "wii":
            platformlogo.src = "img/Wii.svg";
            break;
            case "pc":
            platformlogo.src = "img/windows.svg";
            break;
        }
    }

    onMount(async () => {
        
    });

    function OpenLevelLoader() {
        SetData("levelloaderdata", data)
        window.open("#/levelloader", "_self");
    }
</script>

<main>
    <div class="gamenode" style="background-image: url('{imgBackgroundURL}')">

        <div style="float:right;margin-left:15px;">
            <img class="gamelogo" src={imgLogoURL} alt="" />
        </div>

        <div style="margin-left:auto;margin-top:10px;">
            <button on:click={OpenGame} class="gameplaybutton">Play</button>
            <button on:click={OpenLevelLoader} class="gamesettings">...</button>
            <br>
                    <img style="width:15px;height:15px;float:right;margin-top:3px;" alt="platform" bind:this={platformlogo} src="img/Wii.svg">
        </div>

        <plaintext class="error">{errorMSG}</plaintext>
        <plaintext class="nameofbuild">sds</plaintext>
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
        display:flex;
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

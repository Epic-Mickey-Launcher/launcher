<main>
    <body>
        <div  bind:this={options}>
            <h1>Welcome to the Epic Mickey Launcher  <img style="display:inline-block;width:60px;position:relative;top:10px;rotate:calc(10deg)" src="img/icon.png" alt=""></h1>
            <button on:click={()=> Continue()} class="b1">Quick Start</button>
            <div style="margin-right:30px;display:inline;"></div>
            <button on:click={()=>Exit("#/Games")} class="b1">Do everything manually</button>
        </div>
    
        <div style="display:none;" bind:this={quickstart}>
            <div style="background-color:black;width:max-content;padding:30px;margin:auto;">
                <h2>How to download & use Dolphin</h2>
                <plaintext>Go to the Dolphin website & Download the latest beta version (https://nb.dolphin-emu.org/download/?ref=btn)</plaintext>
                <p>
                <plaintext>Unzip the dolphin install & open Dolphin.exe</plaintext>
                <p>
                <h3>How to dump Epic Mickey 1 & 2 for Wii using Dolphin</h3>
                <plaintext>Get an ISO of EM1 or EM2 if you haven't already</plaintext>
                <plaintext>Add the folder where your EM 1 or 2 ISO is located to dolphin</plaintext>
                <plaintext>Right click on your ISO in dolphin</plaintext>
                <plaintext>Go to properties then Filesystem</plaintext>
                <plaintext>Right click on Disc and then select Extract entire disc</plaintext>
                <plaintext>Select a folder you want to dump the game to and press Select</plaintext>
                <plaintext>Once it's finished you can add it to EML</plaintext>
            </div>
    
            <p>
    
                <div style="background-color:black;width:700px;padding:30px;margin:auto;">
                    <h2>How to add EM 1 & 2 to EML</h2>
                    <plaintext>Go to the Games tab</plaintext>
                    <plaintext>Press the Add Game Build button</plaintext>
                    <plaintext>Choose the folder where you've dumped your game (Folder outside of the DATA folder)</plaintext>
                    <plaintext>EML will automatically find the version of the game you've selected</plaintext>
                    <plaintext>Enjoy.</plaintext>
                 </div>
    
            <p>
    
            <button on:click={SetDolphinPath}>Assign Dolphin Path (optional)</button>
            <plaintext style="display:inline"><em>{currentDolphinPath}</em></plaintext>
            <p>
    
                <button on:click={() => Exit("Games")} class="b1">Continue</button>
                <p>
                    <button on:click={() => Exit("addgame")} class="b1">Add Game Build</button>
    
        </div>
    
    </body>
</main>

<script>

    import { onMount } from "svelte";
    import { InitConfFiles, ReadJSON, WriteToJSON } from "./library/configfiles.js";
    import { open } from "@tauri-apps/api/dialog";

    async function SetDolphinPath() {

        const selectedPath = await open({
            title: "Select folder",
            multiple: false,
        });

        if (selectedPath.includes("Dolphin.exe")) {
            let dat = await ReadJSON("conf.json");
            dat.dolphinPath = selectedPath;
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentDolphinPath();
        }
    }

    let currentDolphinPath = "";
    onMount(async () => {
        await SetCurrentDolphinPath();
    });

    async function SetCurrentDolphinPath() {
        let c = await ReadJSON("conf.json");
        currentDolphinPath = c.dolphinPath;
    }

    function Continue()
    {
        options.style.display = "none"
        quickstart.style.display = "block"
    }

    function Exit(url){
    InitConfFiles()
     window.open("#/" + url, "_self")
    }

    let options;
    let quickstart;
    
</script>


<style>
    .b1 {
        border:none;
        width:200px;
        height:50px;
        border-radius:20px;
    }
    .b1:hover{
        transform: scale(1.1)
    }
    body
    {    
    width:100%;
    top:0px;
    min-height: 100%;
    right:0px;
    z-index: 500;
    position: absolute;
    display: flex;
    justify-content: center;
    background: linear-gradient(#ff00aa,#870099); 
    }
</style>
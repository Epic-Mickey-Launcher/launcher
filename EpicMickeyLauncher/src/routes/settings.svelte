<script>
    import { onMount } from "svelte";
    import { ReadJSON, WriteToJSON } from "./library/configfiles.js";
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    async function SetDolphinPath() {
        const selectedPath = await open({
            title: "Select Dolphin.exe",
            directory: false,
            multiple: false,
        });

        console.log(selectedPath);

        if (selectedPath.includes("Dolphin.exe") || selectedPath.includes("Dolphin.app")) {
            let dat = await ReadJSON("conf.json");
            dat.dolphinPath = selectedPath;
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
        }
    }

    async function SetWITPath() {
        const selectedPath = await open({
            title: "Select wit.exe",
            directory: false,
            multiple: false,
        });

        console.log(selectedPath);

        if (selectedPath.includes("wit.exe")) {
            let dat = await ReadJSON("conf.json");
            dat.WITPath = selectedPath;
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
        }
    }

    let currentDolphinPath = "";
    let currentWITPath = "";
    let os = "";

    //HACK: there has to be a better way to do this
    const DOLPHIN_LINK_WINDOWS = "https://dl.dolphin-emu.org/builds/91/ba/dolphin-master-5.0-19368-x64.7z";
    const WIT_LINK_WINDOWS = "https://wit.wiimm.de/download/wit-v3.05a-r8638-cygwin64.zip";
    const NKIT_LINK_WINDOWS = "https://cdn.discordapp.com/attachments/1010372370743177257/1111291487020396616/NKit.zip";

    async function DownloadWIT(){
        invoke("download_zip", {url: WIT_LINK_WINDOWS, foldername: "WIT"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            let fn = WIT_LINK_WINDOWS.split('/')[WIT_LINK_WINDOWS.split('/').length - 1].replace(".zip", "");
            dat.WITPath = path + "/" + fn + "/bin/wit.exe";
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
        })
    }

    async function DownloadNKit(){
        invoke("download_zip", {url: NKIT_LINK_WINDOWS, foldername: "NKit"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            dat.NkitPath = path + "/ConvertToISO.exe";
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
        })
    }

    async function DownloadDolphin(){
        invoke("download_zip", {url: DOLPHIN_LINK_WINDOWS, foldername: "Dolphin"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            dat.dolphinPath = path + "/Dolphin-x64/Dolphin.exe";
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
        })
    }

    onMount(async () => {
        invoke("get_os").then((_os) => {
               os = _os;
               
        })
        await SetCurrentPaths();
    });

    async function SetCurrentPaths() {
        let c = await ReadJSON("conf.json");
        currentDolphinPath = c.dolphinPath;
        currentWITPath = c.WITPath;
    }
</script>

<h1>Settings</h1>
<hr />
<p />
<button on:click={SetDolphinPath}>Assign Dolphin Path</button>
<span style="display:inline"><em>{currentDolphinPath}</em></span>
<p>
<button on:click={SetWITPath}>Assign WIT Path</button>
<span style="display:inline"><em>{currentWITPath}</em></span>
<h2>Automatically Download & Assign</h2>
<button on:click={DownloadDolphin}>Download Dolphin</button>
<button on:click={DownloadWIT}>Download Wiims ISO Tool</button>
<button on:click={DownloadWIT}>Download NKit</button>
<style></style>

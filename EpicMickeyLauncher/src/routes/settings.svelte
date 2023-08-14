<script>
    import { onMount } from "svelte";
    import { DeleteAllConfigFiles, FileExists, ReadJSON, WriteToJSON } from "./library/configfiles.js";
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import ModInstall from "./components/ModInstall.svelte";
    import { removeFile } from "@tauri-apps/api/fs";
    import { getTauriVersion } from "@tauri-apps/api/app";
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
    let currentNkitPath = "";
    let os = "";

    //HACK: there has to be a better way to do this
    const DOLPHIN_LINK_WINDOWS = "https://dl.dolphin-emu.org/builds/3b/69/dolphin-master-5.0-19870-x64.7z";
    const WIT_LINK_WINDOWS = "https://wit.wiimm.de/download/wit-v3.05a-r8638-cygwin64.zip";
    const NKIT_LINK_WINDOWS = "https://cdn.discordapp.com/attachments/1010372370743177257/1112527174478614538/NKit.zip";

    async function DownloadWIT(){
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modName = "Wiimms ISO Tools";
        modInstallElement.modIcon = "img/waren.png";
        modInstallElement.showDownloadProgression = true;
        invoke("download_tool", {url: WIT_LINK_WINDOWS, foldername: "WIT"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            let fn = WIT_LINK_WINDOWS.split('/')[WIT_LINK_WINDOWS.split('/').length - 1].replace(".zip", "");
            dat.WITPath = path + "/" + fn + "/bin/wit.exe";
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
            modInstallElement.$destroy();
        })
    }

    async function DownloadNKit(){
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.description = "This will take a while...";
        modInstallElement.modName = "NKit";
        modInstallElement.modIcon = "img/waren.png";
        modInstallElement.showDownloadProgression = true;

        invoke("download_tool", {url: NKIT_LINK_WINDOWS, foldername: "NKit"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            dat.NkitPath = path;
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
            modInstallElement.$destroy();
        })
    }

    async function DownloadDolphin(){

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modName = "Dolphin";
        modInstallElement.modIcon = "img/dolphin.png";
        modInstallElement.showDownloadProgression = true;
        invoke("download_tool", {url: DOLPHIN_LINK_WINDOWS, foldername: "Dolphin"}).then(async (path) => {
            let dat = await ReadJSON("conf.json");
            dat.dolphinPath = path + "/Dolphin-x64/Dolphin.exe";
            await WriteToJSON(JSON.stringify(dat), "conf.json");
            SetCurrentPaths();
            modInstallElement.$destroy();
        })
    }

    onMount(async () => {
        invoke("get_os").then((_os) => {
               os = _os;
               if(os == "macos"){
wit_button.disabled = true
dolphin_button.disabled = true
nkit_button.disabled = true
        }
        })


        version = await getTauriVersion()
        await SetCurrentPaths();
    });

    let version = ""

    async function SetCurrentPaths() {
        let c = await ReadJSON("conf.json");
        currentDolphinPath = c.dolphinPath;
        currentWITPath = c.WITPath;
        currentNkitPath = c.NkitPath;
    }

    async function SetDolphinEmulatorOverride(){

        const selectedPath = await open({
            title: "Select Dolphin.exe",
            directory: true,
            multiple: false,
        });


        invoke("set_dolphin_emulator_override", {path: selectedPath});
    }

    async function RemoveAllConfFiles()
    {
        let confirmation = await confirm("Are you sure?");
        if(confirmation)
        {

            let delete_docs_folder = await confirm("Do you want to delete the EML documents folder? This can fix issues with iso extraction but will also delete all of your games and tools.")
            if(delete_docs_folder)
            {
                await invoke("delete_docs_folder")
            }

            let c = await ReadJSON("games.json");
        c.forEach(async (d) => {
            let path = d.path + "/EMLMods.json"
            let fileExists = await FileExists(path);
            if(fileExists)
            {
                await removeFile(path);
            }
        })
        await DeleteAllConfigFiles();
        window.open("#/Games", "_self");
        }
    }

let dolphin_button;
let nkit_button;
let wit_button;

</script>

<h1>Settings</h1>
<hr />
<p />
<button on:click={SetDolphinPath}>Assign Dolphin Path</button>
<span style="display:inline"><em>{currentDolphinPath}</em></span>
<p>
<button on:click={SetWITPath}>Assign WIT Path</button>
<span style="display:inline"><em>{currentWITPath}</em></span>
<p>Nkit Path: {currentNkitPath}</p>

<p>
<button on:click={SetDolphinEmulatorOverride} >Override Dolphin Emulator Config Folder</button>
</p>

<h2>Automatically Download & Assign</h2>
<button bind:this={dolphin_button} on:click={DownloadDolphin}>Download Dolphin</button>
<button bind:this={wit_button} on:click={DownloadWIT}>Download Wiims ISO Tool</button>
<button bind:this={nkit_button} on:click={DownloadNKit}>Download NKit</button>
<h2>Factory Reset</h2>
<button on:click={RemoveAllConfFiles} >Remove all config files</button>
<p></p>
<script lang="ts">
    import {mount, onMount, unmount} from "svelte";
    import {DeleteAllConfigFiles, FileExists,} from "./library/configfiles.js";
    import {open} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
    import ModInstall from "./components/ModInstall.svelte";
    import {remove} from "@tauri-apps/plugin-fs";
    import {type ConfigFile} from "./library/types.js";
    import {LoadConfig, LoadGamesConfig, SaveConfig} from "./library/config";
    import {RetrieveFileByAlias} from "./library/filealias";
    import {DownloadDolphin} from "./library/dolphin";

    let config: ConfigFile = $state();

    async function SetDolphinPath() {
        const selectedPath = await open({
            title: "Select Dolphin.exe",
            directory: false,
            multiple: false,
        });

        if (
            selectedPath.includes("Dolphin.exe") ||
            selectedPath.includes("Dolphin.app") ||
            selectedPath.includes("dolphin-emu")
        ) {
            config.dolphinPath = selectedPath;
            await SaveConfig(config);
        }
    }

    async function DownloadDolphinEmu() {
        let modInstallElement = mount(ModInstall, {
            target: document.body,
        });
        modInstallElement.modName = "Dolphin";
        modInstallElement.modIcon = "img/dolphin.png";
        modInstallElement.showDownloadProgression = true;
        await DownloadDolphin()
        await unmount(modInstallElement);
    }

    onMount(async () => {
        await SetCurrentPaths();
    });


    async function OpenDolphinFolder() {
        await invoke("open_dolphin", {path: config.dolphinPath});
    }

    async function SetCurrentPaths() {
        config = await LoadConfig();
    }

    async function DeleteModCache() {
        await invoke("delete_mod_cache_all");
    }

    async function RemoveAllConfFiles() {
        let confirmation = confirm("Are you sure?");
        if (confirmation) {
            let delete_docs_folder = confirm(
                "Do you want to delete the EML documents folder? This can fix issues with iso extraction but will also delete all of your games and tools.",
            );
            if (delete_docs_folder) {
                await invoke("delete_docs_folder");
            }
            let games = await LoadGamesConfig();
            for (const game of games) {
                let path = await RetrieveFileByAlias("eml-mods-json", game);
                let fileExists = await FileExists(path);
                if (fileExists) {
                    await remove(path);
                }
            }
            await DeleteAllConfigFiles();
            window.open("#/Games", "_self");
        }
    }
</script>

{#if config != null}
    <h1>Settings</h1>
    <hr/>
    <p></p>
    <h2>Dolphin Settings</h2>
    <button onclick={DownloadDolphinEmu}
    >
        {config.dolphinPath === "" ? "Download" : "Redownload"}
    </button
    >
    {#if config.dolphinPath !== ""}<span style="font-size:7px;color:lime;"
    >*Downloaded</span
    >{/if}
    <br/>
    <button disabled={config.dolphinPath === ""} onclick={OpenDolphinFolder}>Open Dolphin
    </button>
    <h2>Factory Reset</h2>
    <button onclick={RemoveAllConfFiles}>Remove all config files</button>
    <br/>
    <button onclick={DeleteModCache}>Delete mod cache</button>
    <p></p>
{/if}
<span>© 2024 Jonas Kalsvik</span>

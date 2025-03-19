<script lang="ts">
    import {mount, onMount, unmount} from "svelte";
    import {DeleteAllConfigFiles, FileExists,} from "./library/configfiles.js";
    import {open} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
    import ModInstall from "./components/ModInstall.svelte";
    import {copyFile, remove} from "@tauri-apps/plugin-fs";
    import {type ConfigFile, Game, Platform} from "./library/types.js";
    import {LoadConfig, LoadGamesConfig, SaveConfig} from "./library/config";
    import {RetrieveFileByAlias} from "./library/filealias";
    import {DownloadDolphin} from "./library/dolphin";

    let config: ConfigFile = $state();
    let modTemplateGenerator: HTMLDialogElement = $state();

    let title: string = $state("");
    let description: string = $state("");
    let iconPath: string = $state("");
    let game: Game = $state(Game.EM1);
    let platform: Platform = $state(Platform.Wii);

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

    async function SetIcon() {
        iconPath = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    extensions: ["png"],
                    name: "PNG File"
                }
            ]
        })
    }

    async function CreateModTemplate() {
        if (game == Game.EMR && platform != Platform.PC) {
            console.log("EMR Mod Template can only be generated for PC.")
            return
        }
        if (game == Game.EM1 && platform != Platform.Wii) {
            console.log("EM1 Mod Template can only be generated for Wii.")
            return
        }

        if (iconPath == "") {
            console.log("Please set an icon.")
            return
        }

        const selected = await open({
            multiple: false,
            directory: true,
        });

        await invoke("generate_mod_project", {
            game: game,
            platform: platform,
            path: selected,
            name: title,
            description: description
        })

        await copyFile(iconPath, selected + "/icon.png");
        await invoke("open_link", {url: selected})
        modTemplateGenerator.close()
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

    <dialog bind:this={modTemplateGenerator}>
        <h2>Mod Template Generator</h2>

        <input placeholder="Mod Title" style="width:100%;" bind:value={title}>
        <p></p>
        <textarea placeholder="Description (formatted in markdown)" style="width: 100%;min-height: 20vw;"
                  bind:value={description}></textarea>
        <p></p>
        <span>Game: </span>
        <select bind:value={game} style="color: black">
            <option value={Game.EM1}>EM1</option>
            <option value={Game.EM2}>EM2</option>
            <option value={Game.EMR}>EMR</option>
        </select>
        <p></p>
        <span>Platform: </span>
        <select bind:value={platform} style="color: black">
            <option value={Platform.Wii}>Wii</option>
            <option value={Platform.PC}>PC</option>
        </select>
        <p></p>
        <button style="width:100%;" onclick={SetIcon}>
            {#if iconPath === ""}
                Set Icon
            {:else}
                Icon Set!
            {/if}
        </button>
        <p></p>
        <button style="width:100%;" onclick={CreateModTemplate}>Generate</button>
        <br>
        <button style="width:100%;" onclick={() => modTemplateGenerator.close()}>Exit</button>
    </dialog>

    <h1>Settings</h1>
    <hr/>
    <h2>Client Settings</h2>
    <button onclick={() => modTemplateGenerator.showModal()}>Create Mod Template</button>
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
<span>© 2025 Jonas Kalsvik</span>

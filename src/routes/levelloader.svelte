<script lang="ts">
    import {mount, onMount, unmount} from "svelte";
    import levelsData from "./data/levels.json";
    import levelsDataEM2 from "./data/levels_em2.json";
    import SettingsModNode from "./components/SettingsModNode.svelte";
    import {ReadFile, WriteFile,} from "./library/configfiles";
    import {invoke} from "@tauri-apps/api/core";
    import {open} from "@tauri-apps/plugin-dialog";
    import ModInstall from "./components/ModInstall.svelte";
    import {exists} from "@tauri-apps/plugin-fs";
    import {GameInstance} from "./library/instance.svelte";
    import {activeInstance, RemoveTrackedGame} from "./library/config";
    import {Game, Platform, Region} from "./library/types";

    let data: GameInstance;
    let mainSettings: HTMLDivElement = $state();
    let levelLoader: HTMLDivElement = $state();
    let unsavedCmdline: string;
    let currentLevelsToShow = $state([]);
    let currentLevelJSON: any[] = $state([]);
    let cmdline = "";
    let levelEndIndex: number;
    let modNodes: SettingsModNode[] = [];
    let currentInstance: GameInstance = $state()

    let selectedLevel = $state("");
    let selectedCategoryName = $state("");
    let selectedCategoryImg = $state("");

    function SelectCategory(category: any) {
        selectedCategoryImg = category.banner
        currentLevelsToShow = category.levels;
        selectedCategoryName = category.name;
    }

    function OpenLevelLoader() {
        mainSettings.style.display = "none";
        levelLoader.style.display = "block";
        selectedLevel = cmdline.substring(0, levelEndIndex);
    }

    function OpenDirectory() {
        invoke("get_os").then((os) => {
            let p = os == "windows" ? data.path.replace("/", "\\") : data.path;
            invoke("open_path_in_file_manager", {path: p});
        });
    }

    async function RefreshMods() {
        modNodes.forEach((r) => {
            unmount(r)
        });
    }

    onMount(async () => {
        data = activeInstance
        currentInstance = activeInstance;

        await RefreshMods();
        let cmdlineExists = await exists(data.path + "/files/cmdline.txt");

        if (cmdlineExists) {
            cmdline = await ReadFile(data.path + "/files/cmdline.txt");

            unsavedCmdline = cmdline;

            for (let index = 0; index < cmdline.length; index++) {
                if (cmdline.at(index) === "-") {
                    levelEndIndex = index - 1;
                    break;
                }
            }
            selectedLevel = cmdline.substring(0, levelEndIndex);
        }


        if (
            data.gameConfig.game == Game.EM2 &&
            data.gameConfig.platform === Platform.Wii
        ) {
            currentLevelJSON = levelsDataEM2;
            SelectCategory(currentLevelJSON[0])
        } else if (data.gameConfig.game == Game.EM1) {
            currentLevelJSON = levelsData;
            SelectCategory(currentLevelJSON[0])
        } else {
            currentLevelJSON = [];
        }

    });

    async function InstallLocalMod() {
        let data = activeInstance
        const selectedPath = await open({
            title: "Select archive",
            multiple: false,
        });
        let filename = selectedPath.toString().split("\\").pop().split("/").pop();

        let modInstallElement = mount(ModInstall, {
            target: document.body,
        });
        modInstallElement.modIcon = "img/waren.png";
        modInstallElement.modName = filename;
        modInstallElement.action = "Installing";
        modInstallElement.description =
            "This might take a while depending on your storage device's speed.";

        await data.AddMod(null, selectedPath.toString())
        await unmount(modInstallElement);


        setTimeout(async () => {
            await RefreshMods();
        }, 120);
    }

    function SetLevel(lvl: any) {
        unsavedCmdline = "";
        let newCmdLine = lvl.path;
        unsavedCmdline = newCmdLine + " -Set PlayerEnableAllAbilities=true";
        levelEndIndex = lvl.path.length;
        selectedLevel = lvl.path;
    }

    async function PlayGame() {
        cmdline = unsavedCmdline;
        await WriteFile(cmdline, data.path + "/files/cmdline.txt");
        await data.Play()
    }

    async function DeleteFromGameList() {
        await RemoveTrackedGame(activeInstance.path)
        window.open("#/", "_self");
    }

    async function ExitLevelLoader(type: number) {
        cmdline = unsavedCmdline;

        await WriteFile(cmdline, data.path + "/files/cmdline.txt");

        if (type == 1) {
            mainSettings.style.display = "block";
            levelLoader.style.display = "none";
        } else {
            window.open("#/", "_self");
        }
    }

    function GoBackToGames() {
        window.open("#/", "_self");
    }
</script>

<main>
    <div bind:this={mainSettings}>
        <h2>Settings for <i>{activeInstance.gameIdentity.name + " (" +
        activeInstance.gameConfig.platform.toUpperCase() + (activeInstance.gameConfig.region !== Region.None ? (", " + activeInstance.gameConfig.region) : "") + ")"}</i>
        </h2>
        <hr/>
        <p></p>
        {#if currentLevelJSON.length > 0}
            <button onclick={OpenLevelLoader}
            >Change Level
            </button
            >
        {/if}
        <button onclick={OpenDirectory}>Open Directory</button>
        <button onclick={DeleteFromGameList}>Delete from Game List</button>
        <button onclick={GoBackToGames}>Back</button>

        <h1>Mods</h1>
        <hr/>
        <p></p>
        {#if currentInstance != null}
            <div>
                {#each currentInstance.mods as mod}
                    <SettingsModNode modData={mod} gameInstance={currentInstance}></SettingsModNode>
                {/each}
            </div>
        {/if}
        <p>
            <button onclick={InstallLocalMod}>Install Local Mod</button>
        </p>
    </div>

    <div bind:this={levelLoader} style="display:none;">
        <h1 style="text-align:center;">Level Loader</h1>
        <p></p>
        <div
                style="display:flex;align-items:center;justify-content:center;position:relative;"
        >
            <div
                    style="overflow:hidden; margin-right:256px;position:absolute;height:112px;width:256px;top:0px;overflow:hidden;border-radius:10px 0px 0px 0px;text-align:center;"
            >
                <img
                        alt=""
                        src={selectedCategoryImg}
                        style="filter:blur(3px);height:112px;width:256px;"
                />
                <span
                        style="z-index:5;font-size:50px;position:relative;bottom:80px;;left:0px;width:112px;font-size:25px;text-align:center;"
                >{selectedCategoryName}</span
                >
            </div>
            <br/>
            <div
                    style="width:256px;height:400px; margin-top:112px; border-radius:0px 0px 0px 10px; overflow: scroll;background-color:#1f1f1f;"
            >
                <div
                        style="position:relative;display:grid;width:256px;height:400px;grid-auto-flow: row; rows:2em;"
                >
                    {#each currentLevelJSON as category}
                        <button
                                onclick={() => SelectCategory(category)}
                                style="width:100%;height:100%;">{category.name}</button
                        >
                    {/each}
                </div>
            </div>

            <div
                    style="width:256px;height:512px;border-radius:0px 10px 10px 0px; overflow:hidden;background-color:#1f1f1f;overflow-y: scroll;"
            >
                <div style="position:relative;width:256px;display:grid;">
                    {#each currentLevelsToShow as lvl}
                        <button onclick={() => SetLevel(lvl)} style="">{lvl.name}</button>
                    {/each}
                </div>
            </div>
        </div>

        <p>Selected Level: {selectedLevel}</p>

        <hr/>

        <p>
            <button class="play" onclick={() => PlayGame()}>Play</button>
            <button
                    class="play"
                    onclick={() => ExitLevelLoader(1)}
                    style="        border-radius: 0px 10px 10px 0px;">Save Level and Return
            </button
            >
        </p>
        <p>
      <span
      >Level Loader Data <s>stolen</s> borrowed from RampantLeaf & SlayCap</span
      >
        </p>
    </div>

</main>
<style>
    .play {
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
        border-radius: 10px 0 0 10px;
        transition-duration: 0.2s;
    }

    .play:hover {
        background: linear-gradient(
                0deg,
                rgba(2, 0, 36, 1) 0%,
                rgba(0, 0, 0, 1) 0%,
                rgba(229, 0, 255, 1) 0%,
                rgba(133, 0, 196, 1) 100%
        );
    }
</style>
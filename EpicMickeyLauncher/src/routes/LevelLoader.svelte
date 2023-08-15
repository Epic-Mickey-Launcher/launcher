<script>
    import { onMount } from "svelte";
    import levelsData from "./data/levels.json";
    import levelsDataEM2 from "./data/levels_em2.json";
    import ModNode from "./components/SettingsModNode.svelte";
    import {
        ReadFile,
        ReadJSON,
        WriteFile,
        WriteToJSON,
    } from "./library/configfiles";
    import { GetData, objectbuffer } from "./library/datatransfer.js";
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import ModInstall from "./components/ModInstall.svelte";
    import { exists } from "@tauri-apps/api/fs";
    let data;

    let levelLoaderButtons;
    let mainSettings;
    let levelLoader;
    let unsavedCmdline;
    let changelevelbutton;
    let currentLevelsToShow = [];

    let currentLevelJSON;
    
    function SelectCategory(name) {
        LoadImage(name);
        currentLevelsToShow = currentLevelJSON.find(
            (r) => r.categoryname == name
        ).levels;
    }

    let search = "";

    let selectedLevel = "";

    let selectedCategoryName = "";
    let selectedCategoryImg = "img/EM1banner.png";

    function LoadImage(categoryname) {
        selectedCategoryName = categoryname;

        switch (categoryname) {
            case "Mean Street":
                selectedCategoryImg = "img/em1levelloader/ms.png";
                break;
            case "OsTown":
                selectedCategoryImg = "img/em1levelloader/ostown.png";
                break;
            case "Bog Easy":
                selectedCategoryImg = "img/em1levelloader/be.png";
                break;
            case "Projector Levels":
                selectedCategoryImg = "img/em1levelloader/2d.png";
                break;
            case "Dark Beauty Castle":
                selectedCategoryImg = "img/em1levelloader/dbc.png";
                break;
            case "Ventureland":
                selectedCategoryImg = "img/em1levelloader/vl.png";
                break;
            case "Mickey Junk Mountain":
                selectedCategoryImg = "img/em1levelloader/mjm.png";
                break;
            case "E3":
                selectedCategoryImg = "img/em1levelloader/e3.png";
                break;
            case "Tomorrow City":
                selectedCategoryImg = "img/em1levelloader/tc.png";
                break;
            case "Interior":
                selectedCategoryImg = "img/em1levelloader/interior.png";
                break;
            case "Tests":
                selectedCategoryImg = "img/em1levelloader/test.png";
                break;
            case "Gremlin Village":
                selectedCategoryImg = "img/em1levelloader/gv.png";
                break;
            case "Pirates of the Wasteland":
                selectedCategoryImg = "img/em1levelloader/potw.png";
                break;
            case "Miscellaneous":
                selectedCategoryImg = "img/em1levelloader/misc.png";
                break;
            case "Lonesome Manor":
                selectedCategoryImg = "img/em1levelloader/lm.png";
                break;
        }
    }

    function OpenLevelLoader() {
        mainSettings.style.display = "none";
        levelLoader.style.display = "block";
        selectedLevel = cmdline.substring(0, levelEndIndex);
    }

    function OpenDirectory() {
        let p = data.path.replace("/", "\\");

        invoke("open_process", { path: "explorer.exe", args: p });
    }

    function CreateModNode(element, index) {
        let modNode = new ModNode({
            target: modNodeGrid,
        });

        console.log(data);

        modNode.index = index;
        modNode.gamedata = data;
        modNode.modName = element.name;
        modNode.json = JSON.stringify(element);
        modNode.dumploc = data.path;
        modNode.active = element.active;
        modNode.setChecked(element.active);
    }

    let cmdline = "";

    let modNodeGrid;

    let levelEndIndex;

    data = GetData("levelloaderdata");

    if(data.game == "EM2" && data.platform == "wii" ){
              currentLevelJSON = levelsDataEM2;
        }
        else if (data.game == "EM1"){
            currentLevelJSON = levelsData;
        }
        else {
            currentLevelJSON=[];
        }

    onMount(async () => {

        let ModsData = await ReadFile(data.path + "/EMLMods.json");

        let ModsDataObject = JSON.parse(ModsData);

        ModsDataObject.forEach((element) => {
            let i = ModsDataObject.indexOf(element);
            CreateModNode(element, i);
        });

        let cmdlineexists = await exists(data.path + "/files/cmdline.txt");

        if (cmdlineexists) {
            cmdline = await ReadFile(data.path + "/files/cmdline.txt");

            unsavedCmdline = cmdline;

            for (let index = 0; index < cmdline.length; index++) {
                if (cmdline.at(index) === "-") {
                    levelEndIndex = index - 1;
                    break;
                }
            }
            selectedLevel = cmdline.substring(0, levelEndIndex);
        } else {
            changelevelbutton.style.display = "none";
        }
    });

    async function InstallLocalMod() {
        const selectedPath = await open({
            title: "Select ZIP",
            multiple: false,
        });
        let filename = selectedPath
            .toString()
            .split("\\")
            .pop()
            .split("/")
            .pop();

        let gameid;
        if (data.game == "EM1") {
            gameid = "SEME4Q";
        } else {
            gameid = "SERE4Q";
        }

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = "img/waren.png";
        modInstallElement.modName = filename;
        modInstallElement.action = "Installing";
        modInstallElement.description =
            "This might take a while depending on your storage device's speed.";

        invoke("download_mod", {
            url: selectedPath,
            name: filename,
            modid: Date.now().toString(),
            dumploc: data.path,
            gameid: gameid,
            platform: data.platform,
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
            let currentMods = JSON.parse(
                await ReadFile(data.path + "/EMLMods.json")
            );
            currentMods.push(changedFiles);
            await WriteFile(
                JSON.stringify(currentMods),
                data.path + "/EMLMods.json"
            );
            modInstallElement.$destroy();
            CreateModNode(changedFiles, currentMods.length);
        });
    }

    function SetLevel(lvl) {
        unsavedCmdline = "";
        //turn all of the garbage on
        unsavedCmdline = unsavedCmdline.replace("false", "true");

        let newCmdLine = lvl.path;

        unsavedCmdline = newCmdLine + " -Set PlayerEnableAllAbilities=true";

        levelEndIndex = lvl.path.length;

        selectedLevel = lvl.path;
    }

    async function PlayGame() {
        cmdline = unsavedCmdline;

        await WriteFile(cmdline, data.path + "/files/cmdline.txt");

        let d = await ReadJSON("conf.json");
        invoke("playgame", {
            dolphin: d.dolphinPath,
            exe: data.path + "/sys/main.dol",
        }).then((res) => {
            if (res == 1) {
                alert(
                    "Game failed to open. Make sure that you have specified Dolphin's executable path in the settings."
                );
            }
        });
    }

    async function DeleteFromGameList() {
        let dat = await ReadJSON("games.json");

        let toDelete = await dat.find(r => r.path === data.path);

        dat.splice(dat.indexOf(toDelete), 1);

        await WriteToJSON(JSON.stringify(dat), "games.json");

        window.open("#/", "_self");
    }

    async function ExitLevelLoader(type) {
        cmdline = unsavedCmdline;

        await WriteFile(cmdline, data.path + "/files/cmdline.txt");

        if (type == 1) {
            mainSettings.style.display = "block";
            levelLoader.style.display = "none";
        } else {
            await window.open("#/", "_self");
        }
    }

    function GoBackToGames() {
        window.open("#/", "_self");
    }
</script>

<main>
    <div bind:this={mainSettings}>
        <plaintext>Epic Mickey Settings</plaintext>
        <hr />
        <p />
        <button bind:this={changelevelbutton} on:click={OpenLevelLoader}
            >Change Level</button
        >
        <button on:click={OpenDirectory}>Open Directory</button>
        <button on:click={DeleteFromGameList}>Delete from Game List</button>
        <button on:click={GoBackToGames}>Back</button>

        <h1>Mods</h1>
        <hr />
        <p />
        <div bind:this={modNodeGrid} />

        <button on:click={InstallLocalMod}>Install Local Mod</button>
    </div>
    
    <div bind:this={levelLoader} style="display:none;">
        <h1 style="text-align:center;">Level Loader</h1>
        <p />
        <div style="display:flex;align-items:center;justify-content:center;">
            <div
                style="overflow:hidden; position:absolute;height:112px;width:256px;bottom:480px;right:790px;overflow:hidden;border-radius:10px 0px 0px 0px;text-align:center;"
            >
                <img
                    style="filter:blur(3px);height:112px;width:256px;"
                    alt=""
                    src={selectedCategoryImg}
                />
                <span
                    style="z-index:5;font-size:50px;position:relative;bottom:80px;left:0px;width:112px;font-size:25px;text-align:center;"
                    >{selectedCategoryName}</span
                >
            </div>
            <br />
            <div
                style="width:256px;height:400px; margin-top:112px; border-radius:0px 0px 0px 10px; overflow: hidden;background-color:#1f1f1f;"
            >
                <div
                    style="position:relative;display:grid;width:256px;height:400px;grid-auto-flow: row; rows:2em;"
                >
                    {#each currentLevelJSON as category}
                        <button
                            on:click={() =>
                                SelectCategory(category.categoryname)}
                            style="width:100%;height:100%;"
                            >{category.categoryname}</button
                        >
                    {/each}
                </div>
            </div>

            <div
                style="width:256px;height:512px;border-radius:0px 10px 10px 0px; overflow:hidden;background-color:#1f1f1f;overflow-y: scroll;"
            >
                <div style="position:relative;width:256px;display:grid;">
                    {#each currentLevelsToShow as lvl}
                        <button on:click={() => SetLevel(lvl)} style=""
                            >{lvl.name}</button
                        >
                    {/each}
                </div>
            </div>
        </div>

        <p>Selected Level: {selectedLevel}</p>

        <hr />

        <p>
            <button class="play" on:click={() => PlayGame()}>Play</button>
            <button
                class="play"
                style="        border-radius: 0px 10px 10px 0px;"
                on:click={() => ExitLevelLoader(1)}
                >Save Level and Return</button
            >
            <plaintext><s>stolen</s> borrowed from RampantLeaf & SlayCap</plaintext>
        </p>
    </div>

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
            border-radius: 10px 0px 0px 10px;
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
</main>

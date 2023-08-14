<svelte:options accessors={true} />

<script>
    import { onMount } from "svelte";
    import ModInstall from "./ModInstall.svelte";
    export let modName = "mod";
    export let dumploc = "";
    export let json = "";
    export let index = 0;
    export let active = false;
    export let gamedata;
    let node;
    import { invoke } from "@tauri-apps/api/tauri";

    import { ReadFile, WriteFile } from "../library/configfiles";
    import { destroy_component } from "svelte/internal";
    let checkBox;

    onMount(async () => {});

    export function setChecked(check) {
        checkBox.checked = check;
    }

    async function DeleteMod() {
        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = "/img/waren.png";
        modInstallElement.modName = modName;
        modInstallElement.action = "Deleting";
        modInstallElement.description = "This might take a while...";
        console.log(gamedata)
        let gameid;
        if (gamedata.game == "EM1") {
            gameid = "SEME4Q";
        } else {
            gameid = "SERE4Q";
        }

        invoke("delete_mod", {
            json: json,
            dumploc: dumploc,
            gameid: gameid,
            platform: gamedata.platform
        }).then(async () => {
            let datastring = await ReadFile(dumploc + "/EMLMods.json");
            let data = JSON.parse(datastring);

            let delete_index = data.indexOf(JSON.parse(json));

            data.splice(delete_index, 1);
            await WriteFile(JSON.stringify(data), dumploc + "/EMLMods.json");

            modInstallElement.$destroy();
            node.remove();
        });
    }

    async function ToggleMod() {
        let jsonToObject = JSON.parse(json);
        jsonToObject.active = !jsonToObject.active;

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = "/img/waren.png";
        modInstallElement.modName = modName;
        modInstallElement.action = jsonToObject.active
            ? "Enabling"
            : "Disabling";
        modInstallElement.description = "This might take a while...";

        let gameid;
        if (gamedata.game == "EM1") {
            gameid = "SEME4Q";
        } else {
            gameid = "SERE4Q";
        }

        let jsonString = JSON.stringify(jsonToObject);
        invoke("change_mod_status", {
            json: jsonString,
            dumploc: dumploc,
            modid: jsonToObject.modid,
            gameid: gameid,
            active: jsonToObject.active,
            modname: jsonToObject.name,
            platform: gamedata.platform
        }).then(async () => {
            let datastring = await ReadFile(dumploc + "/EMLMods.json");
            let data = JSON.parse(datastring);
            json = jsonString;
            active = !jsonToObject.active;
            data[index] = jsonToObject;
            await WriteFile(JSON.stringify(data), dumploc + "/EMLMods.json");
            modInstallElement.$destroy();
        });
    }
</script>

<div style="background-color:rgb(22, 22, 22);padding:10px;" bind:this={node}>
    <label for="check">{modName} | Enabled: </label>
    <input
        bind:this={checkBox}
        on:click={ToggleMod}
        id="check"
        type="checkbox"
    />
    <button on:click={DeleteMod} style="background-color:red;">Delete</button>
</div>

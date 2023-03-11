<script>
    import { onMount } from "svelte";
    import { ReadJSON, WriteToJSON } from "./library/configfiles.js";
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
</script>

<h1>Settings</h1>
<hr />
<p />
<button on:click={SetDolphinPath}>Assign Dolphin Path</button>
<plaintext style="display:inline"><em>{currentDolphinPath}</em></plaintext>

<style></style>

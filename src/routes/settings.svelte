<script lang="ts">
  import { onMount } from "svelte";
  import {
    DeleteAllConfigFiles,
    FileExists,
    ReadJSON,
    WriteToJSON,
  } from "./library/configfiles.js";
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

    if (
      selectedPath.includes("Dolphin.exe") ||
      selectedPath.includes("Dolphin.app") ||
      selectedPath.includes("dolphin-emu")
    ) {
      let dat = await ReadJSON("conf.json");
      dat.dolphinPath = selectedPath;
      await WriteToJSON(JSON.stringify(dat), "conf.json");
      SetCurrentPaths();
    }
  }

  let currentDolphinPath = "";
  let currentWITPath = "";
  let currentNkitPath = "";
  let os = "";
  let dolphin_button: HTMLButtonElement;
  let version = "";

  const DOLPHIN_LINK_WINDOWS = "https://kalsvik.no/res/dolphin_windows.zip";
  const DOLPHIN_LINK_LINUX = "https://kalsvik.no/res/dolphin_linux.7z";
  const DOLPHIN_LINK_MACOS = "https://kalsvik.no/res/dolphin_mac.zip";

  async function DownloadDolphin() {
    let modInstallElement = new ModInstall({
      target: document.body,
    });
    modInstallElement.modName = "Dolphin";
    modInstallElement.modIcon = "img/dolphin.png";
    modInstallElement.showDownloadProgression = true;
    let url = "";
    if (os == "windows") url = DOLPHIN_LINK_WINDOWS;
    else if (os == "macos") url = DOLPHIN_LINK_MACOS;
    else if (os == "linux") url = DOLPHIN_LINK_LINUX;
    console.log(os);

    invoke("download_tool", { url: url, foldername: "Dolphin" }).then(
      async (path) => {
        let dat = await ReadJSON("conf.json");

        if (os == "windows") dat.dolphinPath = path + "/Dolphin.exe";
        else if (os == "macos") dat.dolphinPath = path + "/Dolphin.app";
        else if (os == "linux") dat.dolphinPath = path + "/dolphin-emu";

        await invoke("create_portable", { dolphinpath: dat.dolphinPath });
        await WriteToJSON(JSON.stringify(dat), "conf.json");
        SetCurrentPaths();
        modInstallElement.$destroy();
      },
    );
  }

  onMount(async () => {
    invoke("get_os");
    version = await getTauriVersion();
    //await SetCurrentPaths();
  });

  async function SetCurrentPaths() {
    let c = await ReadJSON("conf.json");
    currentDolphinPath = c.dolphinPath;
    currentWITPath = c.WITPath;
    currentNkitPath = c.NkitPath;
  }

  async function DeleteModCache() {
    await invoke("delete_mod_cache_all");
  }

  async function RemoveAllConfFiles() {
    let confirmation = await confirm("Are you sure?");
    if (confirmation) {
      let delete_docs_folder = await confirm(
        "Do you want to delete the EML documents folder? This can fix issues with iso extraction but will also delete all of your games and tools.",
      );
      if (delete_docs_folder) {
        await invoke("delete_docs_folder");
      }
      let c = await ReadJSON("games.json");
      c.forEach(async (d: { path: string }) => {
        let path = d.path + "/EMLMods.json";
        let fileExists = await FileExists(path);
        if (fileExists) {
          await removeFile(path);
        }
      });
      await DeleteAllConfigFiles();
      window.open("#/Games", "_self");
    }
  }
</script>

<h1>Settings</h1>
<hr />
<p />
<button on:click={SetDolphinPath}>Assign Dolphin Path</button>
<span style="display:inline"><em>{currentDolphinPath}</em></span>
<p></p>
<h2>Automatically Download & Assign</h2>
<button bind:this={dolphin_button} on:click={DownloadDolphin}
  >Download Dolphin</button
>
<h2>Factory Reset</h2>
<button on:click={RemoveAllConfFiles}>Remove all config files</button>
<br />
<button on:click={DeleteModCache}>Delete mod cache</button>
<p></p>

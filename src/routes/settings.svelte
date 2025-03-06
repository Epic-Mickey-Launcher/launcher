<script lang="ts">
  import { onMount } from "svelte";
  import {
    DeleteAllConfigFiles,
    FileExists,
    ReadJSON,
    WriteToJSON,
  } from "./library/configfiles.js";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import ModInstall from "./components/ModInstall.svelte";
  import { remove } from "@tauri-apps/plugin-fs";
  import { getTauriVersion } from "@tauri-apps/api/app";
  import Question from "./components/Question.svelte";
  import { ConfigFile } from "./library/types.js";
  import { LoadConfig, SaveConfig } from "./library/config.js";

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
      SaveConfig(config);
    }
  }

  let config: ConfigFile;
  let os = "";
  let dolphin_button: HTMLButtonElement;
  let version = "";
  let tempRevokeLabel = false;

  const GITHUB_CLIENT_ID = "Ov23liwSwM9SY1O7uD9q";
  const DOLPHIN_LINK_WINDOWS =
    "https://kalsvik.no/res/dolphin_windows64.tar.gz";
  const DOLPHIN_LINK_LINUX = "https://kalsvik.no/res/dolphin_linux.tar.gz";
  const DOLPHIN_LINK_MACOS = "https://kalsvik.no/res/dolphin_mac.tar.gz";

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
    invoke("download_tool", { url: url, foldername: "Dolphin" }).then(
      async (path) => {
        if (os == "windows") config.dolphinPath = path + "/Dolphin.exe";
        else if (os == "macos") config.dolphinPath = path + "/Dolphin.app";
        else if (os == "linux") config.dolphinPath = path + "/dolphin-emu";

        await invoke("create_portable", { dolphinpath: config.dolphinPath });
        modInstallElement.$destroy();
      },
    );
  }

  onMount(async () => {
    os = await invoke("get_os");
    version = await getTauriVersion();
    await SetCurrentPaths();
  });

  async function SetCurrentPaths() {
    config = await LoadConfig();
  }

  async function DeleteModCache() {
    await invoke("delete_mod_cache_all");
  }

  async function RevokeGitHub() {
    config.gitHubSecret = "";
    SaveConfig(config);
  }
  async function ConnectGitHub() {
    await invoke("open_link", {
      url:
        "https://github.com/login/oauth/authorize?client_id=" +
        GITHUB_CLIENT_ID +
        "&scope=repo",
    });

    tempRevokeLabel = true;
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
          await remove(path);
        }
      });
      await DeleteAllConfigFiles();
      window.open("#/Games", "_self");
    }
  }
</script>

{#if config != null}
  <h1>Settings</h1>
  <hr />
  <p />
  <h2>Client Settings</h2>
  <span>Enable Developer Mode</span>
  <input type="checkbox" bind:value={config.developerMode} />
  <Question
    content="This enables the Mod Publisher tool which allows you to make/publish/update mods."
  ></Question>
  <h2>Developer Settings</h2>
  {#if config.gitHubSecret === "" && !tempRevokeLabel}
    <button on:click={ConnectGitHub}>Connect GitHub Account</button>
  {:else}
    <button on:click={RevokeGitHub}>Revoke GitHub Connection</button>
  {/if}
  <h2>Dolphin Settings</h2>
  <button bind:this={dolphin_button} on:click={DownloadDolphin}
    >Download Dolphin</button
  >
  {#if config.dolphinPath != ""}<span style="font-size:7px;color:lime;"
      >*Downloaded</span
    >{/if}
  <br />
  <button on:click={SetDolphinPath}>Set Dolphin Config Path</button>
  <Question
    content="You can use this to either move the config to another place, or link a pre-existing dolphin config."
  ></Question>
  <h2>Factory Reset</h2>
  <button on:click={RemoveAllConfFiles}>Remove all config files</button>
  <br />
  <button on:click={DeleteModCache}>Delete mod cache</button>
  <p></p>
{/if}
<span>© 2024 Jonas Kalsvik</span>

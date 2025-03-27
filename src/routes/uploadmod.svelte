<script lang="ts">
  import { mount, onMount, unmount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { GetToken, POST, serverLink } from "./library/networking";
  import { GetData, SetData } from "./library/datatransfer";
  import ModInstall from "./components/ModInstall.svelte";
  import { remove } from "@tauri-apps/plugin-fs";

  let modVerified: boolean;
  let modIconData: string;
  let modName: string;

  let updatingID = null;

  async function UploadMod() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Archive",
          extensions: ["zip", "tar", "7z", "rar", "tar.gz"],
        },
      ],
    });

    let modInstallElement = mount(ModInstall, {
      target: document.body,
      props: {
        modIcon: "img/emicon.png",
        action: "Verifying your Mod...",
        modName: "",
        description: "This could take a while...",
      },
    });

    let validation: any = await invoke("validate_mod", {
      url: selected.toString(),
      destination: "",
      mode: "local",
    });

    if (!validation.validated) {
      await alert("Mod could not be validated!: " + validation.result);
      await unmount(modInstallElement);
      return;
    }

    modInstallElement.action = "Uploading";
    modInstallElement.modIcon = validation.modicon;
    modInstallElement.modName = validation.modname;

    let path: string = await invoke("package_mod_for_publish", {});
    console.log(await GetToken());
    let modTunnelID = await POST(
      "mod/publish",
      {
        Token: await GetToken(),
        ID: updatingID,
      },
      false,
    );
    console.log(modTunnelID);

    await invoke("upload_file_chunks", {
      inputFile: path,
      chunkSizeMb: 5,
      serverUrl: serverLink,
      tunnelId: modTunnelID.body,
    });

    await remove(path);

    await unmount(modInstallElement);

    await alert(
      "Upload Request Sent Successfully! You'll get a message when the mod is published.",
    );

    window.open("#/modmarket", "_self");
  }

  async function VerifyMod() {}

  onMount(async () => {
    if (false) {
      alert("You are on an outdated version! Please update to upload mods.");
      window.open("#/", "_self");
      //return
    }
    if (GetData("moduploadid") != null) {
      updatingID = GetData("moduploadid");
      SetData("moduploadid", null);
    }
  });
</script>

<h1>Upload Mod</h1>

<hr />
<div>
  {#if updatingID != null}
    <span>Updating Mod: {updatingID}</span>
    <p></p>
  {/if}
  <button class="inputfile" onclick={UploadMod}
    >Upload Mod Archive (.zip/.tar/.7z/.rar)</button
  >
</div>

<style>
  .inputfile {
    margin: auto;
    display: block;
    width: 500px;
    height: 500px;
    text-align: center;
    background-color: rgb(52, 52, 52);
    border: 1px solid gray;
    border-radius: 30px;
  }

  .dropdown {
    appearance: none;
    -webkit-appearance: none;
    background-color: black;
    padding: 2px;
  }
</style>

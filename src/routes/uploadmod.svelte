<script lang="ts">
  import { mount, onMount, unmount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { POST, securitySettings, serverLink } from "./library/networking";
  import { GetData, SetData } from "./library/datatransfer";
  import ModInstall from "./components/ModInstall.svelte";
  import { remove } from "@tauri-apps/plugin-fs";
  import { loggedInAccount } from "./library/account";
  import Captcha from "./components/captcha.svelte";

  let updatingID = null;
  let captcha: Captcha;
  async function UploadMod() {
    let token = "";

    if (captcha != null) {
      token = captcha.GetToken();
      console.log(token);
      if (token == "") {
        await alert("Please complete the captcha before proceeding.");
        return;
      }
    }

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
    let modTunnelID = await POST(
      "mod/publish",
      {
        Token: loggedInAccount.token,
        ID: updatingID,
        Captcha: token,
      },
      false,
    );

    console.log(modTunnelID);

    if (modTunnelID.error) {
      await remove(path);
      await unmount(modInstallElement);
      window.open("#/modmarket", "_self");
      return;
    }

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

  onMount(async () => {
    if (loggedInAccount == null) {
      alert("You are not logged into an account!");
      window.open("#/", "_self");
      return;
    }

    if (false) {
      alert("You are on an outdated version! Please update to upload mods.");
      window.open("#/", "_self");
      return;
    }
    if (GetData("moduploadid") != null) {
      updatingID = GetData("moduploadid");
      SetData("moduploadid", null);
    }
  });
</script>

<h1>Upload Mod</h1>

<hr />
<div style="display:flex; flex-direction: column;">
  {#if updatingID != null}
    <span>Updating Mod: {updatingID}</span>
    <p></p>
  {/if}
  {#if securitySettings.ModUploadRequiresCaptcha}
    <Captcha bind:this={captcha}></Captcha>
  {/if}
  <button class="inputfile" onclick={UploadMod}
    >Upload Mod Archive (.zip/.tar/.7z/.rar)</button
  >
</div>

<style>
  .inputfile {
    width: 400px;
    height: 100px;
    background-color: rgb(52, 52, 52);
    border: 1px solid gray;
    border-radius: 30px;
  }
</style>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    GetId,
    GetToken,
    POST,
    UploadMod,
    outdated,
  } from "./library/networking";
  import { open } from "@tauri-apps/api/dialog";
  import ModInstall from "./components/ModInstall.svelte";
  import { onMount } from "svelte";
  import { GetData, SetData } from "./library/datatransfer";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { Invoke, Subscribe } from "./library/callback";
  import Loading from "./components/loading.svelte";

  let gitRepositoryURL: string;

  let replacingMod: string;
  let automaticallyPublish: HTMLInputElement;
  let uploadModDiv: HTMLDivElement;
  let waitDiv: HTMLDivElement;
  let resultDiv: HTMLDivElement;
  let largeMod: HTMLDivElement;

  function modIsLarge() {
    uploadModDiv.style.display = "none";
    largeMod.style.display = "block";
  }

  onMount(async () => {
    if (false) {
      await alert(
        "You are on an outdated version! Please update to upload mods.",
      );
      window.open("#/", "_self");
      //return
    }

    replacingMod = GetData("modupload_id");
    SetData("modupload_id", "");

    let cb = async () => {
      let id = await GetId();
      let request = await POST("getenterrequest", { id: id });
      if (request.body.modid != "") {
        setTimeout(() => {
          console.log(request);
          SetData("modpage_id", request.body.modid.toString());
          window.open("#/modpage", "_self");
        }, 1000);
      } else {
        setTimeout(cb, 1000);
      }
    };

    Subscribe(
      "onModUpload",
      (id: any) => {
        setTimeout(() => {
          SetData("modpage_id", id);
          window.open("#/modpage", "_self");
        }, 1000);
      },
      false,
    );
  });

  let files: FileList;

  $: if (files) {
    let file = files[0];

    if (file.name.endsWith(".zip") || file.name.endsWith(".tar")) {
      uploadFile(file.name);
    }
  }
  function dropFile() {}

  let inputlink: HTMLInputElement;

  let result = "Success!";

  async function UploadLink() {
    let token = await GetToken();

    let modInstallElement = new ModInstall({
      target: document.body,
    });
    modInstallElement.action = "Downloading";
    modInstallElement.modIcon = "img/icon.png";
    modInstallElement.modName = "your mod for verification purposes.";

    invoke("validate_mod", { url: inputlink.value, local: false }).then(
      async (v: any) => {
        if (v.validated) {
          modInstallElement.action = "Uploading";
          modInstallElement.modIcon = "img/icon.png";
          modInstallElement.modName = v.modname;
          await POST("moduploadnonhosted", {
            token: token,
            link: inputlink.value,
            replacing: replacingMod,
            extension: v.extension,
            automaticPublish: automaticallyPublish.value,
          });
          replacingMod = null;
          modInstallElement.$destroy();
          Invoke("onModUploadRequest", {});
          result =
            "Mod Request Successful! You will be redirected to your mod once the server has validated and published it...";
          waitDiv.style.display = "none";
          largeMod.style.display = "none";
          resultDiv.style.display = "block";
        } else {
          modInstallElement.$destroy();
          await alert("Mod Request Failed!");
        }
      },
    );
  }

  async function SelectFile() {
    const selectedPath = await open({
      title: "Select folder",
      directory: false,
      multiple: false,
      filters: [
        {
          name: "Supported Archives",
          extensions: ["zip", "tar", "7z"],
        },
      ],
    });

    let path = selectedPath.toString();

    if (path != "") {
      uploadFile(path);
    }
  }

  async function uploadFile(path: string) {
    let data: any = await invoke("validate_archive", { path: path });

    if (!data.under_limit) {
      modIsLarge();
    } else {
      uploadModDiv.style.display = "none";
      waitDiv.style.display = "block";

      //we need to make a file class since multer doesn't allow normal bytes

      let binary = await readBinaryFile(path);

      let file = new File([binary], "mod", {
        type: "application/octet-stream",
      });

      UploadMod(
        file,
        replacingMod,
        data.extension,
        Boolean(automaticallyPublish.value),
      );
      replacingMod = null;
    }
  }
</script>

<h1>Upload Mod</h1>

<hr />

<div style="" bind:this={uploadModDiv}>
  <div></div>
  <div bind:this={waitDiv} style="display:none;"><h1>Please Wait...</h1></div>
  <div bind:this={resultDiv} style="display:none;">
    <h1>{result}</h1>
    <Loading></Loading>
  </div>

  <div bind:this={uploadModDiv}>
    <br />
    <span>Git Repository URL: </span>
    <input
      bind:value={gitRepositoryURL}
      placeholder="https://goobergit.xyz/supermod.git"
    /> <button>Verify</button> <span style="font-size:10px;">Success!</span>
    <br />
    <span>Git Branch</span><select>
      <option id="master">master</option><option> </option></select
    >
    <p>
      <button>Upload</button>
      <span style="font-size:10px;"
        >EML will locally verify this repository by downloading it and checking
        if all required files are present.
      </span>
    </p>
    <p>
      <a href="https://emldocs.kalsvik.no">Guide</a>
    </p>
  </div>
</div>
<input
  on:drop={dropFile}
  bind:files
  id="fileupload"
  style="display:none;"
  type="file"
/>

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
</style>

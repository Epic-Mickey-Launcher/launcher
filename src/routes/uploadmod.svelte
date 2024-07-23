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
  let gitVerifiedRepositoryURL: string;
  let gitBranches: any = [];
  let gitVerified: boolean;

  let modVerified: boolean;
  let modIconData: string;
  let modName: string;
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

  async function getBranches() {
    let res = await POST("git/branches", { GitUrl: gitRepositoryURL }, false);
    gitVerifiedRepositoryURL = gitRepositoryURL;
    if (res.error) {
      gitVerified = false;
      gitVerifiedRepositoryURL = "";
      return;
    }
    gitBranches = res.body.split(" ");
    gitVerified = true;
  }

  async function UploadMod() {
    let result = await POST("mod/publish", {
      GitRepositoryUrl: gitVerifiedRepositoryURL,
      Name: modName,
      Token: await GetToken(),
      Publish: true,
    });
    if (result.error) return;
  }

  async function VerifyMod() {
    let validationInfo: any = await invoke("validate_mod", {
      url: gitVerifiedRepositoryURL,
      local: false,
    });
    modVerified = validationInfo.validated;
    modIconData = validationInfo.modicon;
    modName = validationInfo.modname;
  }

  onMount(async () => {
    if (false) {
      await alert(
        "You are on an outdated version! Please update to upload mods.",
      );
      window.open("#/", "_self");
      //return
    }
  });
</script>

<h1>Upload Mod</h1>

<hr />

<div style="" bind:this={uploadModDiv}>
  <div></div>
  <div bind:this={waitDiv} style="display:none;"><h1>Please Wait...</h1></div>
  <div bind:this={resultDiv} style="display:none;">
    <Loading></Loading>
  </div>

  <div bind:this={uploadModDiv}>
    <br />
    {#if !modVerified}
      <span>Git Repository URL: </span>
      <input
        bind:value={gitRepositoryURL}
        on:input={() => (gitVerified = false)}
        placeholder="https://goobergit.xyz/supermod.git"
      />
      {#if gitVerified}<span style="font-size:10px;">Success!</span>{/if}
      <br />
      {#if gitVerified}
        <span>Git Branch: </span><select class="dropdown">
          {#each gitBranches as branch}
            <option id={branch}>{branch}</option>
          {/each}
        </select>
      {/if}

      <p></p>
      {#if !gitVerified}
        <button on:click={getBranches}>Verify</button>
      {:else}
        <button on:click={VerifyMod}>Verify Locally then Upload</button>
        <span style="font-size:10px;"
          >EML will locally verify this repository by downloading it and
          checking if all required files are present.
        </span>
      {/if}
    {:else}
      <img src={modIconData} style="width:128px" alt="" />
      <span
        style="font-size:40px;position: relative;bottom:60px;margin-left:5px;"
        >{modName}</span
      >
      <br />
      <span>Ready to upload.</span>
      <br />
      <span>De-list upon upload: </span><input type="checkbox" />

      <br />
      <span
        >I have read the Mod Publishing Guidelines and accept its terms:
      </span><input type="checkbox" />
      <br />
      <button on:click={UploadMod}>Publish</button>
    {/if}
    <p></p>
    <p>
      <a href="https://emldocs.kalsvik.no">Guide</a>
    </p>
  </div>
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

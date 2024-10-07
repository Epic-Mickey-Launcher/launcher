<svelte:options accessors={true} />

<script lang="ts">
  import { GetData, SetData } from "../library/datatransfer";
  import { InternetModToUnifiedMod } from "../library/gameid";
  import { GetImagePath, ImageType, POST } from "../library/networking";
  import { GameConfig, Mod } from "../library/types";
  import User from "./User.svelte";
  import DownloadMod from "./downloadMod.svelte";

  export let gameData: GameConfig;
  export let modData: Mod;
  let downloadStatus = "Download";
  export let modNodeDiv: HTMLDivElement;
  let downloadMod: DownloadMod;
  $: innerWidth = 0;
  let downloadButton: HTMLButtonElement;

  let downloadButtonDisabled = false;
  let visible = true;
  let comments = [];
  let likes = 0;
  let description = "";
  let loaded = false;

  function ViewPage() {
    SetData("modpage_id", modData.ID);
    SetData("modpage_dumploc", gameData.path);
    window.open("#/modpage", "_self");
  }

  export async function Init() {
    downloadMod = new DownloadMod({
      target: modNodeDiv,
    });

    downloadMod.Initialize(gameData, false, InternetModToUnifiedMod(modData));
    downloadMod.updatecb = () => {
      downloadButtonDisabled = downloadMod.downloadButtonDisabled;
      downloadStatus = downloadMod.downloadButtonStatus;
    };

    let response = await POST("comment/count", { PageID: modData.ID }, false);
    if (response.error) return;
    comments = response.body;

    likes = modData.CachedLikes;

    let len = 0;

    for (let i = 0; i < innerWidth; i += 30) {
      len += 1;
    }

    description = modData.Description;

    if (modData.Description.length > len) {
      let newDesc = modData.Description.substring(0, len).trim();
      newDesc += "...";
      description = newDesc;
    }

    loaded = true;
  }

  async function Download() {
    await downloadMod.Download();
  }
</script>

<svelte:window bind:innerWidth />
{#if visible}
  <div bind:this={modNodeDiv} class="modNodeDiv">
    {#if loaded}
      <div>
        <span
          class="spanHyperLink"
          on:click={ViewPage}
          style="font-weight:bold;text-overflow: ellipsis;">{modData.Name}</span
        >

        {#if modData.Version > 0}
          <span
            style="padding:1px 4px;background-color: rgba(0,0,0,0.7);margin-left:5px;border-radius:4px;font-size:12px;"
            >v{modData.Version}</span
          >
        {/if}
        <p>
          <User ID={modData.Author} textSize={14} imageSize={15}></User>
        </p>
        <p>
          <span style="text-overflow: ellipsis;">{description}</span>
        </p>
      </div>

      <div class="imgArea">
        <img
          class="modNodeImg"
          alt=""
          src={GetImagePath(modData.ID, ImageType.Mod, false)}
        />
        <br />
        <span style="font-size:8px;">Likes: {likes}</span>
        <span style="font-size:8px;">Downloads: {modData.Downloads}</span>
        <span style="font-size:8px;">Comments: {comments}</span>
        <button
          bind:this={downloadButton}
          on:click={Download}
          disabled={downloadButtonDisabled}>{downloadStatus}</button
        >
        <br />
      </div>
    {/if}
  </div>
{/if}

<style>
  .modNodeDiv {
    position: relative;
    flex-wrap: wrap;
    z-index: 1;
    background-color: rgb(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    border-radius: 20px;
    padding: 10px 10px;
    width: 50%;
    height: 100px;
    display: flex;
    margin-right: auto;
    margin-left: auto;
    margin-bottom: 20px;
    box-shadow: 2px 2px 10px rgb(0, 0, 0);
    transition-duration: 0.3s;
    opacity: 0;
    transform: translateX(-100px);
  }

  .imgArea {
    position: absolute;
    right: 5px;
    display: inline;
    margin-left: auto;
    text-align: right;
    justify-content: right;
  }
  .modNodeImg {
    width: 80px;
    height: 80px;
    border-radius: 10px;
  }
</style>

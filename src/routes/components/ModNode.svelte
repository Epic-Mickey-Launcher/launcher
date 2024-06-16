<svelte:options accessors={true} />

<script lang="ts">
  import { ReadJSON } from "../library/configfiles";
  import { GetData, SetData } from "../library/datatransfer";
  import { POST } from "../library/networking";
  import DownloadMod from "./downloadMod.svelte";

  export let modName = "";
  export let description = "";
  export let iconLink = "";
  export let author = "";
  export let modid = "";
  export let likes = 0;
  export let comments = 0;
  export let downloads = 0;
  export let visible = true;
  let authoraccountexists = true;
  export let authorname = "";
  export let gamedata;
  export let moddata;
  let downloadStatus = "Download";
  let modNodeDiv: HTMLDivElement;
  let downloadMod: DownloadMod;
  $: innerWidth = 0;
  let color = "white";
  let canupdate = false;
  let downloadButton: HTMLButtonElement;
  async function SetJsonData() {
    let jsonData = await ReadJSON("games.json");

    return jsonData;
  }
  function ViewPage() {
    SetData("modpage_id", modid);
    SetData("modpage_dumploc", gamedata.path);
    window.open("#/modpage", "_self");
  }

  export async function Init() {
    downloadMod = new DownloadMod({
      target: modNodeDiv,
    });

    let gameinfo = GetData("gameinfo");
    console.log(gameinfo);

    downloadMod.Initialize(gameinfo, false, moddata);
    downloadMod.updatecb = () => {
      downloadButton.disabled = downloadMod.downloadButtonDisabled;
      downloadStatus = downloadMod.downloadButtonStatus;
    };

    let response = await POST("user/username", { id: author }, false);
    if (response.error) return;
    authorname = response.body;

    response = await POST("comment/count", { PageID: modid }, false);
    if (response.error) return;
    comments = response.body;

    likes = moddata.CachedLikes;

    let len = 0;

    for (let i = 0; i < innerWidth; i += 30) {
      len += 1;
    }

    if (description.length > len) {
      let newDesc = description.substring(0, len);
      newDesc += "...";
      description = newDesc;
    }
  }

  async function OpenProfileOfAuthor() {
    if (!authoraccountexists) return;
    SetData("profile_id", author);
    window.open("#/profilepage", "_self");
  }

  async function Download() {
    await downloadMod.Download();
  }
</script>

<svelte:window bind:innerWidth />
{#if visible}
  <div bind:this={modNodeDiv} class="modNodeDiv">
    <div>
      <span
        class="spanHyperLink"
        on:click={ViewPage}
        style="font-weight:bold;text-overflow: ellipsis;">{modName}</span
      >
      <p>
        <span>
          Author:<button
            style="margin-left:5px;color:{color};text-overflow: ellipsis;"
            on:click={OpenProfileOfAuthor}
            class="hyperlinkbutton">{authorname}</button
          >
        </span>
      </p>
      <p>
        <span style="text-overflow: ellipsis;">{description}</span>
      </p>
    </div>

    <div class="imgArea">
      <img class="modNodeImg" alt="" src={iconLink} />
      <br />
      <span style="font-size:8px;">Likes: {likes}</span>
      <span style="font-size:8px;">Downloads: {downloads}</span>
      <span style="font-size:8px;">Comments: {comments}</span>
      <button bind:this={downloadButton} on:click={Download}
        >{downloadStatus}</button
      >
      <br />
    </div>
  </div>
{/if}

<style>
  .break {
    flex-basis: 100%;
    height: 0;
  }

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
    transition-duration: 0.1s;
  }

  .modNodeDiv:hover {
    transform: scale(1.01);
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

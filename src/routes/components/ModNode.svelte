<script lang="ts">
  import { SetData } from "../library/datatransfer";
  import { InternetModToUnifiedMod } from "../library/gameid";
  import { GetImagePath, ImageType, POST } from "../library/networking";
  import { type Mod, ModState } from "../library/types";
  import User from "./User.svelte";
  import DownloadMod from "./downloadMod.svelte";
  import type { GameInstance } from "../library/instance.svelte";
  import { activeInstance, SetActiveGameInstance } from "../library/config";
  import { mount, unmount } from "svelte";
  import ModInstall from "./ModInstall.svelte";
  import { loggedInAccount } from "../library/account";

  let downloadStatus = $state("Download");

  interface Props {
    gameInstance: GameInstance;
    modData: Mod;
  }

  let modNodeDiv: HTMLDivElement = $state();

  let { gameInstance, modData = $bindable() }: Props = $props();
  let downloadMod: DownloadMod;
  let innerWidth = $state(0);

  let downloadButton: HTMLButtonElement = $state();

  let downloadButtonDisabled = $state(false);
  let visible = $state(true);
  let comments = $state([]);
  let likes = $state(0);
  let description = $state("");
  let loaded = $state(false);

  function ViewPage() {
    SetData("modpage_id", modData.ID);
    SetActiveGameInstance(gameInstance);
    window.open("#/modpage", "_self");
  }

  export async function Unload() {
    //lmao, fix.
    console.log("unloading");
    visible = false;
  }

  export async function Load() {
    console.log("Loading Mod: " + modData.Name);
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

    await UpdateState();

    loaded = true;
  }

  async function UpdateState() {
    let modState = await activeInstance.CheckMod(
      InternetModToUnifiedMod(modData),
    );
    if (modState == ModState.Installed) {
      downloadStatus = "Installed";
      downloadButtonDisabled = true;
    } else if (modState == ModState.Incompatible) {
      downloadStatus = "Incompatible";
      downloadButtonDisabled = true;
    } else if (modState == ModState.UpdateAvailable) {
      downloadStatus = "Update";
      downloadButtonDisabled = false;
    } else {
      downloadStatus = "Download";
      downloadButtonDisabled = false;
    }
  }

  async function Download() {
    let modInstallElement = mount(ModInstall, {
      target: document.body,
    });
    modInstallElement.modIcon = GetImagePath(modData.ID, ImageType.Mod);
    modInstallElement.modName = modData.Name;
    modInstallElement.action = "Downloading";
    modInstallElement.description = "This might take a while...";
    await gameInstance.AddMod(InternetModToUnifiedMod(modData));
    await UpdateState();
    await unmount(modInstallElement);
  }

  export { modData, modNodeDiv };
</script>

<svelte:window bind:innerWidth />
{#if visible}
  <div bind:this={modNodeDiv} class="modNodeDiv">
    {#if loaded}
      <div>
        <button
          class="hyperlinkbutton"
          onclick={ViewPage}
          style="font-weight:bold;text-overflow: ellipsis;color:{!modData.Verified ||
          !modData.Published
            ? 'yellow'
            : 'white'};">{modData.Name}</button
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
          alt="Mod Icon"
          src={GetImagePath(modData.ID, ImageType.Mod, false)}
        />
        <br />

        {#if loggedInAccount != null}
          {#if !modData.Verified}
            <img
              title="Manual Review Ongoing... Mod is not visible to public."
              alt="manual review ongoing"
              style="width:8px;padding-right: 5px;"
              src="img/manualreviewongoing.svg"
            />
          {/if}

          {#if !modData.Published}
            <img
              alt="not published"
              title="Mod is not published and is not visible to public."
              style="width:8px;padding-right: 5px;"
              src="img/notpublished.svg"
            />
          {/if}

          {#if modData.Author == loggedInAccount.id}
            <img
              alt="mod owner"
              title="You own this mod."
              style="width:16px;padding-right: 5px;"
              src="img/modowner.svg"
            />
          {/if}
        {/if}
        <span style="font-size:8px;">Likes: {likes}</span>
        <span style="font-size:8px;">Downloads: {modData.Downloads}</span>
        <span style="font-size:8px;">Comments: {comments}</span>
        <button
          bind:this={downloadButton}
          onclick={Download}
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

<script lang="ts">
  import { onMount } from "svelte";
  import { GetData } from "./library/datatransfer";
  import {
    GetId,
    GetImagePath,
    GetToken,
    ImageType,
    POST,
    loggedin,
  } from "./library/networking";
  import { GameConfig, Mod, Platform } from "./library/types";
  import { ReadFile, ReadJSON, WriteFile } from "./library/configfiles";
  import CommentNode from "./components/CommentNode.svelte";
  import { exists } from "@tauri-apps/plugin-fs";
  import Loading from "./components/loading.svelte";
  import DownloadMod from "./components/downloadMod.svelte";
  import { parse } from "marked";
  import DOMPurify from "dompurify";
  import User from "./components/User.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Commit from "./components/Commit.svelte";
  import { InternetModToUnifiedMod } from "./library/gameid";
  let commentInput: HTMLTextAreaElement;
  let update = false;
  let downloadButton: HTMLButtonElement;
  let id = "";
  let allRegions = [];
  let downloads = 0;
  let likes = 0;
  let liked = false;
  let removeCommentButton = false;
  let time = "";
  let youtubevideoembed: HTMLIFrameElement;
  let downloadMod: DownloadMod;
  let localid: string;
  let ownercontrols: HTMLDivElement;
  let commits = [];
  let commentsCount = 0;
  let comments = [];
  let hearticon: SVGSVGElement;
  let selectRegion = false;
  let youtubelink: string;
  let downloadStatus = "Download";
  let modPublished = true;
  let mainDiv: HTMLDivElement;
  let sendButton: HTMLButtonElement;
  let gameInfo: GameConfig;
  let modInfo: Mod;

  async function SetJsonData() {
    let jsonData = await ReadJSON("games.json");

    return jsonData;
  }

  onMount(async () => {
    Init();
  });

  //sux. merge these

  function FocusOutCommentInput() {
    if (commentInput.value.trim() != "") {
      sendButton.style.opacity = "1";
      sendButton.style.pointerEvents = "all";
      sendButton.style.marginTop = "32px";
      sendButton.style.zIndex = "32";
    } else {
      sendButton.style.pointerEvents = "none";
      sendButton.style.opacity = "0";
      sendButton.style.marginTop = "-16px";
    }
  }
  function OnCommentInput() {
    if (commentInput.value.trim() != "") {
      sendButton.style.opacity = "0.3";
      sendButton.style.pointerEvents = "none";
      sendButton.style.marginTop = "32px";
    } else {
      sendButton.style.pointerEvents = "none";
      sendButton.style.opacity = "0";
      sendButton.style.marginTop = "-16px";
    }
  }

  async function Init() {
    let modid = GetData("modpage_id");
    let id = await GetId();
    localid = id;
    let token = await GetToken();

    let res = await POST("mod/get", { id: modid, token: token });
    if (res.error) return;
    modInfo = res.body;

    res = await POST("mod/commits", { ID: modid });
    if (res.error) return;
    commits = res.body;

    res = await POST("like/liked", { Token: token, PageID: modid });
    if (res.error) return;
    liked = res.body.Liked;

    RefreshComments();

    downloads = modInfo.Downloads;
    likes = modInfo.CachedLikes;
    if (liked) {
      hearticon.style.fill = "red";
    }

    if (modInfo.Video != null && modInfo.Video != "") {
      youtubelink = "https://www.youtube.com/embed/" + modInfo.Video;
    }

    let d = new Date(parseInt(modInfo.ID));

    time = d.toLocaleString();

    modPublished = modInfo.Published;

    CheckIfDownloaded();

    downloadMod = new DownloadMod({
      target: mainDiv,
    });

    mainDiv.style.opacity = "1";
  }

  async function OpenCommitHistory() {
    commitHistory.showModal();
  }

  async function PostComment() {
    if (
      commentInput.value.trim().length > 0 &&
      commentInput.value.trim().length < 300
    ) {
      removeCommentButton = true;
      let token = await GetToken();
      let res = await POST(
        "comment/send",
        {
          Token: token,
          PageID: modInfo.ID,
          Content: commentInput.value.trim(),
        },
        false,
      );
      if (!res.error) {
        RefreshComments();
      }
      commentInput.value = "";
      FocusOutCommentInput();
      removeCommentButton = false;
    }
  }

  async function RefreshComments() {
    comments = [];

    let res = await POST("comment/query", { PageID: modInfo.ID });
    if (res.body != null) {
      commentsCount = res.body.length;
      comments = res.body;
    }
  }

  async function DeleteMod() {
    let confirmed = await confirm(
      "Are you sure you want to delete " + modInfo.Name + "?",
    );

    if (confirmed) {
      let id = await GetToken();
      let res = await POST("mod/delete", { Token: id, ID: modInfo.ID }, false);
      if (!res.error) {
        window.open("#/modmarket", "_self");
      }
    }
  }

  async function UpdateMod() {
    //todo: this just prints the current commit hash, not the remote one.
    let conf = await confirm(
      "This will update the mod to the latest commit (" +
        commits[0].Hash +
        ") and increment the version to " +
        (modInfo.Version + 1) +
        ". Are you sure?",
    );

    if (conf) {
      await alert(
        "This update might take a while depending on the size of your mod. We will send you a message when it's done.",
      );

      let res = await POST(
        "mod/update",
        {
          Token: await GetToken(),
          ID: modInfo.ID,
        },
        false,
      );

      if (res.error) return;
      Init();
    }
  }

  async function PublishMod() {
    let token = await GetToken();
    let res = await POST("publishmod", { token: token, id: modInfo.ID });
    if (res.body.finished === true) {
      window.open("#/modmarket", "_self");
    }
  }

  async function LikeMod() {
    if (!loggedin) {
      await alert("Please log in to like.");
      return;
    }
    let token = await GetToken();
    let response = await POST("like/add", {
      Token: token,
      PageID: modInfo.ID,
    });

    if (response.error) return;
    likes += response.body.Liked ? 1 : -1;
    if (response.body.Liked) {
      hearticon.style.fill = "red";
    } else {
      hearticon.style.fill = "white";
    }
  }

  async function OnPressDownload() {
    if (allRegions.length > 1) {
      selectRegion = true;
    } else {
      Download();
    }
  }

  async function ChangeUrl() {
    let url = await prompt("New URL:");

    if (!url.toLowerCase().startsWith("https://")) {
      await alert("URL must be https.");
      return;
    }

    let token = await GetToken();

    await POST("mod/changegit", {
      ID: modInfo.ID,
      Token: token,
      GitRepositoryUrl: url,
    });
  }

  async function Download(regionoverride = null) {
    if (regionoverride != null) {
      selectRegion = false;
      gameInfo = regionoverride;
      id = regionoverride.id;
    }

    downloadMod.Initialize(gameInfo, false, InternetModToUnifiedMod(modInfo));
    downloadMod.Download();
    downloadMod.updatecb = () => {
      CheckIfDownloaded();
    };
  }

  async function CheckInstall(path: string) {
    let file_exists = await exists(path + "/EMLMods.json");

    if (!file_exists) {
      await WriteFile("[]", path + "/EMLMods.json");
    }

    let dataStr = await ReadFile(path + "/EMLMods.json");

    let dataJson = JSON.parse(dataStr);
    let json = dataJson.find((r: { modid: any }) => r.modid == modInfo.ID);
    downloadStatus = "Download";
    if (json != null) {
      if (json.update != modInfo.Version) {
        return "update";
      } else {
        return "installed";
      }
    }
    return "none";
  }

  async function Report() {
    if (!loggedin) {
      await alert("Please log in to make a report.");
      return;
    }

    let conf = await confirm(
      "Are you sure you want to report " + modInfo.Name + "?",
    );
    if (conf) {
      let reportReason = await prompt("Report Reason: ");

      let res = await POST(
        "user/report",
        {
          Token: await GetToken(),
          TargetID: modInfo.ID,
          ReportReason: reportReason,
        },
        false,
      );

      if (res.error) return;

      await alert("Report has been sent to the Moderation Team!");
    }
  }

  async function CheckIfDownloaded() {
    let Gamesjson = await SetJsonData();

    let haveGame = false;

    let platform = modInfo.Platform;

    if (modInfo.Platform == null) {
      platform = Platform.Wii;
    }

    allRegions = Gamesjson.filter(
      (r: GameConfig) =>
        r.platform.toLowerCase() == platform.toLowerCase() &&
        r.game.toLowerCase() == modInfo.Game.toLowerCase(),
    );

    if (allRegions.length == 1) {
      gameInfo = allRegions[0];
      id = allRegions[0].id;
      haveGame = true;
    } else if (allRegions.length > 1) {
      haveGame = true;

      for (let i = 0; i < allRegions.length; i++) {
        let res = await CheckInstall(allRegions[i].path);
        allRegions[i].installed = res;
      }

      let allInstalled = allRegions.filter((r) => r.installed == "installed");
      if (allInstalled.length == allRegions.length) {
        downloadButton.disabled = true;
        downloadStatus = "Already Installed";
      }
    } else if (allRegions.length == 0) {
      downloadButton.disabled = true;
      downloadStatus = `${modInfo.Game} (${platform}) not installed!`;
    }

    if (haveGame && allRegions.length == 1) {
      let res = await CheckInstall(gameInfo.path);
      if (res == "update") {
        update = true;
        downloadStatus = "Update Available";
      } else if (res == "installed") {
        downloadButton.disabled = true;
        downloadStatus = "Already Installed";
      }
    }
  }

  function CloseRegion() {
    selectRegion = false;
  }

  let commitHistory: HTMLDialogElement;
</script>

<dialog style="width:50%;" bind:this={commitHistory}>
  <h1>Commit History</h1>
  <hr />
  <div
    style="width:80%;background-color: rgb(10, 10, 10);padding:10px;border-radius: 10px;margin:auto;overflow-y: scroll;height:600px;"
  >
    {#each commits as commit}
      <Commit
        timestamp={commit.Timestamp}
        content={commit.CommitContent}
        author={commit.Author}
        hash={commit.Hash.substring(32)}
      ></Commit>
    {/each}
  </div>
  <button on:click={() => commitHistory.close()}>Close</button>
</dialog>

{#if modInfo == null}
  <span style="margin-left:45%;">
    <Loading></Loading>
  </span>
{/if}

{#if modInfo != null}
  <div
    bind:this={mainDiv}
    style="display:flex;width:100%;height:100%;justify-content:center;transition-duration:0.3s;opacity:0;"
  >
    <div style="display:flex;flex-direction:column;gap:0px;">
      <img
        src={GetImagePath(modInfo.ID, ImageType.Mod, false)}
        alt=""
        style="border-radius:20px 20px 0 0;margin-right:20px;width:200px;height:200px;z-index:1;"
      />
      <br />
      <div style="filter:drop-shadow(0px 0px 10px black)">
        <div
          class="commentBox"
          style="width:200px;height:235px; background-color: rgb(20 20 20);border-radius: 0px 0px 10px 10px;position: relative;overflow-y:scroll;top:-20px;scrollbar-width: none;"
        >
          <div style="position:sticky;top:0px;">
            <textarea
              style="background-color: rgb(30 30 30);border: none;resize: none;border-bottom: 1px white solid;width:196px;"
              placeholder="Comment..."
              bind:this={commentInput}
              on:input={OnCommentInput}
              on:focusout={FocusOutCommentInput}
            ></textarea>
            {#if !removeCommentButton}
              <button
                bind:this={sendButton}
                on:click={PostComment}
                title="send"
                class="sendComment"
                ><img
                  src="img/send.svg"
                  alt=""
                  style="width:8px;margin:auto;z-index: 3;"
                /></button
              >
            {/if}
          </div>

          <div>
            {#each comments as comment}
              <CommentNode
                onDelete={RefreshComments}
                id={comment.Author}
                content={comment.Content}
                commentID={comment.ID}
              ></CommentNode>
            {/each}
          </div>
        </div>
      </div>
    </div>
    <div>
      <span style="font-size:30px;">{modInfo.Name}</span>
      <svg style="width:25px;height:25px;margin-left:10px;fill:white;"
        ><path
          d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
        /><path d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z" /></svg
      > <span style="margin-left: 3px;">{downloads}</span>
      <button
        title={liked ? "Remove Like" : "Add Like"}
        on:click={LikeMod}
        style="background:transparent;border:none;width:25px;height:25px;margin-left:10px;"
        ><svg
          bind:this={hearticon}
          width="25px"
          height="25px"
          style="fill:white;"
          ><path
            d="M12,23.462l-.866-.612C9.994,22.044,0,14.783,0,8.15A7.036,7.036,0,0,1,6.75.875,6.57,6.57,0,0,1,12,3.582,6.57,6.57,0,0,1,17.25.875,7.036,7.036,0,0,1,24,8.15c0,6.633-9.994,13.894-11.134,14.7ZM6.75,3.875A4.043,4.043,0,0,0,3,8.15c0,3.916,5.863,9.21,9,11.611,3.137-2.4,9-7.695,9-11.611a4.043,4.043,0,0,0-3.75-4.275A4.043,4.043,0,0,0,13.5,8.15h-3A4.043,4.043,0,0,0,6.75,3.875Z"
          /></svg
        ></button
      ><span style="margin-left: 10px;">{likes}</span><span
        style="margin-right:14px;"
      ></span>

      <button
        on:click={Report}
        style="border:none;background-color:transparent"
      >
        <img src="img/report.svg" style="width:25px;" alt="" />
      </button>
      <span style="margin-right:14px;"></span>
      <span
        title="This value is not indicative of the actual version of the mod, but instead how many times it's been updated."
        style="background-color:black;padding:3px;border-radius:5px;"
        >v{modInfo.Version}</span
      >
      <p>
        <User textSize={15} ID={modInfo.Author}></User>
        <span>| Published on: {time}</span> <br />
        <span
          >Repository URL:
          {#if modInfo.RepositoryUrl == "legacy"}
            None (Legacy Mod)
          {:else}
            <button
              on:click={() =>
                invoke("open_link", { url: modInfo.RepositoryUrl })}
              class="hyperlinkbutton">Link</button
            >
          {/if}
        </span>
        <br />
        <span
          >Latest Commit:

          {#if modInfo.RepositoryUrl == "legacy"}
            None (Legacy Mod)
          {:else if commits.length > 0}
            <button class="hyperlinkbutton" on:click={OpenCommitHistory}
              >{commits[0].Hash.substring(32)}</button
            >
          {/if}
        </span>
      </p>
      <p></p>
      <p>
        {#if youtubelink != null}
          <iframe
            title="YouTube Video"
            width="320"
            height="180"
            allow="fullscreen;"
            bind:this={youtubevideoembed}
            src={youtubelink}
          />
        {/if}
      </p>
      <p>
        <button bind:this={downloadButton} on:click={OnPressDownload}
          >{downloadStatus}</button
        >
        <button on:click={() => window.open("#/modmarket", "_self")}
          >Go back to Mod Market</button
        >
      </p>
      <p />
      {#if localid == modInfo.Author}
        <div bind:this={ownercontrols}>
          <button
            disabled={modInfo.RepositoryUrl == "legacy"}
            title={modInfo.RepositoryUrl == "legacy"
              ? "You must upgrade this mod to use Git before you can update."
              : ""}
            on:click={UpdateMod}>Update Mod</button
          >
          <button on:click={ChangeUrl}>Change Git Repository URL</button>
          <button on:click={DeleteMod}>Delete Mod</button>
        </div>
      {/if}
      <p></p>
      <div
        style="max-width:750px; word-wrap: break-word;padding:10px;border-radius:5px;background-color:rgb(20, 20, 20);overflow-y:scroll;max-height: 600px;position:relative;"
      >
        <span style="max-width:300px;"
          >{@html DOMPurify.sanitize(parse(modInfo.Description))}</span
        >
      </div>
    </div>
  </div>

  {#if !modPublished}
    <span
      style="display:flex;width:100;align-items:center;justify-content:center;"
    >
      <p>
        <span style="color:yellow;"
          >This mod has not been published yet and is only visible to you.</span
        >
        <button style="margin-left:10px;" on:click={PublishMod}>Publish</button>
      </p></span
    >
  {/if}
{/if}

{#if selectRegion}
  <div class="selectregion">
    <span style="position:relative;top:250px;">
      <span>Select region to download to</span>
      <p></p>
      <div
        style="background-color: rgb(38 38 38); width:20%; height:30%;  border-radius:10px; margin:auto; filter:drop-shadow(0 0 5px black)"
      >
        {#each allRegions as region}
          {#if region.installed != "installed"}
            <button style="width:100%;" on:click={() => Download(region)}
              >{region.region}</button
            >
          {:else}
            <button style="width:100%;" disabled>{region.region}</button>
          {/if}
          <br />
        {/each}
      </div>
      <p>
        <button on:click={() => CloseRegion()} class="hyperlinkbutton"
          >Back</button
        >
      </p></span
    >
  </div>
{/if}

<style>
  .commentBox::-webkit-scrollbar {
    width: 0px;
  }
  .sendComment {
    background-color: white;
    height: 16px;
    width: 16px;
    position: absolute;
    right: 5px;
    opacity: 0;
    pointer-events: none;
    top: -16px;
    border-radius: 100%;
    display: flex;
    justify-content: center;
    transition-duration: 0.4s;
  }
  .selectregion {
    width: 100%;
    height: 100%;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: rgba(0, 0, 0, 0.3);
    border-radius: 5px;
    -webkit-backdrop-filter: blur(10px);
    position: fixed;
    backdrop-filter: blur(10px);
    z-index: 500;
    align-items: center;
    align-self: center;
    text-align: center;
  }
</style>

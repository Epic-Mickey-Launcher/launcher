<script lang="ts">
  import { mount, onMount, unmount } from "svelte";
  import { GetData, SetData } from "./library/datatransfer";
  import {
    GetId,
    GetImagePath,
    GetToken,
    ImageType,
    loggedin,
    POST,
  } from "./library/networking";
  import {
    Game,
    type GameConfig,
    type Mod,
    ModState,
    Platform,
  } from "./library/types";
  import CommentNode from "./components/CommentNode.svelte";
  import Loading from "./components/loading.svelte";
  import DownloadMod from "./components/downloadMod.svelte";
  import { parse } from "marked";
  import DOMPurify from "dompurify";
  import User from "./components/User.svelte";
  import { activeInstance } from "./library/config";
  import { InternetModToUnifiedMod } from "./library/gameid";
  import ModInstall from "./components/ModInstall.svelte";
  import {
    GameInstance,
    GetAllInstancesWithSameGame,
  } from "./library/instance.svelte";

  let commentInput: HTMLTextAreaElement = $state();
  let update = false;
  let downloadButton: HTMLButtonElement = $state();
  let id = "";
  let allRegions = $state([]);
  let downloads = $state(0);
  let likes = $state(0);
  let liked = $state(false);
  let time = $state("");
  let youtubevideoembed: HTMLIFrameElement = $state();
  let downloadMod: DownloadMod;
  let localid: string = $state();
  let ownercontrols: HTMLDivElement = $state();
  let commentsCount = 0;
  let comments = $state([]);
  let hearticon: SVGSVGElement = $state();
  let selectRegion = $state(false);
  let youtubelink: string = $state();
  let downloadStatus = $state("Download");
  let modPublished = $state(true);
  let mainDiv: HTMLDivElement = $state();
  let sendButton: HTMLButtonElement = $state();
  let gameInfo: GameConfig;
  let modInfo: Mod = $state();
  let formattedDescription = $state();

  let compatibleInstances: GameInstance[] = $state([]);
  let selectedInstance: GameInstance = $state();

  onMount(async () => {
    await Init();
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
    modInfo = res.body as Mod;

    compatibleInstances = GetAllInstancesWithSameGame(
      modInfo.Game.toUpperCase() as Game,
      modInfo.Platform.toUpperCase() as Platform,
    );
    selectedInstance = activeInstance;
    if (
      (await selectedInstance.CheckMod(InternetModToUnifiedMod(modInfo))) ==
      ModState.Incompatible
    ) {
      if (compatibleInstances.length > 0) {
        selectedInstance = compatibleInstances[0];
      } else {
        await alert("You don't have a compatible game to install this mod to!");
        window.open("#/modmarket", "_self");
      }
    }

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

    let date = new Date(parseInt(modInfo.ID));
    time = date.toLocaleString();
    modPublished = modInfo.Published;

    await GetFormattedDescription();
    await CheckIfDownloaded();

    downloadMod = mount(DownloadMod, {
      target: mainDiv,
    });

    mainDiv.style.opacity = "1";
  }

  async function SelectNewInstance() {
    console.log("new instance selected: " + selectedInstance.uniqueID);
    await CheckIfDownloaded();
  }

  async function PostComment() {
    if (
      commentInput.value.trim().length > 0 &&
      commentInput.value.trim().length < 300
    ) {
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
        await RefreshComments();
      }
      commentInput.value = "";
      FocusOutCommentInput();
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
    SetData("moduploadid", modInfo.ID);
    window.open("#/uploadmod", "_self");
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
    await Download();
  }

  async function Download(regionoverride = null) {
    let modInstallElement = mount(ModInstall, {
      target: document.body,
    });
    modInstallElement.modIcon = GetImagePath(modInfo.ID, ImageType.Mod);
    modInstallElement.modName = modInfo.Name;
    modInstallElement.action = "Downloading";
    modInstallElement.description = "This might take a while...";
    await selectedInstance.AddMod(InternetModToUnifiedMod(modInfo));
    await CheckIfDownloaded();
    await unmount(modInstallElement);
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
      if (reportReason.trim() == "") return;
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
    let state = await selectedInstance.CheckMod(
      InternetModToUnifiedMod(modInfo),
    );
    if (state == ModState.NotInstalled) {
      downloadButton.textContent = "Download";
      downloadButton.disabled = false;
    } else if (state == ModState.UpdateAvailable) {
      downloadButton.textContent = "Update";
      downloadButton.disabled = false;
    } else if (state == ModState.Incompatible) {
      downloadButton.textContent = `Incompatible with ${selectedInstance.GetShortName(false)}`;
      downloadButton.disabled = true;
    } else if (state == ModState.Installed) {
      downloadButton.textContent = "Installed";
      downloadButton.disabled = true;
    }
  }

  async function GetFormattedDescription() {
    formattedDescription = DOMPurify.sanitize(await parse(modInfo.Description));
  }
</script>

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
              oninput={OnCommentInput}
              onfocusout={FocusOutCommentInput}
            ></textarea>

            <button
              bind:this={sendButton}
              onclick={PostComment}
              title="send"
              class="sendComment"
              ><img
                src="img/send.svg"
                alt=""
                style="width:8px;margin:auto;z-index: 3;"
              /></button
            >
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
      <svg style="width:25px;height:25px;margin-left:10px;fill:white;">
        <path
          d="M12.033,19.011a3.489,3.489,0,0,0,2.475-1.024l3.919-3.919-2.121-2.121-2.782,2.782L13.5,0l-3,0,.024,14.709L7.76,11.947,5.639,14.068l3.919,3.919A3.487,3.487,0,0,0,12.033,19.011Z"
        />
        <path d="M21,16v5H3V16H0v5a3,3,0,0,0,3,3H21a3,3,0,0,0,3-3V16Z" />
      </svg>
      <span style="margin-left: 3px;">{downloads}</span>
      <button
        title={liked ? "Remove Like" : "Add Like"}
        onclick={LikeMod}
        style="background:transparent;border:none;width:25px;height:25px;margin-left:10px;"
        aria-label={liked ? "Remove Like" : "Add Like"}
      >
        <svg
          bind:this={hearticon}
          width="25px"
          height="25px"
          style="fill:white;"
        >
          <path
            d="M12,23.462l-.866-.612C9.994,22.044,0,14.783,0,8.15A7.036,7.036,0,0,1,6.75.875,6.57,6.57,0,0,1,12,3.582,6.57,6.57,0,0,1,17.25.875,7.036,7.036,0,0,1,24,8.15c0,6.633-9.994,13.894-11.134,14.7ZM6.75,3.875A4.043,4.043,0,0,0,3,8.15c0,3.916,5.863,9.21,9,11.611,3.137-2.4,9-7.695,9-11.611a4.043,4.043,0,0,0-3.75-4.275A4.043,4.043,0,0,0,13.5,8.15h-3A4.043,4.043,0,0,0,6.75,3.875Z"
          />
        </svg>
      </button>
      <span style="margin-left: 10px;">{likes}</span><span
        style="margin-right:14px;"
      ></span>

      <button onclick={Report} style="border:none;background-color:transparent">
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
        <br />
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
          ></iframe>
        {/if}
      </p>
      <p>
        <button bind:this={downloadButton} onclick={OnPressDownload}
          >{downloadStatus}</button
        >

        <select
          style="color:black;"
          bind:value={selectedInstance}
          oninput={() => SelectNewInstance()}
        >
          {#each compatibleInstances as instance}
            <option selected={instance === activeInstance} value={instance}
              >{instance.gameConfig.uniqueID}</option
            >
          {/each}
        </select>
        <button onclick={() => window.open("#/modmarket", "_self")}
          >Go back to Mod Market
        </button>
      </p>
      {#if localid === modInfo.Author}
        <div
          bind:this={ownercontrols}
          style="background-color: rgb(21,21,21);padding:15px;border-radius: 5px;"
        >
          <span style="margin-right: 5px;color:yellow;">Owner Controls: </span>
          <button onclick={UpdateMod}>Update Mod </button>
          <button onclick={DeleteMod}>Delete Mod</button>
        </div>
      {/if}
      <p></p>
      <div
        style="max-width:750px; word-wrap: break-word;padding:10px;border-radius:5px;background-color:rgb(20, 20, 20);overflow-y:scroll;max-height: 600px;position:relative;"
      >
        <span style="max-width:300px;">{@html formattedDescription}</span>
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
        <button style="margin-left:10px;" onclick={PublishMod}>Publish</button>
      </p></span
    >
  {/if}
{/if}

<style>
  .commentBox::-webkit-scrollbar {
    width: 0;
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

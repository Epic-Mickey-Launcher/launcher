<svelte:options accessors={true} />

<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { getVersion } from "@tauri-apps/api/app";
  import {
    GET,
    GETEXT,
    SetLoggedIn,
    loggedin,
    SetOutdated,
    GetId,
    GetImagePath,
    ImageType,
    GetToken,
    POST,
    statusMessageLink,
  } from "../library/networking";
  import { Subscribe } from "../library/callback.js";
  import { invoke } from "@tauri-apps/api/tauri";
  import { SetData } from "../library/datatransfer";
  import { to_number } from "svelte/internal";
  import Loading from "./loading.svelte";
  let pfp: string;
  let serverNotice = false;
  let latestDownloadLink = "";
  let messagesList = [];
  let messagesCount = 0;
  let updateHyperLink: HTMLButtonElement;
  let connectionIssues: boolean;
  let version = "";
  let messagesOpen = false;

  let statusTextColor = "";
  let statusBannerColor = "";
  let statusText = "";
  let statusVisible = false;

  export async function ForceSetPFP(p: any) {
    pfp = p;
  }

  async function DeleteMessage(id: string) {
    drawMessages = false;
    let res = await POST(
      "user/deletemessage",
      {
        Token: await GetToken(),
        ID: id,
      },
      false,
    );
    if (res.body == null) return;
    let index = messagesList.findIndex((r) => r.ID == id);
    messagesCount--;
    messagesList.splice(index, 1);
    drawMessages = true;
  }

  interface MessageLinkElement {
    Value: string;
    Type: string;
    Range: number[];
  }

  function ParseMessage(message: string): MessageLinkElement[] {
    let declareType = 0;
    let declareValue = 0;
    let rangeIndex = [0, 0];

    let typeBuffer = "";
    let valueBuffer = "";
    let plainTextBuffer = "";

    let buffer = message;

    let elements: MessageLinkElement[] = [];

    for (let i = 0; i < buffer.length; i++) {
      let char = buffer[i];

      if (char == "(" && declareType == 0) {
        elements.push({
          Value: plainTextBuffer,
          Range: [],
          Type: "plain",
        });
        plainTextBuffer = "";
        declareType = 1;
        rangeIndex[0] = i;
        continue;
      }

      if (char == ")" && declareType == 1) {
        declareType = 2;
        continue;
      }

      if (char == "[" && declareType == 2) {
        declareValue = 1;
        continue;
      }

      if (char == "]" && declareType == 2 && declareValue == 1) {
        declareValue = 2;
        rangeIndex[1] = i;

        if (isNaN(parseFloat(valueBuffer))) {
          let err: MessageLinkElement[] = [
            {
              Value:
                "Security Warning! Element Value was not numeric! Please contact moderators!",
              Type: "plain",
              Range: [],
            },
          ];
          return err;
        }

        let element: MessageLinkElement = {
          Value: valueBuffer,
          Type: typeBuffer,
          Range: rangeIndex,
        };

        elements.push(element);

        declareType = 0;
        declareValue = 0;
        valueBuffer = "";
        typeBuffer = "";

        continue;
      }

      if (declareValue == 1) {
        valueBuffer += char;
      } else if (declareType == 1) {
        typeBuffer += char;
      } else if (declareType == 0 && declareValue == 0) {
        plainTextBuffer += char;
      }
    }

    elements.push({
      Value: plainTextBuffer,
      Range: [],
      Type: "plain",
    });

    return elements;
  }

  async function GetMessages() {
    if (!loggedin) return;
    drawMessages = false;
    let res = await POST("user/messages", { Token: await GetToken() });
    if (res.error) return;
    if (res.body != null) {
      messagesList = res.body;
    }
    drawMessages = true;
  }

  async function GetMessageCount() {
    if (!loggedin) return;
    if (messagesOpen) {
      return;
    }
    let res = await POST(
      "user/messagecount",
      { Token: await GetToken() },
      false,
    );
    setTimeout(() => {
      GetMessageCount();
    }, 10000);
    if (res.body == null || res.error) return;
    messagesCount = to_number(res.body);
  }

  let getMessagesRoutineStarted = false;

  onMount(async () => {
    version = await getVersion();
    let callbackOnEnterNewWindow = async () => {
      let res = await GET("server/ping");
      connectionIssues = res.error;

      if (connectionIssues) return;

      res = await GETEXT(statusMessageLink);

      statusText = res.message;
      statusBannerColor = res.bannerColor;
      statusTextColor = res.textColor;
      statusVisible = res.visible;
    };

    let cb = async () => {
      let id = await GetId();
      if (loggedin) {
        SetLoggedIn(true);
        pfp = GetImagePath(id, ImageType.User);

        if (!getMessagesRoutineStarted) {
          getMessagesRoutineStarted = true;
          GetMessageCount();
        }
      } else {
        SetLoggedIn(false);
        pfp = "img/loggedoutpfp.jpeg";
      }
    };

    // @ts-ignore
    Subscribe("SignedIn", cb, true);
    Subscribe("OnNewWindow", callbackOnEnterNewWindow, true);
    let info = await GETEXT(
      "https://api.github.com/repos/Epic-Mickey-Launcher/launcher/releases",
    );
    let info_stable = info.filter((r: { prerelease: any }) => !r.prerelease);
    let newest_release = info_stable[0];
    let current_version = await getVersion();
    if (newest_release.tag_name != current_version) {
      SetOutdated();
      latestDownloadLink = newest_release.html_url;
      updateHyperLink.style.display = "block";
    }
  });

  async function openMessage() {
    if (!loggedin) {
      await alert("You need an account to view messages.");
      return;
    }
    messagesOpen = true;
    messagesDiv.style.opacity = "1";
    messagesDiv.style.transform = "scale(1)";
    messagesDiv.style.pointerEvents = "all";
    GetMessages();
  }
  function closeMessage() {
    messagesOpen = false;
    messagesDiv.style.opacity = "0";
    messagesDiv.style.transform = "scale(0.5)";
    messagesDiv.style.pointerEvents = "none";
  }

  let drawMessages = true;
  let messagesDiv;

  export const HeaderVisible = writable(true);

  function OpenLatestDownloadPage() {
    invoke("open_link", { url: latestDownloadLink });
  }

  let header: HTMLDivElement;

  function OpenPage(page: string) {
    window.open("#/" + page, "_self");
  }

  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });

  async function PfpButton() {
    if (loggedin) {
      OpenPage("profilepage");
    } else {
      OpenPage("register");
    }
  }
</script>

<main>
  {#if statusVisible}
    <div
      style="width: 100%;background-color:{statusBannerColor};height:20px;position: relative;margin:auto;text-align:center;"
    >
      <span style="color:{statusTextColor};">{statusText}</span>
    </div>
  {/if}
  {#if HeaderVisible}
    <p />
    <div class="header" bind:this={header}>
      <img
        src="/img/eml.svg"
        alt=""
        title={version}
        style="width:300px;padding:5px 0px;position:relative;right:30px;"
      />

      <p style="margin-right:20px" />

      <button
        on:click={() => OpenPage("modmarket")}
        class="headerButton startheaderbuttons">Mod Market</button
      >
      <button on:click={() => OpenPage("games")} class="headerButton"
        >Games</button
      >

      <button
        on:click={() => OpenPage("settings")}
        class="headerButton endheaderbuttons">Settings</button
      >

      <button
        class="hyperlinkbutton"
        on:click={OpenLatestDownloadPage}
        bind:this={updateHyperLink}
        style="margin:auto 10px;color:lime;display:none;"
        >Update Available!</button
      >

      {#if connectionIssues}
        <img
          alt=""
          style="width:32px;margin-left:12px;"
          src="img/warning.svg"
          title="Cannot connect to online services!"
        />
      {/if}
      <div class="pfpbutton">
        <div style="position: relative;float:left;">
          <div bind:this={messagesDiv} class="messages">
            <button
              style="position:absolute;right:0px;border:none;background-color: transparent;font-size: 30px;"
              on:click={() => closeMessage()}>x</button
            >
            <h1 style="text-align: center;">Messages</h1>
            <hr />

            {#if drawMessages}
              {#if messagesList.length == 0}
                <span style="text-align: center;">
                  <h2>):</h2>
                  <h3>forever alone</h3>
                </span>
              {/if}
              {#each messagesList as msg}
                <div
                  style="width:90%;height:50px;background-color: rgba(10, 10, 10, 1);margin:auto;padding:5px;border-radius:5px;overflow-y:scroll;position:relative;margin-top:5px;"
                >
                  <span style="font-size: 12px;"
                    >{msg.From == "0" ? "Admin" : msg.From} | {new Date(
                      Number(msg.ID),
                    ).toLocaleString()}</span
                  >
                  <button
                    on:click={() => DeleteMessage(msg.ID)}
                    style="font-size: 7px;right:0px;top:0px;position: absolute;border-radius:0px 5px 0px 0px;"
                    >Delete</button
                  >
                  <br />
                  <span
                    style="font-size:10px;text-wrap:balance;overflow-wrap:break-word;line-height:2px;"
                    >{#each ParseMessage(msg.Content) as element}
                      {#if element.Type == "plain"}
                        {element.Value}
                      {:else if element.Type == "mod"}
                        <button
                          class="hyperlinkbutton"
                          style="font-size:15px;"
                          on:click={() => {
                            SetData("modpage_id", element.Value);
                            window.open("#/modpage", "_self");
                          }}
                          >{#await POST( "mod/get", { ID: element.Value }, ) then res}
                            {res.body.Name}
                          {/await}</button
                        >
                      {/if}
                    {/each}</span
                  >
                </div>
              {/each}
            {:else}
              <Loading></Loading>
            {/if}
          </div>
          <button
            class="notifications"
            style="background-color: transparent;border:none;margin-top: 20px;position: relative;"
            on:click={() => openMessage()}
          >
            <img
              src="img/notifications.svg"
              style="width:16px;margin-right: 12px;"
            />

            {#if messagesCount > 0}
              <div
                style="position:absolute;background-color: red;border-radius:100%;width:16px;height:16px;bottom:20px;right:9px;"
              >
                <span style="letter-spacing:-2px;"
                  >{messagesCount > 9 ? "+9" : messagesCount}</span
                >
              </div>
            {/if}
          </button>
        </div>
        <button
          on:click={() => PfpButton()}
          style="border:none;background-color: Transparent;"
          class="pfphover"
        >
          <img src={pfp} alt="" title="Sign Up" class="pfp" />
        </button>
      </div>
    </div>
    {#if serverNotice}
      <div
        style="width:90vw;text-align:center;padding:10px;background-color:yellow;margin:auto;border-radius:0px 0px 8px 8px;"
      >
        <span style="color:black"></span>
      </div>
    {/if}
    <p style="margin-bottom:60px" />
  {/if}
</main>

<style>
  .startheaderbuttons {
    border-radius: 10px 0 0 10px;
  }

  .endheaderbuttons {
    border-radius: 0 10px 10px 0;
  }
  .messages {
    width: 300px;
    height: 400px;
    position: absolute;
    background-color: rgba(20, 20, 20, 1);
    top: 10px;
    right: 10px;
    border-radius: 15px;
    z-index: 2;
    transition-duration: 0.25s;
    transform: scale(0.5);
    opacity: 0;
    pointer-events: none;
    overflow-y: scroll;
    transform-origin: top right;
  }
  .pfpbutton {
    margin: auto;
    display: flex;
    margin-right: 10px;
  }

  .pfp {
    pointer-events: none;
    position: relative;
    transition-duration: 0.3s;
    width: 50px;
    height: 50px;
    border-radius: 100%;
  }
  .notifications {
    transition-duration: 0.3s;
  }
  .notifications:hover {
    transform: scale(1.1);
  }
  .pfphover:hover .pfp {
    transform: scale(1.1);
  }

  .header {
    box-shadow: 2px 2px 10px rgb(0, 0, 0);
    border-radius: 10px;
    display: flex;
    z-index: 1;
    columns: 1rem 1rem;
    width: 100%;
    backdrop-filter: blur(2px);
    -webkit-backdrop-filter: blur(10px);
    padding: 5px 0px;
    justify-content: left;
  }
  .headerButton {
    width: 20%;
    border: none;
    backdrop-filter: blur(4px);
    background-color: rgb(0, 0, 0, 0.5);
    -webkit-backdrop-filter: blur(10px);
    transition-duration: 0.1s;
  }
  .headerButton:hover {
    background-color: rgb(43, 43, 43);
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { getVersion } from "@tauri-apps/api/app";
  import {
    GET,
    GETExternal,
    SetOutdated,
    GetImagePath,
    ImageType,
    POST,
    statusMessageLink,
    SetOfflineMode,
    offlineMode,
    outdated,
  } from "../library/networking";
  import { Subscribe } from "../library/callback.js";
  import { invoke } from "@tauri-apps/api/core";
  import { SetData } from "../library/datatransfer";
  import Loading from "./loading.svelte";
  import { loggedInAccount, LoginWithSession } from "../library/account";
  import { exit } from "@tauri-apps/plugin-process";
  let pfp: string = $state();
  let serverNotice = false;
  let latestDownloadLink = "";
  let messagesList = $state([]);
  let messagesCount = $state(0);
  let updateHyperLink: HTMLButtonElement = $state();
  let connectionIssues: boolean = $state();
  let version = $state("");
  let messagesOpen = false;
  let newVersion = $state("");
  let statusTextColor = $state("");
  let statusBannerColor = $state("");
  let statusText = $state("");
  let statusVisible = $state(false);
  let offlineModeState = $state(false);
  export async function SetVisible(visible: boolean) {
    console.log("Header Visible: " + visible);
    header.style.display = visible ? "flex" : "none";
  }

  async function DeleteMessage(id: string) {
    drawMessages = false;
    let res = await POST(
      "user/deletemessage",
      {
        Token: loggedInAccount.token,
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
    // it might be time to learn regex
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
    if (loggedInAccount == null) return;
    drawMessages = false;
    let res = await POST("user/messages", { Token: loggedInAccount.token });
    if (res.error) return;
    if (res.body != null) {
      messagesList = res.body;
    }
    drawMessages = true;
  }

  async function GetMessageCount() {
    if (loggedInAccount == null) return;
    if (messagesOpen) {
      return;
    }
    let res = await POST(
      "user/messagecount",
      { Token: loggedInAccount.token },
      false,
    );
    if (res.body == null || res.error) return;
    messagesCount = Number(res.body);

    setTimeout(async () => {
      await GetMessageCount();
    }, 10000);
  }

  let getMessagesRoutineStarted = false;

  onMount(async () => {
    version = await getVersion();
    let callbackOnEnterNewWindow = async () => {
      if (offlineMode) return;
      console.log("pinging server");
      try {
        await GET("server/ping");
      } catch {
        connectionIssues = true;
      }

      if (connectionIssues) {
        console.log("couldn't ping server");
        let setOfflineMode = await confirm(
          "The EML server failed to respond! This could either be because the server is down, or because you don't have internet. Do you want to enable Offline Mode or quit?",
        );

        if (setOfflineMode) {
          await alert(
            "Offline Mode has been enabled. To turn it off, you must restart EML.",
          );
          offlineModeState = true;
          SetOfflineMode();
          window.open("#/", "_self");
        } else {
          await exit(0);
        }
        return;
      }

      let res = await GETExternal(statusMessageLink);
      //todo: rough, remove this
      if (res.message.includes("FOR OUTDATED CLIENTS:") && outdated == false) {
        statusVisible = false;
        return;
      }
      statusText = res.message;
      statusBannerColor = res.bannerColor;
      statusTextColor = res.textColor;
      statusVisible = res.visible;
      await refreshAccount();
    };

    Subscribe("OnNewWindow", callbackOnEnterNewWindow, true);
    let info = await GETExternal(
      "https://api.github.com/repos/Epic-Mickey-Launcher/launcher/releases",
    );
    let info_stable = info.filter(
      (r: { prerelease: boolean }) => !r.prerelease,
    );
    let newest_release = info_stable[0];
    let current_version = await getVersion();
    if (newest_release.tag_name != current_version) {
      SetOutdated();
      latestDownloadLink = newest_release.html_url;
      updateHyperLink.style.display = "block";
      newVersion = newest_release.tag_name;
    }

    await refreshAccount();
  });
  async function refreshAccount() {
    if (loggedInAccount == null) {
      LoginWithSession(); // attempt login if not already logged in
    }

    if (loggedInAccount != null) {
      pfp = GetImagePath(loggedInAccount.id, ImageType.User);
      if (!getMessagesRoutineStarted) {
        getMessagesRoutineStarted = true;
        await GetMessageCount();
      }
    } else {
      pfp = "img/loggedoutpfp.jpeg";
    }
  }
  async function openMessage() {
    if (loggedInAccount == null) {
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

  let drawMessages = $state(true);
  let messagesDiv: HTMLDivElement = $state();

  function OpenLatestDownloadPage() {
    invoke("open_link", { url: latestDownloadLink });
  }

  let header: HTMLDivElement = $state();

  function OpenPage(page: string) {
    window.open("#/" + page, "_self");
  }

  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });

  async function PfpButton() {
    if (loggedInAccount != null) {
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

  <p></p>
  <div class="header" bind:this={header}>
    <img
      src="/img/eml.svg"
      alt="EML Logo"
      title={version}
      style="width:300px;padding:5px 0px;position:relative;right:30px;"
    />

    <p style="margin-right:20px"></p>

    {#if !offlineModeState}
      <button
        onclick={() => OpenPage("modmarket")}
        class="headerButton startheaderbuttons">Mods</button
      >
    {:else}
      <button
        disabled
        title="Cannot enter mod index when in offline mode!"
        style="color:red;"
        class="headerButton startheaderbuttons">Mods</button
      >
    {/if}

    <button onclick={() => OpenPage("games")} class="headerButton">Games</button
    >

    <button
      onclick={() => OpenPage("settings")}
      class="headerButton endheaderbuttons">Settings</button
    >

    <button
      class="hyperlinkbutton"
      onclick={OpenLatestDownloadPage}
      bind:this={updateHyperLink}
      style="margin:auto 10px;color:lime;display:none;"
      >Update Available!<br />({version}->{newVersion})</button
    >

    {#if connectionIssues || offlineMode}
      <img
        alt="Connection Warning"
        style="width:32px;margin-left:12px;"
        src="img/warning.svg"
        title={offlineMode
          ? "Offline Mode Enabled."
          : "Cannot connect to online services!"}
      />
    {/if}
    <div class="pfpbutton">
      <div style="position: relative;float:left;">
        <div bind:this={messagesDiv} class="messages">
          <button
            style="position:absolute;right:0px;border:none;background-color: transparent;font-size: 30px;"
            onclick={() => closeMessage()}>x</button
          >
          <h1 style="text-align: center;">Messages</h1>
          <hr />

          {#if drawMessages && !offlineModeState}
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
                  >{msg.From == "0" ? "System" : msg.From} | {new Date(
                    Number(msg.ID),
                  ).toLocaleString()}</span
                >
                <button
                  onclick={() => DeleteMessage(msg.ID)}
                  style="font-size: 7px;right:0px;top:0px;position: absolute;border-radius:0px 5px 0px 0px;"
                  >Delete</button
                >
                <br />
                <span
                  style="font-size:10px;text-wrap:balance;overflow-wrap:break-word;line-height:2px;"
                  >{#each ParseMessage(msg.Content) as element}
                    {#if element.Type === "plain"}
                      {element.Value}
                    {:else if element.Type === "mod"}
                      <button
                        class="hyperlinkbutton"
                        style="font-size:15px;"
                        onclick={() => {
                          SetData("modpage_id", element.Value);
                          window.open("#/modpage", "_self");
                        }}
                        >{#await POST("mod/get", { ID: element.Value }, true, true) then res}
                          {#if res.body != null}
                            {res.body.Name}
                          {:else}
                            Unknown
                          {/if}
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

        {#if !offlineModeState}
          <button
            class="notifications"
            style="background-color: transparent;border:none;margin-top: 20px;position: relative;"
            onclick={() => openMessage()}
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
        {/if}
      </div>
      {#if !offlineModeState}
        <button
          onclick={() => PfpButton()}
          style="border:none;background-color: Transparent;"
          class="pfphover"
        >
          <img src={pfp} alt="Sign Up" title="Sign Up" class="pfp" />
        </button>
      {/if}
    </div>
  </div>
  {#if serverNotice}
    <div
      style="width:90vw;text-align:center;padding:10px;background-color:yellow;margin:auto;border-radius:0px 0px 8px 8px;"
    >
      <span style="color:black"></span>
    </div>
  {/if}
  <p style="margin-bottom:60px"></p>
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
    scrollbar-width: none;
  }
  .pfpbutton {
    display: flex;
    margin: auto 10px auto auto;
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
    columns: 1rem 1;
    width: 100%;
    backdrop-filter: blur(2px);
    -webkit-backdrop-filter: blur(2px);
    padding: 5px 0;
    justify-content: left;
  }
  .headerButton {
    width: 20%;
    border: none;
    background-color: rgb(0, 0, 0, 0.5);
    transition-duration: 0.1s;
  }
  .headerButton:hover {
    background-color: rgb(43, 43, 43);
  }
</style>

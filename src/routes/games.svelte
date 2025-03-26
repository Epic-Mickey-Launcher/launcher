<script lang="ts">
  import OneTimeNotice from "./components/OneTimeNotice.svelte";
  import GameNode from "./components/GameNode.svelte";
  import { mount, onMount, unmount } from "svelte";
  import { GetGameIdentity } from "./library/gameid";
  import AddGameC from "./components/addgame.svelte";
  import { GameInstance } from "./library/instance.svelte";
  import { AddTrackedGame, GetLoadedGameInstances } from "./library/config";
  import { GetPath } from "./library/configfiles";
  import { open } from "@tauri-apps/plugin-dialog";
  import { ExtractISO } from "./library/isoextract";
  import type { IdentifyGameResult } from "./library/types";
  import ModInstall from "./components/ModInstall.svelte";
  import Dialog from "./components/dialog.svelte";

  let AddGameComponent;
  let addGameButton: HTMLDivElement = $state();
  let gameNodeDiv: HTMLDivElement = $state();
  let blackoutDiv: HTMLDivElement = $state();
  let bannerDiv: HTMLDivElement = $state();
  let hoveredGame = $state("EM1");
  let nodes = [];
  let addGameButtonDisabled = false;
  let noGamesNotice = $state(false);
  let OneTimeNoticeComponent = $state(OneTimeNotice);

  onMount(async () => {
    addGameButton.classList.toggle("expandgamebutton");
    await GetPath();
    let gameInstances = GetLoadedGameInstances();
    if (gameInstances.length == 0) {
      noGamesNotice = true;
    }
    gameInstances.forEach((dat: GameInstance) => {
      CreateNode(dat);
    });

    let delay = 0;
    nodes.forEach((card) => {
      setTimeout(() => {
        card.node.style.opacity = 1;
      }, delay * 1000);
      delay += 0.05;
    });
  });

  function DefaultAddGameButton() {
    if (!addGameButtonDisabled) return;
    addGameButton.classList.toggle("expandgamebutton");
    addGameButtonDisabled = false;
  }

  async function AddGameButton() {
    if (addGameButtonDisabled) return;
    addGameButton.classList.toggle("expandgamebutton");
    addGameButtonDisabled = true;
  }

  async function AddGame(isImage: boolean) {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Image",
          extensions: ["rvz", "iso", "wbfs"],
        },
      ],
      directory: !isImage,
    });

    let path = selected;

    if (path == null) return;

    let modInstallElement = mount(ModInstall, {
      target: document.body,
      props: {
        modIcon: isImage ? "img/dolphin.png" : "img/emicon.png",
        action: isImage
          ? "Extracting your game with Dolphin..."
          : "Adding your game...",
        modName: "",
        description: "This won't take long!",
      },
    });

    if (isImage) {
      path = await ExtractISO(
        selected,
        (identifiedGame: IdentifyGameResult) => {
          let gameIdentity = GetGameIdentity(identifiedGame.game);

          modInstallElement.action = "Adding";
          modInstallElement.modName = gameIdentity.name;
          modInstallElement.modIcon = gameIdentity.resources.iconImageUrl;
        },
      );
    }

    let card = await CreateNode(await AddTrackedGame(path));

    await unmount(modInstallElement);

    setTimeout(() => {
      card.node.style.opacity = 1;
    }, 100);
  }

  //todo: remove useless variables game, directory, platform
  async function CreateNode(instance: GameInstance) {
    let release = GetGameIdentity(instance.gameConfig.game);
    const element = mount(GameNode, {
      target: gameNodeDiv,
      props: {
        imgBackgroundURL: release.resources.cardImageUrl,
        imgLogoURL: release.resources.logoImageUrl,
        gameInstance: instance,
      },
    });
    nodes.push(element);
    return element;
  }
</script>

<AddGameC bind:this={AddGameComponent} />
<div bind:this={blackoutDiv} class="blackout"></div>
<div bind:this={bannerDiv} class="gamebanner">
  <img
    alt=""
    src="img/{hoveredGame}bannerfull.png"
    style="width:65vw;margin:auto;overflow:hidden;"
  />
</div>

<h1 style="text-align:center;filter:drop-shadow(0 0 4px black)">Games</h1>
<hr style="width:500px" />

<div style="display:flex;justify-content:center">
  <div bind:this={gameNodeDiv} class="gamegrid"></div>
</div>
<p style="margin-bottom:50px;display:flex;"></p>
<div style="display:flex;justify-content: center;">
  <div
    bind:this={addGameButton}
    class="addgamebutton expandgamebutton"
    onclick={AddGameButton}
    onkeydown={() => {}}
    onpointerleave={DefaultAddGameButton}
    role="button"
    tabindex="0"
  >
    <span style="position: absolute;top:3px;">+</span>
    <button class="addgamesubbutton" onclick={() => AddGame(true)}
      >Add Game via Image
    </button>
    <button class="addgamesubbutton" onclick={() => AddGame(false)}
      >Add Game via Files
    </button>
  </div>
</div>
{#if noGamesNotice}
  <p></p>
  <Dialog
    content={[
      "Quite lonely around here...<br/>Press the + Button to add a game!",
    ]}
  ></Dialog>
{/if}
<p></p>
<OneTimeNoticeComponent
  content="EML uses a Dolphin Config separate from your global one. To apply any changes, you must open Dolphin in EML config mode. You can do so by pressing CTRL + D now."
  id="dolphinconfig"
/>

<style>
  .addgamebutton {
    overflow: hidden;
    text-align: center;
    font-size: 20px;
    border-radius: 10px;
    width: 100px;
    height: 30px;
    border: 1px solid;
    border-color: rgb(138, 138, 138);
    background-color: rgb(82, 82, 82);
    transition-duration: 0.1s;
    align-items: center;
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .addgamebutton:hover {
    background-color: rgb(20 20 20);
  }

  .expandgamebutton {
    width: 150px;
    height: 60px;
    pointer-events: none;
  }

  .expandgamebutton:hover {
    background-color: rgb(82, 82, 82);
  }

  .expandgamebutton span {
    color: transparent;
  }

  .expandgamebutton .addgamesubbutton {
    color: white;
    display: block;
    pointer-events: all;
    background-color: rgb(55 55 56);
  }

  .addgamesubbutton {
    width: 150px;
    height: 50%;
    font-size: 12px;
    background-color: transparent;
    border: none;
    color: transparent;
    display: none;
  }

  .addgamesubbutton:hover {
    background-color: rgb(20 20 20);
  }

  .gamegrid {
    display: flex;
    justify-items: center;
    gap: 24px;
  }

  .gamebanner {
    overflow: hidden;
    align-items: center;
    position: fixed;
    width: 100vw;
    height: 100vh;
    z-index: -499;
    top: 0;
    mask: linear-gradient(
      90deg,
      rgba(255, 255, 255, 0) 0%,
      rgba(255, 255, 255, 0) 19%,
      rgba(255, 255, 255, 1) 38%,
      rgba(255, 255, 255, 1) 50%,
      rgba(255, 255, 255, 1) 62%,
      rgba(255, 255, 255, 0) 81%,
      rgba(255, 255, 255, 0) 100%
    );
    display: flex;
    justify-content: center;
    overflow: hidden;
    opacity: 0;
    transition: 0.5s;
    filter: brightness(2);
  }

  .blackout {
    overflow: hidden;
    transition: 1s;
    position: fixed;
    width: 100vw;
    height: 100vh;
    background-color: black;
    z-index: -500;
    opacity: 0;
    top: 0;
  }
</style>

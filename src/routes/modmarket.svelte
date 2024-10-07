<script lang="ts">
  import { GetToken, POST } from "./library/networking";
  import { onMount } from "svelte";
  import ModNode from "./components/ModNode.svelte";
  import { ReadJSON } from "./library/configfiles";
  import { SetData } from "./library/datatransfer.js";
  import Loading from "./components/loading.svelte";
  import Dialog from "./components/dialog.svelte";
  import { GetBackgroundModMarket } from "./library/background";
  import { invoke } from "@tauri-apps/api/core";
  import { Mod } from "./library/types";
  import { GetGameIdentity } from "./library/gameid";
  let load = true;
  let jsonData: any[];

  async function SetJsonData() {
    jsonData = await ReadJSON("games.json");
    return jsonData;
  }

  onMount(async () => {
    await SetJsonData();

    let bg = GetBackgroundModMarket();

    background.style.backgroundImage = `url(${bg.path})`;
    background_credits = bg.credits;
    if (jsonData[0] != null) {
      currentSelectedGame = jsonData[0];
      SetData("gameinfo", currentSelectedGame);
      await GetAllMods();
    } else {
      nogames = true;
    }
  });

  let chunks = [];
  let chunkindex = 1;
  let ModList: HTMLDivElement;
  let GamesDropdown: HTMLSelectElement;
  let filter: any;
  let filterDropdown: HTMLSelectElement;
  let selectedgamebuild: any;
  let currentSelectedGame: { game: any; platform: any };
  let nogames = false;
  let allspawnednodes = [];

  async function LoadModList(changePlatform = false, c = 1) {
    chunkindex = c;

    allspawnednodes.forEach((element) => {
      element.$destroy();
    });

    if (changePlatform) {
      SetData("gameinfo", selectedgamebuild);
      currentSelectedGame = selectedgamebuild;
    }

    await GetAllMods();
  }

  let search: HTMLInputElement;

  let featuredModId = "";
  let featuredModImage = "";

  function GoToFeaturedMod() {
    SetData("modpage_id", featuredModId);
    window.open("#/modpage", "_self");
  }

  async function Search() {
    await LoadModList(false, 1);
  }

  function IntToArray(int: number) {
    chunks = [];
    for (let i = 1; i < int + 1; i++) {
      chunks.push(i);
    }
  }

  async function GetAllMods() {
    load = true;
    let token = await GetToken();

    let d = {
      Game: currentSelectedGame.game,
      Platform: currentSelectedGame.platform,
      Token: token,
      PageIndex: chunkindex - 1,
      Order: filter,
      SearchQuery: search.value.toLowerCase(),
    };

    let data = await POST("mod/query", d);
    IntToArray(data.body.RawQuerySize);
    allspawnednodes = [];

    await data.body.ModObjs.forEach(async (e: Mod) => {
      let modNode = new ModNode({
        target: ModList,
      });

      modNode.modData = e;
      modNode.gameData = jsonData.find(
        (r: { game: any; platform: any }) =>
          r.game.toUpperCase() == currentSelectedGame.game.toUpperCase() &&
          r.platform.toUpperCase() ==
            currentSelectedGame.platform.toUpperCase(),
      );
      e.Platform = e.Platform.toUpperCase();
      e.Game = e.Game.toUpperCase();
      modNode.Init();

      allspawnednodes.push(modNode);
    });

    load = false;

    let delay = 0.03;

    allspawnednodes.forEach(async (node) => {
      setTimeout(async () => {
        if (node == null) {
          return;
        }
        node.modNodeDiv.style.opacity = 1;
        node.modNodeDiv.style.transform = "translateX(0px)";
      }, delay * 1000);
      delay += 0.03;
    });
  }

  let background: HTMLDivElement;
  let background_credits: string;
</script>

<div
  bind:this={background}
  style="background-attachment:fixed;position: fixed;width:100vw;height:100vh;top:0px;z-index:-1;background-image:url(img/backgrounds/back1.webp);background-position:center;background-size:cover;left:0px;"
>
  <span style="bottom:3px;position:fixed;font-size:10px;left:3px;"
    >{background_credits}</span
  >
</div>

{#if featuredModId != "" && !nogames}
  <span
    style="width:100%;display:flex;margin:auto;flex-direction:column;text-align:center;width:500px;"
  >
    <span class="featuredModText">Available Now!</span>
    <p>
      <img
        on:click={GoToFeaturedMod}
        class="featuredModBanner"
        alt=""
        style="border-radius:10px;filter: drop-shadow(1px 1px 4px black);"
        src={featuredModImage}
      />
    </p></span
  >
  <p />
{/if}

{#if !nogames}
  <div
    style="background-color: rgb(0, 0, 0, 0.6);backdrop-filter: blur(5px);-webkit-backdrop-filter: blur(5px);display:flex; width:70vw; justify-content:center;margin:auto;padding:10px;border-radius: 10px;"
  >
    <div class="dropdown">
      <select
        bind:value={selectedgamebuild}
        on:change={() => LoadModList(true, 1)}
        bind:this={GamesDropdown}
      >
        {#await SetJsonData()}
          <p>Loading Mod List...</p>
        {:then data}
          {#each data as gamebuild}
            <option value={gamebuild}>
              {GetGameIdentity(gamebuild.game).name +
                " (" +
                gamebuild.platform.toUpperCase() +
                ", " +
                gamebuild.region +
                ")"}
            </option>
          {/each}
        {/await}
      </select>
    </div>
    <div class="dropdown">
      <select
        bind:value={filter}
        on:change={() => LoadModList()}
        bind:this={filterDropdown}
      >
        <option value={0}>Newest</option>
        <option value={1}>Oldest</option>
        <option value={2}>Most Downloads</option>
        <option value={3}>Least Downloads</option>
        <option value={4}>Most Likes</option>
        <option value={5}>Least Likes</option>
      </select>
    </div>

    <input
      bind:this={search}
      on:change={() => Search()}
      placeholder="Search"
      style="border:none;border-radius:3px;background-color:black;border:1px white solid;padding:3px;"
    />
    <button
      style="width:40px;height:40px;border:none;background:none;margin-left:10px;"
      on:click={() => window.open("#/uploadmod", "_self")}
      ><img src="img/upload.svg" style="width:20px;" /></button
    >
  </div>
{/if}
<p />
{#if load}
  {#if nogames}
    <div class="warning">
      <div style="position:relative;top:50vh;pointer-events:all;">
        <p>You don't have any game builds set up yet!</p>
        <p>
          <button
            on:click={() =>
              invoke("open_link", { url: "https://emldocs.kalsvik.no" })}
            class="hyperlinkbutton">Guide</button
          >
        </p>
      </div>
    </div>
  {:else}
    <span style="margin-left:45%;">
      <Loading></Loading>
    </span>
  {/if}
{:else if allspawnednodes.length == 0}
  <h1 style="text-align:center;">No mods could be found with your filters.</h1>
  <Dialog content={["Tip: Try to simplify your search to only one word."]}
  ></Dialog>
{/if}
<p></p>
<div style="margin-right:auto;margin-left:auto;" bind:this={ModList} />

<div style="display:flex;justify-content:center;">
  {#each chunks as num}
    {#if num == chunkindex}
      <button
        style="transform:scale(1.1);background-color:rgb(40, 40, 40);"
        on:click={() => LoadModList(false, num)}>{num}</button
      >
    {:else}
      <button on:click={() => LoadModList(false, num)}>{num}</button>
    {/if}
  {/each}
</div>

<div
  style="zindex:-2;display:flex;margin:auto;justify-content:center;width:100%;"
></div>

<style>
  .warning {
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    width: 100%;
    height: 100%;
    position: absolute;
    display: flex;
    justify-content: center;
    margin-top: auto;
    margin-bottom: auto;
    margin-right: auto;
    margin-left: auto;
    text-align: center;
    pointer-events: none;
  }

  .dropdown select {
    appearance: none;
    -webkit-appearance: none;
    background-color: black;
    padding: 10px;
  }

  .dropdown {
    position: relative;
    margin-right: 30px;
    color: black;
  }

  .featuredModBanner {
    transition-duration: 0.1s;
  }

  .featuredModBanner:hover {
    transform: scale(1.05);
  }
</style>

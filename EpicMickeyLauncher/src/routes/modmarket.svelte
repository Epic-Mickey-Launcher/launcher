<script>
   let jsonData;

   async function SetJsonData() {
      jsonData = await ReadJSON("games.json");

      return jsonData;
   }

   import {
      GET,
      POST,
      serverLink,
      staticAssetsLink,
   } from "./library/networking.js";
   import { onMount } from "svelte";
   import ModNode from "./components/ModNode.svelte";
   import ModsData from "./data/mods.json";
   import ModsDataEM2 from "./data/modsem2.json";
   import { ReadJSON } from "./library/configfiles.js";

   let warning;

   onMount(async () => {
      await SetJsonData();

      if (jsonData[0] != null) {
         currentSelectedGame = jsonData[0].game;
         await GetAllMods(currentSelectedGame);
      } else {
         warning.style.display = "block";
      }
   });

   let ModList;
   let GamesDropdown;
   let selectedgamebuild;
   let currentSelectedGame;

   let allspawnednodes = [];

   async function LoadModList() {
      allspawnednodes.forEach((element) => {
         element.$destroy();
      });

      currentSelectedGame = selectedgamebuild;

      await GetAllMods(selectedgamebuild);
   }

   async function GetAllMods(modlisttoget) {
      let data = await GET("getmods");

      data.modlist.forEach(async (e) => {
         if (e.game == currentSelectedGame) {
            let modNode = new ModNode({
               target: ModList,
            });

            modNode.modid = e.id;
            modNode.modName = e.name;
            modNode.moddataobj = e;
            modNode.iconLink = staticAssetsLink + e.icon;
            modNode.description = e.description;
            modNode.downloadLink = staticAssetsLink + e.download;
            modNode.author = e.author;
            modNode.gamedata = jsonData.find(
               (r) => r.game == currentSelectedGame
            );
            modNode.Init();

            allspawnednodes.push(modNode);
         }
      });
   }
</script>

<div
   style="display:flex; width:100%; justify-content:center;background-color: rgb(20, 20, 20);width:48%;margin:auto;padding:10px;border-radius: 20px 20px 0px 0px;"
>
   <select
      class="dropdown"
      bind:value={selectedgamebuild}
      on:change={() => LoadModList()}
      bind:this={GamesDropdown}
   >
      {#await SetJsonData()}
         <p>Loading Mod List...</p>
      {:then data}
         {#each data as gamebuild}
            <option value={gamebuild.game}>
               {gamebuild.game}
            </option>
         {/each}
      {/await}
   </select>
   <a href="#/uploadmod">Upload Mod</a>
   <input placeholder="Search" style="margin-left:30px;" />
</div>

<div style="margin-right:auto;margin-left:auto;" bind:this={ModList} />

<!--
TODO: add a limit to amount of mods that can be on one page and filter them through different chunks
<div style="display:flex;margin:auto;justify-content:center;width:100%;">
   <button style="margin-right:10px;" class="hyperlinkbutton">1</button>
   <button style="margin-right:10px;" class="hyperlinkbutton">2</button>
   <button style="margin-right:10px;" class="hyperlinkbutton">3</button>
</div>
-->

<div class="warning" bind:this={warning}>
   <p style="position:relative;top:400px;">
      You don't have any game builds set up yet!
   </p>
</div>

<style>
   .warning {
      display: none;
      z-index: 0;
      width: 100%;
      height: 100%;
      top: 0;
      bottom: 0;
      left: 0;
      right: 0;
      background-color: rgba(43, 15, 15, 0.3);
      border-radius: 5px;
      -webkit-backdrop-filter: blur(10px);
      position: fixed;
      backdrop-filter: blur(10px);
      margin-top: auto;
      margin-bottom: auto;
      margin-right: auto;
      margin-left: auto;
      text-align: center;
   }
   .dropdown {
      margin-right: 30px;
      background-color: black;
   }
</style>

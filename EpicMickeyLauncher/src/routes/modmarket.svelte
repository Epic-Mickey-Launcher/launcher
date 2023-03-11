<script>
   let jsonData;

   async function SetJsonData() {
      jsonData = await ReadJSON("games.json");

      return jsonData;
   }

   import { GET, POST } from "./library/networking.js";
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
      let data;
      if (modlisttoget == "EM1") {
         data = ModsData;
      } else {
         data = ModsDataEM2;
      }
      data.forEach(async (e) => {
         let modNode = new ModNode({
            target: ModList,
         });
         modNode.modName = e.title;
         modNode.iconLink = e.iconLink;
         modNode.description = e.description;
         modNode.downloadLink = e.downloadLink;
         modNode.author = e.author;
         modNode.gamedata = jsonData.find((r) => r.game == currentSelectedGame);
         modNode.Init();

         allspawnednodes.push(modNode);
      });
   }
</script>

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

<div style="margin-right:auto;margin-left:auto;" bind:this={ModList} />

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
      background-color: black;
   }
</style>

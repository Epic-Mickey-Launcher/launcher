<script>
   let jsonData;

   async function SetJsonData() {
      jsonData = await ReadJSON("games.json");

      return jsonData;
   }

   import {
      GET,
      GetToken,
      POST,
      serverLink,
      staticAssetsLink,
   } from "./library/networking.js";
   import { onMount } from "svelte";
   import ModNode from "./components/ModNode.svelte";
   import { GetFullName, ReadJSON } from "./library/configfiles.js";
   import { SetData } from "./library/datatransfer.js";
    import Loading from "./components/loading.svelte";
    import Dialog from "./components/dialog.svelte";
   let warning;
   let load = true;

   onMount(async () => {
      await SetJsonData();

      let json = await GET("trygetfeaturedmod");

      if (json.id != "") {
         featuredModImage = json.imglink;
         featuredModId = json.id;
      }

      if (jsonData[0] != null) {
         currentSelectedGame = jsonData[0];
         SetData("gameinfo", currentSelectedGame);
         await GetAllMods();
      } else {
         warning.style.display = "block";
      }
   });

   let chunks = [];
   let chunkindex = 1;
   let ModList;
   let GamesDropdown;
   let filter;
   let filterDropdown;
   let selectedgamebuild;
   let currentSelectedGame;


   let allspawnednodes = [];

   async function LoadModList(changePlatform = false, c = 1) {

      chunkindex = c;

      allspawnednodes.forEach((element) => {
         element.$destroy();
      });

      if(changePlatform)
      {
         SetData("gameinfo", selectedgamebuild);
      currentSelectedGame = selectedgamebuild;
      }

      await GetAllMods();
   }

   let search;

   let featuredModId = "";
   let featuredModImage = "";



   function GoToFeaturedMod() {
      SetData("modpage_id", featuredModId);
      window.open("#/modpage", "_self");
   }

   async function Search() {
      console.log("come")
     await LoadModList(false, 1);
   }

   function IntToArray(int)
   {
      chunks = []
      for(let i = 1; i < int + 1; i++)
      {
         chunks.push(i);
      }
   }

   async function GetAllMods() {
      load = true;
      let token = await GetToken();

      let d = { token: token, chunkindex:chunkindex - 1, filter:filter, game:currentSelectedGame.game, platform:currentSelectedGame.platform, inputfilter:search.value.toLowerCase() };

      let data = await POST("getmodchunk", d);

      allspawnednodes = [];

      let mods = data.chunk;
      IntToArray(data.chunks)

  
         await mods.forEach(async (e) => {
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
            modNode.update = e.update;
            modNode.modplatform = e.platform;
            modNode.modgame = e.game;
            modNode.downloads = e.downloads;
            modNode.likes = e.likes;
            modNode.comments = e.comments;

            modNode.moddata = e;
            modNode.json = JSON.stringify(e);
            modNode.gamedata = jsonData.find(
               (r) =>
                  r.game == currentSelectedGame.game &&
                  r.platform == currentSelectedGame.platform
            );
            modNode.Init();

            allspawnednodes.push(modNode);
         
      });

      load = false;
    

     

      
   }
</script>

{#if featuredModId != ""}
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

<div
   style="display:flex; width:70vw; justify-content:center;background-color: rgb(20, 20, 20);margin:auto;padding:10px;border-radius: 10px;"
>
   <select
      class="dropdown"
      bind:value={selectedgamebuild}
      on:change={() => LoadModList(true, 1)}
      bind:this={GamesDropdown}
   >
      {#await SetJsonData()}
         <p>Loading Mod List...</p>
      {:then data}
         {#each data as gamebuild}
            <option value={gamebuild}>
               {GetFullName(gamebuild.game) + " (" + gamebuild.platform.toUpperCase() + ", " + gamebuild.region + ")"}
            </option>
         {/each}
      {/await}
   </select>

   <select
   class="dropdown"
   bind:value={filter}
   on:change={() => LoadModList()}
   bind:this={filterDropdown}
>
    <option value={1}>Newest</option>
    <option value={2}>Oldest</option>
    <option value={3}>Most Downloads</option>
    <option value={4}>Least Downloads</option>
    <option value={5}>Most Likes</option>
    <option value={6}>Least Likes</option>
</select>

   <button on:click={() => window.open("#/uploadmod", "_self")}>Upload Mod</button>
   <input
      bind:this={search}
      on:change={() => Search()}
      placeholder="Search"
      style="margin-left:30px;border:none;border-radius:3px;"
   />
</div>
<p />
{#if load}
 <span style="margin-left:45%;">
   <Loading></Loading>
 </span>
 {:else if allspawnednodes.length == 0}
  <h1 style="text-align:center;">No mods could be found with your filters.</h1>
  <Dialog content="Tip: Try to simplify your search to only one word."></Dialog>
 {/if}
<p></p>
<div style="margin-right:auto;margin-left:auto;" bind:this={ModList} />

<div style="display:flex;justify-content:center;">
   {#each chunks as num}
   {#if num == chunkindex}
   <button style="transform:scale(1.1);background-color:rgb(40, 40, 40);" on:click={() => LoadModList(false, num)}>{num}</button>
   {:else}
   <button on:click={() => LoadModList(false, num)}>{num}</button>
   {/if}
   {/each}
</div>

<div style="display:flex;margin:auto;justify-content:center;width:100%;">

</div>


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
      background-color: white;
      color: black;
      background: none;
   }

   .featuredModBanner {
      transition-duration: 0.1s;
   }

   .featuredModBanner:hover {
      transform: scale(1.05);
   }
</style>

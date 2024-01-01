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
   import { ReadJSON } from "./library/configfiles.js";
   import { SetData } from "./library/datatransfer.js";
   let warning;

   onMount(async () => {
      await SetJsonData();

      let json = await GET("trygetfeaturedmod");

      if (json.id != "") {
         featuredModImage = json.imglink;
         featuredModId = json.id;
      }

      if (jsonData[0] != null) {
         currentSelectedGame = jsonData[0];
         await GetAllMods();
      } else {
         warning.style.display = "block";
      }
   });

   let ModList;
   let GamesDropdown;
   let filter;
   let filterDropdown;
   let selectedgamebuild;
   let currentSelectedGame;


   let allspawnednodes = [];

   async function LoadModList(changePlatform = false) {

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

   function Search() {
      allspawnednodes.forEach((e) => {
         let element = e;
         if (search.value != "") {
            element.visible = element.modName
               .toLowerCase()
               .includes(search.value.toLowerCase());
            if (!element.visible) {
               element.visible = element.authorname
                  .toLowerCase()
                  .includes(search.value.toLowerCase());
            }
         } else {
            element.visible = true;
         }
      });
   }

   async function GetAllMods() {
      let token = await GetToken();

      let data = await POST("getmods", { token: token });

      allspawnednodes = [];
      search.value = "";

      let mods = data.modlist;
      for (let i = 0; i < mods.length; i++) {
         let impressions = await POST("getmodimpressions", {
         token: token,
         mod: mods[i].id,
         });

         mods[i].likes = impressions.likes;
         mods[i].downloads = impressions.downloads;
      }
      let finalModList = [];

      console.log(filter);

      let cb = async () => {
         await finalModList.forEach(async (e) => {
         //HACK: dumb way of bypassing a db update

         let comparingPlatform = "wii";
         if (e.platform != null) {
            comparingPlatform = e.platform;
         }

         let platform = "wii";
         if (currentSelectedGame.platform != null) {
            platform = currentSelectedGame.platform;
         }

         if (
            e.game == currentSelectedGame.game &&
            comparingPlatform == platform
         ) {
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

            let comments = await POST("getcomments", { pageid: e.id });
            if(comments != null)
            {
               modNode.comments = comments.comments.length;
            }
            let impressions = await POST("getmodimpressions", {
            token: token,
            mod: e.id,
        });

            modNode.downloads = impressions.downloads;
            modNode.likes = impressions.likes;

      
            modNode.json = JSON.stringify(e);
            modNode.gamedata = jsonData.find(
               (r) =>
                  r.game == currentSelectedGame.game &&
                  r.platform == currentSelectedGame.platform
            );
            modNode.Init();

            allspawnednodes.push(modNode);
         }
      });
      }

      switch(filter)
      {

         case 5:
         finalModList =  mods.slice(0).sort(function (a,b) {
               return b.likes - a.likes;
            });
            break;

            case 6:
            finalModList =  mods.slice(0).sort(function (a,b) {
               return a.likes - b.likes;
            });
            break;

            case 4:
         finalModList =  mods.slice(0).sort(function (a,b) {
               return a.downloads - b.downloads;
            });
            break;

            case 3:
            finalModList =  mods.slice(0).sort(function (a,b) {
               return b.downloads - a.downloads;
            });
            break;

            case 1:
            finalModList =  mods.slice(0).sort(function (a,b) {
               return b.id - a.id;
            });
            break
            case 2:
            finalModList =  mods.slice(0).sort(function (a,b) {
               return a.id - b.id;
            })
            break;
      }
cb()
     

      
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
   style="display:flex; width:100%; justify-content:center;background-color: rgb(20, 20, 20);width:48%;margin:auto;padding:10px;border-radius: 10px;"
>
   <select
      class="dropdown"
      bind:value={selectedgamebuild}
      on:change={() => LoadModList(true)}
      bind:this={GamesDropdown}
   >
      {#await SetJsonData()}
         <p>Loading Mod List...</p>
      {:then data}
         {#each data as gamebuild}
            <option value={gamebuild}>
               {gamebuild.game + "(" + gamebuild.platform + ")"}
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
      on:input={Search}
      placeholder="Search"
      style="margin-left:30px;border:none;border-radius:3px;"
   />
</div>
<p />
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

   .featuredModBanner {
      transition-duration: 0.1s;
   }

   .featuredModBanner:hover {
      transform: scale(1.05);
   }
</style>

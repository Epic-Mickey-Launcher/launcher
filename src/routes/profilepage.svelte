<script lang="ts">
  import { onMount } from "svelte";
  import Userprofilemodnode from "./components/userprofilemodnode.svelte";
  import {
    GetId,
    GetImagePath,
    GetToken,
    ImageType,
    POST,
    loggedin,
  } from "./library/networking";
  import { GetData, SetData } from "./library/datatransfer";
  import { Subscribe } from "./library/callback";
  import Loading from "./components/loading.svelte";

  let profileDiv: HTMLElement;
  let isownerofprofile: boolean;
  let modNodeGroup: HTMLDivElement;
  let username = "";
  let bio = "";
  let pfplink = "";
  let profilepage: HTMLDivElement;
  let err: HTMLDivElement;
  let mods = [];
  let emblemName = "";
  let emblemColor = "";
  let profileinfo: {
    mods: any[];
    username: string;
    bio: string;
    id: string;
    emblems: any[];
  };
  let joindate = "";
  let modLength = 0;
  let loaded = false;

  onMount(async () => {
    let cb = async () => {
      loaded = false;
      let selfID = await GetId();
      let idofprofile = await GetData("profile_id");
      SetData("profile_id", undefined);
      if (idofprofile == undefined) {
        console.log("loading own profile " + selfID);
        idofprofile = selfID;

        if (!loggedin) {
          profilepage.style.display = "none";
          err.style.display = "block";
          return;
        }
      }

      let request = await POST(
        "user/username",
        {
          id: idofprofile,
        },
        false,
      );
      if (request.error) return;
      username = request.body;

      request = await POST(
        "user/bio",
        {
          id: idofprofile,
        },
        false,
      );
      if (request.error) return;
      bio = request.body;

      pfplink = GetImagePath(idofprofile, ImageType.User);

      isownerofprofile = selfID == idofprofile;

      let timestamp = parseInt(idofprofile);
      let d = new Date(timestamp);
      joindate = d.toLocaleString();

      loaded = true;
      profileDiv.style.opacity = "1";

      let query = await POST("mod/query", {
        AuthorID: idofprofile,
        Token: await GetToken(),
      });
      if (query.body.ModObjs != null) {
        mods = query.body.ModObjs;
        modLength = mods.length;
      } else {
        modLength = 0;
      }
    };

    Subscribe("SignedIn", cb, false);
  });
</script>

{#if !loaded}
  <span style="position: absolute; top:45%; left:45%;">
    <Loading></Loading>
  </span>
{/if}
<main bind:this={profileDiv} style="opacity: 0;">
  <div bind:this={profilepage} style="text-align:center;">
    <img class="pfp" src={pfplink + "?"} alt="" />
    <br />
    <span style="font-size:30px;">{username}</span>
    <br />
    <span style="font-size:10px;">EML Member since {joindate}</span>
    <p>
      <span>{bio}</span>
    </p>
    {#if emblemName != ""}
      <div
        style="border: 2px solid {emblemColor};width:120px;margin:auto;border-radius:30px;"
      >
        <p style="color:{emblemColor};">{emblemName}</p>
      </div>
    {/if}
    <p />

    {#if modLength > 0}
      <hr />
      <span style="font-size:30px;">Mods</span>
      <p></p>
      <div style="width:100%;display:flex;justify-content:center;">
        <div
          bind:this={modNodeGroup}
          style="display:grid;justify-content:center;gap:5px;width:100%;grid-template-columns: repeat(5, 0fr);"
        >
          {#each mods as mod}
            <Userprofilemodnode {mod}></Userprofilemodnode>
          {/each}
        </div>
      </div>
    {/if}

    <p>
      {#if isownerofprofile}
        <button
          on:click={() => window.open("#/accountsettings", "_self")}
          class="hyperlinkbutton">Edit Profile</button
        >
      {/if}
    </p>
  </div>

  <div bind:this={err} style="display:none;">
    <h2>You do not have an account.</h2>
  </div>
</main>

<style>
  * {
    transition-duration: 0.3s;
  }

  .pfp {
    z-index: 20;
    border-radius: 100%;
    width: 200px;
  }
</style>

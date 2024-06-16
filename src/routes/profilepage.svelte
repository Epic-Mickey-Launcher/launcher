<script lang="ts">
  import { onMount } from "svelte";
  import Userprofilemodnode from "./components/userprofilemodnode.svelte";
  import {
    GetId,
    GetImagePath,
    ImageType,
    POST,
    loggedin,
  } from "./library/networking";
  import { GetData } from "./library/datatransfer";
  import { Subscribe } from "./library/callback";
  import Loading from "./components/loading.svelte";

  let isownerofprofile: boolean;
  let modNodeGroup: HTMLDivElement;

  let username = "";
  let bio = "";
  let pfplink = "";

  let profilepage: HTMLDivElement;
  let err: HTMLDivElement;

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
  let modLength = 1;
  let loaded = false;

  onMount(async () => {
    let cb = async () => {
      loaded = false;

      let selfID = await GetId();

      let idofprofile = await GetData("profile_id");

      if (idofprofile == undefined) {
        console.log("loading own profile " + selfID);
        idofprofile = selfID;

        if (!loggedin) {
          profilepage.style.display = "none";
          err.style.display = "block";
          return;
        }
      }

      console.log(idofprofile);

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

      if (false) {
        modLength = profileinfo.mods.length;

        username = profileinfo.username;
        bio = profileinfo.bio;
        pfplink = GetImagePath(profileinfo.id, ImageType.User);

        let emblem = profileinfo.emblems.sort(
          (a: { weight: number }, b: { weight: number }) => {
            return b.weight - a.weight;
          },
        )[0];

        if (emblemName != null) {
          emblemName = emblem.emblemname;
          emblemColor = emblem.color;
        }

        profileinfo.mods.forEach(
          (m: { description: any; name: any; id: string }) => {
            let desc = m.description;

            if (desc.length > 70) {
              desc = desc.substring(0, 70) + "...";
            }

            new Userprofilemodnode({
              target: modNodeGroup,
              props: {
                name: m.name,
                description: desc,
                id: m.id,
                modicon: GetImagePath(m.id, ImageType.Mod),
              },
            });
          },
        );
      }
    };
    Subscribe("SignedIn", cb, false);
  });
</script>

{#if !loaded}
  <span style="margin-left:45%;">
    <Loading></Loading>
  </span>
{/if}

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
      <div style="width:100vw;overflow-x:auto;">
        <div
          bind:this={modNodeGroup}
          style="display:flex;justify-content:left;gap:5px;"
        ></div>
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

<style>
  .pfp {
    z-index: 20;
    border-radius: 100%;
    width: 200px;
  }
</style>

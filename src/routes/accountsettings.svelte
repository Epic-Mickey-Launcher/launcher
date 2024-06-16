<script lang="ts">
  import { onMount } from "svelte";
  import {
    GetId,
    GetImagePath,
    GetToken,
    ImageType,
    MultipartPOST,
    OnSignedIn,
    POST,
    SetLoggedIn,
    loggedin,
  } from "./library/networking";
  import { WriteToken } from "./library/configfiles";

  let username: HTMLInputElement;
  let password: HTMLInputElement;
  let email: HTMLInputElement;
  let bio: HTMLTextAreaElement;
  let pfpUrl: string = "";

  let files: any;
  $: if (files) {
    let file = files[0];
    GetPfpData(file);
  }

  async function ApplyChanges() {
    if (loggedin) {
      console.log(await GetToken());
      let response = await POST(
        "user/set/email",
        { token: await GetToken(), email: email.value },
        false,
      );
      if (response.error) {
        //todo: something probably
      }
    }
  }

  async function Logout() {
    WriteToken("");
    SetLoggedIn(false);
    window.open("#/", "_self");
  }

  async function DeleteAccount() {
    let confirmation = await confirm(
      "Are you sure you want to delete your account? Any comments, mods & likes from your account will remain unless they are manually deleted.",
    );
    if (confirmation) {
      let token = await GetToken();
      let res = await POST("deleteacc", { token: token });

      if (!res.error) {
        WriteToken("");
        window.open("#/", "_self");
      }
    }
  }

  async function GetPfpData(file: File) {
    let token = await GetToken();
    let response = await MultipartPOST("user/set/pfp", {
      image: file,
      token: token,
    });
    if (!response.error) window.open("#/accountsettings", "_self");
  }

  onMount(async () => {
    let id = await GetId();
    pfpUrl = GetImagePath(id, ImageType.User);
    OnSignedIn(async () => {
      let response = await POST("user/username", { ID: id }, false);
      if (response.error) return;
      username.value = response.body;
      response = await POST("user/bio", { ID: id }, false);
      if (response.error) return;
      bio.value = response.body;
    });
  });
</script>

<span>Change Username:</span>
<input bind:this={username} placeholder="Jimbob83" />
<p>
  <span style="display:flex;">
    <span style="margin:auto 0;">Change Bio:</span>
    <textarea cols="30" bind:this={bio} placeholder="i like video games" />
  </span>
</p>
<p>
  <span>Change Password:</span>
  <input placeholder="New Password" bind:this={password} type="password" />
</p>

<p>
  <span>Change E-Mail</span>
  <input placeholder="jimbob83@yahoo.com" bind:this={email} />
</p>

<p>
  <span>Upload a new profile picture: </span>
  <input accept="image/*" bind:files type="file" />
  <img src={pfpUrl} alt="" style="width:30px;margin-bottom:-10px;" />
</p>
<p>
  <button on:click={Logout}>Log Out</button>
</p>
<p />
<button on:click={DeleteAccount}>Delete Account</button>
<p />
<button style="width:30%;" on:click={ApplyChanges}>Apply Changes</button>

<script lang="ts">
  import { run } from "svelte/legacy";

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
    ClearInMemoryToken,
  } from "./library/networking";
  import { WriteToken } from "./library/configfiles";

  let currentUsername = "";

  let username: HTMLInputElement = $state();
  let password: HTMLInputElement = $state();
  let email: HTMLInputElement = $state();
  let bio: HTMLTextAreaElement = $state();
  let pfpUrl: string = $state("");

  let files: any = $state();

  async function ApplyChanges() {
    if (loggedin) {
      if (email.value.trim() != "") {
        await POST(
          "user/set/email",
          { token: await GetToken(), email: email.value },
          false,
        );
      }

      if (
        username.value.trim() != "" &&
        username.value.trim() != currentUsername
      ) {
        await POST(
          "user/set/username",
          { Token: await GetToken(), Username: username.value },
          false,
        );
      }

      if (bio.value.trim() != "") {
        await POST(
          "user/set/bio",
          { Token: await GetToken(), Bio: bio.value },
          false,
        );
      }

      if (password.value.trim() != "") {
        await POST(
          "user/set/password",
          { Token: await GetToken(), Password: password.value },
          false,
        );
      }
    }

    await alert("All changes applied!");
    window.open("#/profilepage", "_self");
  }

  async function Logout() {
    WriteToken("");
    ClearInMemoryToken();
    SetLoggedIn(false);
    window.open("#/", "_self");
  }

  async function DeleteAccount() {
    let confirmation = await confirm(
      "Are you sure you want to delete your account? Any comments, mods & likes from your account will remain unless they are manually deleted.",
    );
    if (confirmation) {
      let token = await GetToken();
      let res = await POST("user/delete", { Token: token }, false);

      if (!res.error) {
        WriteToken("");
        window.open("#/", "_self");
      }
    }
  }

  async function SaveMailOptions() {
    let token = await GetToken();
    await POST(
      "user/set/email/options",
      {
        Token: token,
        Messages: optionMessages,
      },
      false,
    );

    emailOptions.close();
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
    let token = await GetToken();

    pfpUrl = GetImagePath(id, ImageType.User);
    OnSignedIn(async () => {
      let response = await POST("user/username", { ID: id }, false);
      if (response.error) return;
      username.value = response.body;
      currentUsername = response.body;
      response = await POST("user/bio", { ID: id }, false);
      if (response.error) return;
      bio.value = response.body;

      response = await POST("user/email", { Token: token }, false);
      if (response.error) return;
      if (response.body == "") return;
      placeholderEmail = response.body;
    });
  });
  let emailOptions = $state();
  let optionMessages: string = $state();
  let placeholderEmail = $state("jimbob83@yahoo.com");
  run(() => {
    if (files) {
      let file = files[0];
      GetPfpData(file);
    }
  });
</script>

<span>Change Username:</span>
<input bind:this={username} placeholder="Jimbob83" />
<p>
  <span style="display:flex;">
    <span style="margin:auto 0;">Change Bio:</span>
    <textarea cols="30" bind:this={bio} placeholder="i like video games"
    ></textarea>
  </span>
</p>
<p>
  <span>Change Password:</span>
  <input placeholder="••••••••••••••" bind:this={password} type="password" />
</p>

<p>
  <span>Change E-Mail</span>
  <input placeholder={placeholderEmail} bind:this={email} /><button
    onclick={() => emailOptions.showModal()}>...</button
  >
</p>

<p>
  <span>Upload a new profile picture: </span>
  <input accept="image/*" bind:files type="file" />
  <img src={pfpUrl} alt="" style="width:30px;margin-bottom:-10px;" />
</p>
<p>
  <button onclick={Logout}>Log Out</button>
</p>
<p></p>
<button onclick={DeleteAccount}>Delete Account</button>
<p></p>
<button style="width:30%;" onclick={ApplyChanges}>Apply Changes</button>

<dialog bind:this={emailOptions}>
  <p>
    <span>Messages: </span>
    <select bind:value={optionMessages}>
      <option value="2">Send All</option>
      <option value="1">Send All (excl. system messages)</option>
      <option value="0">Don't send any</option>
    </select>
  </p>
  <button onclick={SaveMailOptions}>Save</button>
</dialog>

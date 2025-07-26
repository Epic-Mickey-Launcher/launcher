<script lang="ts">
  import {
    GetImagePath,
    ImageType,
    MultipartPOST,
    POST,
  } from "./library/networking";
  import { onMount } from "svelte";
  import { loggedInAccount, LoginWithSession, Logout } from "./library/account";

  let currentUsername = "";

  let username: HTMLInputElement = $state();
  let password: HTMLInputElement = $state();
  let email: HTMLInputElement = $state();
  let bio: HTMLTextAreaElement = $state();
  let pfpUrl: string = $state("");
  let emailVerifiedState = $state(null);
  let emailOptions: HTMLDialogElement = $state();
  let optionMessages: string = $state();
  let placeholderEmail: string = $state("jimbob83@yahoo.com");

  let files: any = $state();

  async function ApplyChanges() {
    if (loggedInAccount != null) {
      if (email.value.trim() != "") {
        let response = await POST(
          "user/set/email",
          { token: loggedInAccount.token, email: email.value },
          false,
        );
        if (!response.error) {
          await alert(
            "Check your E-Mail for a confirmation link to complete the process.",
          );
        }
      }

      if (
        username.value.trim() != "" &&
        username.value.trim() != currentUsername
      ) {
        await POST(
          "user/set/username",
          { Token: loggedInAccount.token, Username: username.value },
          false,
        );
      }

      if (bio.value.trim() != "") {
        await POST(
          "user/set/bio",
          { Token: loggedInAccount.token, Bio: bio.value },
          false,
        );
      }

      if (password.value.trim() != "") {
        await POST(
          "user/set/password",
          { Token: loggedInAccount.token, Password: password.value },
          false,
        );
      }
    }

    await LoginWithSession(); // relogin to apply potentially new loggedInAccount info
    await alert("All changes applied!");
    window.open("#/profilepage", "_self");
  }

  async function SignOut() {
    Logout();
    window.open("#/", "_self");
  }

  async function DeleteAccount() {
    let confirmation = await confirm(
      "Are you sure you want to delete your account? Any comments, mods & likes from your account will remain unless they are manually deleted.",
    );
    if (confirmation) {
      let token = loggedInAccount.token;
      let res = await POST("user/delete", { Token: token }, false);

      if (!res.error) {
        await SignOut();
      }
    }
  }

  async function SaveMailOptions() {
    let token = loggedInAccount.token;
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

  async function ResendEmailConfirm() {
    let token = loggedInAccount.token;
    await POST("user/email/resend", { Token: token });
    await alert("E-Mail resent!");
  }

  async function GetPfpData(file: File) {
    let token = loggedInAccount.token;
    let response = await MultipartPOST("user/set/pfp", {
      image: file,
      token: token,
    });
    if (!response.error) window.open("#/accountsettings", "_self");
  }

  onMount(async () => {
    if (loggedInAccount == null) {
      window.open("#/", "_self");
      return;
    }

    let id = loggedInAccount.id;
    let token = loggedInAccount.token;

    pfpUrl = GetImagePath(id, ImageType.User);
    let response = await POST("user/username", { ID: id }, false);
    if (response.error) return;
    username.value = response.body;
    currentUsername = response.body;
    response = await POST("user/bio", { ID: id }, false);
    if (response.error) return;
    bio.value = response.body;

    response = await POST("user/email/verified", { Token: token });
    if (response.error) return;
    emailVerifiedState = response.body;

    response = await POST("user/email", { Token: token }, false);
    if (response.error) return;
    if (response.body == "") return;
    placeholderEmail = response.body;
  });
  $effect(() => {
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
    <textarea bind:this={bio} cols="30" placeholder="i like video games"
    ></textarea>
  </span>
</p>
<p>
  <span>Change Password:</span>
  <input bind:this={password} placeholder="••••••••••••••" type="password" />
</p>

<p>
  <span>Change E-Mail:</span>
  {#if emailVerifiedState == null || emailVerifiedState.Verified === true}
    <input bind:this={email} placeholder={placeholderEmail} /><button
      onclick={() => emailOptions.showModal()}>...</button
    >
  {:else}
    <span
      style="background-color: yellow;color:black;padding:3px;border-radius: 3px;"
      >Waiting for confirmation of email: {emailVerifiedState.Email}
      <button
        onclick={ResendEmailConfirm}
        class="hyperlinkbutton"
        style="color:black;margin-left:7px;">Resend</button
      ></span
    >
  {/if}
</p>

<p>
  <span>Upload a new profile picture: </span>
  <input accept="image/*" bind:files type="file" />
  <img alt="PFP Preview" src={pfpUrl} style="width:30px;margin-bottom:-10px;" />
</p>
<p>
  <button onclick={SignOut}>Log Out</button>
</p>
<p></p>
<button onclick={DeleteAccount}>Delete Account</button>
<p></p>
<button onclick={ApplyChanges} style="width:30%;">Apply Changes</button>

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

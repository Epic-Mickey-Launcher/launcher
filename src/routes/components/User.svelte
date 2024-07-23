<svelte:options accessors />

<script lang="ts">
  import { onMount } from "svelte";
  import { GetImagePath, ImageType, POST } from "../library/networking";
  import { SetData } from "../library/datatransfer";
  export let ID: string;
  export let showPfp: boolean = true;
  export let showText: boolean = true;
  export let textSize: number = 12;
  export let imageSize: number = 12;
  let pfpUrl = "";
  let username = "";
  onMount(async () => {
    pfpUrl = GetImagePath(ID, ImageType.User, false);
    let res = await POST("user/username", { id: ID }, false, true);
    if (res.error) {
      username = "Unknown User";
      return;
    }
    username = res.body;
  });

  function OpenProfilePage() {
    if (username == "Unknown User") {
      return;
    }
    SetData("profile_id", ID);
    window.open("#/profilepage", "_self");
  }
</script>

{#if showText}
  <button
    class="hyperlinkbutton"
    on:click={OpenProfilePage}
    style="font-size: {textSize}px;">{username}</button
  >
{/if}

{#if showPfp}
  <img alt="" src={pfpUrl} style="border-radius: 100%; width:{imageSize}px;" />
{/if}

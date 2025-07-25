<script lang="ts">
  import { onMount } from "svelte";
  import { GetImagePath, ImageType, POST } from "../library/networking";
  import { SetData, cachedUsers } from "../library/datatransfer";
  interface Props {
    ID: string;
    showPfp?: boolean;
    showText?: boolean;
    textSize?: number;
    imageSize?: number;
  }

  let {
    ID,
    showPfp = true,
    showText = true,
    textSize = 12,
    imageSize = 12,
  }: Props = $props();
  let pfpUrl = $state("");
  let username = $state("");
  onMount(async () => {
    pfpUrl = GetImagePath(ID, ImageType.User, false);
    let cachedUser = cachedUsers.find((r) => r.ID == ID);
    if (cachedUser == null) {
      let res = await POST("user/username", { id: ID }, false, true);
      if (res.error) {
        username = "Unknown User";
        return;
      }
      username = res.body;
      cachedUsers.push({
        CachedPfp: "",
        Username: res.body,
        ID: ID,
      });
    } else {
      username = cachedUser.Username;
    }
  });

  function OpenProfilePage() {
    if (username == "Unknown User") {
      return;
    }
    SetData("profile_id", ID);
    window.open("#/profilepage", "_self");
  }

  export { ID, showPfp, showText, textSize, imageSize };
</script>

{#if showText}
  <button
    class="hyperlinkbutton"
    onclick={OpenProfilePage}
    style="font-size: {textSize}px;">{username}</button
  >
{/if}

{#if showPfp}
  <img
    alt="Profile Picture"
    src={pfpUrl}
    style="border-radius: 100%; width:{imageSize}px;"
  />
{/if}

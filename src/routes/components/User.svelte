<svelte:options accessors />

<script lang="ts">
  import { GetImagePath, ImageType } from "../library/networking";
  export let ID: string = "";
  export let showPfp: boolean = true;
  export let showText: boolean = true;
  export let textSize: number = 12;
  export let imageSize: number = 12;
</script>

{#if showText}
  <button class="hyperlinkbutton" style="font-size: {textSize}px;"
    >Kalsvik</button
  >
{/if}

{#if showPfp}
  <img
    alt=""
    src={GetImagePath(ID, ImageType.User, false)}
    style="border-radius: 100%; width:{imageSize}px;"
  />
{/if}

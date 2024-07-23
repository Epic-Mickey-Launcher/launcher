<svelte:options accessors={true} />

<script lang="ts">
  import { SetData } from "../library/datatransfer";
  import { GetImagePath, ImageType } from "../library/networking";

  export let mod: any;

  function OnClick() {
    SetData("modpage_id", mod.ID);
    window.open("#/modpage", "_self");
  }

  function ShortenDescription(buffer: string): string {
    if (buffer.length < 100) return buffer;

    return buffer.substring(0, 100).trim() + "...";
  }
</script>

<div class="modNode">
  <img
    src={GetImagePath(mod.ID, ImageType.Mod)}
    alt=""
    style="width:200px;border-radius:10px;"
  />
  <p>
    <button on:click={OnClick} class="hyperlinkbutton">{mod.Name}</button>
    <br />
    <span style="font-size:10px;">{ShortenDescription(mod.Description)}</span>
  </p>
</div>

<style>
  .modNode {
    border-radius: 10px;
    background-color: #242424;
    width: 200px;
    height: 320px;
    margin: auto;
    padding: 3px;
    filter: drop-shadow(1px 1px 5px black);
    transition-duration: 0.1s;
  }
  .modNode:hover {
    transform: translateY(-5px);
  }
</style>

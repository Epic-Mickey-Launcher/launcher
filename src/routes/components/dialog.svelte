<svelte:options accessors={true} />

<script lang="ts">
  import { onMount } from "svelte";

  let index = 0;
  let currentDialog = "";
  export let content = [];

  onMount(() => {
    currentDialog = content[0];
  });

  function IncrementDialog() {
    index++;

    if (index >= content.length) {
      index = 0;
    }

    currentDialog = content[index];
  }
</script>

<div style="display:flex;width:100%;">
  <div style="display:flex;position:relative;text-align:center;margin:auto;">
    <img style="opacity:0.6" src="img/dialog.png" />
    <img
      style="position:absolute;left:0px;bottom:0px;width:60px;"
      src="img/gus.png"
    />
    <span
      style="font-family: highlander;position:absolute;text-align:left; left:60px;top:5px;filter:drop-shadow(0 0 2px black);width:300px;"
      >{@html currentDialog}</span
    >
    <span
      style="font-family: highlander;position:absolute; text-align:center; left:17px;top:60px;"
      >Gus</span
    >
    {#if content.length > 1}
      <img
        alt=""
        src="img/abutton.png"
        on:click={IncrementDialog}
        style="position:absolute; width:24px; right:-5px;bottom:0px;"
      />
    {/if}
  </div>
</div>

<style>
  @font-face {
    font-family: highlander;
    src: url(/fonts/highlander.otf);
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";

  let index = 0;
  let currentDialog = $state("");
  let { content = [] } = $props();

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

  export { content };
</script>

<div style="display:flex;width:100%;">
  <div style="display:flex;position:relative;text-align:center;margin:auto;">
    <img src="img/dialog.png" style="opacity:0.6" />
    <img
      src="img/gus.png"
      style="position:absolute;left:0px;bottom:0px;width:60px;"
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
        alt="Proceed Button"
        src="img/abutton.png"
        onclick={IncrementDialog}
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

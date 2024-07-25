<script lang="ts">
  import { onMount } from "svelte";
  import {
    CheckOneTimeNoticeBlacklist,
    WriteOneTimeNoticeBlacklist,
  } from "../library/configfiles";

  export let id: string;
  export let content: string;
  let visible = false;

  onMount(async () => {
    visible = !(await CheckOneTimeNoticeBlacklist(id));
  });

  async function Hide() {
    await WriteOneTimeNoticeBlacklist(id);
    visible = false;
  }
</script>

<main>
  {#if visible}
    <div
      style="background-color:rgba(0,0,0,0.4);width:400px;height:200px;position:relative;border-radius:12px;margin:auto;"
    >
      <span
        style="position:absolute;top:10px;left:15px;font-size:10px;color:gray;"
        >Heads-up!</span
      >
      <span style="position:absolute;top:25px;left:15px;">{content}</span>
      <p></p>
      <button
        style="position:absolute;bottom:15px;left:15px;"
        class="hyperlinkbutton"
        on:click={() => Hide()}>OK</button
      >
    </div>
  {/if}
</main>

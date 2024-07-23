<svelte:options accessors={true} />

<script lang="ts">
  import { onMount } from "svelte";

  export let timestamp: string;
  export let content: string;
  export let hash: string;
  export let author: string;

  let time: string;
  onMount(() => {
    let converted = Number(timestamp);
    time = new Date(converted).toLocaleString();
  });
</script>

<div class="commitDiv">
  <span>{author} | {hash} | {time}</span><br />
  <span style="color:yellow;font-size: 12px;">{content}</span>
</div>

<style>
  .commitDiv {
    margin: auto;
    width: 90%;
    height: 50px;
    padding: 10px;
    margin-bottom: 5px;
    border-radius: 5px;
    background-color: rgb(20 20 20);
    overflow-y: scroll;
  }
</style>

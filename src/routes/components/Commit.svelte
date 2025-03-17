<script lang="ts">
  import {onMount} from "svelte";

  interface Props {
        timestamp: string;
        content: string;
        hash: string;
        author: string;
    }

    let {
        timestamp,
        content,
        hash,
        author
    }: Props = $props();

    let time: string = $state();
    onMount(() => {
        let converted = Number(timestamp);
        time = new Date(converted).toLocaleString();
    });

    export {
        timestamp,
        content,
        hash,
        author,
    }
</script>

<div class="commitDiv">
    <span>{author} | {hash} | {time}</span><br/>
    <span style="color:yellow;font-size: 12px;">{content}</span>
</div>

<style>
    .commitDiv {
        scrollbar-width: none;
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

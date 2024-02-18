<svelte:options accessors={true} />

<script>
    import { onMount } from "svelte";
    import { SetData } from "../library/datatransfer";

    export let name;
    export let description;
    export let modicon;
    export let id;

    function OnClick() {
        SetData("modpage_id", id);
        window.open("#/modpage", "_self");
    }
</script>

<div class="modNode">
    <img src={modicon} alt="" style="width:200px;border-radius:10px;" />
    <p>
        <button on:click={OnClick} class="hyperlinkbutton">{name}</button>
        <br />
        <span style="font-size:10px;">{description}</span>
    </p>
</div>

<style>
    .modNode {
        border-radius:10px;
        background-color: #242424;
        width: 200px;
        height: 320px;
        margin: auto;
        padding: 3px;
        filter: drop-shadow(1px 1px 5px black);
        transition-duration: 0.1s;
    }
    .modNode:hover{
        transform: translateY(-5px);
    }
</style>

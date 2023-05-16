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

<div
    style="background-color:black;width:200px;height:320px;margin:auto;display:inline-block;margin-right:10px;"
>
    <img src={modicon} alt="" style="width:200px;" />
    <p>
        <button on:click={OnClick} class="hyperlinkbutton">{name}</button>
        <br />
        <span style="font-size:10px;">{description}</span>
    </p>
</div>

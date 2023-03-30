<svelte:options accessors={true} />

<script>
export let name;
export let description;
export let modicon;
export let id;
</script>

<div style="background-color:black;width:200px;height:320px;margin:auto;display:inline-block;margin-right:10px;">
    <img src={modicon} alt="" style="width:200px;">
    <p>
    <button class="hyperlinkbutton">{name}</button>
    <br>
     <span style="font-size:10px;">{description}</span>
 </div>

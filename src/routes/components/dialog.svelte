<svelte:options accessors={true} />

<script>
    export let content = "";
</script>

<div style="display:flex;width:100%;">
    <div style="display:flex;position:relative;text-align:center;margin:auto;">
        <img style="opacity:0.6" src="img/dialog.png">
        <img style="position:absolute;left:0px;bottom:0px;width:60px;" src="img/gus.png">
        <span style="font-family: highlander;position:absolute;text-align:left; left:60px;top:5px;filter:drop-shadow(0 0 2px black);width:300px;">{content}</span>
        <span style="font-family: highlander;position:absolute; text-align:center; left:17px;top:60px;">Gus</span>
    </div>
</div>


<style>

@font-face {
  font-family: highlander;
  src: url(/fonts/highlander.otf);
}
</style>
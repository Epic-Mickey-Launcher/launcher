<svelte:options accessors={true}/>

<main>

    <div class="gamenode">
        <img class="gamebuttonimage" src={imgBackgroundURL} alt="">
        <img class="gamelogo" src={imgLogoURL} alt="">
        <button on:click={OpenGame} class="gameplaybutton">Play</button>
        <button on:click={OpenLevelLoader}  class="gamesettings">...</button>
        <plaintext class ="error">{errorMSG}</plaintext>
        <plaintext class="nameofbuild">sds</plaintext>
    </div>
</main>


<script>
    import {objectbuffer} from "../library/datatransfer.js"
    import { invoke } from '@tauri-apps/api/tauri'
    import { ReadJSON } from "../library/configfiles.js";
    export let game = ""
    export let filepath = ""

    export let imgBackgroundURL = undefined
    export let imgLogoURL = undefined
    export let errorMSG = ""

    async function OpenGame() {
        let d = await ReadJSON("conf.json")
        invoke('playgame', {dolphin: d.dolphinPath, exe:filepath + "/sys/main.dol"} )
    }

    function OpenLevelLoader() {
        objectbuffer.set({game: game, path: filepath})
        window.open("#/levelloader", "_self")
    }
</script>


<style>
    .nameofbuild{
        pointer-events: none;
        position: relative;
        opacity: 0;
        transition-duration: 0.3s;
        bottom: 125px;
        left: 20px;
    }

 .gamenode {
  margin-right: auto;
  margin-left: auto;
  width: 500px;
  height: 80px;
  
  position: relative;
 }

 .error{
    position: relative;
    left:520px;
    bottom: 135px;
 }

 .gameplaybutton{
  position: relative;
  left: 200px;
  bottom:90px;
  padding: 10px 20px;
  background: rgb(2,0,36);
  background: linear-gradient(143deg, rgba(2,0,36,1) 0%, rgba(0,0,0,1) 0%, rgba(229,0,255,1) 0%, rgba(133,0,196,1) 100%);
  border: none;
  border-radius: 10px 0px 0px 10px;
  transition-duration: 0.2s;
 }

 .gameplaybutton:hover{
    background: linear-gradient(0deg, rgba(2,0,36,1) 0%, rgba(0,0,0,1) 0%, rgba(229,0,255,1) 0%, rgba(133,0,196,1) 100%);
 }
 
 .gamesettings:hover{
    background: linear-gradient(0deg, rgba(2,0,36,1) 0%, rgba(0,0,0,1) 0%, rgba(229,0,255,1) 0%, rgba(133,0,196,1) 100%);
 }
 .gamesettings{
  position: relative;
  left: 200px;
  bottom:90px;
  padding: 10px 5px;
  background: rgb(2,0,36);
  background: linear-gradient(143deg, rgba(2,0,36,1) 0%, rgba(0,0,0,1) 0%, rgba(229,0,255,1) 0%, rgba(133,0,196,1) 100%);
  border: none;
  border-radius: 0px 10px 10px 0px;
 }
 .gamelogo {
  width: 200px;
  height: 50px;
  position: relative;
  left: 20px;
  bottom:70px;
  filter: drop-shadow(1px 3px 5px rgba(0, 0, 0, 0.877));
  transition-duration: 0.3s;
 }

 .gamelogo:hover{
    opacity: 0;
 }

 .gamebuttonimage {
  border-radius: 10px;
  box-shadow: 2px 2px 10px rgb(0, 0, 0);
 }
</style>
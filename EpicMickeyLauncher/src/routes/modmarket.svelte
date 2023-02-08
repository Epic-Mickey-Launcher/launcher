
<script>
import { GET, POST } from "./library/networking.js";
import { onMount } from 'svelte';
import ModNode from './components/ModNode.svelte';
import ModsData from './data/mods.json';
onMount(async () => {
     await GetAllMods()
})

let ModList;

async function GetAllMods() 
{
   let data = ModsData
   data.forEach(async (e) => {
     let modNode = new ModNode({
        target: ModList
     })
     modNode.modName = e.title;
     modNode.iconLink = e.iconLink;
     modNode.description = e.description;
     modNode.downloadLink = e.downloadLink;
     modNode.author = e.author;
     modNode.Init()
   })
}
</script>


<div style="margin-right:auto;margin-left:auto;" bind:this={ModList}>

</div>
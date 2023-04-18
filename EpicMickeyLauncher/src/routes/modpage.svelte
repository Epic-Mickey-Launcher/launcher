<script>
    import { onMount } from "svelte";
    import { GetData } from "./library/datatransfer";
    import { POST, staticAssetsLink } from "./library/networking";


    let modid;
    let authorname = ""
    let modinfo;

    onMount(async () => {
        modid = GetData("modpage_id");
        modinfo = await POST("getmod", {id:modid})
        let userinfo = await POST("getaccount", {id:modinfo.author})
        if(userinfo.username == null){
             authorname = "Deleted Account"
        }
        else{
            authorname = userinfo.username;
        }
    })
</script>

{#if modinfo != null}
<div style="display:flex;width:100%;height:100%;justify-content:center;">
    <img src={staticAssetsLink + modinfo.icon} alt="" style="border-radius:20px;margin-right:20px;width:200px;height:200px;">
    <div>
        <span style="font-size:30px;">{modinfo.name}</span>
        <p>
        <button class="hyperlinkbutton">{authorname}</button>
        <p>
        <span>{modinfo.description}</span>
        <p>
        <button>Download</button> <span style="margin:auto 10px;font-size:10px;">59 Downloads</span>
        <p>
    </div>
</div>

{/if}

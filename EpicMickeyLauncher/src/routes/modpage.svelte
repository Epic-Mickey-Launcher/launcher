<script>
    import { onMount } from "svelte";
    import { GetData, SetData } from "./library/datatransfer";
    import { GetId, GetToken, POST, loggedin, staticAssetsLink } from "./library/networking";
    import { ReadFile, WriteFile } from "./library/configfiles";
    import ModInstall from "./components/ModInstall.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { Subscribe } from "./library/callback";

    let authoraccountexists = true;
    let modid;
    let authorname = ""
    let dumploc;
    let modinfo;

    onMount(async () => {
        modid = GetData("modpage_id");
        dumploc = GetData("modpage_dumploc");
        modinfo = await POST("getmod", {id:modid})
        let userinfo = await POST("getaccount", {id:modinfo.author})
        if(userinfo.username == null){
             authorname = "Unknown Account"
             authoraccountexists = false;
        }
        else{
            authorname = userinfo.username;
        }


        Subscribe("SignedIn", (m) => {
        if(m.error != 1)
        {
            console.log(modinfo)
            if(modinfo.author == m.id)
            {
                ownercontrols.style.display = "block"
            }
        }
        })

        if(loggedin)
        {
            let id = await GetId();
            if(modinfo.author == id)
            {
                ownercontrols.style.display = "block"
            }
        }

        CheckIfDownloaded()
    })

    async function DeleteMod()
    {
        let id = await GetToken();
        let res = await POST("deletemod", {token:id, id: modid})
        if(res.error === 0)
        {
            window.open("#/modmarket", "_self")
        }
    }


    function Download()
    {
        console.log(dumploc)

        let gameid;
        gameid = "SEME4Q";

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = staticAssetsLink + modinfo.icon;
        modInstallElement.modName = modinfo.name;

        invoke("download_mod", {
            url: staticAssetsLink + modinfo.download,
            name: modinfo.name,
            dumploc: dumploc,
            modid: modinfo.id.toString(),
            gameid: gameid,
        }).then(async (json) => {
            let changedFiles = JSON.parse(json);
            let currentMods = JSON.parse(
                await ReadFile(dumploc + "/EMLMods.json")
            );
            currentMods.push(changedFiles);
            await WriteFile(
                JSON.stringify(currentMods),
                dumploc+ "/EMLMods.json"
            );

            modInstallElement.$destroy();
            CheckIfDownloaded()
        });
    }
    let downloadButton;
    async function CheckIfDownloaded()
    {
       let dataStr = await ReadFile( dumploc+ "/EMLMods.json");
       let dataJson = JSON.parse(dataStr);
       let json = dataJson.find(r => r.modid == modid);
       downloadStatus = "Download";
       if(json != null)
       {
         downloadButton.disabled = true;
          downloadStatus = "Already Installed";
       }
    }

    async function OpenProfileOfAuthor(){
        if(!authoraccountexists)return;
        SetData("profile_id", modinfo.author)
        window.open("#/profilepage", "_self")
    }

    let ownercontrols;
    let downloadStatus = "Download";

</script>

{#if modinfo != null}
<div style="display:flex;width:100%;height:100%;justify-content:center;">
    <img src={staticAssetsLink + modinfo.icon} alt="" style="border-radius:20px;margin-right:20px;width:200px;height:200px;">
    <div>
        <span style="font-size:30px;">{modinfo.name}</span>
        <p>
        <button on:click={OpenProfileOfAuthor} class="hyperlinkbutton">{authorname}</button>
        <p>
        <span>{modinfo.description}</span>
        <p>
        <button bind:this={downloadButton}  on:click={Download}>{downloadStatus}</button>
        <button on:click={() => window.open("#/modmarket", "_self")}>Go back to Mod Market</button>
        <p>
        <div style="display:none;" bind:this={ownercontrols}>
            <button>Update Mod</button> <button on:click={DeleteMod}>Delete Mod</button>
        </div>
    </div>
</div>

{/if}

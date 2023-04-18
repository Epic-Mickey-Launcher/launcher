
<script>
    import { onMount } from "svelte";
    import Userprofilemodnode from "./components/userprofilemodnode.svelte";
    import { Subscribe } from "./library/callback";
    import { GetUserInfo, loggedin, OnSignedIn, POST, staticAssetsLink } from "./library/networking";
    
    let isownerofprofile;
    let modNodeGroup;

    let username;
    let bio;
    let pfplink;
    

    onMount(async () => {

        let cb = async (m) => {

       let userinfo = m;

       console.log(userinfo)

       let profileinfo = await POST("getprofileinfo", {id: userinfo.id, username:null})

       console.log(profileinfo)

        isownerofprofile = userinfo.id == profileinfo.id;
       
      

       username = profileinfo.username;
       bio = profileinfo.bio;
       pfplink = staticAssetsLink + "img/" + profileinfo.pfp;

       profileinfo.mods.forEach(m => {
            new Userprofilemodnode({
                target:modNodeGroup,
                props:{
                    name: m.name,
                    description: m.description,
                    id: m.id,
                    modicon: staticAssetsLink + m.icon,
                }
            })
       })
        }

        OnSignedIn(cb)

        // @ts-ignore
       
    })
</script>

<div style="text-align:center;">
    <img class="pfp" src={pfplink} alt="">
    <br>
    <span style="font-size:30px;">{username}</span>
    <p>
    <span>{bio}</span>
    <div style="border: 2px solid yellow;width:120px;margin:auto;border-radius:30px;display:none;"><p style="color:yellow;">role</p></div>
    <p>
    <hr>
    <span style="font-size:30px;">Mods</span>
    <p>

        <span bind:this={modNodeGroup} style="display:flex;width:fit-content;margin:0 auto;">

        </span>

    <p>
        {#if isownerofprofile}
        <button on:click={() => window.open("#/accountsettings", "_self")} class="hyperlinkbutton">Edit Profile</button>
        {/if}
  
    </div>

<style>
.pfp{
    z-index: 20;
    border-radius: 100%;
    width:200px;
}
</style>
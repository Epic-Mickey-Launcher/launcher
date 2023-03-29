
<script>
    import { onMount } from "svelte";
    import Userprofilemodnode from "./components/userprofilemodnode.svelte";
    import { GetUserInfo, POST, userinfo } from "./library/networking";
    
    let modNodeGroup;

    onMount(async () => {

        let cb = async () => {
            let modnode = new Userprofilemodnode({
        target:modNodeGroup,
       })

       let userinfo = await GetUserInfo();

       let profileinfo = await POST("getprofileinfo", {id: userinfo.id, username:null})

       console.log(profileinfo)
        }

        // @ts-ignore
        window.onSignIn.push(cb)
    })
</script>

<div style="text-align:center;">
    <img class="pfp" src="/img/waren.png" alt="">
    <br>
    <span style="font-size:30px;">sexymustache1</span>
    <p>
    <span>My dick is turning purple!</span>
    <div style="border: 2px solid yellow;width:120px;margin:auto;border-radius:30px;"><p style="color:yellow;">role</p></div>
    <p>
    <hr>
    <span style="font-size:30px;">Mods</span>
    <p>

        <span bind:this={modNodeGroup}>

        </span>
  
    </div>

<style>
.pfp{
    z-index: 20;
    border-radius: 100%;
    width:200px;
}
</style>
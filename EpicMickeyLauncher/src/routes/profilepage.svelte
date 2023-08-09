<script>
    import { onMount } from "svelte";
    import Userprofilemodnode from "./components/userprofilemodnode.svelte";
    import {
        OnSignedIn,
        POST,
        loggedin,
        staticAssetsLink,
    } from "./library/networking";
    import { GetData, SetData } from "./library/datatransfer";
    import { Subscribe } from "./library/callback";

    let isownerofprofile;
    let modNodeGroup;

    let username = "";
    let bio = "";
    let pfplink = "";

    let profilepage;
    let err;

    let emblemName = "";
    let emblemColor = "";

    let callback;

    onMount(async () => {
        let cb = async (m) => {
            let userinfo = m;

            //used for visiting other users profiles
            let idofprofile = await GetData("profile_id");

            let profileinfo;

            if (idofprofile != null) {
                profileinfo = await POST("getprofileinfo", {
                    id: idofprofile,
                    username: null,
                });
                SetData("profile_id", null);
            } else {
                if (!loggedin) {
                    profilepage.style.display = "none";
                    err.style.display = "block";
                    return;
                }
                profileinfo = await POST("getprofileinfo", {
                    id: userinfo.id,
                    username: null,
                });
            }

            isownerofprofile = userinfo.id == profileinfo.id;

            username = profileinfo.username;
            bio = profileinfo.bio;
            pfplink = staticAssetsLink + "img/" + profileinfo.pfp;

         
                let emblem = profileinfo.emblems.sort((a, b) => {
                    return b.weight - a.weight;
                })[0];

                if(emblemName != null)
                {
                    emblemName = emblem.emblemname;
                emblemColor = emblem.color;
                }

            profileinfo.mods.forEach((m) => {
                new Userprofilemodnode({
                    target: modNodeGroup,
                    props: {
                        name: m.name,
                        description: m.description,
                        id: m.id,
                        modicon: staticAssetsLink + m.icon,
                    },
                });
            });
        };
        Subscribe("SignedIn", cb);
    });
</script>

<div bind:this={profilepage} style="text-align:center;">
    <img class="pfp" src={pfplink} alt="" />
    <br />
    <span style="font-size:30px;">{username}</span>
    <p>
        <span>{bio}</span>
    </p>
    {#if emblemName != ""}
    <div
        style="border: 2px solid {emblemColor};width:120px;margin:auto;border-radius:30px;"
    >
        <p style="color:{emblemColor};">{emblemName}</p>
    </div>
    {/if}
    <p />
    <hr />
    <span style="font-size:30px;">Mods</span>
    <p>
        <span
            bind:this={modNodeGroup}
            style="display:flex;width:fit-content;margin:0 auto;"
        />
    </p>
    <p>
        {#if isownerofprofile}
            <button
                on:click={() => window.open("#/accountsettings", "_self")}
                class="hyperlinkbutton">Edit Profile</button
            >
        {/if}
    </p>
</div>

<div bind:this={err} style="display:none;">
    <h2>You do not have an account.</h2>
</div>

<style>
    .pfp {
        z-index: 20;
        border-radius: 100%;
        width: 200px;
    }
</style>

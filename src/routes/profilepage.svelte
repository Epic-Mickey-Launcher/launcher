<script lang="ts">
  import {onMount} from "svelte";
  import Userprofilemodnode from "./components/userprofilemodnode.svelte";
  import {GetId, GetImagePath, GetToken, ImageType, loggedin, POST,} from "./library/networking";
  import {GetData, SetData} from "./library/datatransfer";
  import {Subscribe} from "./library/callback";
  import Loading from "./components/loading.svelte";

  let profileDiv: HTMLElement = $state();
    let isownerofprofile: boolean = $state();
    let modNodeGroup: HTMLDivElement = $state();
    let username = $state("");
    let bio = $state("");
    let pfplink = $state("");
    let profilepage: HTMLDivElement = $state();
    let err: HTMLDivElement = $state();
    let mods = $state([]);
    let joindate = $state("");
    let modLength = $state(0);
    let loaded = $state(false);

    onMount(async () => {
        let cb = async () => {
            loaded = false;
            let selfID = await GetId();
            let idofprofile = await GetData("profile_id");
            SetData("profile_id", undefined);
            if (idofprofile == undefined) {
                console.log("loading own profile " + selfID);
                idofprofile = selfID;

                if (!loggedin) {
                    profilepage.style.display = "none";
                    err.style.display = "block";
                    return;
                }
            }

            let request = await POST(
                "user/username",
                {
                    id: idofprofile,
                },
                false,
            );
            if (request.error) return;
            username = request.body;

            request = await POST(
                "user/bio",
                {
                    id: idofprofile,
                },
                false,
            );
            if (request.error) return;
            bio = request.body;

            pfplink = GetImagePath(idofprofile, ImageType.User);

            isownerofprofile = selfID == idofprofile;

            let timestamp = parseInt(idofprofile);
            let d = new Date(timestamp);
            joindate = d.toLocaleString();

            loaded = true;
            profileDiv.style.opacity = "1";

            let query = await POST("mod/query", {
                AuthorID: idofprofile,
                Token: await GetToken(),
            });
            if (query.body.ModObjs != null) {
                mods = query.body.ModObjs;
                modLength = mods.length;
            } else {
                modLength = 0;
            }
        };

        Subscribe("SignedIn", cb, false);
    });
</script>

{#if !loaded}
  <span style="position: absolute; top:45%; left:45%;">
    <Loading></Loading>
  </span>
{/if}
<main bind:this={profileDiv} style="opacity: 0;">
    <div bind:this={profilepage} style="text-align:center;">
        <img alt="" class="pfp" src={pfplink + "?"}/>
        <br/>
        <span style="font-size:30px;">{username}</span>
        <br/>
        <span style="font-size:10px;">EML Member since {joindate}</span>
        {#if false}
            <div style="display:flex;justify-content: center;">
                <div
                        class="emblem"
                        style="position:relative; display:flex;justify-content: center;"
                >
                    <div style="">
                        <img src="img/emblem/linux.svg" style="width:30px;z-index: 3;"/>
                    </div>

                    <span
                            style="position: absolute; padding:5px; border-radius:5px; z-index: 120; font-size:10px; text-align: center;text-wrap: nowrap;top:40px;"
                    >TrophyName <br/>
          <span style="font-size: 5px;">TrophyDesc</span>
        </span>
                </div>
            </div>
        {/if}
        <div></div>
        <p>
            <span>{bio}</span>
        </p>
        <p></p>

        {#if modLength > 0}
            <hr/>
            <span style="font-size:30px;">Mods</span>
            <p></p>
            <div style="width:100%;display:flex;justify-content:center;">
                <div
                        bind:this={modNodeGroup}
                        style="display:grid;justify-content:center;gap:5px;width:100%;grid-template-columns: repeat(5, 0fr);"
                >
                    {#each mods as mod}
                        <Userprofilemodnode {mod}></Userprofilemodnode>
                    {/each}
                </div>
            </div>
        {/if}

        <p>
            {#if isownerofprofile}
                <button
                        onclick={() => window.open("#/accountsettings", "_self")}
                        class="hyperlinkbutton">Edit Profile
                </button
                >
            {/if}
        </p>

        <div bind:this={err} style="display:none;">
            <h2>You do not have an account.</h2>
        </div>
    </div>
</main>

<style>
    * {
        transition-duration: 0.3s;
    }

    .emblem {
        width: 30px;
        height: 30px;
        padding: 5px;
        border-radius: 100%;
    }

    .emblem:hover span {
        background-color: black;
        color: white;
    }

    .emblem span {
        background-color: transparent;
        color: transparent;
    }

    .emblem:hover {
        background: rgb(0, 0, 0);
        background: radial-gradient(
                circle,
                rgba(0, 0, 0, 1) 30%,
                rgba(0, 0, 0, 0) 100%
        );
        transform: scale(3);
    }

    .pfp {
        z-index: 20;
        border-radius: 100%;
        width: 200px;
    }
</style>

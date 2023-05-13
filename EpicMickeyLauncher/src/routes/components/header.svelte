<svelte:options accessors={true} />
<script>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { ReadToken } from "../library/configfiles";
    import {Login, SetLoggedIn, loggedin, staticAssetsLink } from "../library/networking";
    import {Subscribe} from "../library/callback.js"
    let pfp;

    export async function ForceSetPFP(p)
    {
       pfp = p
    }


    onMount(async () => {
        let cb = (userinfo) => {
            if (userinfo.error != 1) {
                SetLoggedIn(true)
                pfp = staticAssetsLink + "img/" + userinfo.pfp + "?" + new Date().getTime();;
            }
            else{
                SetLoggedIn(false)
                pfp = "img/loggedoutpfp.jpeg";
            }
        };

        // @ts-ignore
        Subscribe("SignedIn", cb, true)
    });


    export const HeaderVisible = writable(true);

    let header;

    function OpenPage(page) {
        window.open("#/" + page, "_self");
    }

    document.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });


    async function PfpButton(){
        if(loggedin)
        {
            OpenPage("profilepage")
        }
        else{
            OpenPage("register")
        }
    }

</script>

<main>
    {#if HeaderVisible}
        <div class="header" bind:this={header}>
            <img
                src="/img/emlLogo.png"
                alt=""
                style="z-index:1;margin-left:10px;"
            />

            <p style="margin-right:20px" />

            <button
                on:click={() => OpenPage("modmarket")}
                class="headerButton startheaderbuttons">Mod Market</button
            >
            <button on:click={() => OpenPage("games")} class="headerButton"
                >Games</button
            >

            <button
                on:click={() => OpenPage("settings")}
                class="headerButton endheaderbuttons">Settings</button
            >

            <div class="pfpbutton">
                <button on:click={() => PfpButton()} style="position:absolute;width:50px;height:50px;top:20px;border:none;background-color: Transparent;"></button>
                <img
                src={pfp}
                alt=""
                title="Sign Up"
                class="pfp"
                />
            </div>
    
        </div>

        <p style="margin-bottom:60px" />
    {/if}
</main>

<style>
    .startheaderbuttons {
        border-radius: 10px 0 0 10px;
    }

    .endheaderbuttons {
        border-radius: 0 10px 10px 0;
    }

    .pfpbutton{
        margin: auto;
        margin-right: 10px;
    }

    .pfp {
        pointer-events: none;
        position: relative;
        transition-duration: 0.3s;
        width: 50px;
        height: 50px;
        border-radius: 100%;
    }
    .pfp:hover {
        transform: scale(1.1);
    }

    .header {
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
        border-radius: 10px;
        display: flex;
        columns: 1rem 1rem;
        width: 100%;
        background: rgb(66,66,66);
background: linear-gradient(143deg, rgba(66,66,66,1) 0%, rgba(62,62,62,1) 100%);
        padding: 5px 0px;
        justify-content: left;
    }
    .headerButton {
        z-index: 1;
        width: 20%;
        border: none;
        background-color: rgb(36, 36, 36);
        transition-duration: 0.1s;
    }
    .headerButton:hover {
        background-color: rgb(43, 43, 43);
    }
</style>

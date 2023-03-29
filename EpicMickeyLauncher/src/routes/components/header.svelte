<script>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { ReadToken } from "../library/configfiles";
    import {Login, staticAssetsLink } from "../library/networking";
    import {Subscribe} from "../library/callback.js"
    let pfp;

    onMount(async () => {
        let cb = (userinfo) => {
            console.log(userinfo)
            if (userinfo.error == 0) {
                pfp = staticAssetsLink + "img/" + userinfo.pfp;
                console.log(pfp)
                accountbutton.style.display = "block";
            }
        };

        // @ts-ignore
        Subscribe("SignedIn", cb)
    });

    let accountbutton;

    export const HeaderVisible = writable(true);

    let header;

    function OpenPage(page) {
        window.open("#/" + page, "_self");
    }

    document.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });
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
                style="display:none;"
                bind:this={accountbutton}
                on:click={() => OpenPage("profilepage")}
                class="headerButton">My account</button
            >
            <button
                on:click={() => OpenPage("settings")}
                class="headerButton endheaderbuttons">Settings</button
            >

            <div class="pfpbutton">
                <button on:click={() => OpenPage("register")} style="position:absolute;width:50px;height:50px;top:20px;border:none;background-color: Transparent;"></button>
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
        border-radius: 10px;
        display: flex;
        columns: 1rem 1rem;
        width: 100%;
        background-color: #555555;
        padding: 5px 0px;
        justify-content: left;
    }
    .headerButton {
        z-index: 1;
        width: 10%;
        border: none;
        background-color: rgb(36, 36, 36);
        transition-duration: 0.1s;
    }
    .headerButton:hover {
        background-color: rgb(43, 43, 43);
    }
</style>

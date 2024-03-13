<svelte:options accessors={true} />

<script>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { ReadToken } from "../library/configfiles";
    import { getVersion } from "@tauri-apps/api/app";
    import {
        GET,
        GETEXT,
        Login,
        POST,
        SetLoggedIn,
        loggedin,
        staticAssetsLink,
    } from "../library/networking";
    import { Subscribe } from "../library/callback.js";
    import { invoke } from "@tauri-apps/api/tauri";
    let pfp;
    let latestDownloadLink = "";
    let updateHyperLink;
    let connectionIssues;
    let version = ""
    export async function ForceSetPFP(p) {
        pfp = p;
    }

    onMount(async () => {
        version = await getVersion();
        let callbackOnEnterNewWindow = async () => {
            try {
                connectionIssues = false;
                await GET("checkhealth");
            } catch {
                connectionIssues = true;
            }
        };

        let cb = async (userinfo) => {
            if (userinfo.error != 1) {
                SetLoggedIn(true);
                pfp =
                    staticAssetsLink +
                    "img/" +
                    userinfo.pfp +
                    "?" +
                    new Date().getTime();
            } else {
                SetLoggedIn(false);
                pfp = "img/loggedoutpfp.jpeg";
            }
        };

        // @ts-ignore
        Subscribe("SignedIn", cb, true);
        Subscribe("OnNewWindow", callbackOnEnterNewWindow, true);
        let info = await GETEXT(
            "https://api.github.com/repos/KjubDusJub/Epic-Mickey-Launcher/releases"
        );
        let info_stable = info.filter((r) => !r.prerelease);
        let newest_release = info_stable[0];
        let current_version = await getVersion();
        if (newest_release.tag_name != current_version) {
            latestDownloadLink = newest_release.html_url;
            updateHyperLink.style.display = "block";
        }
    });

    export const HeaderVisible = writable(true);

    function OpenLatestDownloadPage() {
        invoke("open_link", { url: latestDownloadLink });
    }

    let header;

    function OpenPage(page) {
        window.open("#/" + page, "_self");
    }

    document.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });

    async function PfpButton() {
        if (loggedin) {
            OpenPage("profilepage");
        } else {
            OpenPage("register");
        }
    }
</script>

<main>
    {#if HeaderVisible}
        <p />
        <div class="header" bind:this={header}>
            <img
                src="/img/eml.svg"
                alt=""
                title={version}
                style="width:300px;padding:5px 0px;position:relative;right:30px;"
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

            <button
                class="hyperlinkbutton"
                on:click={OpenLatestDownloadPage}
                bind:this={updateHyperLink}
                style="margin:auto 10px;color:lime;display:none;"
                >Update Available!</button
            >

            {#if connectionIssues}
                <img
                    alt=""
                    style="width:32px;margin-left:12px;"
                    src="img/warning.svg"
                    title="Cannot connect to online services!"
                />
            {/if}

            <div class="pfpbutton">
                <button
                    on:click={() => PfpButton()}
                    style="position:absolute;width:50px;height:50px;top:20px;border:none;background-color: Transparent;"
                />
                <img src={pfp} alt="" title="Sign Up" class="pfp" />
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

    .pfpbutton {
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

    .pfpbutton:hover .pfp {
        transform: scale(1.1);
    }

    .header {
        box-shadow: 2px 2px 10px rgb(0, 0, 0);
        border-radius: 10px;
        display: flex;
        z-index: 1;
        columns: 1rem 1rem;
        width: 100%;
        backdrop-filter: blur(2px);
        -webkit-backdrop-filter: blur(10px);
        padding: 5px 0px;
        justify-content: left;
    }
    .headerButton {
        width: 20%;
        border: none;
        backdrop-filter: blur(4px);
        background-color: rgb(0, 0, 0, 0.5);
        -webkit-backdrop-filter: blur(10px);
        transition-duration: 0.1s;
    }
    .headerButton:hover {
        background-color: rgb(43, 43, 43);
    }
</style>

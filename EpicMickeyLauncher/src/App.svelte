<script lang="ts">
    import Header from "./routes/components/header.svelte";
    import { InitConfFiles, ReadToken } from "./routes/library/configfiles.js";
    import Router from "svelte-spa-router";
    import Games from "./routes/Games.svelte";
    import AddGame from "./routes/addgame.svelte";
    import QuickStart from "./routes/quickstart.svelte";
    import Register from "./routes/register.svelte";
    import Settings from "./routes/settings.svelte";
    import uploadmod from "./routes/uploadmod.svelte";
    import LevelLoader from "./routes/LevelLoader.svelte";
    import ModMarket from "./routes/modmarket.svelte";
    import ProfilePage from "./routes/profilepage.svelte";
    import modpage from "./routes/modpage.svelte";
    import accountsettings from "./routes/accountsettings.svelte";
    import { Login, loggedin } from "./routes/library/networking";
    import { Invoke } from "./routes/library/callback";
    import { emit, listen } from '@tauri-apps/api/event'

    let header;

    async function RouteLoaded() {
        //login

        let token = await ReadToken();
        if (token != "") {
            Login({ token: token });
        } else {
            Invoke("SignedIn", { error: 1 });
            header.ForceSetPFP("img/loggedoutpfp.jpeg");
        }
    }

    async function ListenLoop()
    {
        await listen("scheme_request_received", (event) => {

            let parsed = event.payload.message;

            parsed = parsed.substring(5, event.payload.length - 1);

          window.open("#/modpage", "_self")
        });
    }

    let routes = {
        "/": Games,
        "/levelloader": LevelLoader,
        "/modmarket": ModMarket,
        "/addgame": AddGame,
        "/uploadmod": uploadmod,
        "/settings": Settings,
        "/quickstart": QuickStart,
        "/register": Register,
        "/modpage": modpage,
        "/profilepage": ProfilePage,
        "/accountsettings": accountsettings,
        "/*": Games,
    };
    InitConfFiles();
    ListenLoop();

</script>

<main>
    <Header bind:this={header} />
    <Router {routes} on:routeLoaded={RouteLoaded} />
</main>

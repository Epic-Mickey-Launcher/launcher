<script lang="ts">
    import Header from "./routes/components/header.svelte"
    import {InitConfFiles} from "./routes/library/configfiles.js"
    import Router from "svelte-spa-router"
    import Games from "./routes/Games.svelte"
    import AddGame from "./routes/addgame.svelte"
    import QuickStart from "./routes/quickstart.svelte"
    import Register from "./routes/register.svelte"
    import Settings from "./routes/settings.svelte"
    import uploadmod from "./routes/uploadmod.svelte"
    import LevelLoader from "./routes/LevelLoader.svelte"
    import ModMarket from "./routes/modmarket.svelte";
    import ProfilePage from "./routes/profilepage.svelte";
    let routes = {
        "/": Games,
        "/levelloader": LevelLoader,
        "/modmarket": ModMarket,
        "/addgame": AddGame,
        "/uploadmod": uploadmod,
        "/settings": Settings,
        "/quickstart": QuickStart,
        "/register": Register,
        "/profilepage": ProfilePage,
        "/*": Games,
    }
    InitConfFiles()
</script>

<main>
    <Header/>
    <Router {routes}/>
</main>
<script lang="ts">
    import Header from "./routes/components/header.svelte"
    import {InitConfFiles} from "./routes/library/configfiles.js"
    import Router from "svelte-spa-router"
    import Games from "./routes/Games.svelte"
    import AddGame from "./routes/addgame.svelte"
    import QuickStart from "./routes/quickstart.svelte"
    import Settings from "./routes/settings.svelte"
    import LevelLoader from "./routes/LevelLoader.svelte"
    import ModMarket from "./routes/modmarket.svelte";
    let routes = {
        "/": Games,
        "/levelloader": LevelLoader,
        "/modmarket": ModMarket,
        "/addgame": AddGame,
        "/settings": Settings,
        "/quickstart": QuickStart,
        "/*": Games,
    }
    InitConfFiles()
</script>

<main>
    <Header/>
    <Router {routes}/>
</main>
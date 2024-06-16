<script lang="ts">
  let header: any;
  import Header from "./routes/components/header.svelte";
  import {
    InitConfFiles,
    ReadJSON,
    ReadToken,
    GetPath,
  } from "./routes/library/configfiles";
  import Router from "svelte-spa-router";
  import Games from "./routes/games.svelte";
  import AddGame from "./routes/addgame.svelte";
  import QuickStart from "./routes/quickstart.svelte";
  import Register from "./routes/register.svelte";
  import Settings from "./routes/settings.svelte";
  import uploadmod from "./routes/uploadmod.svelte";
  import LevelLoader from "./routes/levelloader.svelte";
  import ModMarket from "./routes/modmarket.svelte";
  import ProfilePage from "./routes/profilepage.svelte";
  import modpage from "./routes/modpage.svelte";
  import accountsettings from "./routes/accountsettings.svelte";
  import { Login, UserInfo } from "./routes/library/networking";
  import { Invoke } from "./routes/library/callback";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  async function RouteLoaded() {
    await GetPath();
    Invoke("OnNewWindow", null);
    let token = await ReadToken();
    if (token != "") {
      let userInfo: UserInfo = {
        token: token,
      };

      Login(userInfo);
    } else {
      Invoke("SignedIn", { error: 1 });
      header.ForceSetPFP("img/loggedoutpfp.jpeg");
    }
  }

  async function ListenLoop() {
    await listen("scheme_request_received", (event: any) => {
      let parsed = event.payload.message;
      parsed = parsed.substring(5, event.payload.length - 1);
      window.open("#/modpage", "_self");
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

  let funcKeyDown: boolean;
  function keyDown(e: KeyboardEvent) {
    if (e.code == "17") {
      funcKeyDown = true;
    }
  }

  function keyUp(e: KeyboardEvent) {
    if (e.code == "17") {
      funcKeyDown = false;
    }

    if (funcKeyDown) {
      switch (e.code) {
        case "85":
          invoke("open_config_folder");
          break;
        case "68":
          ReadJSON("conf.json").then((d) => {
            invoke("open_dolphin", { path: d.dolphinPath });
          });
          break;
      }
    }
  }
</script>

<main>
  <Header bind:this={header} />
  <Router {routes} on:routeLoaded={RouteLoaded} />
</main>

<svelte:window on:keydown={keyDown} on:keyup={keyUp} />

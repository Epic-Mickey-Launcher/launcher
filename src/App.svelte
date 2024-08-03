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
  import { Login, UserInfo, SetServerURL } from "./routes/library/networking";
  import { Invoke } from "./routes/library/callback";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { exit } from "@tauri-apps/api/process";
  import { getMatches } from "@tauri-apps/api/cli";

  getMatches().then((matches) => {
    if (matches.args["server"].value != false) {
      SetServerURL(matches.args["server"].value);
    }
  });

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

  async function ErrorCatcher() {
    await listen("on_rust_error", async (event: any) => {
      let res = await confirm(
        'Rust Backend Error!\n"' +
          event.payload +
          "\\nThis error might cause further instability. It is recommended that you restart EML to avoid this. Do you want to quit?",
      );

      if (res) {
        exit(0);
      }
    });
  }

  async function ListenLoop() {
    await listen("scheme_request_received", (event: any) => {
      let parsed = event.payload.message;
      parsed = parsed.substring(5, event.payload.length - 1);
      window.open("#/modpage", "_self");
    });
  }
  ErrorCatcher();

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
    if (e.code == "17" || e.code == "ControlLeft" || e.code == "ControlRight") {
      funcKeyDown = true;
    }
  }

  function keyUp(e: KeyboardEvent) {
    if (e.code == "17" || e.code == "ControlLeft" || e.code == "ControlRight") {
      funcKeyDown = false;
    }

    if (funcKeyDown) {
      console.log(e.code);
      switch (e.code) {
        case "KeyY":
        case "85":
          invoke("open_config_folder");
          break;
        case "KeyD":
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

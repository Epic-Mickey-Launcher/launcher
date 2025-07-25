<script lang="ts">
  import SelectGameForMod from "./routes/components/SelectGameForMod.svelte";
  import Header from "./routes/components/header.svelte";
  import {
    GetPath,
    InitConfFiles,
    ReadToken,
  } from "./routes/library/configfiles";
  import Router from "svelte-spa-router";
  import Games from "./routes/games.svelte";
  import QuickStart from "./routes/quickstart.svelte";
  import Register from "./routes/register.svelte";
  import Settings from "./routes/settings.svelte";
  import UploadMod from "./routes/uploadmod.svelte";
  import LevelLoader from "./routes/levelloader.svelte";
  import ModMarket from "./routes/modmarket.svelte";
  import ProfilePage from "./routes/profilepage.svelte";
  import ModPage from "./routes/modpage.svelte";
  import AccountSettings from "./routes/accountsettings.svelte";
  import { Invoke } from "./routes/library/callback";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { exit } from "@tauri-apps/plugin-process";
  import { onOpenUrl, register } from "@tauri-apps/plugin-deep-link";
  import { onMount } from "svelte";
  import DownloadMod from "./routes/components/downloadMod.svelte";
  import { ConvertGamesConfigToTrackedGames } from "./routes/library/legacy";
  import {
    LoadConfig,
    LoadGameInstancesFromTrackingFile,
    SetHeader,
    SetOS,
  } from "./routes/library/config";
  import { LoginWithSession } from "./routes/library/account";

  let DownloadModComponent = $state(DownloadMod);
  let SelectGameForModComponent = $state(SelectGameForMod);
  let HeaderComponent = $state(Header);
  let RouterComponent = $state(Router);
  let initialConfigLoaded = $state(false);
  let header: any = $state();

  ErrorCatcher();
  ListenLoop();

  register("eml");

  onMount(async () => {
    await SetOS();
    await GetPath();
    let config = await LoadConfig();

    await InitConfFiles();
    await ConvertGamesConfigToTrackedGames();
    await LoadGameInstancesFromTrackingFile();
    SetHeader(header);
    if (config == null) {
      window.open("#/quickstart", "_self");
    }
    initialConfigLoaded = true;

    LoginWithSession(); // attempt log in on application start up
  });

  onOpenUrl(async (urls) => {
    alert(urls[0]);
    console.log("catched url call");
    let url = new URL(urls[0]);
    console.log(url.hostname);
    switch (url.hostname) {
      case "gb":
        break;
    }
  });

  async function RouteLoaded() {
    await GetPath();
    Invoke("OnNewWindow", null);
  }

  async function ErrorCatcher() {
    await listen("on_rust_error", async (event: any) => {
      let res = await confirm(
        'Rust Backend Error!\n"' +
          event.payload +
          "\\nThis error might cause further instability. It is recommended that you restart EML to avoid this. Do you want to quit?",
      );

      if (res) {
        await exit(0);
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

  let routes = {
    "/": Games,
    "/levelloader": LevelLoader,
    "/modmarket": ModMarket,
    "/uploadmod": UploadMod,
    "/settings": Settings,
    "/quickstart": QuickStart,
    "/register": Register,
    "/modpage": ModPage,
    "/profilepage": ProfilePage,
    "/accountsettings": AccountSettings,
    "/*": Games,
  };

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
      switch (e.code) {
        case "KeyY":
        case "85":
          invoke("open_config_folder");
          break;
        case "KeyD":
        case "68":
          LoadConfig().then((d) => {
            invoke("open_dolphin", { path: d.dolphinPath });
          });
          break;
      }
    }
  }
</script>

<SelectGameForModComponent />
<DownloadModComponent />

<main>
  <HeaderComponent bind:this={header} />
  {#if initialConfigLoaded}
    <RouterComponent on:routeLoaded={RouteLoaded} {routes} />
  {/if}
</main>

<svelte:window onkeydown={keyDown} onkeyup={keyUp} />

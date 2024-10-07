<script lang="ts">
  let header: any;
  import Header from "./routes/components/header.svelte";
  import {
    InitConfFiles,
    ReadJSON,
    ReadToken,
    GetPath,
  } from "./routes/library/configfiles";
  import { Game, GameConfig, Mod, Platform } from "./routes/library/types";
  import Router from "svelte-spa-router";
  import Games from "./routes/games.svelte";
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
  import { invoke } from "@tauri-apps/api/core";
  import { exit } from "@tauri-apps/plugin-process";
  import { getMatches } from "@tauri-apps/plugin-cli";
  import Modpublisher from "./routes/modpublisher.svelte";
  import {
    onOpenUrl,
    register,
    getCurrent,
  } from "@tauri-apps/plugin-deep-link";
  import { onMount } from "svelte";
  import { LoadConfig, SaveConfig } from "./routes/library/config";
  import DownloadMod from "./routes/components/downloadMod.svelte";
  import ModInstall from "./routes/components/ModInstall.svelte";
  import SelectGameForMod from "./routes/components/SelectGameForMod.svelte";
  import { LocalModToUnifiedMod } from "./routes/library/gameid";

  let selectGame: SelectGameForMod;
  let installingMod: boolean;
  let downloadMod: DownloadMod;

  register("eml");
  onOpenUrl(async (urls) => {
    let url = new URL(urls[0]);
    console.log(url.hostname);
    switch (url.hostname) {
      case "github":
        let auth = url.searchParams.get("auth");
        let config = await LoadConfig();
        config.gitHubSecret = auth;
        await SaveConfig(config);
        break;
      case "gb":
        console.log("money");
        let gbURL = url.pathname.substring(1, url.pathname.length);
        let conf = await confirm(
          "This mod is external and is not verified by EML. Are you sure you want to install this mod? (" +
            gbURL +
            ")",
        );
        if (conf) {
          let downloadURL = gbURL.split(",")[0];
          let id = gbURL.split(",")[2];

          installingMod = true;
          let installMod = new ModInstall({
            target: document.body,
          });
          installMod.action = "Downloading and Validating";
          installMod.modName = "External Mod";
          installMod.description =
            "This might take a while depending on your internet speed.";
          let validationInfo: any = await invoke("validate_mod", {
            url: downloadURL,
            destination: ".extern",
            mode: "extern",
          });

          console.log(validationInfo);
          if (validationInfo.validated) {
            installMod.$destroy();
            console.log(
              ((validationInfo.data.game.toUpperCase() as Game) +
                " " +
                validationInfo.data.platform.toUpperCase()) as Platform,
            );
            selectGame.GetValidGame(
              validationInfo.data.game.toUpperCase() as Game,
              validationInfo.data.platform.toUpperCase() as Platform,
              id,
              0,
              async (game: GameConfig) => {
                console.log(game);
                if (game != null) {
                  let unifiedMod = LocalModToUnifiedMod(validationInfo.data);
                  console.log(unifiedMod);
                  unifiedMod.version = 0;
                  unifiedMod.id = id;

                  downloadMod.Initialize(
                    game,
                    true,
                    unifiedMod,
                    ".extern",
                    validationInfo.modicon,
                    true,
                  );

                  await downloadMod.Download();
                }
              },
            );
          } else {
            console.log("two style");
            //await invoke("clean_temp_install_directory", {
            //  destination: ".extern",
            //});
          }
        }
    }
  });

  onMount(async () => {});

  getMatches().then(async (matches) => {
    if (matches.args["server"].value != null) {
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
    "/uploadmod": uploadmod,
    "/settings": Settings,
    "/quickstart": QuickStart,
    "/register": Register,
    "/modpage": modpage,
    "/profilepage": ProfilePage,
    "/accountsettings": accountsettings,
    "/modpublisher": Modpublisher,
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

<SelectGameForMod bind:this={selectGame}></SelectGameForMod>
<DownloadMod bind:this={downloadMod}></DownloadMod>

<main>
  <Header bind:this={header} />
  <Router {routes} on:routeLoaded={RouteLoaded} />
</main>

<svelte:window on:keydown={keyDown} on:keyup={keyUp} />

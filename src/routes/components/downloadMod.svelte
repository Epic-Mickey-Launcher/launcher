<script lang="ts">
  import { GetImagePath, ImageType } from "../library/networking";

  import { type GameConfig, Platform, type UnifiedMod } from "../library/types";

  import ModInstall from "./ModInstall.svelte";
  import { GetGameWiiID } from "../library/gameid";
  import { mount } from "svelte";

  let gamedata: GameConfig;
  let moddata: UnifiedMod;

  interface Props {
    downloadButtonStatus?: string;
    downloadButtonDisabled?: boolean;
    canupdate?: boolean;
    updatecb?: any;
    downloading?: boolean;
  }

  let {
    downloadButtonStatus = $bindable(""),
    downloadButtonDisabled = $bindable(false),
    canupdate = $bindable(false),
    updatecb = () => {},
    downloading = $bindable(false),
  }: Props = $props();
  let local = false;
  let installPath = "";
  let imgUrl = "";

  export async function Initialize(
    _gamedata: any,
    _local: boolean,
    _moddata: UnifiedMod,
    _installPath: string = "",
    _imgDataUrl: string = "",
    overrideCheck = false,
  ) {
    local = _local;
    moddata = _moddata;
    installPath = _installPath;
    gamedata = _gamedata;
    imgUrl = _imgDataUrl;

    if (local) {
      moddata.id = moddata.name.replace(" ", "").toLowerCase() + Date.now();
    }
  }

  async function CheckIfDownloaded() {}

  export async function Download() {
    downloading = true;
    let gameID =
      gamedata.platform == Platform.Wii ? GetGameWiiID(gamedata) : "";

    let modInstallElement = mount(ModInstall, {
      target: document.body,
    });
    modInstallElement.modIcon = local
      ? imgUrl
      : GetImagePath(moddata.id, ImageType.Mod, false);
    modInstallElement.modName = moddata.name;
    modInstallElement.showDownloadProgression = true;
  }

  export {
    downloadButtonStatus,
    downloadButtonDisabled,
    canupdate,
    updatecb,
    downloading,
  };
</script>

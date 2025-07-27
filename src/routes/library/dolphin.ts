import { invoke } from "@tauri-apps/api/core";
import { currentOperatingSystem, LoadConfig, SaveConfig } from "./config";
import { serverLink } from "./networking";
import { DolphinType, OperatingSystemType } from "./types";

export const DOLPHIN_LINK_WINDOWS =
  serverLink + "tool/download?tool=dolphin&target=windows";
export const DOLPHIN_LINK_LINUX =
  serverLink + "tool/download?tool=dolphin&target=linux";
export const DOLPHIN_LINK_MACOS =
  serverLink + "tool/download?tool=dolphin&target=macos";

export async function DownloadDolphinFlatpak() {
  await invoke("download_dolphin_flatpak");
  await UseFlatpak();
}

export async function UseBundled() {
  let config = await LoadConfig();
  config.dolphinType = DolphinType.Bundled;
  await SaveConfig(config);
}

export async function UseFlatpak() {
  let config = await LoadConfig();
  config.dolphinType = DolphinType.Flatpak;
  await SaveConfig(config);
}

export async function DownloadDolphin() {
  let config = await LoadConfig();

  let url = "";
  if (currentOperatingSystem == OperatingSystemType.Windows)
    url = DOLPHIN_LINK_WINDOWS;
  else if (currentOperatingSystem == OperatingSystemType.MacOS)
    url = DOLPHIN_LINK_MACOS;
  else if (currentOperatingSystem == OperatingSystemType.Linux)
    url = DOLPHIN_LINK_LINUX;

  let path = await invoke("download_tool", { url: url, foldername: "Dolphin" });

  if (currentOperatingSystem == OperatingSystemType.Windows)
    config.dolphinPath = path + "/Dolphin.exe";
  else if (currentOperatingSystem == OperatingSystemType.MacOS)
    config.dolphinPath = path + "/Dolphin.app";
  else if (currentOperatingSystem == OperatingSystemType.Linux)
    config.dolphinPath = path + "/dolphin-emu";
  await invoke("dolphin_auto_set_custom_textures", {});
  await invoke("create_portable", { dolphinpath: config.dolphinPath });
  await SaveConfig(config);

  await UseBundled();
}

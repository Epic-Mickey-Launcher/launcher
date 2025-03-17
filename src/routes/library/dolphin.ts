import {invoke} from "@tauri-apps/api/core";
import {LoadConfig, SaveConfig} from "./config";
import {serverLink} from "./networking";

export const DOLPHIN_LINK_WINDOWS =
    serverLink + "tool/download?tool=dolphin&target=windows";
export const DOLPHIN_LINK_LINUX = serverLink + "tool/download?tool=dolphin&target=linux";
export const DOLPHIN_LINK_MACOS = serverLink + "tool/download?tool=dolphin&target=macos";

export async function DownloadDolphin() {
    let config = await LoadConfig()
    let os: string = await invoke("get_os")

    let url = "";
    if (os == "windows") url = DOLPHIN_LINK_WINDOWS;
    else if (os == "macos") url = DOLPHIN_LINK_MACOS;
    else if (os == "linux") url = DOLPHIN_LINK_LINUX;

    let path = await invoke("download_tool", {url: url, foldername: "Dolphin"})

    if (os == "windows") config.dolphinPath = path + "/Dolphin.exe";
    else if (os == "macos") config.dolphinPath = path + "/Dolphin.app";
    else if (os == "linux") config.dolphinPath = path + "/dolphin-emu";

    await invoke("create_portable", {dolphinpath: config.dolphinPath});
    await SaveConfig(config)
}
<svelte:options accessors={true} />

<script>
    import { invoke } from "@tauri-apps/api";
    import { ReadFile, ReadJSON, WriteFile } from "../library/configfiles";
    import { GetData } from "../library/datatransfer";
    import { GetModIconPath, GetToken, POST, serverLink } from "../library/networking";

    import ModInstall from "./ModInstall.svelte";
    import { exists } from "@tauri-apps/api/fs";

    let gamedata;
    let moddata;

    export let downloadButtonStatus = "";
    export let downloadButtonDisabled = false;
    export let canupdate = false;
    export let updatecb = () => {};
    export let downloading = false;

    export async function Initialize(_gamedata, local, _moddata, overrideCheck = false) {
        moddata = _moddata;
        gamedata = _gamedata;
        if (!overrideCheck) {
            await CheckIfDownloaded();
        }
    }

    async function GetJsonData() {
        let jsonData = await ReadJSON("games.json");
        return jsonData;
    }

    async function CheckIfDownloaded() {
        let haveGame = false;

        let platform = moddata.Platform;

        if (platform == undefined) {
            platform = "wii";
        }

        console.log(platform)

        if (gamedata.platform == platform && gamedata.game == moddata.Game) {
            haveGame = true;
        }

        if (haveGame) {
            let dataStr = await ReadFile(gamedata.path + "/EMLMods.json");
            let dataJson = JSON.parse(dataStr);
            let json = dataJson.find((r) => r.modid == moddata.ID);
            downloadButtonStatus = "Download";
            if (json != null) {
                if (json.update != moddata.Version) {
                    canupdate = true;
                    downloadButtonStatus = "Update Available";
                } else {
                    downloadButtonDisabled = true;
                    downloadButtonStatus = "Already Installed";
                }
            }
        } else {
            downloadButtonDisabled = true;
            downloadButtonStatus = `${moddata.Game} (${platform}) not installed!`;
        }

        updatecb()
    }

    export async function Download() {
        downloading = true;
        let gameid = gamedata.id;

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.modIcon = GetModIconPath(moddata.ID);
        modInstallElement.modName = moddata.Name;
        modInstallElement.showDownloadProgression = true;

        let datastring = await ReadFile(gamedata.path + "/EMLMods.json");
        let data = JSON.parse(datastring);
        let existingmod = data.find((r) => r.modid == moddata.ID);

        let platform = gamedata.platform;

        if (canupdate) {
            modInstallElement.action = "Updating";
            await invoke("delete_mod", {
                json: JSON.stringify(existingmod),
                dumploc: gamedata.path,
                gameid: gameid,
                platform: platform,
                modid: moddata.ID,
                active: existingmod.active,
            });
            let delete_index = data.indexOf(existingmod);
            data.splice(delete_index, 1);
            await WriteFile(
                JSON.stringify(data),
                gamedata.path + "/EMLMods.json",
            );
            await invoke("delete_mod_cache", { modid: moddata.ID });
        }

        if (platform == null) {
            platform = "wii";
        }

        await invoke("download_mod", {
            url: serverLink + "mod/download?id=" + moddata.ID,
            name: moddata.Name,
            dumploc: gamedata.path,
            modid: moddata.ID.toString(),
            gameid: gameid,
            platform: platform,
        })
            let json_exists = await exists(gamedata.path + "/EMLMods.json");
            let current_mods = [];
            if (json_exists) {
                current_mods = JSON.parse(
                    await ReadFile(gamedata.path + "/EMLMods.json"),
                );
            }

            current_mods.push({
                name: moddata.Name,
                modid: moddata.ID,
                active: true,
                update: moddata.Version,
            });

            await WriteFile(
                JSON.stringify(current_mods),
                gamedata.path + "/EMLMods.json",
            );
            modInstallElement.$destroy();
            let token = await GetToken();
            await POST("addmodimpression", {
                token: token,
                modid: moddata.ID,
                impression: { download: true, like: false },
            });
            CheckIfDownloaded();
        downloading = false;
    }
</script>

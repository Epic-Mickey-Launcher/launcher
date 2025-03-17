<script lang="ts">
    import {Game, type GameConfig, Platform} from "../library/types";
    import {LoadGamesConfig} from "../library/config";

    let selectRegion = $state(false);
    let cb: any;
    let modID: string;
    let version: number;
    let filteredGames: GameConfig[] = $state();

    export async function GetValidGame(
        _game: Game,
        _platform: Platform,
        _modID: string,
        _version: number,
        callback: any,
    ) {
        let gamesConfig = await LoadGamesConfig();
        console.log(gamesConfig);
        filteredGames = gamesConfig.filter(
            (r) =>
                r.game.toUpperCase() == _game.toUpperCase() &&
                r.platform.toUpperCase() == _platform.toUpperCase(),
        );

        console.log(filteredGames);

        modID = _modID;
        version = _version;

        if (filteredGames.length == 0) {
            callback(null);
            return;
        }

        if (filteredGames.length > 1) {
            CheckIfDownloaded();
        } else {
            let result = await CheckInstall(filteredGames[0].path);
            if (result != "installed") {
                callback(filteredGames[0]);
            } else {
                callback(null);
            }
            return;
        }
    }

    async function CheckInstall(path: string) {
        let dataJson = await LoadGamesConfig();
        let json = dataJson.find((r: { modid: any }) => r.modid == modID);
        if (json != null) {
            if (json.update != version) {
                return "update";
            } else {
                return "installed";
            }
        }
        return "none";
    }

    async function CheckIfDownloaded() {
        let haveGame = false;

        for (let i = 0; i < filteredGames.length; i++) {
            let res = await CheckInstall(filteredGames[i].path);
            filteredGames[i].installed = res;
        }

        let allInstalled = filteredGames.filter((r) => r.installed == "installed");
        if (allInstalled.length == filteredGames.length) {
            cb(null);
            return;
        }

        selectRegion = true;
    }
</script>

{#if selectRegion}
    <div class="selectregion">
    <span style="position:relative;top:250px;">
      <span>Select region to download to</span>
      <p></p>
      <div
              style="background-color: rgb(38 38 38); width:20%; height:30%;  border-radius:10px; margin:auto; filter:drop-shadow(0 0 5px black)"
      >
        {#each filteredGames as region}
          {#if region.installed != "installed"}
            <button style="width:100%;" onclick={() => cb(region)}
            >{region.region}</button
            >
          {:else}
              <button style="width:100%;" disabled>{region.region}</button>
          {/if}
            <br/>
        {/each}
      </div>
      <p>
        <button onclick={() => cb(null)} class="hyperlinkbutton">Back</button>
      </p></span
    >
    </div>
{/if}

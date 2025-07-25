<script lang="ts">
  import { mount, onMount, unmount } from "svelte";
  import ModInstall from "./ModInstall.svelte";
  import type { GameInstance } from "../library/instance.svelte";
  import type { InstalledMod } from "../library/types";

  interface Props {
    modData: InstalledMod;
    gameInstance: GameInstance;
  }

  let { modData, gameInstance }: Props = $props();
  let checkBox: HTMLInputElement = $state();
  let setStateAllowed = $state(true);

  onMount(async () => {
    checkBox.checked = modData.active;
    setStateAllowed = !modData.local;
  });

  async function DeleteMod() {
    let modInstallElement = mount(ModInstall, {
      target: document.body,
    });
    modInstallElement.modIcon = "/img/waren.png";
    modInstallElement.modName = modData.name;
    modInstallElement.action = "Deleting";
    modInstallElement.description = "This might take a while...";

    console.log("deleting");

    let instance = gameInstance as GameInstance;
    await instance.RemoveMod(modData.modid);
    await unmount(modInstallElement);
  }

  async function ToggleMod() {
    let modInstallElement = mount(ModInstall, {
      target: document.body,
      props: {
        modIcon: "/img/waren.png",
        modName: modData.name,
        action: !modData.active ? "Enabling" : "Disabling",
        description: "This might take a while...",
      },
    });

    let instance = gameInstance as GameInstance;
    await instance.SetModActive(modData.modid, !modData.active);

    await unmount(modInstallElement);
  }
</script>

<div
  style="background-color:rgb(22, 22, 22);padding:10px;width:50%;margin-top: 6px;margin-bottom: 6px;border-radius: 8px;"
>
  <span>{modData.name}</span>
  <button
    onclick={DeleteMod}
    style="background-color:red;float:right;bottom:2.5px;position: relative;border:none;border-radius: 4px;"
    >Delete</button
  >

  {#if setStateAllowed}
    <input
      bind:this={checkBox}
      style="float:right;margin-right:16px;"
      id="check"
      onclick={ToggleMod}
      type="checkbox"
    />
    <span style="float:right;">Enabled: |</span>
  {:else}
    <span> | <span style="color:darkgray;">Local Mod</span></span>
  {/if}
</div>

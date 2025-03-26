<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  let progress = $state("0");
  let MBTotal = $state(0);
  let MBDownloaded = $state(0);

  function formatBytes(bytes: number, decimals = 2) {
    if (!+bytes) return "0 Bytes";

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = [
      "Bytes",
      "KiB",
      "MiB",
      "GiB",
      "TiB",
      "PiB",
      "EiB",
      "ZiB",
      "YiB",
    ];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
  }

  onMount(async () => {
    console.log("Listening for download-stat");

    await listen("download-action", (event: any) => {
      action = event.payload;
    });

    await listen("download-description", (event: any) => {
      description = event.payload;
    });

    await listen("download-stat", (event: any) => {
      MBTotal = event.payload.download_total;
      MBDownloaded = event.payload.download_remaining;
      if (MBTotal > 0 && MBDownloaded > 0) {
        progress = ((MBDownloaded / MBTotal) * 100).toString();
        showDownloadProgression = true;
      } else {
        showDownloadProgression = false;
      }

      if (event.payload.action != "") {
        action = event.payload.action;
      }

      if (event.payload.description != "") {
        description = event.payload.description;
      }
    });
  });

  interface Props {
    modName?: string;
    modIcon?: string;
    showDownloadProgression?: boolean;
    action?: string;
    description?: string;
  }

  let {
    modName = "",
    modIcon = "",
    showDownloadProgression = $bindable(false),
    action = $bindable("Downloading"),
    description = $bindable(
      "This might take a while depending on your internet speed.",
    ),
  }: Props = $props();

  export { modName, modIcon, showDownloadProgression, action, description };
</script>

<div class="installing">
  <div
    style="position:fixed;align-items: center; align-self: center;text-align: center;left:0; right:0; top:30%; "
  >
    <div style="position: relative;">
      <img
        alt=""
        class="loading-spinner"
        src="/img/Loading_indicator_circle.svg"
      />
      <img alt="" class="installingmodlogo" src={modIcon} />
    </div>
    <p></p>
    <span>{action} {modName}</span>
    <p></p>
    <span>{description}</span>
    {#if showDownloadProgression}
      <p></p>
      <span>{formatBytes(MBDownloaded)} / {formatBytes(MBTotal)}</span>
      <p></p>
      <progress value={progress} max="100"></progress>
    {/if}
  </div>
</div>

<style>
  .installing {
    width: 100%;
    height: 100%;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: rgba(0, 0, 0, 0.3);
    border-radius: 5px;
    -webkit-backdrop-filter: blur(10px);
    position: fixed;
    backdrop-filter: blur(10px);
    z-index: 500;
    align-items: center;
    align-self: center;
    text-align: center;
    display: flex;
  }

  .installingmodlogo {
    position: absolute;
    border-radius: 100%;
    top: 25px;
    left: 0;
    right: 0;
    margin-left: auto;
    margin-right: auto;
    width: 150px;
    height: 150px;
  }

  .loading-spinner {
    animation: rotate 1.5s linear infinite;
    margin-left: auto;
    margin-right: auto;
    width: 200px;
    height: 200px;
  }

  @keyframes rotate {
    to {
      transform: rotate(360deg);
    }
  }
</style>

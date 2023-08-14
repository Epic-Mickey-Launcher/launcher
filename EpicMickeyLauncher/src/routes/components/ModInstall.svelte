<svelte:options accessors={true} />


<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
    import { onMount } from "svelte";
    import { emit, listen } from '@tauri-apps/api/event'

  export let modName = "";
  export let modIcon = "";

  let MBTotal = 0;
  let MBDownloaded = 0;

  export let showDownloadProgression = false

  function formatBytes(bytes, decimals = 2) {
    if (!+bytes) return '0 Bytes'

    const k = 1024
    const dm = decimals < 0 ? 0 : decimals
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']

    const i = Math.floor(Math.log(bytes) / Math.log(k))

    console.log(bytes)

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
}


 onMount(async () => {

    console.log("Listening for download-stat")
    
   const lis = await listen("download-stat", (event) => {
    
    MBTotal = event.payload.Download_Total
    MBDownloaded = event.payload.Download_Remaining
   })
  
  })


  export let action = "Downloading";
  export let description =
    "This might take a while depending on your internet speed.";
</script>

<div class="installing">
  <div
    style="position:fixed;align-items: center; align-self: center;text-align: center;left:0; right:0; top:30%; "
  >
    <img
      class="loading-spinner"
      alt=""
      src="/img/Loading_indicator_circle.svg"
    />
    <plaintext>{action} {modName}</plaintext>
    <plaintext>{description}</plaintext>
    {#if showDownloadProgression}
    <plaintext>{formatBytes(MBDownloaded)} / {formatBytes(MBTotal)}</plaintext>
    {/if}
    <img class="installingmodlogo" alt="" src={modIcon} />
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
  }

  .installingmodlogo {
    position: fixed;
    top: 33%;
    left: 45.3%;
    border-radius: 100%;
    bottom: 0;
    width: 150px;
    height: 150px;
  }

  .loading-spinner {
    animation: rotate 1.5s linear infinite;
    width: 200px;
    height: 200px;
  }

  @keyframes rotate {
    to {
      transform: rotate(360deg);
    }
  }
</style>

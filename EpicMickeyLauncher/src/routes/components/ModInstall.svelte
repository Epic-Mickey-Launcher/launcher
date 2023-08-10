<svelte:options accessors={true} />

<script>
  export let modName = "";
  export let modIcon = "";

  export let MBTotal = 0;
  export let MBDownloaded = 0;

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
    {#if MBTotal != 0}
    <plaintext>25MB / 100MB</plaintext>
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

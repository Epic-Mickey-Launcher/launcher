<script lang="ts">
  import { onMount } from "svelte";
  import { serverLink } from "../library/networking";

  let token: string = "";
  let captchaFrame: HTMLIFrameElement;
  onMount(() => {});

  export function GetToken() {
    return token;
  }
  export function Refresh() {
    captchaFrame.src = captchaFrame.src;
  }

  function Hook() {
    window.addEventListener("message", (event) => {
      token = event.data;
    });
  }
</script>

<iframe
  bind:this={captchaFrame}
  onload={() => Hook()}
  title="Cloudflare Turnstile"
  style="border:none; width:308px;height:81px;"
  src={serverLink + "captcha/new"}
></iframe>

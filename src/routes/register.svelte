<script lang="ts">
  import { onMount } from "svelte";
  import Dialog from "./components/dialog.svelte";
  import { Subscribe } from "./library/callback";
  import { POST, Register, SignIn, UserInfo } from "./library/networking";
  import { GetBackgroundLogin } from "./library/background";
  import Loading from "./components/loading.svelte";

  let user: any;
  let pass: any;
  let loadingDialog: HTMLDialogElement;
  let background: HTMLDivElement;
  let email: any;
  let forgotPasswordDialog: HTMLDialogElement;

  onMount(() => {
    background.style.backgroundImage = `url(${GetBackgroundLogin()})`;
  });

  async function SendEmail() {
    let response = await POST("user/otp", { email: email }, false);
    if (response.error) return;
  }

  async function Login(type: number) {
    loadingDialog.showModal();
    Subscribe(
      "SignedIn",
      (c: { error: number }) => {
        if (c.error != 1) {
          window.open("#/profilepage", "_self");
        }
      },
      false,
    );

    let userInfo: UserInfo = { username: user, password: pass };

    if (type == 1) {
      //login
      await SignIn(userInfo);
    } else {
      //register
      await Register(userInfo);
    }
    loadingDialog.close();
  }
</script>

<div
  bind:this={background}
  style="background-attachment:fixed;position: fixed;width:100vw;height:100vh;top:0px;z-index:-1;background-image:url(img/backgrounds/back1.webp);background-position:center;background-size:cover;"
></div>

<main>
  <div style="text-align:center;">
    <h1 style="filter:drop-shadow(0 0 3px black)">Register / Login</h1>
    <hr />
    <input
      class="inputfield"
      bind:value={user}
      placeholder="Username / E-Mail"
    />
    <p style="margin-top: 2px;">
      <input
        class="inputfield"
        bind:value={pass}
        placeholder="Password"
        type="password"
      />
    </p>

    <dialog bind:this={loadingDialog}>
      <span>Logging in...</span>
    </dialog>
    <dialog bind:this={forgotPasswordDialog}>
      <span
        >If you have an E-Mail linked to your account, you can request a
        One-Time-Password so that you may log into your account and change your
        Password.</span
      >
      <p>
        <input placeholder="E-Mail" class="inputfield" bind:value={email} />
      </p>

      <button on:click={SendEmail}>Send Request</button>
      <button on:click={() => forgotPasswordDialog.close()}>Back</button>
    </dialog>
    <button
      on:click={() => forgotPasswordDialog.showModal()}
      class="hyperlinkbutton">Forgot your password?</button
    >
    <p>
      <button class="registerbutton" on:click={() => Login(2)}>Register</button>
      <button class="registerbutton" on:click={() => Login(1)}>Sign In</button>
    </p>
    <div style="margin-top:10vh;">
      <Dialog
        content={[
          "Make sure to use a unique password from your other accounts for optimal security!",
          "All password are hashed with the Bcrypt algorithm in our database!",
          "Our <a href='https://placeholder.trololol'>server source code</a> is completely open for anyone to view and use!",
        ]}
      ></Dialog>
    </div>
  </div>
</main>

<style>
  .registerbutton {
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
  }
  .inputfield {
    border: none;
    font-size: 20px;
    padding: 5px;
    border-radius: 5px;
  }
</style>

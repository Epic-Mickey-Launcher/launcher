<script lang="ts">
  import Dialog from "./components/dialog.svelte";
  import { POST, securitySettings } from "./library/networking";
  import { LoginWithPassword, Register } from "./library/account";
  import { GetBackgroundLogin } from "./library/background";
  import Captcha from "./components/captcha.svelte";

  let user: string = $state();
  let pass: string = $state();
  let mail: string = $state();
  let loadingDialog: HTMLDialogElement = $state();
  let background: HTMLDivElement = $state();
  let resetPasswordEmail: any = $state();
  let forgotPasswordDialog: HTMLDialogElement = $state();
  let registering: boolean = $state(false);
  let captcha: Captcha = $state();
  let captchaForgotPassword: Captcha = $state();

  $effect(() => {
    background.style.backgroundImage = `url(${GetBackgroundLogin().path})`;
  });

  async function SendEmail() {
    if (captchaForgotPassword.GetToken() == "") {
      await alert("Please complete the captcha before proceeding.");
      return;
    }

    let res = await POST(
      "user/otp",
      { email: resetPasswordEmail, captcha: captchaForgotPassword.GetToken() },
      false,
      false,
    );
    if (!res.error) {
      await alert("Request sent!");
    }
    forgotPasswordDialog.close();
  }

  async function Login(type: number) {
    let token = "";
    if (securitySettings.RegistrationRequiresCaptcha && registering) {
      token = captcha.GetToken();
      if (token == "") {
        await alert("Please complete the captcha before proceeding.");
        return;
      }
    }

    loadingDialog.showModal();

    let result: boolean = false;
    if (type == 1) {
      //login
      result = await LoginWithPassword(user, pass);
    } else {
      //register
      if (user.includes("@")) {
        await alert(
          "You are not allowed to use your E-Mail as your username upon registration!",
        );
        return;
      }
      result = await Register(user, pass, mail, token);
      if (result == true) {
        if (mail != "") {
          await alert(
            "Check your E-Mail for a confirmation to properly bind it to your account. Be sure to check your spam folder if its not in your inbox.",
          );
        }
      } else if (securitySettings.RegistrationRequiresCaptcha) {
        captcha.Refresh();
      }
    }

    if (result) {
      window.open("#/profilepage", "_self");
    } else loadingDialog.close();
  }
</script>

<div
  bind:this={background}
  style="background-attachment:fixed;position: fixed;width:100vw;height:100vh;top:0px;z-index:-1;background-position:center;background-size:cover;"
></div>

<main>
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
      <input
        bind:value={resetPasswordEmail}
        class="inputfield"
        placeholder="E-Mail"
      />
    </p>
    <Captcha bind:this={captchaForgotPassword}></Captcha>
    <br />
    <button onclick={SendEmail}>Send Request</button>
    <button onclick={() => forgotPasswordDialog.close()}>Back</button>
  </dialog>
  <h1 style="text-align: center;">{registering ? "Register" : "Sign In"}</h1>
  <hr />
  <div style="text-align:center;width:100%;">
    <div style="display:flex; justify-content: center;">
      <div
        style="display:flex;  flex-direction: column;width:30%;justify-self:center;justify-items:center;gap:3px;"
      >
        <input
          bind:value={user}
          class="inputfield"
          placeholder={registering ? "Username" : "Username / E-Mail"}
        />

        {#if registering}
          <input
            bind:value={mail}
            class="inputfield"
            placeholder="E-Mail (Optional)"
          />
        {/if}
        <input
          bind:value={pass}
          class="inputfield"
          placeholder="Password"
          type="password"
        />
      </div>
    </div>

    {#if !registering}
      <p>
        <button
          class="hyperlinkbutton"
          onclick={() => forgotPasswordDialog.showModal()}
          >Forgot your password?
        </button>
      </p>
      <p>
        <button class="hyperlinkbutton" onclick={() => (registering = true)}
          >Don't have an account? <b>Register!</b></button
        >
      </p>
    {:else}
      <p>
        <button class="hyperlinkbutton" onclick={() => (registering = false)}
          >Already have an account? <b>Login!</b></button
        >
      </p>
    {/if}
    {#if securitySettings.RegistrationRequiresCaptcha && registering}
      <Captcha bind:this={captcha}></Captcha>
    {/if}

    <p>
      {#if registering}
        <button class="registerbutton" onclick={() => Login(2)}>Register</button
        >
      {:else}
        <button class="registerbutton" onclick={() => Login(1)}>Sign In</button>
      {/if}
    </p>
    <div style="margin-top:10vh;">
      <Dialog
        content={[
          "Make sure to use a unique password for optimal security!",
          "All password are hashed with the Bcrypt algorithm in our database!",
          "Our server source code is completely open for anyone to view and use!",
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

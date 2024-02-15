<script>
    import Dialog from "./components/dialog.svelte";
   import { Subscribe } from "./library/callback";
   import { Register, SignIn } from "./library/networking";

   let user;
   let pass;

   async function Login(type) {
      Subscribe(
         "SignedIn",
         (c) => {
            if (c.error != 1) {
               window.open("#/profilepage", "_self");
            }
         },
         false
      );

      if (type == 1) {
         //login
         await SignIn({ username: user, password: pass });
      } else {
         //register
         await Register({ username: user, password: pass });
      }
   }
</script>

<main>

   <div style="text-align:center;">
      <h1>Register / Login</h1>
      <hr />
      <input bind:value={user} placeholder="Username" />
      <br />
      <input bind:value={pass} placeholder="Password" type="password" />
      <p>
         <button on:click={() => Login(2)}>Register</button>
         <button on:click={() => Login(1)}>Sign In</button>
      </p>
      <Dialog content="Make sure to use a unique password from your other accounts for optimal security!"></Dialog>
   </div>
</main>

<style>

</style>
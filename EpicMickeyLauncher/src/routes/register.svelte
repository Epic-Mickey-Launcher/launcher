<script>
    import { Subscribe } from "./library/callback";
    import { Register, SignIn } from "./library/networking";

    let user;
    let pass;


   async function Login(type){
      
      Subscribe("SignedIn", (c) => {
      if(c.error != 1)
      {
         window.open("#/profilepage", "_self")
      }
   }, false)

   if(type == 1)
   {
//login
await SignIn({username:user, password:pass});

   }
   else{
//register
await Register({username:user, password:pass})
   }
    }

</script>
<h1>Register / Login</h1>
<hr>
<input bind:value={user} placeholder="Username"> 
<br>
<input bind:value={pass} placeholder="Password" type="password">
<p>
<button on:click={() => Login(2)}>Register</button> <button on:click={() => Login(1)}>Sign In</button>
<p style="font-size:10px;">PS. PLEASE do not use any password you use on any other sites, this is my first time releasing something like this to the public, so the chance of a security breach is way higher than usual!!</p>

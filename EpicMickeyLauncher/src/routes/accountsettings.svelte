<script>
    import { onMount } from "svelte";
    import { GetUserInfo, OnSignedIn, POST } from "./library/networking";

    let username;
    let password;
    let bio;
    let pfpdata;

    let files;
    $: if (files) {
		let file = files[0];

        if(file.name.endsWith(".png")){
            getpfpdata(file)
        }
        else{
            console.log("pfp needs to be a png file")
        }
	}
    async function ApplyChanges(){
         let userinfo = await GetUserInfo()
         if(userinfo != null){
            let data = {username: username.value, bio: bio.value, password: password.value, pfpdata: pfpdata, token:userinfo.token}
         let res = await POST("changeaccountsettings", data)
         switch(res.error){
           case 0:
            alert("All changes have been applied successfully!")
            window.open("#/profilepage", "_self")
            break
            case 1:
            alert("Person with the same username exists.")
                break
                case 2:
                alert("Password is less than 8 characters.")
                    break
                    case 3:
                        alert("pfp")
                        break 
        }
         }
    }
    function getpfpdata(file) {
   var reader = new FileReader();
   reader.readAsDataURL(file);
   reader.onload = function () {
      pfpdata = reader.result;
   };
   reader.onerror = function (error) {
     
   };
}

    let cb = (userinfo) => {

        

    }

    onMount(() => {
        OnSignedIn(cb)
           //confirm("Are you sure you want to delete your account? Anything you have published and not deleted will remain on the platform, however, all of your account data will be erased.")
    })
</script>

<span>Change Username:</span> <input bind:this={username} placeholder="Jimbob83">
<p>
<span style="display:flex;">
    <span style="margin:auto 0;" >Change Bio:</span> <textarea cols="30" bind:this={bio} placeholder="i like video games"></textarea>
</span>
<p>
    <span>Change Password:</span> <input placeholder="New Password" bind:this={password} type="password">
<p>
<span>Upload a new profile picture: </span> <input bind:files={files} type="file"> <img src="img/waren.png" alt="" style="width:30px;margin-bottom:-10px;">

<p>
<button>Log Out</button>
<p></p>
<button>Delete Account</button>
<p></p>
<button style="width:30%;" on:click={ApplyChanges}>Apply Changes</button>
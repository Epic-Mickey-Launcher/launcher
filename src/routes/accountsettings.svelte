<script>
    import { onMount } from "svelte";
    import {
        GetToken,
        GetUserInfo,
        OnSignedIn,
        POST,
        SetLoggedIn,
        loggedin,
    } from "./library/networking";
    import { SetData } from "./library/datatransfer";
    import { WriteToken } from "./library/configfiles";

    let username;
    let password;
    let bio;
    let pfpdata;

    let files;
    $: if (files) {
        let file = files[0];
            getpfpdata(file);
    }
    async function ApplyChanges() {
        let userinfo = await GetUserInfo();
        if (userinfo != null) {
            let data = {
                username: username.value,
                bio: bio.value,
                password: password.value,
                pfpdata: pfpdata,
                token: userinfo.token,
            };
            let res = await POST("changeaccountsettings", data);
            switch (res.error) {
                case 0:
                    alert("All changes have been applied successfully!");
                    window.open("#/profilepage", "_self");
                    break;
                case 1:
                    alert("Person with the same username exists.");
                    break;
                case 2:
                    alert("Password is less than 8 characters.");
                    break;
                case 3:
                    alert("Profile Picture Error!");
                    break;
            }
        }
    }

    async function Logout() {
        WriteToken("");
        SetLoggedIn(false);
        window.open("#/", "_self");
    }

    async function DeleteAccount() {
        let confirmation = await confirm(
                "Are you sure you want to delete your account? Any comments, mods & likes from your account will remain unless they are manually deleted."
            );
        if (
          confirmation  
        ) {
            let token = await GetToken();
            let res = await POST("deleteacc", { token: token });

            if (res.error === 0) {
                WriteToken("");

                window.open("#/", "_self");
            }
        }
    }

    function getpfpdata(file) {
        var reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = function () {
            pfpdata = reader.result;
        };
        reader.onerror = function (error) {};
    }

    let cb = (userinfo) => {

    

    };

    onMount(() => {
        OnSignedIn(cb);
        //confirm("Are you sure you want to delete your account? Anything you have published and not deleted will remain on the platform, however, all of your account data will be erased.")
    });
</script>

<span>Change Username:</span>
<input bind:this={username} placeholder="Jimbob83" />
<p>
    <span style="display:flex;">
        <span style="margin:auto 0;">Change Bio:</span>
        <textarea cols="30" bind:this={bio} placeholder="i like video games" />
    </span>
</p>
<p>
    <span>Change Password:</span>
    <input placeholder="New Password" bind:this={password} type="password" />
</p>
<p>
    <span>Upload a new profile picture: </span> <input accept="image/*" bind:files type="file" />
    <img src="img/waren.png" alt="" style="width:30px;margin-bottom:-10px;" />
</p>
<p>
    <button on:click={Logout}>Log Out</button>
</p>
<p />
<button on:click={DeleteAccount}>Delete Account</button>
<p />
<button style="width:30%;" on:click={ApplyChanges}>Apply Changes</button>

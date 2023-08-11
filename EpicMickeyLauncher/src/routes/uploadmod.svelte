<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { GetToken, POST, UploadMod } from "./library/networking";
    import { WriteFile } from "./library/configfiles";
    import ModInstall from "./components/ModInstall.svelte";
    import { onMount } from "svelte";
    import { GetData, SetData } from "./library/datatransfer";

    let replacingMod;

    let automaticallyPublish;

    let uploadModDiv;
    let waitDiv;
    let resultDiv;
    let largeMod;

    function modIsLarge() {
        uploadModDiv.style.display = "none";
        largeMod.style.display = "block";
    }

    onMount(async () => {
        replacingMod = GetData("modupload_id");
        SetData("modupload_id", "");
    });

    let files;

    $: if (files) {
        let file = files[0];

        if (file.name.endsWith(".zip") || file.name.endsWith(".tar")) {
            uploadFile(file, () => {
                waitDiv.style.display = "none";
                resultDiv.style.display = "block";
            });
        } else {
        }
    }

    function dropFile() {}

    let inputlink;

    let result = "Success!";

    async function UploadLink() {
        let token = await GetToken();

        let modInstallElement = new ModInstall({
            target: document.body,
        });
        modInstallElement.action = "Downloading";
        modInstallElement.modIcon = "img/icon.png";
        modInstallElement.modName = "your mod for verification purposes.";

        invoke("validate_mod", { url: inputlink.value, local: false }).then(
            async (v) => {
                if (v.validated) {
                    modInstallElement.action = "Uploading";
                    modInstallElement.modIcon = "img/icon.png";
                    modInstallElement.modName = v.modname;
                    await POST("moduploadnonhosted", {
                        token: token,
                        link: inputlink.value,
                        replacing: replacingMod,
                        automaticPublish: automaticallyPublish.value
                    });
                    replacingMod = null;
                    modInstallElement.$destroy();
                    result =
                        "Mod Request Successful! Your mod will be available in the mod market once it has been verified.";
                    waitDiv.style.display = "none";
                    largeMod.style.display = "none";
                    resultDiv.style.display = "block";
                } else {
                    modInstallElement.$destroy();
                    await alert("Mod Request Failed!")
                }
            }
        );
    }

    async function uploadFile(file, cb) {
        if (file.size > 100000000) {
            modIsLarge();
        } else {
            uploadModDiv.style.display = "none";
            waitDiv.style.display = "block";
        
            UploadMod(file, cb, replacingMod, file.type, automaticallyPublish.value);
            replacingMod = null;
        }
    }
</script>

<div style="display:flex;flex-direction:column;justify-content:center;align-items:center;" bind:this={uploadModDiv}>
    <button on:click={modIsLarge}
        >My mod is larger than 100MB</button
    >
    <p>
        <label class="inputfile" for="fileupload">
            <span style="position: relative;  top: 50%;"
                >Click the box to upload.</span
            >
        </label>
    </p>

    <span>
        <input bind:this={automaticallyPublish} type="checkbox" checked> <span>Automatically publish mod on upload.</span>
    </span>
</div>

<div style="display:none;" bind:this={largeMod}>
    <h2>
        If your mod is over 100MB, we cannot host it for you. You will have to specify a direct download to it.
    </h2>
    <h3>Platform Recommendations: Discord(Nitro), Google Drive, OneDrive & DropBox</h3>
    <input
        bind:this={inputlink}
        style="width:600px;"
        placeholder="https://downloadsite.com/mod.zip"
    />
    <button on:click={UploadLink}>Upload</button>
</div>

<div bind:this={waitDiv} style="display:none;"><h1>Please Wait...</h1></div>
<div bind:this={resultDiv} style="display:none;"><h1>{result}</h1></div>

<input
    on:drop={dropFile}
    bind:files
    id="fileupload"
    style="display:none;"
    type="file"
/>

<style>
    .inputfile {
        margin: auto;
        display: block;
        width: 500px;
        height: 500px;
        text-align: center;
        background-color: rgb(52, 52, 52);
        border: 1px solid gray;
        border-radius: 30px;
    }
</style>

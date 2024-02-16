<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { GetId, GetToken, POST, UploadMod } from "./library/networking";
    import { open } from "@tauri-apps/api/dialog";
    import ModInstall from "./components/ModInstall.svelte";
    import { onMount } from "svelte";
    import { GetData, SetData } from "./library/datatransfer";
    import { ReadFile } from "./library/configfiles";
    import { readBinaryFile } from "@tauri-apps/api/fs";
    import { Invoke, Subscribe } from "./library/callback";
    import Loading from "./components/loading.svelte";

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
        Subscribe("onModUpload", (id) => {
            setTimeout(() => {
                SetData("modpage_id", id);
                window.open("#/modpage", "_self");
            }, 1000);
        });
        let cb = async () => {
            console.log("doofoo")
            let id = await GetId();

            let request = await POST("getenterrequest", { id: id });

            if (request.modid != "") {
                setTimeout(() => {
                    console.log(request)
                    SetData("modpage_id", request.modid.toString());
                    window.open("#/modpage", "_self");
                }, 1000);
            } else {
                setTimeout(cb, 1000);
            }
        };

        Subscribe("onModUploadRequest", () => {
            setTimeout(cb, 1000);
        });
    });

    let files;

    $: if (files) {
        let file = files[0];

        if (file.name.endsWith(".zip") || file.name.endsWith(".tar")) {
            uploadFile(file, (id) => {});
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
                        extension: v.extension,
                        automaticPublish: automaticallyPublish.value,
                    });
                    replacingMod = null;
                    modInstallElement.$destroy();
                    Invoke("onModUploadRequest", {})
                    result =
                        "Mod Request Successful! You will be redirected to your mod once the server has validated and published it...";
                    waitDiv.style.display = "none";
                    largeMod.style.display = "none";
                    resultDiv.style.display = "block";
                } else {
                    modInstallElement.$destroy();
                    await alert("Mod Request Failed!");
                }
            },
        );
    }

    async function SelectFile() {
        const selectedPath = await open({
            title: "Select folder",
            directory: false,
            multiple: false,
            filters: [
                {
                    name: "Supported Archives",
                    extensions: ["zip", "tar", "7z"],
                },
            ],
        });

        let path = selectedPath.toString();

        if (path != "") {
            uploadFile(path, () => {
                window;
            });
        }
    }

    async function uploadFile(path, cb) {
        let data = await invoke("validate_archive", { path: path });

        console.log(data);

        if (!data.under_limit) {
            modIsLarge();
        } else {
            uploadModDiv.style.display = "none";
            waitDiv.style.display = "block";

            //we need to make a file class since multer doesn't allow normal bytes

            let binary = await readBinaryFile(path);

            let file = new File([binary], "mod", {
                type: "application/octet-stream",
            });

            UploadMod(
                file,
                cb,
                replacingMod,
                data.extension,
                automaticallyPublish.value,
            );
            replacingMod = null;
        }
    }
</script>

<h1>Upload Mod</h1>

<hr />

<div
    style="display:flex;flex-direction:column;justify-content:center;align-items:center;position:absolute;top:50%;left:40%;"
    bind:this={uploadModDiv}
>
    <button on:click={modIsLarge}>My mod is larger than 100MB</button>

    <p>
        <button on:click={SelectFile}>Upload File</button>
    </p>
    <p>
        <span>
            <input bind:this={automaticallyPublish} type="checkbox" checked />
            <span>Automatically publish mod on upload.</span>
        </span>
    </p>
</div>

<div style="display:none;" bind:this={largeMod}>
    <h2>
        If your mod is over 100MB, we cannot host it for you. You will have to
        specify a direct download to it.
    </h2>
    <h3>
        Platform Recommendations: Discord(Nitro), Google Drive, OneDrive &
        DropBox
    </h3>
    <input
        bind:this={inputlink}
        style="width:600px;"
        placeholder="https://downloadsite.com/mod.zip"
    />
    <button on:click={UploadLink}>Upload</button>
</div>

<div bind:this={waitDiv} style="display:none;"><h1>Please Wait...</h1></div>
<div bind:this={resultDiv} style="display:none;"><h1>{result}</h1><Loading></Loading></div>

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

<script lang="ts">
    import {onMount} from "svelte";
    import {Uploader} from "./library/uploader";

    $effect(() => {
        if (files) {
            // Note that `files` is of type `FileList`, not an Array:
            // https://developer.mozilla.org/en-US/docs/Web/API/FileList
            console.log(files);

            for (const file of files) {
                console.log(`${file.name}: ${file.size} bytes`);
            }
        }
    });

    let files: FileList;

    let modVerified: boolean;
    let modIconData: string;
    let modName: string;

    async function UploadMod(f) {
        Uploader.FileUpload(files[0])
    }

    async function VerifyMod() {

    }


    onMount(async () => {
        if (false) {
            alert(
                "You are on an outdated version! Please update to upload mods.",
            );
            window.open("#/", "_self");
            //return
        }
    });
</script>

<h1>Upload Mod</h1>

<hr/>
<div>
    <input bind:files on:input={UploadMod} type="file">
</div>

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

    .dropdown {
        appearance: none;
        -webkit-appearance: none;
        background-color: black;
        padding: 2px;
    }
</style>

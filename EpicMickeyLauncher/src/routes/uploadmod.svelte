
<div bind:this={uploadModDiv}>
    <label class="inputfile" for="fileupload">
        <span style="  position: relative;  top: 50%;">Click the box to upload.</span>
    </label>
</div>

<div bind:this={waitDiv} style="display:none;"><h1>Please Wait...</h1></div>
<div bind:this={resultDiv} style="display:none;"><h1>Success!</h1></div>

<input on:drop={dropFile} bind:files={files} id="fileupload" style="display:none;" type="file"/>

<style>
    .inputfile{
        margin:auto;
         display: block; 
         width:500px;
         height:500px;
         text-align: center;
         background-color: rgb(52, 52, 52); 
         border: 1px solid gray;
         border-radius: 30px;
    }
</style>

<script>
    import { UploadMod } from "./library/networking";

    let uploadModDiv;
    let waitDiv;
    let resultDiv;


    let files;

    $: if (files) {
		let file = files[0];

        if(file.name.endsWith(".zip")){
            uploadFile(file, () => {
                waitDiv.style.display = "none"
                resultDiv.style.display = "block"
            })
            uploadModDiv.style.display = "none"
            waitDiv.style.display = "block"

        }
        else{
            console.log("nuh uh")
        }
	}

    function dropFile(){
     
    }

    function uploadFile(file, cb) {
     UploadMod(file, cb)
    }

</script>

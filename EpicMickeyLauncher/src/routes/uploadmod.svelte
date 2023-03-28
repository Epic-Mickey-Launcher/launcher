<label class="inputfile" for="fileupload">
    <span style="  position: relative;  top: 50%;">Click the box to upload.</span>
</label>
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



    let files;

    $: if (files) {
		let file = files[0];

        if(file.name.endsWith(".zip")){
            uploadFile(file)
        }
        else{
            console.log("nuh uh")
        }
	}

    function dropFile(){
        console.log("farts gibbel")
    }

    function uploadFile(file) {
   var reader = new FileReader();
   reader.readAsDataURL(file);
   reader.onload = function () {
     UploadMod(reader.result)
   };
   reader.onerror = function (error) {
     
   };
}

</script>

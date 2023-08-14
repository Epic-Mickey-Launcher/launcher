import {ReadFile, FileExists, WriteFile} from "./configfiles.js"
import { invoke } from "@tauri-apps/api/tauri";

export async function ConvertModJsonToNew(_path)
{
   let modjsonpath = _path + "/EMLMods.json";

   let file_exists = await FileExists(modjsonpath);

   if (file_exists)
   {
       let f = await ReadFile(modjsonpath);

       let j = JSON.parse(f);

       let result_json = []

       if (j.length > 0)
       {
           if (j[0]["texturefiles"] != null)
           {
               for (let element of j)  {
                     await invoke("write_mod_info", {path: _path + "/" +element.modid, files: element.files, textures: element.texturefiles});
       
                    result_json.push({
                        name: element.name,
                        modid: element.modid,
                        active: element.active,
                        update: element.update,
                    })
       
               }
               await WriteFile(JSON.stringify(result_json), modjsonpath);
           }
       }
   }
}
import { writable } from 'svelte/store';

export const stringbuffer = writable("");
export const objectbuffer = writable({});

let dataarray = []

export function SetData(name, value){
    let data = dataarray.find(r => r.name == name)
    if(data != null){
        let index = dataarray.indexOf(data)
        dataarray.splice(index, 1)
    }
   dataarray.push({name: name, value: value})
}
export function GetData(name){ 
   let data = dataarray.find(r => r.name == name)
   if(data != null){
      return data.value;
   }
   else{
    return null;
   }
}
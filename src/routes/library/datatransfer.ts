import { writable } from "svelte/store";
import { type CachedUser } from "./types";

export const stringbuffer = writable("");
export const objectbuffer = writable({});

export let cachedUsers: CachedUser[] = [];

export interface DataEntry {
  name: string;
  value: any;
}

let dataArray: DataEntry[] = [];

export function SetData(name: string, value: any) {
  let data = dataArray.find((r) => r.name == name);
  if (data != null) {
    let index = dataArray.indexOf(data);
    dataArray.splice(index, 1);
  }
  dataArray.push({
    name: name,
    value: value,
  });
}

export function GetData(name: string) {
  let data = dataArray.find((r) => r.name == name);
  if (data != null) {
    return data.value;
  } else {
    return null;
  }
}

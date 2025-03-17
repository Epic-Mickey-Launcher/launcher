//Retrieves file with one of the aliases in entry.
import {path} from "@tauri-apps/api";
import FileAlias from "../data/filealias.json"
import type {FileAliasEntry} from "./types";
import {copyFile, exists, remove} from "@tauri-apps/plugin-fs";

export async function RetrieveFileByAlias(name: string, baseDir: string = "", modernize: boolean = true) {
    let fileAliasEntry: FileAliasEntry = FileAlias.find((r: FileAliasEntry) => r.Name == name)

    let currentAliasPath = await path.join(baseDir, fileAliasEntry.CurrentAlias);
    let currentAliasFileExists = await exists(currentAliasPath)

    if (currentAliasFileExists) {
        return currentAliasPath
    }

    for (const other of fileAliasEntry.OtherAliases) {
        let otherAliasPath = await path.join(baseDir, other);
        let otherAliasFileExists = await exists(otherAliasPath)

        if (otherAliasFileExists) {

            if (modernize) {
                await copyFile(otherAliasPath, currentAliasPath);
                await remove(otherAliasPath);
                return currentAliasPath;
            }

            return otherAliasPath
        }
    }
    return currentAliasPath
}

export async function GetCurrentAlias(name: string, baseDir: string = "") {
    let fileAliasEntry: FileAliasEntry = FileAlias.find((r: FileAliasEntry) => r.Name == name)
    if (fileAliasEntry == null) return ""
    return await path.join(baseDir, fileAliasEntry.CurrentAlias)
}
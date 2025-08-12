import { LoadConfig } from "./config";
import { invoke } from "@tauri-apps/api/core";
import { GetGameIdentity, IdentifyISO } from "./gameid";
import { DolphinType } from "./types";

export async function ExtractISO(
  source: string,
  onVerified: any,
): Promise<string> {
  let config = await LoadConfig();
  let result: string = await invoke("check_iso", {
    path: source,
    flatpak: config.dolphinType == DolphinType.Flatpak,
    dolphin: config.dolphinPath,
  });

  console.log(result);

  let identifiedGame = await IdentifyISO(result);

  if (identifiedGame == null) {
    console.log("This game is not supported by EML.");
    return "";
  }

  onVerified(identifiedGame);

  console.log(
    "Image Validated as: " +
      identifiedGame.game +
      ", " +
      identifiedGame.platform +
      ", " +
      identifiedGame.region,
  );

  let gameIdentity = GetGameIdentity(identifiedGame.game);

  return await invoke("extract_iso", {
    gamename: `${gameIdentity.name.toLowerCase().replaceAll(" ", "_")}_(${result.toLowerCase()})`,
    isopath: source,
    flatpak: config.dolphinType == DolphinType.Flatpak,
    dolphin: config.dolphinPath,
  });
}

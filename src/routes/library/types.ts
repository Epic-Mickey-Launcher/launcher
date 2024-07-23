export interface User {
  Username: string
  Password: string
  Bio: string
  Token: string
  ID: string
  Email: string
  EmailHash: string
}

export interface InstalledMod {
  name: string,
  modid: string,
  active: boolean,
  update: number,
}

export interface Mod {
  ID: string
  Author: string
  Version: number
  Video: string
  Game: string
  Platform: string
  Downloads: number
  Published: boolean
  Name: string
  Description: string
  RepositoryUrl: string
  Dependencies: string[]
  CachedLikes: number
}

export interface ConfigFile {
  dolphinPath: string;
  WITPath: string;
  NkitPath: string;
}

export interface Comment {
  ID: string
  Page: string
  Author: string
  Content: string
}

export interface Ticket {
  ID: string
  Action: string
  Title: string
  TargetID: string
  Meta: string
  Author: string
  ResultMessage: string
  Result: number
}

export interface Message {
  ID: string
  Content: string
  From: string
  To: string
}

export interface MessageElement {
  Type: string,
  Value: string
}

export interface GameConfig {
  path: string,
  game: Game,
  platform: Platform,
  id: string,
  region: Region,
}

export enum Game { EM1 = "EM1", EM2 = "EM2", None = "NONE" }
export enum Platform { Wii = "Wii", PC = "PC", None = "NONE" }
export enum Region {
  NTSC_U = "NTSC-U",
  NTSC_J = "NTSC-J",
  NTSC_K = "NTSC-K",
  PAL_DE_ES_IT = "PAL.DE,ES,IT",
  PAL_EN_SE_DK = "PAL.EN,SE,DK",
  PAL_SE_DK_NO = "PAL.SE,DK,NO",
  PAL_EN_FR_NL = "PAL.EN,FR,NL",
  PAL_FR_DE_IT = "PAL.FR,DE,IT",
  PAL_EN_FR_ES_NL_PT_TR = "PAL.EN,FR,ES,NL,PT,TR",
  None = ""
}

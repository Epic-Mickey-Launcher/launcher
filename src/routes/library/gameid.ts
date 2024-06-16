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
export interface GameData {
  game: Game,
  platform: Platform,
  region: Region,
  gamePath: string,
  id: string
}

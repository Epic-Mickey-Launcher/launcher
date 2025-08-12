export interface User {
  Username: string;
  Password: string;
  Bio: string;
  Token: string;
  ID: string;
  Email: string;
  EmailHash: string;
}

export interface CachedUser {
  Username: string;
  ID: string;
  CachedPfp: string;
}

export interface InstalledMod {
  name: string;
  modid: string;
  active: boolean;
  update: number;
  local: boolean;
}

export interface UnifiedMod {
  id: string;
  game: Game;
  platform: Platform;
  name: string;
  version: number;
}

export interface Mod {
  ID: string;
  Author: string;
  Version: number;
  Video: string;
  Game: string;
  Platform: string;
  Downloads: number;
  Published: boolean;
  Name: string;
  Description: string;
  ShortDescription: string;
  Dependencies: string[];
  CachedLikes: number;
  Verified: boolean;
}

export interface FileAliasEntry {
  Name: string;
  CurrentAlias: string;
  OtherAliases: string[];
}

export enum ModState {
  Installed,
  UpdateAvailable,
  NotInstalled,
  Incompatible,
}

export enum PublisherRemoteType {
  GitHub = "github",
  Git = "git",
  None = "none",
}

export enum OperatingSystemType {
  Linux = "linux",
  MacOS = "macos",
  Windows = "windows",
}

export enum DolphinType {
  Linked = "linked", // dolphin is symlinked to /usr/bin/dolphin-emu
  Bundled = "bundled", // using dolpin downloaded & provided by EML
  Flatpak = "flatpak", // using dolphin from flatpak (mostly just for Steam Deck support)
}

export interface PublisherModData {
  projectPath: string;
  sshKey: string;
  remoteType: PublisherRemoteType;
}

export interface PublisherMod {
  name: string;
  description: string;
  game: string;
  platform: string;
  icon_path: string;
  video: string;
  downloads: number;
  dependencies: string[];
  short_description: string;
  custom_textures_path: string;
  custom_game_files_path: string;
  scripts_path: string;
}

export interface ConfigFile {
  dolphinPath: string;
  dolphinType: DolphinType;
  dolphinConfigPath: string;
  developerMode: boolean;
}

export interface Comment {
  ID: string;
  Page: string;
  Author: string;
  Content: string;
}

export interface Ticket {
  ID: string;
  Action: string;
  Title: string;
  TargetID: string;
  Meta: string;
  Author: string;
  ResultMessage: string;
  Result: number;
}

export interface Message {
  ID: string;
  Content: string;
  From: string;
  To: string;
}

export interface MessageElement {
  Type: string;
  Value: string;
}

export interface GameConfig {
  path: string;
  game: Game;
  platform: Platform;
  region: Region;
  uniqueID: string;
  steamVersion: boolean;
}

export interface GameIdentity {
  name: string;
  steamID: string;
  id: Platform;
  releases: any[];
  resources: any;
}

export interface IdentifyGameResult {
  game: Game;
  platform: Platform;
  region: Region;
  identifier: string;
}

export interface IdentifyInfo {
  path: string;
  id: string;
}

export interface GameIdentityRelease {
  identifier: string;
  platform: Platform;
  regions: GameIdentityRegion[];
}

export interface GameIdentityRegion {
  region: Region;
  id: string;
}

export enum Game {
  EM1 = "EM1",
  EM2 = "EM2",
  EMR = "EMR",
  None = "NONE",
}

export enum Platform {
  Wii = "WII",
  PC = "PC",
  None = "NONE",
}

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
  None = "",
}

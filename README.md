<img src="https://eml.kalsvik.no/eml.png" width="400">

### Easiest way to Play & Mod Epic Mickey on Windows, Mac, and Linux (+ Steam Deck)

---

## Support for every mainline Epic Mickey game

_(Not Including Epic Mickey 2 on XBOX, Wii U or PS3)_

<img src="https://eml.kalsvik.no/gamesmenu.png" width="400">

## Large Mod Catalouge

_EML contains a bunch of mods uploaded by the community._

<img src="https://eml.kalsvik.no/modmarket.png" width="400">

## Networking Features

_Upload your own mods, Send comments, Vote on mods & customize your profile with an EML Account._

<img src="https://eml.kalsvik.no/profilepage.png" width="400">

## Robust Mod Management

_Easily Disable/Enable mods with one click._

<img src="https://eml.kalsvik.no/modmanagement.png" width="400">

**Made with Svelte & Tauri**

# Installation

## Windows

Download the latest .MSI or .EXE from the latest [Release](https://github.com/Epic-Mickey-Launcher/launcher/releases)

## MacOS

Download the latest .DMG or .app from the latest [Release](https://github.com/Epic-Mickey-Launcher/launcher/releases)

## Steam Deck (Linux)

Epic Mickey Launcher provides an [AppImage](https://github.com/Epic-Mickey-Launcher/launcher/releases) that can be run with one click after downloading. **Make sure that _'Install Dolphin with Flatpak'_ is CHECKED when peforming first-time setup.**

## Linux (Debian)

Download the latest .DEB from the latest [Release](https://github.com/Epic-Mickey-Launcher/launcher/releases)

## Linux (Arch)

Epic Mickey Launcher is available on the [AUR](https://aur.archlinux.org/packages/epicmickeylauncher) and can be
installed via [yay](https://github.com/Jguer/yay) with `yay -S epicmickeylauncher`

# Build Instructions

## Prerequisites

- NodeJS
- PNPM
- Rust
- Git

```
git clone https://github.com/Epic-Mickey-Launcher/launcher.git eml-source
cd eml-source
pnpm install
npm run tauri dev
```

To use a custom EML Server instance, use this environment variable: `SERVER=http://host:42069/`

# Roadmap

IDK

# Credits

- RampantLeaf: Provider of EM1 level loader data. (https://github.com/andrewplus)
- SlayCap: Heavy contributor & Provider of EM1 & EM2 level loader data. (https://x.com/SlayCap28)
- Wiimm: Creator of WIT, which EML formerly relied on for extracting image
  files (https://github.com/Wiimm/wiimms-iso-tools)
- Nanook: Creator of the NKit tools, which EML formerly relied on to convert NKIT images into normal
  images. (https://github.com/Nanook/NKitv1)
- Dolphin Project: Emulates & Extracts Wii versions of the games. (https://github.com/dolphin-emu/dolphin)
- RE-UE4SS: Used by EMR for script mods. (https://github.com/UE4SS-RE/RE-UE4SS)
- Function Point: Awesome Edition. (https://www.youtube.com/@function_point)
- Warren Spector: Director _(or creative director)_ for all of the games & also the face of the loading screens where I don't have an image to show.   
- All EML users: At age 14, I did not expect my tiny dinky project to take off and do so well. It's been a really fun
  learning project, and I have way more in store for the future. Thank you all.

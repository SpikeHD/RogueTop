<h1 align="center">
 <img height="100px" src="https://raw.githubusercontent.com/SpikeHD/roguetop/main/src-tauri/icons/icon.png" />
 <br />
 RogueTop
</h1>
<div align="center">
 <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/roguetop/build.yml" />
 <img src="https://img.shields.io/github/package-json/v/SpikeHD/roguetop" />
 <img src="https://img.shields.io/github/repo-size/SpikeHD/roguetop" />
</div>
<div align="center">
 <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/roguetop" />
 <img src="https://img.shields.io/github/release-date/SpikeHD/roguetop" />
 <img src="https://img.shields.io/github/stars/SpikeHD/roguetop" />
 <img src="https://img.shields.io/github/downloads/SpikeHD/roguetop/total" />
</div>

<div align="center">
 Online and <s>offline</s> <a href="https://github.com/pagefaultgames/pokerogue">PokeRogue</a> client for Windows, Linux and MacOS.
 <br />
 https://discord.gg/agQ9mRdHMZ
</div>

# Installation

## Quick Links

* [Windows Download](https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-pc-windows-msvc-msi.zip)
* Linux
  * [Debian-based Download](https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-unknown-linux-gnu-deb.zip)
  * [RPM-based Download](https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-unknown-linux-gnu-rpm.zip)
* MacOS
  * [Intel-based Download](https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-apple-darwin-dmg.zip)
  * [M-series-based Download](https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-aarch64-apple-darwin-dmg.zip)

> [!NOTE]
> ***MacOS Users***: If opening the app gives you "PokeRogue.app is damaged and cannot be opened", MacOS is lying to you and you may just need to run `sudo xattr -rd com.apple.quarantine /Applications/PokeRogue.app`.
>
> ***Windows Users***: Defender may think RogueTop is a virus. This just happens sometimes, and if SmartScreen blocks it from running, click "More Info" and "Run Anyways". Feel free to scan RogueTop with [Virustotal](https://www.virustotal.com/gui/home/upload)!

## Other Architectures

The file you download depends on your operating system:

* For Windows, download the `.msi` installer file, or the `pc-windows` ZIP file. Builds are also provided for arm64.
* For Linux, download the `.deb` or `.rpm` installer files, or the `unknown-linux` ZIP file. Builds are also provided for armv7 and arm64.
* For MacOS, download the `.dmg` installer file, or the `apple-darwin` ZIP file. Builds are provided for both Intel-based (x86_64) and M-series-based MacOS systems (arm64).

Releases also come in two flavors, regular and ~~"offline"~~. Regular versions are very lightweight, and will load the online version of PokeRogue. ~~Offline versions are larger, but contain the entirety of PokeRogue within the binary, and can be run without an internet connection!~~

## Alternative Installation Methods

You can also download the latest build from [GitHub Actions](https://www.github.com/SpikeHD/RogueTop/actions). Or, if you're feeling brave, you can [build it yourself](#building)!

# Table of Contents

* [Installation](#installation)
  * [Quick Links](#quick-links)
  * [Other Architectures](#other-architectures)
  * [Alternative Installation Methods](#alternative-installation-methods)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [TODO](#todo)
* [Contributing](#contributing)

# Features

* No need to play in a browser tab!
* Small binary size (when using a non-bundled binary), instant start time.
* Always up-to-date (in online mode).

# Building

## Prerequisites

* [NodeJS](https://nodejs.org/en) (or Bun, or whatever)
  * If using NodeJS, this project prefers [pnpm](https://pnpm.io) as a package manager.
* [Rust and Cargo](https://www.rust-lang.org/)
* [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (these depend on what OS you are running)

## Steps

1. Clone the repository:
  ```sh
  git clone git@github.com:SpikeHD/RogueTop.git
  ```
2. Install the dependencies:
  ```sh
  cd RogueTop
  pnpm install
  ```
3. (If bundling for offline mode, NOT required) run the bootstrap script
  ```sh
  # This will clone and build the entirety of PokeRogue into ./src-ext
  ./bootstrap_pokerogue[.sh|.cmd]
  ```
4. (If NOT bundling for offline mode) create a fake `game.dat` folder in the root of the repository to prevent build issues
5. Build it!
  ```sh
  pnpm tauri build
  # Or if you have tauri CLI installed
  cargo tauri build

  # If bundling for offline mode, add the "offline" feature
  cargo tauri build --features offline
  ```

Your built files will be in the `src-tauri/target` directory.

# TODO

- [x] Bundled offline mode (directly in the binary, eg. should work on an airgapped machine)
  - [x] Maybe move to bundled (eg. Tauri resource) but external/read from FS, no need for a 500mb binary lol
  - [ ] TODO finish this
- [ ] Downloadable offline mode (update-able without redownloading a new binary every time)
- [x] Separate "lite" builds that are online-only
- [ ] Hotkeys
  - [ ] Fullscreen
  - [ ] TBD
- [ ] Add `.plist` for MacOS to solve HTTP issue
- [ ] Plugin system/support?
- [ ] Theme system/support?

# Contributing

Issues, PRs, etc. are all welcome!

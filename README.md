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
 Online and offline <a href="https://github.com/pagefaultgames/pokerogue">PokeRogue</a> client for Windows, Linux and MacOS.
 <br />
 https://discord.gg/agQ9mRdHMZ
</div>

# Installation

<table align="center">
  <tr>
    <th>
      <img src="docs/image/windows.png" width="30%" align="center" />
    </th>
    <th>
      <img src="docs/image/apple.png" width="30%" align="center" />
    </th>
    <th>
      <img src="docs/image/debian.png" width="30%" align="center" />
    </th>
  </tr>

  <tr>
    <td width="30%">
      <div align="center">
        <span>x86_64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-pc-windows-msvc-msi.msi">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-x86_64-pc-windows-msvc-msi.msi">Offline</a>
        <br />
        <span>ARM64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-aarch64-pc-windows-msvc-nsis.exe ">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-aarch64-pc-windows-msvc-nsis.exe">Offline</a>
      </div>
    </td>
    <td width="30%">
      <div align="center">
        <span>x86_64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-apple-darwin-dmg.dmg">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-x86_64-apple-darwin-dmg.dmg">Offline</a>
        <br />
        <span>ARM64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-aarch64-apple-darwin-dmg.dmg">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-aarch64-apple-darwin-dmg.dmg">Offline</a>
      </div>
    </td>
    <td width="30%">
      <div align="center">
        <span>x86_64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-x86_64-unknown-linux-gnu-deb.deb">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-x86_64-unknown-linux-gnu-deb.deb">Offline</a>
        <br />
        <span>ARM64:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-aarch64-unknown-linux-gnu-deb.deb">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-aarch64-unknown-linux-gnu-deb.deb">Offline</a>
        <br />
        <span>ARMv7:</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-default-armv7-unknown-linux-gnueabihf-deb.deb">Online-only</a>
        <span>|</span>
        <a href="https://github.com/SpikeHD/RogueTop/releases/latest/download/roguetop-offline-armv7-unknown-linux-gnueabihf-deb.deb">Offline</a>
      </div>
    </td>
  </tr>
</table>

*You can also download portable versions for each platform, which you can find on the [releases](https://github.com/SpikeHD/RogueTop/releases/tag/nightly) page.* 

> [!NOTE]
> ***MacOS Users***: If opening the app gives you "PokeRogue.app is damaged and cannot be opened", MacOS is lying to you and you may just need to run `sudo xattr -rd com.apple.quarantine /Applications/PokeRogue.app`.
>
> ***Windows Users***: Defender may think RogueTop is a virus. This just happens sometimes, and if SmartScreen blocks it from running, click "More Info" and "Run Anyways". Feel free to scan RogueTop with [Virustotal](https://www.virustotal.com/gui/home/upload)!

## Online vs Offline

RogueTop comes in two flavors, regular and "offline". Regular versions are very lightweight (usually **<5mb**), and will load the online version of PokeRogue. Offline versions are significantly larger (around **450mb**), but contain the entirety of PokeRogue within the binary, and can be run without an internet connection! Offline versions allow access in BOTH online and offline scenarios.

## Alternative Installation Methods

You can also download the latest build from [GitHub Actions](https://www.github.com/SpikeHD/RogueTop/actions). Or, if you're feeling brave, you can [build it yourself](#building)!

# Table of Contents

* [Installation](#installation)
  * [Quick Links](#quick-links)
  * [Online vs Offline](#online-vs-offline)
  * [Alternative Installation Methods](#alternative-installation-methods)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [TODO](#todo)
* [Contributing](#contributing)

# Features

* Full offline mode
* Mod support - see the [examples and documentation](https://github.com/SpikeHD/RogueTop/tree/main/examples)
* Discord RPC
* Small binaries, not resource-heavy
* Builds for platforms such as the Raspberry Pi - see the [releases](https://github.com/SpikeHD/RogueTop/releases/latest/) for a full list
* Always up-to-date, in online mode at least :P

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
4. (If NOT bundling for offline mode) create a fake `game.dat` file (ie. run `touch game.dat`) in the root of the repository, to prevent build issues
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
  - [x] TODO finish this
- [ ] Downloadable offline mode (update-able without redownloading a new binary every time)
- [x] Separate "lite" builds that are online-only
- [x] Hotkeys
  - [x] Fullscreen
  - [ ] TBD
- [ ] Mobile support
- [x] Add `.plist` for MacOS to solve HTTP issue
- [x] Mod system/support?
  - [x] Binding of Isaac-like texture replacements?
  - [x] JS-based mods?
  - [ ] Config in UI
  - [ ] Load from ZIP (mods could get quite big, would make them easier to distribute)
- [ ] ~~Theme system/support?~~ This is basically just the mod system

# Contributing

Issues, PRs, etc. are all welcome!

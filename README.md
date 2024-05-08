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

Download the latest [release for your platform](https://github.com/SpikeHD/RogueTop/releases). The file you download depends on your operating system:

* For Windows, download the `.msi` installer file, or the `pc-windows` ZIP file. Builds are also provided for ARM64.
* For Linux, download the `.deb` or `.rpm` installer files, or the `unknown-linux` ZIP file. ~~Builds are also provided for ARMv7 and ARM64.~~ (coming soon)
* For MacOS, download the `.dmg` installer file, or the `apple-darwin` ZIP file. Builds are provided for both Intel-based (x86_64) and M-series-based MacOS systems (arm64).

Releases also come in two flavors, regular and "offline". Regular versions are very lightweight, and will load the online version of PokeRogue. Offline versions are larger, but contain the entirety of PokeRogue within the binary, and can be run without an internet connection!

## Alternative Installation Methods

You can also download the latest build from [GitHub Actions](https://www.github.com/SpikeHD/RogueTop/actions). Or, if you're feeling brave, you can [build it yourself](#building)!

# Table of Contents

* [Installation](#installation)
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
4. Build it!
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
  - [ ] Maybe move to bundled (eg. Tauri resource) but external/read from FS, no need for a 500mb binary lol
- [ ] Downloadable offline mode (update-able without redownloading a new binary every time)
- [x] Separate "lite" builds that are online-only
- [ ] Plugin system/support
- [ ] Theme system/support

# Contributing

Issues, PRs, etc. are all welcome!
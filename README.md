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
 Online and offline PokeRogue client for Windows, Linux and MacOS.
 <br />
 https://discord.gg/agQ9mRdHMZ
</div>

# Installation

Download the latest [release for your platform](https://github.com/SpikeHD/RogueTop/releases). The file you download depends on your operating system:

* For Windows, download the `.msi` installer file, or the `pc-windows` ZIP file. Builds are also provided for ARM64.
* For Linux, download the `.deb` installer file if on a Debian-based system, or the `unknown-linux` ZIP file. Builds are also provided for ARMv7 and ARM64.
* For MacOS, download the `.dmg` installer file, or the `apple-darwin` ZIP file. Builds are provided for both Intel-based (x86_64) and M-series-based MacOS systems (arm64).

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
* Small binary size, instant start time.
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
3. Build it!
  ```sh
  pnpm tauri build
  # Or if you have tauri CLI installed
  cargo tauri build

  # Add the "offline" feature to bundle PokeRogue with the binary
  cargo tauri build --features offline
  ```

your built files will be in the `src-tauri/target` directory.

# TODO

- [ ] Bundled offline mode (directly in the binary, eg. should work on an airgapped machine)
- [ ] Downloadable offline mode (update-able without redownloading a new binary every time)
- [ ] Separate "lite" builds that are online-only
- [ ] Plugin system/support
- [ ] Theme system/support

# Contributing

Issues, PRs, etc. are all welcome!
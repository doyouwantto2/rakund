[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)]()
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=Tauri&logoColor=white)]()
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=for-the-badge)]()

**Rakund** is a high-performance, cross-platform piano simulator built with a Rust backend and SolidJS frontend.

Every note played doesn't just produce a sound; it renders a specific color and physical particle on the screen, creating a mathematically precise fusion of audio and visual data.

---

## Video

[![Demo Video](https://img.youtube.com/vi/eoPtckVYc3c/0.jpg)](https://www.youtube.com/watch?v=eoPtckVYc3c)

### Preview

| Manual Play Mode | Visualizer |
| :---: | :---: |
| <img src="./examples/img/swappy-20260324-173943.png" width="400"/> | <img src="./examples/img/swappy-20260324-174920.png" width="400"/> |
| *Clean UI for manual QWERTY/MIDI input.* | *Real-time color rendering based on note frequencies.* |

---

## Key Features

* **Dual Input Modes:** Play using a standard computer keyboard or external MIDI controller.
* **Low Latency:** Rust backend ensures fast communication between the audio thread and the visual rendering thread.

---

---

## Installation & Build

### Prerequisites
Make sure you have [Rust](https://www.rust-lang.org/tools/install) and the [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/) installed on your system, along with [direnv](https://direnv.net/) and [bun](https://bun.com/).

### Build Instructions
```bash
git clone https://github.com/doyouwantto2/rakund.git
cd rakund
direnv allow
bun install
bun run tauri dev
```


## Setup

### Linux
On linux you need to extract tar files such as salamander.tar and splendid.tar, into config/rakund/instruments so that it would look like this:

.config/
  ├── rakund/
    ├── instruments/
    │   └── salamander/
    │   └── splendid/
    └── songs/

### Window
On window it should be:

.AppData/
  ├── rakund/
    ├── instruments/
    │   └── salamander/
    │   └── splendid/
    └── songs/

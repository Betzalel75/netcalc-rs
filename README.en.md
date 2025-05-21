
# 🧮 netcalc-rs

![License: GPL v3](https://img.shields.io/badge/license-GPLv3-blue)

**netcalc-rs** is a graphical IP addressing calculator built with Rust and the Dioxus framework. It helps users understand and manipulate subnetting, masks, broadcast addresses, and more.

> 🎯 Designed for **desktop-only use**, featuring a modern **Glassmorphism style** with **automatic system light/dark theme detection**.

---

## ✨ Features

- Automatic calculation:
  - Network address
  - Broadcast address
  - Subnet mask
  - Address range
  - Dynamic subnetting
- **Dashboard-style interface**
- **Interactive explanatory widgets** (bars, bubbles…)
- Integrated theory panels (formulas, concept summaries…)
- **One-page scroll sections**
- Light/Dark theme support based on system settings

---

## 🖼️ Preview

_(Insert screenshot or animation here if available)_

---

## ⚙️ Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://dioxuslabs.com/)
- Dependencies:
  - `libgtk-3-dev`
  - `libsoup3-dev` (or `libsoup-3.0-dev`, according to your system)
  - `libjavascriptcoregtk-4.1-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libxdo-dev`

```bash
cargo install dioxus-cli
````

---

### Build (Release)

```bash
cargo build --release
```

Executable will be found in `target/release/netcalc-rs`.

---

### Run (Development Mode)

```bash
dx serve
```

---

## 🖥️ Usage

The app opens as a graphical window. Enter an IP address with CIDR (e.g. `192.168.1.0/24`) and explore the sections to view subnetting details, available addresses, and more.

---

## 📜 License

This project is licensed under the **GPL v3**. See the [LICENSE](LICENSE) file for details.

---


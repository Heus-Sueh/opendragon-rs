# OpenDragon-rs — Project Roadmap & Status

[![Project Status: Active](https://img.shields.io/badge/Status-Active%20Development-success.svg)](https://github.com/GrbavaCigla/opendragon-rs)
[![Rust: Stable](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![GUI: GPUI--CE](https://img.shields.io/badge/GUI-GPUI--CE%20%2B%20gpui--kit-black.svg)](https://gpui-kit.com)

**OpenDragon-rs** is a modern, memory-safe, extensible Linux and cross-platform peripheral configuration suite for Redragon hardware, powered by Rust, GPUI-CE, and empirical reverse engineering.

This document tracks the current implementation status across all workspace crates and outlines the sequential roadmap for upcoming milestones.

---

## 1. Current Implementation Status

### Architecture & Workspace Overview

| Component | Path | Status | Description |
|-----------|------|--------|-------------|
| **Core Abstractions** | `crates/opendragon-core/` | **Stable** | Typed domain models, `Device` & `DeviceBackend` traits, capabilities, color math, and typed errors. |
| **HID Transport** | `crates/opendragon-hid/` | **Stable** | `hidapi` transport, device enumeration, report descriptor decoder, non-destructive probe. |
| **Protocol Engine** | `crates/opendragon-protocol/` | **Stable** | Sonix EVision & Holtek packet builders/parsers with strict safety tags (`ReadOnly`, `KnownSafe`, `Experimental`, `Dangerous`). |
| **Device Drivers** | `crates/opendragon-devices/` | **Partial** | Registry and drivers for K552RGB-PRO, M607, M909, M601, and ACRUX receiver. |
| **CLI Application** | `apps/opendragon-cli/` | **Stable** | Full CLI suite: `list`, `info`, `probe`, `lighting`, `actuation`, `rapid-trigger`, `polling-rate`, `profile`, `macro`, `permissions`. |
| **Desktop GUI** | `apps/opendragon-gui/` | **Active / Redesigned** | Native GPUI-CE desktop app using official `gpui-kit` components and monochrome black & white theme. |
| **Reverse Engineering Tooling** | `tools/hid-diff/` | **Stable** | Differential packet analysis CLI for isolating firmware offsets from official software USB traces. |
| **Linux Permissions** | `udev/70-opendragon.rules` | **Stable** | Modern udev rules with `uaccess` and plugdev tags for non-root hardware access. |

---

### Hardware Support Matrix

| Device | VID:PID | Discovery | Lighting | Polling Rate | Actuation (Hall Effect) | Rapid Trigger | Onboard Profiles | Hardware Macros |
|--------|---------|-----------|----------|--------------|-------------------------|---------------|------------------|-----------------|
| **Kumara Pro K552RGB-PRO** | `0c45:5004` | **Supported** | **Supported** (18 modes) | **Supported** (125–1000Hz) | **Partial** (Protocol mapped, pending hardware verification) | **Partial** (Protocol mapped, pending hardware verification) | **WIP** | **WIP** |
| **Mitra RGB K551** | `0c45:5004` | **Supported** | **Supported** | **Supported** | N/A (Mechanical) | N/A (Mechanical) | **WIP** | **WIP** |
| **M607 Griffin** | `04d9:fc38` | **Supported** | **Supported** | **Supported** | N/A | N/A | **Supported** | **WIP** |
| **M909 Emperor** | `04d9:fc4c` | **Supported** | **Supported** | **Supported** | N/A | N/A | **Supported** | **WIP** |
| **M601-3 Centrophorus** | `04d9:fc18` | **Supported** | **Partial** | **Partial** | N/A | N/A | **WIP** | **WIP** |
| **ACRUX Wireless Receiver** | `1a34:f517` | **Supported** | Unsupported | **WIP** | Unsupported | Unsupported | Unsupported | Unsupported |

---

### Desktop GUI (GPUI-CE) Status

- **Monochrome Design System**: Fully aligned with `gpui-kit`'s default black & white (zinc / shadcn) palette with high-contrast text and zero visual clutter.
- **Component Library**: Migrated to native `gpui-kit` primitives (`Sidebar`, `GroupBox`/`Card`, `ButtonGroup`, `Switch`, `Badge`, `Slider`).
- **Interactive Previews**:
  - **Keyboard Preview**: Vector keycap matrix supporting **ANSI (US)**, **ABNT2 (Brasil)** with `Ç` key, and **ISO (Europe)** layouts with real-time actuation gauge and selection indicators.
  - **Mouse Preview**: Interactive button topology and chassis preview for Redragon mice.
- **Rendering Performance**: Dev-profile package optimization (`opt-level = 2` for dependencies/shaders) ensuring 60+ FPS zero-lag rendering.
- **Real-Time Reactive State**: Automatic device polling, live reconnection handling, and toast notifications.

---

## 2. Roadmap & What Remains to Implement

### Phase 1: Deep Reverse Engineering & Physical Protocol Validation
- [ ] **Sonix EVision Magnetic Actuation (Hall Effect)**:
  - Capture Windows software USB packets for individual key sensitivity calibration.
  - Run `hid-diff` to verify exact byte offsets for per-key actuation thresholds (0.1 mm to 4.0 mm).
  - Validate physical trigger response on real Kumara Pro K552RGB-PRO hardware.
- [ ] **Sonix EVision Rapid Trigger Verification**:
  - Reverse-engineer continuous re-actuation packets (upstroke reset sensitivity and downstroke trigger sensitivity).
  - Add positive and negative hardware controls to verify dynamic reset behavior in gaming scenarios.
- [ ] **Sonix EVision Onboard Profiles & EEPROM**:
  - Safely isolate profile read/write command blocks without triggering bootloader or firmware flash writes.
  - Verify switching between profiles via physical Fn key combos vs GUI commands.
- [ ] **Sonix EVision Hardware Macro Engine**:
  - Reverse-engineer macro payload format (scancodes, key down/up events, and millisecond inter-key delays).
  - Implement checksum validation for multi-packet macro transfer.

---

### Phase 2: GUI Enhancements & Advanced Features
- [ ] **Per-Key Interactive RGB Painter**:
  - Click-and-drag multi-key selection canvas on the keyboard preview.
  - Color wheel / palette picker to apply individual custom LED colors per key and send them directly to the hardware.
- [ ] **Visual Macro Sequence Editor**:
  - GUI recorder for keystrokes with live millisecond delay adjustments.
  - Bind macros to specific keys or mouse buttons (e.g., M4, M5, or Fn layer).
- [ ] **Mouse DPI Curve & Angle Snapping**:
  - Visual DPI stage graph (400, 800, 1600, 3200, etc.) with independent X/Y axis sensitivity toggles.
  - Polling rate and lift-off distance (LOD) toggles.
- [ ] **Persistent User Preferences**:
  - Auto-restore last selected keyboard layout (ABNT2 / ANSI / ISO) and theme mode across application restarts.

---

### Phase 3: Background Daemon & System Integration (`opendragond`)
- [ ] **Headless Daemon Service**:
  - Lightweight systemd background daemon (`opendragond`) to monitor USB plug/unplug events.
- [ ] **Automatic Profile Switching**:
  - Detect focused window / active game process (via Wayland / X11 window managers) and automatically switch active hardware profiles.
- [ ] **D-Bus IPC Interface**:
  - Expose D-Bus endpoints for third-party integration (e.g., GNOME extension, KDE tray widget, or CLI scripts).

---

### Phase 4: Expanded Hardware Support
- [ ] **Redragon Wireless Peripherals**:
  - Reverse-engineer 2.4GHz USB dongle protocols and Bluetooth pairing interfaces for wireless Kumara Pro and wireless mice (e.g., M808 Storm Wireless, M913 Impact Elite).
  - Battery level telemetry query packets and low-power standby modes.
- [ ] **Additional Popular Keyboards**:
  - Redragon Fizz K617 (60%).
  - Redragon Castor K631 (68%).
  - Redragon Draconic K530 (60% Bluetooth).
- [ ] **Additional Mice**:
  - Redragon Cobra M711 / M711-FPS.
  - Redragon Invader M719.

---

### Phase 5: Packaging & Distribution
- [ ] **Flatpak**: Manifest for Flathub with appropriate udev/hid permissions.
- [ ] **AppImage**: Standalone executable bundle with bundled Vulkan/Wayland/X11 libraries.
- [ ] **Arch Linux AUR**: `opendragon-git` PKGBUILD.
- [ ] **Debian / Ubuntu & Fedora**: `.deb` and `.rpm` release artifacts generated via CI.

---

## 3. Contributing & Methodology

Contributions are welcome! Please follow our established engineering practices:
1. **Safety First**: Never perform destructive fuzzing on physical hardware. Read the [Hardware Safety Policy](AGENTS.md#4-hardware-safety-policy-critical).
2. **Evidence-Driven**: Document reverse engineering experiments in `re/devices/<slug>/investigations/` using `hid-diff`.
3. **Quality Gates**: All commits must pass:
   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   cargo build --workspace --release
   ```

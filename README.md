# OpenDragon-rs

[![CI](https://github.com/GrbavaCigla/opendragon-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/GrbavaCigla/opendragon-rs/actions/workflows/ci.yml)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust: Stable](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

**OpenDragon-rs** is a modern, extensible, memory-safe configuration and reverse-engineering framework for **Redragon** peripherals on Linux (and other platforms).

Originally started as an out-of-tree Linux kernel module, OpenDragon has been modernized and re-architected in **Rust** as a modular userspace framework built upon standard HID protocols, non-destructive device probing, and evidence-driven reverse engineering.

The initial flagship hardware focus is the **Redragon Kumara Pro K552RGB-PRO** keyboard.

---

## Features

- **Safe Hardware Discovery & Inspection**:
  Safely enumerate, identify, and inspect Redragon keyboards, mice, and 2.4G wireless receivers.
- **Non-Destructive Device Prober (`opendragon probe`)**:
  Dumps USB descriptors, HID report descriptors, and interfaces to structured, versioned JSON (`probe.json`) with zero commands transmitted to the device.
- **RGB Lighting Configuration**:
  Configure static RGB colors, lighting effects (wave, breathing, rainbow), brightness, and speeds.
- **Performance & Switch Configuration**:
  Support for switch actuation points (magnetic / Hall Effect), Rapid Trigger dynamic reset, and USB polling rates.
- **Built-in Reverse Engineering Tooling (`hid-diff`)**:
  Differential packet comparison utility to isolate byte offsets and reverse-engineer proprietary firmware protocols from official software traces.
- **Strict Hardware Safety**:
  Guard-rail policy protecting physical devices against dangerous commands (firmware writes, bootloader jumps, flash memory resets).

---

## Supported Devices

| Device | VID:PID | Detection | Lighting | Polling Rate | Actuation | Rapid Trigger | Profiles |
|--------|---------|-----------|----------|--------------|-----------|---------------|----------|
| **Kumara Pro K552RGB-PRO** | `0c45:5004` | **Supported** | Experimental | Experimental | WIP | WIP | WIP |
| **Mitra RGB K551** | `0c45:5004` | **Supported** | Experimental | Experimental | Unsupported | Unsupported | WIP |
| **M607 Griffin** | `04d9:fc38` | **Supported** | Supported | Partial | Unsupported | Unsupported | Supported |
| **M909 Emperor** | `04d9:fc4c` | **Supported** | Supported | Partial | Unsupported | Unsupported | Supported |
| **M601-3 Centrophorus** | `04d9:fc18` | **Supported** | WIP | WIP | Unsupported | Unsupported | WIP |
| **ACRUX Wireless Receiver** | `1a34:f517` | **Supported** | Unsupported | WIP | Unsupported | Unsupported | Unsupported |

*For complete details, see [docs/devices.md](docs/devices.md).*

---

## Installation & Setup

### 1. Prerequisites
- **Rust Toolchain (1.80+)**: Recommended via [mise](https://mise.jdx.dev) (`mise use rust@stable`) or [rustup](https://rustup.rs).
- `pkg-config`, `libudev-dev`, `libusb-1.0-0-dev`.

### 2. Building from Source
```bash
git clone https://github.com/GrbavaCigla/opendragon.git
cd opendragon
cargo build --release
```
The compiled binaries are available at:
- `target/release/opendragon`
- `target/release/hid-diff`

### 3. Setting Up Device Permissions (Udev)
To configure hardware without requiring root privileges (`sudo`), install the udev rules:

```bash
sudo cp udev/70-opendragon.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Verify permissions using the built-in diagnostic command:
```bash
opendragon permissions
```

---

## CLI Usage

### Device Enumeration & Discovery
```bash
# List all connected Redragon peripherals
opendragon list

# List all HID peripherals on the system
opendragon list --all

# Show detailed descriptors and interface capabilities
opendragon info
```

### Safe Non-Destructive Probing
```bash
# Capture full hardware descriptors to JSON
opendragon probe --output probe.json
```

### RGB Lighting Control
```bash
# Set static color
opendragon rgb static '#ff0000'

# Set brightness percentage
opendragon rgb brightness 75

# Set dynamic mode
opendragon rgb mode rainbow --speed 60 --brightness 100
```

### Switch Actuation & Rapid Trigger (Hall Effect Keyboards)
```bash
# Inspect current actuation point
opendragon actuation get

# Set switch actuation point in millimeters
opendragon actuation set 1.2

# Configure Rapid Trigger dynamic reset
opendragon rapid-trigger enable --press-mm 0.2 --release-mm 0.2
opendragon rapid-trigger disable
```

### Polling Rate
```bash
opendragon polling-rate set 1000
```

---

## Reverse Engineering & Research

OpenDragon includes an empirical knowledge base and tools to assist in reverse-engineering new peripherals:

- **Differential Packet Analysis**:
  ```bash
  cargo run -p hid-diff -- capture-baseline.txt capture-modified.txt
  ```
- **Knowledge Base (`re/`)**:
  Documented packet formats, register maps, and investigation journals under `re/devices/k552rgb-pro/`.
- **Workflow Guide**:
  Detailed guide available in [docs/reverse-engineering.md](docs/reverse-engineering.md).

---

## Architecture & Documentation

- [docs/architecture.md](docs/architecture.md) — Rust workspace architecture and crate responsibilities.
- [docs/architecture/legacy-opendragon.md](docs/architecture/legacy-opendragon.md) — Legacy C kernel module audit and migration history.
- [docs/devices.md](docs/devices.md) — Device catalog and support matrix.
- [docs/development.md](docs/development.md) — Developer setup, quality checks, and workflows.
- [docs/protocol-development.md](docs/protocol-development.md) — Framing and command safety policies.
- [docs/reverse-engineering.md](docs/reverse-engineering.md) — Evidence-driven RE methodology.
- [docs/adding-a-device.md](docs/adding-a-device.md) — Step-by-step tutorial for adding new hardware.
- [docs/troubleshooting.md](docs/troubleshooting.md) — Solutions to common issues.
- [AGENTS.md](AGENTS.md) — Guidelines for AI agents and automated contributors.

---

## Hardware Safety & Limitations

- **No Destructive Fuzzing**: OpenDragon never transmits randomized packets to real hardware.
- **High-Risk Operations**: Flash memory and bootloader operations are classified as `Dangerous` and blocked unless the `--i-know-what-im-doing` flag is explicitly provided.
- **Physical Verification**: Features are only marked as `Supported` after empirical confirmation on actual hardware.

---

## Contributing

Contributions are welcome!
1. Check [AGENTS.md](AGENTS.md) for coding conventions and definition-of-done criteria.
2. Ensure all quality gates pass:
   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   ```
3. Submit a pull request with clear evidence and documentation.

---

## License

OpenDragon is licensed under the [GNU General Public License v3.0 or later (GPL-3.0-or-later)](LICENSE).

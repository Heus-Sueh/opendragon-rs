# Development Guide

This guide covers building, testing, and contributing to the OpenDragon codebase.

---

## 1. Prerequisites

- **Linux x86_64** (or macOS / Windows for core protocol development)
- **Rust Toolchain (Stable 1.80+)**:
  We recommend managing the Rust toolchain via `mise`:
  ```bash
  mise install rust@stable
  mise use rust@stable
  ```
- **System Development Libraries**:
  - `pkg-config`
  - `libudev` / `systemd-devel`
  - `libusb-1.0`

On Debian/Ubuntu:
```bash
sudo apt install build-essential pkg-config libudev-dev libusb-1.0-0-dev
```

On Arch Linux:
```bash
sudo pacman -S base-devel pkgconf systemd libusb
```

On Fedora:
```bash
sudo dnf install @development-tools systemd-devel libusb1-devel
```

---

## 2. Building the Project

Compile the complete workspace:
```bash
cargo build --workspace
```

Compile optimized release binaries:
```bash
cargo build --workspace --release
```

Binaries will be placed in `target/release/`:
- `target/release/opendragon`
- `target/release/hid-diff`

---

## 3. Running Quality Checks

Before submitting any code changes, verify all quality gates:

```bash
# Check code formatting
cargo fmt --check

# Check linter with zero warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all unit and regression tests
cargo test --workspace
```

---

## 4. Running the CLI in Development

Run CLI commands directly via Cargo:
```bash
# List devices
cargo run -p opendragon-cli -- list

# Detailed device information
cargo run -p opendragon-cli -- info

# Safe non-destructive probe
cargo run -p opendragon-cli -- probe --output probe.json

# Differential packet analysis
cargo run -p hid-diff -- re/captures/legacy/m607/rainbow/low_high.txt re/captures/legacy/m607/rainbow/low_low.txt
```

Enable verbose debug tracing with `RUST_LOG=debug`:
```bash
RUST_LOG=debug cargo run -p opendragon-cli -- probe
```

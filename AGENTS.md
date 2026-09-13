# AGENTS.md - Agent Developer Guide for OpenDragon

Welcome to **OpenDragon**. This document provides mandatory architectural instructions, coding standards, reverse engineering rules, and definition-of-done criteria for AI coding agents and human contributors working in this repository.

---

## 1. Project Mission

OpenDragon is a modern, memory-safe, extensible Linux/cross-platform peripheral configuration suite for **Redragon** hardware. The initial primary hardware target is the **Redragon Kumara Pro K552RGB-PRO** keyboard.

Our objective is **not** simply to accumulate unverified packet code, but to build an **empirical, reusable, and verifiable reverse-engineering knowledge base** alongside robust, type-safe Rust device drivers.

---

## 2. Workspace Organization

```text
opendragon/
├── Cargo.toml                      # Workspace root
├── mise.toml                       # Toolchain configuration (rust@stable)
├── crates/
│   ├── opendragon-core/            # Core traits (Device), capabilities, domain models, typed errors
│   ├── opendragon-hid/             # Hardware transport, discovery, report descriptor parser, safe probe
│   ├── opendragon-protocol/        # Pure packet builders, parsers, checksums, command safety policies
│   └── opendragon-devices/         # Concrete device drivers (K552RGB-PRO, M607, M909) and registry
├── apps/
│   └── opendragon-cli/             # User CLI application (opendragon list, info, probe, rgb, etc.)
├── tools/
│   └── hid-diff/                   # Differential packet analysis utility for reverse engineering
├── udev/
│   └── 70-opendragon.rules         # Modern udev rules (uaccess + plugdev permissions)
├── docs/                           # Documentation and guides
├── re/                             # Empirical reverse-engineering knowledge base
│   ├── devices/                    # Per-device metadata, specifications, and investigations
│   └── captures/                   # Minimal packet fixtures and capture documentation
└── .agents/
    └── skills/                     # Agent skills (hardware-reverse-engineering, etc.)
```

---

## 3. Mandatory Quality Gates & Commands

Before completing any task or proposing changes, the following checks **must pass cleanly with zero warnings or errors**:

```bash
# Code formatting
cargo fmt --check

# Strict linter checks (no warnings allowed)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Unit and integration tests
cargo test --workspace

# Release build
cargo build --workspace --release
```

*Note: If using `mise` for toolchain management, prefix commands with `mise exec -- `.*

---

## 4. Hardware Safety Policy (CRITICAL)

- **NEVER perform destructive fuzzing on physical hardware.**
- Distinguish explicitly between command safety levels:
  - `ReadOnly`: Querying state, reading descriptors.
  - `KnownSafe`: Verified non-destructive configuration changes (e.g., RGB colors).
  - `Experimental`: Functional changes under active validation (actuation points, polling rates).
  - `Dangerous`: Operations touching flash memory, bootloader entry, EEPROM write, or factory reset.
- High-risk operations classified as `Dangerous` MUST require the `--i-know-what-im-doing` flag.
- Never invent Vendor IDs (VID) or Product IDs (PID). Always read them from real hardware or empirical captures.

---

## 5. Reverse Engineering Methodology

When analyzing new hardware or unmapped protocol features:
1. Consult `.agents/skills/hardware-reverse-engineering/SKILL.md`.
2. Follow the **single-variable controlled experiment**:
   - Capture baseline state.
   - Alter exactly one setting in official software.
   - Run `cargo run -p hid-diff -- baseline.json modified.json` to isolate changed bytes.
   - Formulate a falsifiable hypothesis.
   - Write a unit test in `opendragon-protocol` before touching hardware.
   - Validate on real hardware, followed by a negative control.
   - Document the findings in `re/devices/<slug>/investigations/NNNN-<title>.md`.

---

## 6. Rust Coding Standards

- **Identifiers**: `snake_case` for modules, functions, and variables; `PascalCase` for structs, enums, and traits.
- **Explicit Naming**: Prefer `vendor_id`, `product_id`, `report_id`, and `interface_number` over `vid`, `pid`, `rid`, `iface`.
- **Error Handling**: Use `OpenDragonError` from `opendragon-core`. Avoid stringly-typed errors or bare `unwrap()` in production driver code.
- **Safety**: `unsafe` code is strictly prohibited unless wrapped and justified in low-level transport FFI bindings.

---

## 7. Definition of Done (DoD)

A task or feature is considered complete only when:
1. Code compiles, passes `cargo fmt --check`, passes `cargo clippy -- -D warnings`, and passes `cargo test --workspace`.
2. Hardware support is truthfully categorized (`Supported`, `Partial`, `Experimental`, `WIP`, `Unknown`, `Unsupported`).
3. For hardware features marked as `Supported`, observable behavior was verified on physical hardware.
4. Protocol discoveries are documented in `re/devices/<device>/` and cross-referenced in `docs/`.
5. Udev rules, CLI options, and tests are updated.

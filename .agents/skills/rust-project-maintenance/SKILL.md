---
name: rust-project-maintenance
description: Maintenance, code style, quality gates, dependency management, CI workflows, and release processes for the OpenDragon Rust workspace.
---

# Rust Project Maintenance for OpenDragon

This skill provides guidelines and quality gates for maintaining the OpenDragon Cargo workspace.

## 1. Quality Gates (Mandatory Before Commit)

Every contribution must pass the full workspace verification suite:

```bash
# 1. Format check
cargo fmt --check

# 2. Strict Clippy check with zero warnings allowed
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. All workspace unit and regression tests
cargo test --workspace

# 4. Release build validation
cargo build --workspace --release
```

If toolchains are managed with `mise`, prefix with `mise exec -- `:
```bash
mise exec -- cargo fmt --check
mise exec -- cargo clippy --workspace --all-targets --all-features -- -D warnings
mise exec -- cargo test --workspace
```

## 2. Workspace Organization

OpenDragon is organized as a multi-crate Cargo workspace:
- `crates/opendragon-core`: Foundational domain types (`Device`, `Capability`, `RgbColor`, errors). No IO dependencies.
- `crates/opendragon-hid`: Hardware transport, safe discovery, report descriptor parser, non-destructive probe.
- `crates/opendragon-protocol`: Pure functional packet builders, checksums, safety classifications.
- `crates/opendragon-devices`: Peripheral drivers implementing the `Device` trait and registry.
- `apps/opendragon-cli`: User-facing CLI application (`opendragon`).
- `tools/hid-diff`: Reverse-engineering packet diff utility.

### Conventions:
- Crate names: `kebab-case` with `opendragon-` prefix.
- Rust identifiers: `snake_case` for modules, functions, and variables; `PascalCase` for structs, enums, and traits.
- Prefer full descriptive names over obscure abbreviations (`vendor_id` instead of `vid`, `interface_number` instead of `iface`).
- Zero `unsafe` code outside necessary FFI bindings in low-level transport.

## 3. Toolchain & MSRV

- Minimum Supported Rust Version (MSRV): **Rust 1.80+** (workspace edition: `2021`).
- Toolchain manager: `mise` configured via `mise.toml` (`rust@stable`).

## 4. Continuous Integration

CI runs on GitHub Actions (`.github/workflows/ci.yml`) on every push and pull request:
- Automated formatting check (`cargo fmt --check`).
- Strict linter checks (`cargo clippy -- -D warnings`).
- Workspace test execution (`cargo test --workspace`).
- Release binaries compiled for Linux (and Windows where supported).

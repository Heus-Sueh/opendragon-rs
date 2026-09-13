---
name: redragon-device-support
description: Step-by-step workflow for adding, documenting, and testing support for new Redragon keyboards, mice, and accessories in OpenDragon. Use when onboarding a new hardware model or implementing previously unmapped capabilities.
---

# Redragon Device Support Workflow

This guide instructs future agents on adding support for a new Redragon peripheral to OpenDragon.

## 1. Safe Hardware Discovery and Non-Destructive Probe

When physical hardware is connected:
1. Run `opendragon list --all` to find the device's Vendor ID (VID) and Product ID (PID).
2. Run `opendragon info --vid <VID> --pid <PID>` to inspect interfaces, usage pages, and hidraw paths.
3. Run `opendragon probe --vid <VID> --pid <PID> --output re/devices/<slug>/probe-baseline.json` to capture all USB and HID descriptors non-destructively.

## 2. Register Device in Knowledge Base

Create the device knowledge folder:
```bash
mkdir -p re/devices/<slug>/investigations
```
1. Add `re/devices/<slug>/device.toml` with VID, PID, manufacturer, product strings, and interface breakdown.
2. Add `re/devices/<slug>/capabilities.md` recording initial capabilities marked truthfully as `WIP`, `Unknown`, or `Unsupported`.
3. Add `re/devices/<slug>/protocol.md` describing framing and configuration report IDs.

## 3. Register Device in OpenDragon

1. **Known Device Entry**:
   Add the device signature to `KNOWN_DEVICES` in `crates/opendragon-devices/src/registry.rs`:
   ```rust
   DeviceEntry {
       vendor_id: 0xXXXX,
       product_id: 0xYYYY,
       model_name: "Model",
       marketing_name: "Redragon Full Name",
   }
   ```
2. **Implement Device Driver**:
   Create `crates/opendragon-devices/src/<keyboards_or_mice>/<slug>/device.rs`:
   - Implement `opendragon_core::device::Device`.
   - Declare capabilities via `fn capabilities(&self) -> &[Capability]`.
   - Implement capability methods (`set_lighting`, `set_polling_rate`, etc.).
3. **Register Matcher**:
   Add the matcher branch in `DeviceRegistry::create_device()` in `registry.rs`.

## 4. Implement Protocol Packet Builders

If the protocol uses a new framing or packet structure:
1. Implement the packet builder in `crates/opendragon-protocol/src/`.
2. Add unit tests for every packet builder verifying exact byte layouts against test fixtures.
3. Classify commands with `CommandSafetyClassification` (`ReadOnly`, `KnownSafe`, `Experimental`, `Dangerous`).

## 5. Update Udev Rules

Add the device VID/PID to `udev/70-opendragon.rules`:
```udev
ATTRS{idVendor}=="XXXX", ATTRS{idProduct}=="YYYY", TAG+="uaccess", GROUP="plugdev", MODE="0660"
```

## 6. Update Documentation and Matrix

1. Add the device to `docs/devices.md` and the support matrix.
2. Update `README.md` if the device expands supported product lines.
3. Ensure automated tests pass:
   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace
   ```

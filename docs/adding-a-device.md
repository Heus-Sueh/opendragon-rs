# How to Add a New Redragon Device

This tutorial guides you through adding support for a new Redragon peripheral to OpenDragon.

---

## Step 1: Probe the Hardware Non-Destructively

Plug in your device and run:
```bash
opendragon list --all
```
Locate your peripheral's Vendor ID (VID) and Product ID (PID).

Run a non-destructive probe:
```bash
opendragon probe --vid <VID> --pid <PID> --output re/devices/<device-name>/probe-baseline.json
```

---

## Step 2: Create the Knowledge Base Directory

Create `re/devices/<device-name>/`:
- `device.toml`: Record VID, PID, manufacturer, and interface details.
- `capabilities.md`: Record initial capabilities (marked `WIP` or `Unknown`).
- `protocol.md`: Note whether it uses EVision, Holtek, or another protocol.

---

## Step 3: Implement the Device Driver

1. Create `crates/opendragon-devices/src/<keyboards_or_mice>/<device_name>.rs`:
   ```rust
   pub struct RedragonMyDevice {
       info: DeviceInfo,
       transport: Option<HidTransportHandle>,
   }

   impl Device for RedragonMyDevice {
       fn info(&self) -> &DeviceInfo { &self.info }
       fn capabilities(&self) -> &[Capability] { &[Capability::Lighting] }
       fn support_status(&self, cap: Capability) -> SupportStatus { ... }
   }
   ```
2. Register the device in `crates/opendragon-devices/src/registry.rs`:
   - Add to `KNOWN_DEVICES`.
   - Add branch in `DeviceRegistry::create_device()`.

---

## Step 4: Add Udev Rules

Add the device VID/PID to `udev/70-opendragon.rules`:
```udev
KERNEL=="hidraw*", ATTRS{idVendor}=="XXXX", ATTRS{idProduct}=="YYYY", MODE="0666", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="XXXX", ATTRS{idProduct}=="YYYY", MODE="0666", TAG+="uaccess"
```

---

## Step 5: Verify and Test

```bash
cargo test --workspace
cargo run -p opendragon-cli -- info --vid <VID> --pid <PID>
```
Update `docs/devices.md` with your device's compatibility status.

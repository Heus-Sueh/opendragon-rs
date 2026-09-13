# Legacy OpenDragon Architecture Audit

This document preserves the architectural analysis, known protocols, and reverse-engineered knowledge from the original C Linux kernel module implementation of OpenDragon.

---

## 1. Overview of the Legacy Implementation

The original OpenDragon project was developed as an out-of-tree **Linux Kernel Module** (`hid_driver`) written in C:
- Driver Name: `opendragon`
- Primary Header: `src/opendragon.h`
- Entry Point: `src/opendragon.c` using `module_hid_driver()`
- Transport Helper: `src/report.c`
- Subsystem Handlers: `src/functions/light_mode.c` and `src/functions/profile.c`

### Supported Hardware in Legacy Code
The legacy driver matched on the following USB Vendor and Product IDs:

| Device | Vendor ID | Product ID | Target Interface | Implementation Status |
|--------|-----------|------------|------------------|-----------------------|
| Redragon M607 Griffin (M602-RGB) | `0x04d9` | `0xfc38` | Interface 2 | Lighting & Profiles (Partial) |
| Redragon M909 Emperor | `0x04d9` | `0xfc4c` | Interface 2 | Lighting & Profiles (Partial) |
| Redragon M601-3 Centrophorus | `0x04d9` | `0xfc18` | Unspecified | Stated in README as unsupported |

---

## 2. Low-Level USB and HID Transport Mechanics

The legacy driver did not interact with the mouse via standard HID reports sent over interrupt endpoints. Instead, it sent **USB Control Transfers** directly to the control endpoint (`0x00`) using `usb_control_msg_send()`:

```c
const unsigned char endpoint = 0x00;
const unsigned char request = HID_REQ_SET_REPORT; // 0x09
const unsigned char requesttype = USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT; // 0x21
const unsigned short value = 0x0302; // Report Type = Feature (0x03), Report ID = 0x02
const unsigned short index = 0x02; // Interface Number 2
const int timeout = USB_CTRL_SET_TIMEOUT; // 5000 ms
```

### Protocol Details:
- **Request Type**: `0x21` (Host-to-device, Class request, Interface recipient).
- **Request**: `HID_REQ_SET_REPORT` (`0x09`).
- **Value (`wValue`)**: `0x0302`. In USB HID specification, high byte is Report Type (`0x03` = Feature Report) and low byte is Report ID (`0x02`).
- **Index (`wIndex`)**: `0x0002` (Targeting USB interface 2 on the mouse).
- **Packet Length**: 16 bytes (`DATA_LENGTH = 16`) for command packets; 64 bytes for bulk configuration packets.

---

## 3. Protocol Framing and Command Packets

### 3.1. Lighting Mode Packets (`set_light_mode`)

When setting the lighting mode, the legacy driver constructed a sequence of four 16-byte packets:

```text
Packet 1 (Mode initialization):
[0x02, 0xf3, 0x46, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]

Packet 2 (RGB color, mode, speed, umode):
[0x02, 0xf3, 0x49, 0x01, 0x06, 0x00, 0x00, 0x00, R, G, B, lmode, speed, umode, 0x00, 0x00]

Packet 3 (Brightness):
[0x02, 0xf3, 0x4f, 0x01, 0x01, 0x00, 0x00, 0x00, brightness, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]

Packet 4 (Apply / Commit):
[0x02, 0xf1, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
```

#### Field Breakdown:
- `Byte 0`: Report ID (`0x02`).
- `Byte 1`: Command Family (`0xf3` for configuration data, `0xf1` for apply/action).
- `Byte 2`: Subcommand ID (`0x46` = lighting setup, `0x49` = color & effect parameters, `0x4f` = brightness, `0x02` = commit action).
- `Byte 3`: Target sub-index (`0x01` or `0x04` depending on device firmware revision).
- `Byte 4`: Argument byte count (`0x02`, `0x06`, `0x01`, `0x00`).
- `Bytes 5..7`: Zero padding (`0x00, 0x00, 0x00`).
- `Bytes 8..`: Payload arguments.

#### Device-Specific Light Mode Mappings:
- **Redragon M607**:
  - `lmodes`: `[0x01, 0x01, 0x01, 0x02, 0x06, 0x07, 0x01, 0x00]`
  - `umodes`: `[0x04, 0x08, 0x02, 0x00, 0x00, 0x00, 0x10, 0x00]`
- **Redragon M909**:
  - `lmodes`: `[0x02, 0x03, 0x01, 0x05, 0xff, 0xff, 0x04, 0x00]`
  - `umode`: `0x01` (Modes 4 "Go without trace" and 5 "Reactive" are unsupported on M909).

### 3.2. Profile Switching Packets (`set_profile`)

Switching the active hardware profile sends a select packet followed by multiple commit packets:

```text
Select Profile (profile_num = 0..4):
[0x02, 0xf3, 0x2c, 0x00, 0x02, 0x00, 0x00, 0x00, profile_num, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]

Apply sequence:
[0x02, 0xf1, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
[0x02, 0xf1, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
[0x02, 0xf1, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
[0x02, 0xf1, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
```

### 3.3. Bulk Apply / Scroll Configuration (from `research/m607/apply/`)
Bulk commands utilize **Report ID 3** (`0x03`) with a 64-byte payload:
```text
03f320000a000000 <payload...>
```
This report sets pointer acceleration, scroll speed, and button mappings.

---

## 4. Why the Legacy Kernel Architecture Was Replaced

1. **Kernel Space vs Userspace Risk**:
   Writing hardware configuration utilities as kernel drivers is unnecessarily hazardous. A malformed command or memory fault in a kernel module causes a kernel panic / freeze. A userspace process simply exits with a typed error.
2. **Maintenance and Kernel ABI Instability**:
   Every major Linux kernel release potentially breaks out-of-tree kernel modules (changes in `usb_control_msg`, sysfs APIs, struct layouts). Userspace HID access via `hidraw` is a stable, standardized Linux ABI that has remained compatible for decades.
3. **Privileges and Usability**:
   Loading a kernel module requires root privileges (`sudo insmod`). Userspace drivers with appropriate udev rules run entirely unprivileged under standard desktop user accounts.
4. **Flawed Udev Rule**:
   The legacy `install/99-redragon.rules` used:
   ```udev
   SUBSYSTEM=="hid", ATTRS{idProduct}=="fc38", ATTRS{idVendor}=="04d9", GROUP:="plugdev"
   ```
   This rule failed to grant device access because `/dev/hidraw*` devices are in the `hidraw` subsystem, not `hid`.
5. **No Extensibility for Keyboards and Other Peripherals**:
   The data structures were hardcoded for mice (`light_mode_e`, `redragon_drvdata_t`). Modern keyboards like the Kumara Pro K552RGB-PRO with Rapid Trigger, magnetic actuation, and per-key RGB require completely different capability models and packet protocols.

---

## 5. Migration Map into the Rust Architecture

| Legacy Artifact | New Rust Location | Transformation |
|-----------------|-------------------|----------------|
| `src/opendragon.h` | `crates/opendragon-core` | Strong domain types (`RgbColor`, `LightingMode`, `Capability`, `DeviceInfo`) |
| `src/report.c` | `crates/opendragon-hid` | Safe, cross-platform HID transport with `hidapi` and sysfs enumeration fallback |
| `src/functions/light_mode.c` | `crates/opendragon-protocol` | Pure functional packet builders with unit regression tests |
| `src/functions/profile.c` | `crates/opendragon-protocol` | Profile packet builders with unit regression tests |
| `src/opendragon.c` | `crates/opendragon-devices` | Device abstraction trait (`Device`) and device registry |
| `research/` | `re/captures/legacy/` | Structured captures for automated regression tests |
| `install/99-redragon.rules` | `udev/70-opendragon.rules` | Modern udev rules with `uaccess` and `plugdev` support |

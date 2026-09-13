# Redragon Kumara Pro K552RGB-PRO Protocol Architecture

## 1. Overview and Topology

The K552RGB-PRO keyboard identifies under USB vendor ID `0x0C45` (Microdia/Sonix) and product ID `0x5004`. It presents two USB HID interfaces:

1. **Interface 0 (`1.0`)**:
   - Primary standard boot keyboard interface (6KRO).
   - Contains an additional 64-byte Feature Report (Usage Page `0x0C`, Usage `0x00`).
2. **Interface 1 (`1.1`)**:
   - Composite HID interface hosting multiple Report IDs:
     - **Report ID 1**: Bitmap full N-key rollover (NKRO).
     - **Report ID 2**: System Control (Sleep, Power, Wake).
     - **Report ID 3**: Consumer Control (Volume, Media keys).
     - **Report ID 4**: **Vendor Defined Configuration Interface** (Usage Page `0xFF1C`, Usage `0x92`).

---

## 2. Configuration Transport: Report ID 4

Hardware configuration (lighting, polling rate, profile switching, and actuation) is conducted primarily via **Report ID 4** on Interface 1.

### Packet Framing:
- **Direction**: Host-to-Device (Output Report via Interrupt OUT or Set_Report).
- **Packet Length**: 65 bytes total (`1 byte Report ID (0x04)` + `64 bytes payload`).
- **Device Node**: `/dev/hidraw3` (or corresponding interface 1 hidraw device).

```text
Byte 00: Report ID (0x04)
Byte 01: Command Opcode (e.g. 0x28 for Lighting, 0x22 for Polling, 0x30 for Actuation)
Byte 02: Subcommand / Profile / Mode index
Byte 03: Payload length in bytes (N)
Byte 04..3+N: Command payload arguments
Byte 04+N..63: Zero padding
Byte 64: Checksum (Additive byte sum of bytes 01..63)
```

---

## 3. Alternative Transport: Feature Reports (Interface 0)

Certain firmware revisions and OEM configurations also accept configuration via 64-byte Feature Reports over Interface 0 (`/dev/hidraw2`).

- **Report ID**: `0x00` or `0x04`
- **Mechanism**: `HID_REQ_SET_REPORT` (Feature Report `0x03`)

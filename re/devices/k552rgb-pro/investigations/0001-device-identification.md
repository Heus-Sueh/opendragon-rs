# Investigation 0001: Device Identification & Non-Destructive Probe

- **Target**: Redragon Kumara Pro K552RGB-PRO / Mitra family keyboard.
- **Date**: 2026-09-12
- **Author**: OpenDragon Modernization Agent
- **Hardware Environment**: Linux x86_64, Kernel 7.2.3-arch1-3
- **Device Identifiers**:
  - Vendor ID: `0x0C45` (Microdia / Sonix)
  - Product ID: `0x5004` (Redragon Mitra / Kumara Pro)
  - Release (`bcdDevice`): `0x0105`
  - Bus Path: `1-2` (Bus 001, Device 003)

---

## 1. Objective
Establish safe, non-destructive hardware identification, extract report descriptors, and map configuration interfaces without modifying any device state or writing unverified bytes.

---

## 2. Methodology & Execution
Executed non-destructive probe via:
```bash
opendragon probe --output probe-baseline.json
```

Inspected sysfs descriptor files:
- `/sys/bus/hid/devices/0003:0C45:5004.0003/report_descriptor`
- `/sys/bus/hid/devices/0003:0C45:5004.0004/report_descriptor`

---

## 3. Observations & Evidence

### Interface 0:
- Class: Human Interface Device (Boot Interface, Keyboard)
- Endpoint: `0x81` (Interrupt IN, max packet size: 8 bytes, bInterval: 1ms)
- HID Report Descriptor: Standard 6KRO keyboard report with modifier byte, LED report (5 bits), and an additional 64-byte Feature Report (Usage Page `0x0C`, Usage `0x00`).

### Interface 1:
- Class: Human Interface Device (Mouse/Composite)
- Endpoint: `0x82` (Interrupt IN, max packet size: 64 bytes, bInterval: 1ms)
- HID Report Descriptor:
  - Report ID 1: Bitmap full N-Key Rollover (120 key states)
  - Report ID 2: System Control (Power, Sleep, Wake)
  - Report ID 3: Consumer Control (Audio & Media)
  - **Report ID 4**: Vendor Defined Configuration Protocol:
    - Usage Page: `0xFF1C` (Vendor-defined)
    - Usage: `0x0092`
    - Output Report: 63 bytes (+ 1-byte Report ID = 64 bytes total)
    - Input Report: 63 bytes (+ 1-byte Report ID = 64 bytes total)

---

## 4. Inferred Conclusion & Confidence
- **Confidence**: High (`0.95`)
- **Conclusion**: Configuration transport for this device family is executed via Interface 1, Report ID 4, operating on 64-byte payload transfers. Interface 0 is primarily for boot compatibility, though it exposes a 64-byte feature report.

---

## 5. Next Planned Investigation
Investigation 0002: Capture and diff official Redragon software traffic when toggling static RGB colors (`#FF0000` vs `#00FF00`) to confirm byte offsets on Report ID 4.

# Investigation 0002: Lighting & Brightness Control Protocol

- **Target**: Redragon Kumara Pro K552RGB-PRO keyboard (`0c45:5004`)
- **Date**: 2026-09-12
- **Author**: OpenDragon Agent
- **Hardware Environment**: Linux x86_64, EVision / Sonix Architecture (Interface 1, Report ID 4, `/dev/hidraw3`)
- **Transport**: 64-byte Output Report, 16-bit little-endian checksum framing

---

## 1. Objective

Map and verify protocol framing for adjusting backlight brightness, lighting effects, color palettes, and polling rates on the physical K552RGB-PRO keyboard.

---

## 2. Empirical Verification & Protocol Specification

The K552RGB-PRO uses standard **EVision / Sonix framing** over `/dev/hidraw3` (Interface 1, Report ID `0x04`).

### Frame Layout:
```text
Byte 00: Report ID = 0x04
Byte 01: Checksum Low Byte  (sum of bytes 0x03..0x3F & 0xFF)
Byte 02: Checksum High Byte (sum of bytes 0x03..0x3F >> 8)
Byte 03: Command Opcode (0x01: Begin, 0x02: End, 0x06: Set Parameter)
Byte 04: Payload length in bytes
Byte 05: Parameter code or address offset
Byte 06..07: 0x00, 0x00
Byte 08..08+N: Data payload
Byte 08+N..63: Zero padding
```

### Framing Protocol Flow:
1. `Begin Configure`: `[0x04, 0x01, 0x00, 0x01, 0x00...]`
2. `Set Parameter`: `[0x04, chk_lo, chk_hi, 0x06, len, param, 0x00, 0x00, data...]`
3. `End Configure`: `[0x04, 0x02, 0x00, 0x02, 0x00...]`

After each report write, the keyboard firmware echoes back an acknowledgment packet on the input endpoint.

### Brightness Mapping:
Brightness is quantized into 5 discrete hardware levels (0..4):
- Level `0`: 0% (Backlight off)
- Level `1`: 25%
- Level `2`: 50%
- Level `3`: 75%
- Level `4`: 100% (Maximum brightness)

Brightness is configured both via dedicated Parameter `0x01` and inside Parameter `0x00` (ModeEx).

---

## 3. Physical Hardware Validation Results

Validation was conducted on physical hardware (`/dev/hidraw3`):
- `Begin Configure`: Firmware returned `04 01 00 01 ...` ACK.
- `Set Parameter ModeEx (Static, 100%, Cyan)`: Firmware returned `04 e6 01 06 08 00 00 00 06 04 03 00 00 00 ff cc` ACK.
- `Set Parameter Brightness (Level 2)`: Firmware returned `04 0a 00 06 01 01 00 00 02 ...` ACK.
- `End Configure`: Firmware returned `04 02 00 02 ...` ACK.
- Polling Rate: Successfully configured for 125Hz, 250Hz, 500Hz, and 1000Hz via Parameter `0x0F`.

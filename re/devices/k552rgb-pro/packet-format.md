# K552RGB-PRO Packet Format Specification

## Report ID 0x04 Format (Interface 1)

All configuration commands sent to Interface 1 (`/dev/hidraw3`) follow the 64-byte EVision framing structure:
- 1-byte Report ID prefix (`0x04`)
- 2-byte 16-bit little-endian additive checksum over bytes 3..63
- 1-byte Command opcode
- 1-byte Payload length
- 1-byte Parameter ID or offset
- 2-byte Offset MSB / zero padding
- Payload data bytes (offsets 0x08..)
- Trailing zero padding to byte 63

### General Frame Layout

```text
Offset | Field           | Type    | Description
-------+-----------------+---------+---------------------------------------------------
0x00   | report_id       | u8      | Must be 0x04
0x01   | checksum_lo     | u8      | Low byte of 16-bit additive sum of bytes 0x03..0x3F
0x02   | checksum_hi     | u8      | High byte of 16-bit additive sum of bytes 0x03..0x3F
0x03   | command         | u8      | 0x01: Begin, 0x02: End, 0x06: Set Parameter, 0x11: Custom Colors
0x04   | payload_len     | u8      | Number of data bytes in payload
0x05   | param_or_offset | u8      | Parameter code or address offset
0x06   | offset_hi       | u8      | Offset high byte (usually 0x00)
0x07   | reserved        | u8      | Reserved / 0x00
0x08.. | payload[...]    | [u8]    | Parameter data
...    | padding         | [u8]    | Zero-padded to offset 0x3F (63)
```

---

## Command Framing Flow

Every configuration operation is wrapped in a session frame:
1. **Begin Configure**: `[0x04, 0x01, 0x00, 0x01, 0x00, ...]`
2. **Command Packet**: `[0x04, chk_lo, chk_hi, 0x06, len, param, 0x00, 0x00, data...]`
3. **End Configure**: `[0x04, 0x02, 0x00, 0x02, 0x00, ...]`

After each report write, the hardware device echoes back an acknowledgment packet on the input endpoint.

---

## Mapped Parameters (Command 0x06)

### 1. Full Mode & Parameters (Parameter `0x00` - ModeEx, 8 bytes)
```text
Byte 0 (Offset 0x08): Mode code:
                      0x01: Wave Short
                      0x02: Wave Long
                      0x03: Color Wheel
                      0x04: Spectrum Cycle
                      0x05: Breathing
                      0x06: Static ("Normally on")
                      0x07: Reactive ("Pass without trace")
                      0x08: Ripple
                      0x09: Reactive Line
                      0x0A: Starlight
                      0x0B: Blooming
                      0x0C: Rainbow Wave Vertical
                      0x0D: Hurricane
                      0x0E: Accumulate
                      0x10: Visor
                      0x12: Rainbow Circle
                      0x14: Custom
Byte 1 (Offset 0x09): Brightness level (0..4)
Byte 2 (Offset 0x0A): Speed level (0=Fastest .. 5=Slowest)
Byte 3 (Offset 0x0B): Direction (0: Left, 1: Right)
Byte 4 (Offset 0x0C): Random color flag (0 or 1)
Byte 5 (Offset 0x0D): Red channel   (0..255)
Byte 6 (Offset 0x0E): Green channel (0..255)
Byte 7 (Offset 0x0F): Blue channel  (0..255)
```

### 2. Dedicated Brightness (Parameter `0x01`, 1 byte)
```text
Byte 0 (Offset 0x08): Brightness level (0..4)
                      0: Off (0%)
                      1: 25%
                      2: 50%
                      3: 75%
                      4: 100%
```

### 3. USB Polling Rate (Parameter `0x0F`, 1 byte)
```text
Byte 0 (Offset 0x08): Rate code:
                      0x00: 125 Hz
                      0x01: 250 Hz
                      0x02: 500 Hz
                      0x03: 1000 Hz
```

### 4. Profiles (Parameter `0x00`, 1 byte)
```text
Byte 0 (Offset 0x08): Profile index (0, 1, 2 for Profiles 1, 2, 3)
```

### 5. Switch Actuation Point (Command `0x30`, 1 byte)
```text
Byte 0 (Offset 0x08): Actuation distance in 0.1mm units (e.g. 1.2mm = 12, 2.0mm = 20)
```

### 6. Rapid Trigger (Command `0x31`, 3 bytes)
```text
Byte 0 (Offset 0x08): Enabled flag (0x01: enabled, 0x00: disabled)
Byte 1 (Offset 0x09): Press sensitivity in 0.1mm units
Byte 2 (Offset 0x0A): Release sensitivity in 0.1mm units
```

# K552RGB-PRO Packet Format Specification

## Report ID 0x04 Format (Interface 1)

All configuration commands sent to Interface 1 follow the standard 65-byte structure (1-byte Report ID prefix + 64-byte report body).

### General Frame Layout

```text
Offset | Field           | Type  | Description
-------+-----------------+-------+---------------------------------------------------
0x00   | report_id       | u8    | Must be 0x04
0x01   | command         | u8    | Primary command category opcode
0x02   | subcommand      | u8    | Sub-operation or parameter mode
0x03   | payload_len     | u8    | Number of data bytes in payload (0..60)
0x04   | payload[0]      | u8    | First argument
...    | payload[...]    | u8    | Argument data
...    | padding         | [u8]  | Unused bytes zero-padded to offset 0x3F (63)
0x40   | checksum        | u8    | Checksum: wrapping byte sum of offsets 0x01..0x3F
```

---

## Mapped Opcodes

### 1. Static RGB Color (`command = 0x28, subcommand = 0x01`)
```text
Offset 0x04: Red channel   (0..255)
Offset 0x05: Green channel (0..255)
Offset 0x06: Blue channel  (0..255)
```

### 2. Lighting Mode & Parameters (`command = 0x28, subcommand = 0x02`)
```text
Offset 0x04: Mode ID:
             0x00: Off
             0x01: Static
             0x02: Breathing
             0x03: Rainbow
             0x04: Wave
             0x05: Reactive
             0x06: Ripple
             0x07: Flash
Offset 0x05: Brightness level (0..4)
Offset 0x06: Speed level      (0..4)
Offset 0x07: Color Red        (0..255)
Offset 0x08: Color Green      (0..255)
Offset 0x09: Color Blue       (0..255)
```

### 3. Polling Rate (`command = 0x22, subcommand = 0x01`)
```text
Offset 0x04: Rate Code:
             0x01: 125 Hz
             0x02: 250 Hz
             0x03: 500 Hz
             0x04: 1000 Hz
```

### 4. Switch Actuation Point (`command = 0x30, subcommand = 0x01`)
```text
Offset 0x04: Actuation distance in 0.1mm units (e.g. 1.2mm = 12 = 0x0C)
```

### 5. Rapid Trigger (`command = 0x31, subcommand = 0x01`)
```text
Offset 0x04: Enable flag (0x01 = enabled, 0x00 = disabled)
Offset 0x05: Press sensitivity in 0.1mm units (e.g. 0.2mm = 2 = 0x02)
Offset 0x06: Release sensitivity in 0.1mm units (e.g. 0.2mm = 2 = 0x02)
```

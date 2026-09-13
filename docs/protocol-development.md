# Protocol Development Guide

This document describes how protocols are framed, parsed, and tested in OpenDragon.

---

## 1. Separation of Protocol from Transport

In OpenDragon, protocol encoders and parsers live in `crates/opendragon-protocol` and must **never** perform IO or talk directly to operating system device handles:
- **Input**: Strongly-typed configuration structures (`LightingConfig`, `RgbColor`, `ActuationPoint`).
- **Output**: Fixed-size byte arrays or slices (`[u8; 65]`, `Vec<[u8; 16]>`).
- **Benefits**:
  - Tests run in milliseconds without requiring physical hardware.
  - Zero chance of accidental command transmission during parsing.
  - Portable across Linux, Windows, macOS, and simulated environments.

---

## 2. Command Safety Classifications

Every hardware command must be evaluated against the safety taxonomy:

| Level | Definition | Guard Rails |
|-------|------------|-------------|
| `ReadOnly` | Reads descriptors, polls input reports, inspects state | Allowed by default |
| `KnownSafe` | Visual, non-persistent configuration (e.g. RGB colors) | Allowed by default |
| `Experimental` | Functional changes under validation (actuation, polling) | Logged at info level |
| `Dangerous` | Flash writes, EEPROM, bootloader jump, factory reset | **Blocked** unless `--i-know-what-im-doing` |

```rust
use opendragon_protocol::safety::CommandSafetyClassification;

let classification = CommandSafetyClassification::known_safe("Set Static RGB");
classification.assert_can_execute(false)?;
```

---

## 3. Writing Unit Tests for Packet Builders

Always test packet generation against verified capture fixtures:

```rust
#[test]
fn static_rgb_packet_is_encoded_correctly() {
    let color = RgbColor::new(0xff, 0x00, 0x80);
    let packet = EvisionPacketBuilder::build_static_rgb_packet(color).unwrap();

    assert_eq!(packet[0], 0x04); // Report ID
    assert_eq!(packet[1], 0x28); // Opcode
    assert_eq!(packet[4], 0xff); // Red
    assert_eq!(packet[5], 0x00); // Green
    assert_eq!(packet[6], 0x80); // Blue
}
```

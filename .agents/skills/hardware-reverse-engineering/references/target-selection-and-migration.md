# Target Selection and Device Protocol Migration

## Milestone-Driven Target Prioritization

Do not attempt to reverse-engineer all hardware features simultaneously. Rank targets in order of dependency, safety, and verifiable impact:

```text
1. USB Identification (VID/PID, device strings, bcdDevice)
2. Safe Non-Destructive Probing (Descriptors, report IDs, interfaces)
3. Configuration Interface Discovery (Locating the write-capable HID node)
4. Official Software Baseline Capture (Identifying packet size, framing, checksum)
5. Static Lighting (RGB color changes are easily verified visually with zero hardware risk)
6. Dynamic Lighting Effects (Speed, brightness, effect modes)
7. Query / Readback Capabilities (Reading current configuration if device supports it)
8. Hardware Profiles (Switching active profile bank)
9. Polling Rate Adjustment (125Hz to 1000Hz+)
10. Switch Actuation Point (Hall Effect / magnetic switch depth)
11. Rapid Trigger Configuration (Dynamic switch release sensitivity)
12. Macros & Key Remapping
```

## Instrument Before Replacing

Follow this progression for every protocol feature:

```text
Original Software -> Traffic Capture -> Single-Variable Diff -> Hypothesize Packet ->
Unit Test Packet Builder -> Read-Only State Check -> Test Write on Hardware -> Physical Verification
```

### Safety Rules for Device Communication:
- Distinguish explicitly between command risk levels:
  - `ReadOnly`: Querying state, reading descriptors, reading input reports.
  - `KnownSafe`: Visually confirmable, non-persistent configuration changes (e.g. static RGB color).
  - `Experimental`: Newly mapped functional settings (actuation point, polling rate).
  - `Dangerous`: Operations touching flash memory, bootloader entry, EEPROM write, or factory reset.
- Commands classified as `Dangerous` MUST require explicit confirmation or `--i-know-what-im-doing`.
- Never run random fuzzing against real hardware.

## Preserving Portable Boundaries

Keep protocol parsing and packet builders purely functional and independent of operating system transport:
- `opendragon-protocol` creates byte slices and parses raw buffers with zero HIDAPI or IO dependencies.
- `opendragon-hid` handles transport, device nodes, and operating system quirks.
- `opendragon-devices` wires the protocol to the transport for a concrete device.

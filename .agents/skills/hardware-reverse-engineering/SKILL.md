---
name: hardware-reverse-engineering
description: Plan and conduct incremental, evidence-driven reverse engineering for hardware peripherals, USB, and HID protocols. Covers investigation scoping, USB traffic capture, differential packet analysis, report descriptor decoding, checksum identification, safe hardware validation, and structured knowledge preservation.
---

# Hardware reverse engineering

Turn each investigation into verified, revision-bound knowledge that makes the
next investigation cheaper. Do not optimize for guessing large packet sequences or
attempting full protocol implementations from weak evidence.

## Core loop

1. **Define one observable target**: a specific setting (e.g. static RGB color,
   brightness level, polling rate change, switch actuation point, profile switch).
2. **State a falsifiable hypothesis and record exact hardware context**:
   - Device model name and marketing edition;
   - USB Vendor ID (VID), Product ID (PID), and revision (`bcdDevice`);
   - Firmware version (when readable or reported by official software);
   - Official configuration software name and version used for capture;
   - Operating system and capture toolchain used (e.g. Wireshark + USBPcap on Windows, or usbmon on Linux).
3. **Capture the smallest relevant dataset**:
   - Record a baseline state where all settings are known and held constant.
   - Change exactly one variable in the official software (e.g., color `#FF0000` to `#00FF00`).
   - Capture the resulting USB packets or HID reports.
4. **Compare two situations differing in a single variable**:
   - Use differential packet analysis (such as `hid-diff`) to isolate the exact byte offsets that changed.
5. **Formulate the packet hypothesis**:
   - Identify the Report ID, Command Opcode, Subcommand, Payload offsets, argument encoding, and Checksum.
6. **Instrument before implementing**:
   - Write an independent unit test or minimal fixture encoding the hypothesized packet before issuing writes to physical hardware.
7. **Implement only the minimum needed to test the hypothesis**:
   - Create or update the packet builder in `opendragon-protocol` and device method in `opendragon-devices`.
8. **Validate on physical hardware**:
   - Transmit the generated packet using a safe, controlled command.
   - Visually or programmatically verify that the device performed the expected state change.
9. **Test a competing explanation or negative control**:
   - Test a non-primary value (e.g. test `#0000FF` and then `#123456`).
   - If offsets represent RGB channels, verify that swapping green and blue channels behaves as predicted.
10. **Record findings**:
    - Record raw observations, inferred conclusion, confidence level, rejected hypotheses, remaining questions, and the next logical target in `re/devices/<device>/investigations/`.

Read [knowledge-system.md](references/knowledge-system.md) when structuring `re/` metadata,
storing captures, or authoring protocol analysis tools.
Read [target-selection-and-migration.md](references/target-selection-and-migration.md)
when prioritizing hardware capabilities or migrating protocols into device drivers.
Read [validation-and-journal.md](references/validation-and-journal.md)
when reporting results, assessing risk, or writing investigation notes.

## Invariants

- A packet opcode, offset, checksum algorithm, or report ID is not a fact without empirical evidence bound to a specific hardware revision.
- Bulk binary captures (`.pcap`, `.pcapng`) and proprietary software dumps remain local. Commit focused metadata, minimal packet fixtures, project-authored tools, and reproducible conclusions only.
- Respect byte endianness, report lengths, and padding; do not omit mandatory report ID prefixes or trailing padding required by HID descriptors.
- Convert repeated deterministic analysis (e.g. diffing packets, calculating checksums) into tools under `tools/`.
- Never claim runtime correctness from compilation alone: a feature is only `Supported` once verified on physical hardware.
- Never send random or fuzzing commands to unknown endpoints. High-risk operations (firmware updates, EEPROM writes, factory resets) are classified as `Dangerous` and must be shielded behind `--i-know-what-im-doing`.

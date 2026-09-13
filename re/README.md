# OpenDragon Reverse Engineering Knowledge Base

This directory (`re/`) contains verified, revision-bound empirical knowledge for Redragon peripherals.

## Guiding Principles

1. **Evidence-driven knowledge**: Do not commit speculation. An address, report ID, opcode, or capability is only a fact when tied to a specific device, firmware revision, capture, or test.
2. **Local dumps stay local**: Do not commit multi-megabyte proprietary Wireshark dumps or binary memory dumps to Git. Commit focused metadata, project-authored parsers, minimal fixtures, and reproducible conclusions.
3. **Controlled experimentation**: Vary one variable at a time (e.g., `#FF0000` -> `#00FF00` with all other software settings untouched).
4. **Hardware safety first**: Never run destructive fuzzing or send unknown command sequences to hardware. Distinguish explicitly between:
   - `Observed`: Seen in USB trace from official software.
   - `Reproduced`: Sent to device with observable effect.
   - `Partially Understood`: Some fields mapped, others unknown.
   - `Understood`: Offsets, framing, units, and side-effects fully verified.
   - `Unknown`: Unmapped sequence. Treat as dangerous if it touches flash/bootloader/EEPROM.

## Directory Structure

```text
re/
├── README.md
├── devices/
│   └── k552rgb-pro/
│       ├── device.toml                  # Static device metadata and identifiers
│       ├── protocol.md                  # Protocol framing and transport mechanism
│       ├── capabilities.md              # Status of hardware features
│       ├── packet-format.md             # Detailed packet byte offsets and field maps
│       ├── probe-baseline.json          # Machine-readable probe capture from real hardware
│       └── investigations/              # Chronological numbered investigation notes
│           └── 0001-device-identification.md
│
├── captures/
│   ├── README.md                        # Documentation of capture methodologies
│   ├── legacy/                          # Preserved captures from original M607/M909 research
│   └── k552rgb-pro/                     # Minimal reproducible test fixtures
│
└── tools/
    └── hid-diff                         # Differential packet analysis utility (in workspace root tools/)
```

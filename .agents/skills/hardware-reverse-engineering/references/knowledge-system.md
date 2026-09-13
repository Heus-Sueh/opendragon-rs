# Hardware Reverse-Engineering Knowledge System

## Role of Analysis Tools and Dumps

Treat USB capture tools (Wireshark, USBPcap, `usbmon`, `tshark`, `hidraw` sniffers) and static disassembly tools (Ghidra, IDA, strings) as analysis engines, not version-controlled repositories.

Keep large `.pcap`, `.pcapng`, `.gpr`, `.rep`, full memory dumps, and proprietary driver installer binaries outside Git.

Prefer narrow, reproducible exports:
- Decoded HID Report Descriptors (raw hex and parsed item trees).
- Minimal JSON fixtures representing isolated configuration sequences.
- Structured device metadata in TOML (`re/devices/<device>/device.toml`).
- Offsets, opcodes, field definitions, and checksum algorithms documented in Markdown.

An agent should inspect additional traffic only when the current evidence cannot resolve the working hypothesis. Avoid hoarding megabytes of unparsed background polling traffic.

## Versioned Knowledge Layer

Create `re/` data as verified empirical evidence becomes available. The standard shape is:

```text
re/
├── README.md
├── devices/
│   └── <device-slug>/
│       ├── device.toml                  # VID, PID, bcdDevice, manufacturer, product, endpoints
│       ├── protocol.md                  # Framing, interfaces, transport mechanism
│       ├── capabilities.md              # Capabilities status matrix with confidence levels
│       ├── packet-format.md             # Byte offset layouts and field definitions
│       └── investigations/              # Numbered investigation journal
│           ├── 0001-target-name.md
│           └── ...
└── captures/
    ├── README.md
    └── <device-slug>/
```

Every investigation note and packet specification must identify:
- Device model and revision (`VID:PID:bcdDevice`);
- Firmware revision when known;
- Specific interface number and Report ID;
- Evidence source (e.g. Wireshark trace, non-destructive probe, physical validation);
- Confidence rating (`low`, `medium`, or `high`);
- Negative controls or competing explanations tested.

## Deterministic Tooling

Whenever agents perform the same comparison, offset lookup, entropy calculation, or checksum search manually, convert that task into a deterministic command under `tools/` (such as `tools/hid-diff`). The tool must take explicit input files, produce clean diffable output, and exit clearly on format error.

# OpenDragon System Architecture

OpenDragon is a modern, modular, and type-safe userspace framework for configuring and reverse-engineering Redragon peripherals on Linux and other platforms.

---

## 1. Architectural Principles

1. **Userspace Execution**:
   Peripheral configuration belongs in userspace using standard Linux `hidraw` and USB HID protocols. The legacy Linux kernel module architecture has been completely deprecated in favor of a memory-safe Rust implementation.
2. **Clear Layer Separation**:
   - **Domain & Core Types**: Pure abstractions (`Device`, `Capability`, `DeviceInfo`, `RgbColor`, errors). Zero IO dependencies.
   - **Transport Layer**: Device enumeration via `hidapi` with fallback enrichment from Linux `sysfs`. Safe descriptor parsing and non-destructive probing.
   - **Protocol Layer**: Pure functional packet builders and parsers. Command safety classifications.
   - **Device Implementations**: Specific driver bindings connecting protocol packets to physical device interfaces.
   - **Applications**: CLI (`opendragon`), RE diff utilities (`hid-diff`), and future daemon/API layers.
3. **Safety by Design**:
   No unverified or dangerous commands are sent to physical hardware without explicit user acknowledgement. Non-destructive probing allows safe hardware introspection even on unmapped models.

---

## 2. Workspace Organization & Component Responsibilities

```text
opendragon/
├── crates/
│   ├── opendragon-core/            # Domain models, traits, capabilities, and errors
│   ├── opendragon-hid/             # Hardware discovery, HID transport, descriptor parsing, probe
│   ├── opendragon-protocol/        # Packet builders, framing, checksums, and safety policies
│   └── opendragon-devices/         # Concrete device drivers and driver registry
├── apps/
│   └── opendragon-cli/             # User CLI tool
└── tools/
    └── hid-diff/                   # Differential packet analysis utility
```

### `opendragon-core`
Contains all shared types and abstractions:
- `pub trait Device: Send + Sync`: Defines the driver interface (`info()`, `capabilities()`, `set_lighting()`, `set_actuation()`, `set_polling_rate()`).
- `Capability`: Enumeration of hardware features (`Lighting`, `PerKeyLighting`, `PollingRate`, `Actuation`, `RapidTrigger`, `Profile`, `Macro`, `Dpi`).
- `SupportStatus`: Verification status (`Supported`, `Partial`, `Experimental`, `Wip`, `Unknown`, `Unsupported`).
- `DeviceInfo` & `InterfaceInfo`: Discovered hardware descriptors.
- `RgbColor`, `LightingConfig`, `ActuationPoint`, `RapidTriggerConfig`, `PollingRate`.
- `OpenDragonError`: Strongly typed error hierarchy.

### `opendragon-hid`
Handles low-level hardware communication:
- `DeviceEnumerator`: Enumerates devices via HIDAPI and enriches descriptors from world-readable Linux `/sys/bus/hid/devices/` and `/sys/bus/usb/devices/`.
- `HidTransportHandle`: Thread-synchronized device communication handle (`send_feature_report`, `get_feature_report`, `write_output_report`, `read_input_report`).
- `HidReportDescriptor`: Custom pure-Rust parser decoding raw HID report descriptors into structured collections, usages, and report IDs.
- `DeviceProber`: Performs non-destructive hardware probing and produces versioned JSON reports (`probe.json`).

### `opendragon-protocol`
Builds and decodes protocol packets:
- `LegacyMousePacketBuilder`: Preserves reverse-engineered M607 / M909 mouse protocols (16-byte lighting & profile packets, 64-byte bulk apply packets).
- `EvisionPacketBuilder`: EVision / Sonix keyboard framing (64-byte configuration packets on Report ID 0x04).
- `CommandSafetyClassification`: Guard rails categorizing commands into `ReadOnly`, `KnownSafe`, `Experimental`, and `Dangerous`.

### `opendragon-devices`
Maintains concrete device implementations and discovery matching:
- `RedragonKumaraProK552`: Primary keyboard implementation for the K552RGB-PRO family.
- `RedragonM607`: M607 Griffin driver.
- `RedragonM909`: M909 Emperor driver.
- `GenericRedragonDevice`: Fallback driver for unmapped Redragon devices.
- `DeviceRegistry`: Matcher linking discovered USB VID/PIDs to drivers.

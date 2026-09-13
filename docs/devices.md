# Supported Devices & Compatibility Matrix

OpenDragon adheres to an evidence-based support policy: a capability is marked as `Supported` only after it has been verified on physical hardware.

## Standardized Status Definitions

- **Supported**: Verified and working on physical hardware.
- **Partial**: Subset of features confirmed on physical hardware.
- **Experimental**: Driver implementation exists based on protocol reversing; under hardware testing.
- **WIP**: Feature is actively being researched, captured, or developed.
- **Unknown**: Feature existence or protocol mapping not yet established.
- **Unsupported**: The physical device does not possess this hardware feature.

---

## Device Compatibility Matrix

| Device | VID:PID | Detect | RGB | Per-key | Polling | Actuation | Rapid Trigger | Profiles | Macros |
|--------|---------|--------|-----|---------|---------|-----------|---------------|----------|--------|
| **Redragon Kumara Pro K552RGB-PRO** | `0c45:5004` | Supported | Experimental | WIP | Experimental | WIP | WIP | WIP | Unknown |
| **Redragon Mitra RGB K551** | `0c45:5004` | Supported | Experimental | WIP | Experimental | Unsupported | Unsupported | WIP | Unknown |
| **Redragon M607 Griffin** | `04d9:fc38` | Supported | Supported | Unsupported | Partial | Unsupported | Unsupported | Supported | WIP |
| **Redragon M909 Emperor** | `04d9:fc4c` | Supported | Supported | Unsupported | Partial | Unsupported | Unsupported | Supported | WIP |
| **Redragon M601-3 Centrophorus** | `04d9:fc18` | Supported | WIP | Unsupported | WIP | Unsupported | Unsupported | WIP | Unknown |
| **ACRUX / Redragon 2.4G Receiver** | `1a34:f517` | Supported | Unsupported | Unsupported | WIP | Unsupported | Unsupported | Unsupported | Unsupported |

---

## Device Details

### Redragon Kumara Pro K552RGB-PRO
- **VID / PID**: `0x0C45` / `0x5004`
- **MCU Architecture**: Sonix SN32F24x / EVision
- **Config Interface**: Interface 1, Report ID 4 (`Usage Page 0xFF1C`, `Usage 0x0092`)
- **Key Features**: Magnetic switch actuation, Rapid Trigger, RGB backlighting.

### Redragon M607 Griffin (M602-RGB)
- **VID / PID**: `0x04D9` / `0xFC38`
- **MCU Architecture**: Holtek Semiconductor
- **Config Interface**: Interface 2, Feature Report ID 2 (`0x02`), 16-byte packets.

### Redragon M909 Emperor
- **VID / PID**: `0x04D9` / `0xFC4C`
- **MCU Architecture**: Holtek Semiconductor
- **Config Interface**: Interface 2, Feature Report ID 2 (`0x02`), 16-byte packets.

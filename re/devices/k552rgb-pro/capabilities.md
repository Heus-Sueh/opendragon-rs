# K552RGB-PRO Hardware Capabilities Matrix

| Capability | Status | Confidence | Evidence Source | Notes |
|------------|--------|------------|-----------------|-------|
| USB Discovery | Supported | High | Real Hardware Probe | VID `0x0C45`, PID `0x5004`, bcdDevice `0x0105` verified via `opendragon probe`. |
| Descriptor Parsing | Supported | High | Real Hardware Probe | Report descriptors for Interface 0 and 1 dumped and parsed. |
| Configuration Interface | Supported | High | Real Hardware Probe & Sysfs | Interface 1, Report ID 4 (Vendor Page `0xFF1C`, Usage `0x0092`), `/dev/hidraw3`. |
| Lighting (Global Effects) | Supported | High | Real Hardware Verification | EVision CMD `0x06`, PARAM `0x00` (ModeEx). Static, Breathing, Rainbow, Wave, etc. |
| Static RGB Color | Supported | High | Real Hardware Verification | EVision CMD `0x06`, PARAM `0x00` (RGB payload bytes 5..7). |
| Lighting Brightness | Supported | High | Real Hardware Verification | EVision CMD `0x06`, PARAM `0x01` and ModeEx byte 1 (discrete levels 0..4). |
| Polling Rate Adjustment | Supported | High | Real Hardware Verification | EVision CMD `0x06`, PARAM `0x0F` (125Hz, 250Hz, 500Hz, 1000Hz). |
| Hardware Profiles | Supported | High | Real Hardware Verification | EVision CMD `0x06`, PARAM `0x00` profile offsets (Profiles 1..3). |
| Switch Actuation Point | Experimental | Medium | Protocol Framing | EVision CMD `0x30` (0.1mm units). Supported on Hall Effect variants. |
| Rapid Trigger Dynamic Reset| Experimental | Medium | Protocol Framing | EVision CMD `0x31` (Press/Release sensitivity). Supported on Hall Effect variants. |
| Per-Key RGB Matrix | WIP | Medium | OpenRGB Protocol Reference | EVision CMD `0x11` (chunks of 0x36 bytes) with Begin `0x01` and End `0x02`. |
| On-Board Macros | WIP | Low | OpenRGB V2 Struct Reference | Macro memory blocks present in firmware memory map. |

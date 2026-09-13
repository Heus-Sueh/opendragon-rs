# K552RGB-PRO Hardware Capabilities Matrix

| Capability | Status | Confidence | Evidence Source | Notes |
|------------|--------|------------|-----------------|-------|
| USB Discovery | Supported | High | Real Hardware Probe | VID `0x0C45`, PID `0x5004`, bcdDevice `0x0105` verified via `opendragon probe`. |
| Descriptor Parsing | Supported | High | Real Hardware Probe | Report descriptors for Interface 0 and 1 dumped and parsed. |
| Configuration Interface | Supported | High | Sysfs & Report Descriptors | Interface 1, Report ID 4 (Vendor Page `0xFF1C`, Usage `0x0092`). |
| Lighting (Global Effects) | Experimental | Medium | EVision/Sonix Protocol Cross-Ref | Standard EVision opcode `0x28` mapped. |
| Static RGB Color | Experimental | Medium | EVision/Sonix Protocol Cross-Ref | Standard EVision opcode `0x28`, sub `0x01` mapped. |
| Polling Rate Adjustment | Experimental | Medium | Report Descriptor & EVision Framing | Opcode `0x22` mapped. Hardware supports up to 1000Hz (interval=1ms). |
| Actuation Point Setting | WIP | Low | Feature candidate | Present on Hall Effect switch variants. Requires confirmation on specific PCB rev. |
| Rapid Trigger Dynamic Reset| WIP | Low | Feature candidate | Present on Hall Effect switch variants. Requires confirmation on specific PCB rev. |
| Hardware Profiles | WIP | Low | Candidate | Profile selection opcode mapped. |
| Per-Key RGB Matrix | WIP | Low | Candidate | Bulk matrix packet format under investigation. |
| On-Board Macros | Unknown | Low | Not investigated | Protocol structure pending official software trace. |

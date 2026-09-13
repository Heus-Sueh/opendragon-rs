# Hardware Captures Guidelines

This directory stores reproducible traffic captures and packet fixtures for Redragon devices.

## Capture Workflow

1. **Controlled Single-Variable Changes**:
   - Establish baseline capture with all settings default.
   - Modify exactly one setting in official software (e.g., color `#FF0000` -> `#00FF00`).
   - Capture traffic during the change.
   - Run `hid-diff` to isolate the changed offsets.
2. **Commit Policy**:
   - Do not commit large binary `.pcap` or `.pcapng` files directly into Git.
   - Instead, commit minimal JSON fixture files or text hex dump files containing only the active configuration packets.
3. **Capture Directory Layout**:
   - `re/captures/legacy/`: Contains original research text dumps from M607 and M909 mice.
   - `re/captures/k552rgb-pro/`: Contains minimal verified packet sequences for Kumara Pro K552RGB-PRO.

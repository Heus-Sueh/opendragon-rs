# Hardware Reverse Engineering Workflow

OpenDragon employs an incremental, evidence-driven reverse engineering methodology.

---

## 1. Capturing Official Software Traffic

### Windows Workflow:
1. Install **Wireshark** with **USBPcap**.
2. Identify the USB Root Hub to which the Redragon device is attached.
3. Start capturing on the corresponding USBPcap interface.
4. Filter by USB address: `usb.device_address == X`.

### Linux Workflow:
1. Enable `usbmon`:
   ```bash
   sudo modprobe usbmon
   ```
2. Sniff via Wireshark on `usbmonX` or using `tcpdump`:
   ```bash
   sudo tcpdump -i usbmon1 -w capture.pcap
   ```

---

## 2. The Controlled Single-Variable Experiment

Avoid changing multiple settings at once. The only reliable way to map proprietary protocols is single-variable differential analysis:

```text
Baseline State
(RGB: #FF0000, Brightness: 100%, Speed: 50%)
      │
      ▼
Capture 1 (baseline.txt)
      │
      ▼
Change ONE setting in official software:
(Change color to #00FF00, keep all other settings untouched)
      │
      ▼
Capture 2 (modified.txt)
      │
      ▼
Run differential analysis:
cargo run -p hid-diff -- baseline.txt modified.txt
      │
      ▼
Isolate modified offsets:
Offset 0x0004: FF -> 00  (Red)
Offset 0x0005: 00 -> FF  (Green)
```

---

## 3. Tooling: `hid-diff`

Use `hid-diff` to compare packet sequences:
```bash
cargo run -p hid-diff -- capture_a.txt capture_b.txt
```

Example output:
```text
Packet 2 (Length A: 16 bytes, Length B: 16 bytes)
  Report ID: 0x02
  Offset   A      B      Note
  ---------------------------
  0x000C   02     08     changed

  unchanged: 15 byte(s)
  changed:   1 byte(s)
```

---

## 4. Documentation of Discoveries

Record every finding in the knowledge base:
- Metadata in `re/devices/<device>/device.toml`.
- Packet layouts in `re/devices/<device>/packet-format.md`.
- Numbered investigation notes in `re/devices/<device>/investigations/`.

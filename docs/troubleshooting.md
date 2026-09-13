# Troubleshooting Guide

Common issues and solutions when using OpenDragon.

---

## 1. "Permission Denied" When Opening Device Nodes

### Symptom:
```text
Error: Permission denied accessing HID device at /dev/hidrawX. Run with appropriate permissions or configure udev rules.
```

### Cause:
By default, standard Linux system configurations restrict `/dev/hidraw*` device nodes to the `root` user (`0600`).

### Solution:
Install the OpenDragon udev rules to grant unprivileged access via systemd user ACLs and the `plugdev` group:

```bash
# 1. Copy udev rule
sudo cp udev/70-opendragon.rules /etc/udev/rules.d/

# 2. Reload udev rules and trigger device events
sudo udevadm control --reload-rules && sudo udevadm trigger

# 3. Ensure your user belongs to the plugdev group (if applicable)
sudo usermod -a -G plugdev $USER
```

Verify permissions using the built-in diagnostic tool:
```bash
opendragon permissions
```

---

## 2. Device Not Found

### Symptom:
`opendragon list` returns:
```text
No Redragon peripherals detected.
```

### Solution:
1. Ensure the device is plugged in via USB (wireless 2.4G dongles must also be inserted).
2. Run `opendragon list --all` to list all connected HID devices regardless of vendor ID.
3. Check `lsusb` to confirm the operating system recognizes the hardware:
   ```bash
   lsusb
   ```
4. If your device uses a newly released VID/PID not yet in OpenDragon's known candidate list, target it explicitly:
   ```bash
   opendragon info --vid <VID> --pid <PID>
   ```

---

## 3. "Hardware Safety Violation" Error

### Symptom:
```text
Error: Hardware safety violation: Command '...' is classified as Dangerous. Potential risk to hardware/firmware.
```

### Cause:
The command requested would write to sensitive hardware registers, flash memory, or enter bootloader mode.

### Solution:
OpenDragon prevents unintentional firmware damage by default. If you have confirmed the sequence and are conducting low-level firmware research, pass the explicit bypass flag:
```bash
opendragon --i-know-what-im-doing <command>
```

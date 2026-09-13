use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::debug;

use opendragon_core::device::{DeviceInfo, InterfaceInfo};
use opendragon_core::error::OpenDragonError;

use crate::descriptor::HidReportDescriptor;

/// A discovered physical HID peripheral with its interface details.
#[derive(Debug, Clone)]
pub struct DiscoveredDevice {
    pub info: DeviceInfo,
    pub report_descriptors: HashMap<u8, HidReportDescriptor>,
}

pub struct DeviceEnumerator;

impl DeviceEnumerator {
    /// Enumerates all connected HID devices, collecting hardware descriptors,
    /// interfaces, and endpoints.
    pub fn enumerate_all() -> Result<Vec<DiscoveredDevice>, OpenDragonError> {
        let mut devices_map: HashMap<(u16, u16, String), DiscoveredDevice> = HashMap::new();

        // 1. Try HIDAPI enumeration first
        if let Ok(api) = hidapi::HidApi::new() {
            for hid_dev in api.device_list() {
                let vid = hid_dev.vendor_id();
                let pid = hid_dev.product_id();
                let path = hid_dev.path().to_string_lossy().to_string();
                let mfg = hid_dev
                    .manufacturer_string()
                    .unwrap_or("Unknown")
                    .to_string();
                let prod = hid_dev.product_string().unwrap_or("HID Device").to_string();
                let serial = hid_dev.serial_number().map(|s| s.to_string());
                let release = Some(hid_dev.release_number());
                let iface_num = if hid_dev.interface_number() >= 0 {
                    hid_dev.interface_number() as u8
                } else {
                    0
                };

                let dev_key = (vid, pid, serial.clone().unwrap_or_default());
                let entry = devices_map.entry(dev_key).or_insert_with(|| {
                    let mut dev_info = DeviceInfo::new(vid, pid, mfg, prod);
                    dev_info.serial_number = serial;
                    dev_info.release_number = release;
                    DiscoveredDevice {
                        info: dev_info,
                        report_descriptors: HashMap::new(),
                    }
                });

                // Add or update interface info
                let iface = InterfaceInfo {
                    interface_number: iface_num,
                    usage_page: Some(hid_dev.usage_page()),
                    usage: Some(hid_dev.usage()),
                    report_ids: Vec::new(),
                    endpoints: Vec::new(),
                    hidraw_path: Some(path),
                };

                if !entry
                    .info
                    .interfaces
                    .iter()
                    .any(|i| i.interface_number == iface_num)
                {
                    entry.info.interfaces.push(iface);
                }
            }
        }

        // 2. Enrich and fallback using Linux sysfs (world-readable on Linux)
        #[cfg(target_os = "linux")]
        {
            Self::enrich_from_linux_sysfs(&mut devices_map);
        }

        let mut results: Vec<DiscoveredDevice> = devices_map.into_values().collect();
        results.sort_by_key(|d| (d.info.vendor_id, d.info.product_id));
        Ok(results)
    }

    /// Enumerates only Redragon and known OEM-partner devices (Redragon, Sonix, Microdia, ACRUX, Holtek).
    pub fn enumerate_redragon() -> Result<Vec<DiscoveredDevice>, OpenDragonError> {
        let all = Self::enumerate_all()?;
        let filtered = all
            .into_iter()
            .filter(|d| Self::is_redragon_candidate(&d.info))
            .collect();
        Ok(filtered)
    }

    /// Checks if a device matches known Redragon VIDs or strings.
    pub fn is_redragon_candidate(info: &DeviceInfo) -> bool {
        // Known Redragon / OEM Vendor IDs:
        // 0x04D9: Holtek Semiconductor (M607, M909, M601, older Kumara K552)
        // 0x0C45: Microdia / Sonix (Kumara K552, Mitra K551, K552RGB-PRO, EVision)
        // 0x1A34: ACRUX (Redragon wireless 2.4G dongles/receivers)
        // 0x258A: SinoWealth (Redragon mice/keyboards)
        // 0x320F: Redragon native VID / EVision
        let vid = info.vendor_id;
        let is_known_vid = matches!(vid, 0x04d9 | 0x0c45 | 0x1a34 | 0x258a | 0x320f);

        let mfg_lower = info.vendor_name.to_lowercase();
        let prod_lower = info.product_name.to_lowercase();
        let has_redragon_name = mfg_lower.contains("redragon")
            || prod_lower.contains("redragon")
            || prod_lower.contains("kumara")
            || prod_lower.contains("mitra")
            || (mfg_lower.contains("sonix") && vid == 0x0c45);

        is_known_vid || has_redragon_name
    }

    #[cfg(target_os = "linux")]
    fn enrich_from_linux_sysfs(devices_map: &mut HashMap<(u16, u16, String), DiscoveredDevice>) {
        let sysfs_hid = Path::new("/sys/bus/hid/devices");
        if !sysfs_hid.exists() {
            return;
        }

        let entries = match fs::read_dir(sysfs_hid) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let uevent_path = path.join("uevent");
            if !uevent_path.exists() {
                continue;
            }

            let uevent = match fs::read_to_string(&uevent_path) {
                Ok(u) => u,
                Err(_) => continue,
            };

            let mut vid: Option<u16> = None;
            let mut pid: Option<u16> = None;
            let mut hid_name: Option<String> = None;

            for line in uevent.lines() {
                if let Some(id_str) = line.strip_prefix("HID_ID=") {
                    // format: 0003:00000C45:00005004
                    let parts: Vec<&str> = id_str.split(':').collect();
                    if parts.len() == 3 {
                        vid = u16::from_str_radix(parts[1], 16).ok();
                        pid = u16::from_str_radix(parts[2], 16).ok();
                    }
                } else if let Some(name) = line.strip_prefix("HID_NAME=") {
                    hid_name = Some(name.to_string());
                }
            }

            let (vid, pid) = match (vid, pid) {
                (Some(v), Some(p)) => (v, p),
                _ => continue,
            };

            // Read report_descriptor if present
            let report_desc_path = path.join("report_descriptor");
            let parsed_desc = if report_desc_path.exists() {
                fs::read(&report_desc_path)
                    .ok()
                    .map(|b| HidReportDescriptor::parse(&b))
            } else {
                None
            };

            // Locate hidraw node (e.g. /sys/bus/hid/devices/.../hidraw/hidrawX)
            let mut hidraw_node: Option<String> = None;
            let hidraw_dir = path.join("hidraw");
            if let Ok(hr_entries) = fs::read_dir(&hidraw_dir) {
                for hr_entry in hr_entries.flatten() {
                    let name = hr_entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("hidraw") {
                        hidraw_node = Some(format!("/dev/{name}"));
                        break;
                    }
                }
            }

            // Read interface number from sysfs path if available (e.g. 1-2:1.0)
            let mut iface_num: u8 = 0;
            if let Ok(target) = fs::read_link(&path) {
                let target_str = target.to_string_lossy();
                if let Some(pos) = target_str.rfind(":1.") {
                    let sub = &target_str[pos + 3..];
                    if let Some(num_str) = sub.split('/').next() {
                        if let Ok(n) = num_str.parse::<u8>() {
                            iface_num = n;
                        }
                    }
                }
            }

            // Read USB manufacturer / product strings from parent USB device if available
            let mut mfg_name = "Unknown".to_string();
            let mut prod_name = hid_name.unwrap_or_else(|| "HID Device".to_string());
            let mut serial_num: Option<String> = None;
            let mut release_num: Option<u16> = None;

            // Walk direct parent USB device only (e.g. 1-2, not usb1 or pci)
            if let Ok(canonical) = path.canonicalize() {
                let mut cur = canonical.as_path();
                for _ in 0..3 {
                    if let Some(parent) = cur.parent() {
                        cur = parent;
                        let dev_name = cur.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        // Skip root hubs (usbX) and PCI controllers
                        if dev_name.starts_with("usb") || dev_name.contains(':') {
                            continue;
                        }

                        let mfg_file = cur.join("manufacturer");
                        let prod_file = cur.join("product");
                        let serial_file = cur.join("serial");
                        let bcd_file = cur.join("bcdDevice");

                        if mfg_file.exists() && mfg_name == "Unknown" {
                            if let Ok(s) = fs::read_to_string(&mfg_file) {
                                let st = s.trim();
                                if !st.is_empty() {
                                    mfg_name = st.to_string();
                                }
                            }
                        }
                        if prod_file.exists() {
                            if let Ok(s) = fs::read_to_string(&prod_file) {
                                let st = s.trim();
                                if !st.is_empty() && !st.contains("Host Controller") {
                                    prod_name = st.to_string();
                                }
                            }
                        }
                        if serial_file.exists() && serial_num.is_none() {
                            if let Ok(s) = fs::read_to_string(&serial_file) {
                                let st = s.trim();
                                if !st.is_empty() {
                                    serial_num = Some(st.to_string());
                                }
                            }
                        }
                        if bcd_file.exists() && release_num.is_none() {
                            if let Ok(s) = fs::read_to_string(&bcd_file) {
                                if let Ok(n) = u16::from_str_radix(s.trim(), 16) {
                                    release_num = Some(n);
                                }
                            }
                        }
                    }
                }
            }

            let dev_key = (vid, pid, serial_num.clone().unwrap_or_default());
            let entry = devices_map.entry(dev_key).or_insert_with(|| {
                let mut dev_info = DeviceInfo::new(vid, pid, mfg_name.clone(), prod_name.clone());
                dev_info.serial_number = serial_num;
                dev_info.release_number = release_num;
                DiscoveredDevice {
                    info: dev_info,
                    report_descriptors: HashMap::new(),
                }
            });

            // Update manufacturer / product strings if sysfs has better names
            if entry.info.vendor_name == "Unknown" && mfg_name != "Unknown" {
                entry.info.vendor_name = mfg_name;
            }
            if (entry.info.product_name == "HID Device"
                || entry.info.product_name.contains("Host Controller"))
                && !prod_name.contains("Host Controller")
            {
                entry.info.product_name = prod_name;
            }

            if let Some(desc) = &parsed_desc {
                entry.report_descriptors.insert(iface_num, desc.clone());
            }

            // Update interface if not present or missing hidraw path
            if let Some(existing) = entry
                .info
                .interfaces
                .iter_mut()
                .find(|i| i.interface_number == iface_num)
            {
                if existing.hidraw_path.is_none() && hidraw_node.is_some() {
                    existing.hidraw_path = hidraw_node;
                }
                if let Some(desc) = &parsed_desc {
                    existing.report_ids = desc.report_ids.clone();
                    if existing.usage_page.is_none() || existing.usage_page == Some(0) {
                        existing.usage_page = desc.usage_pages.first().copied();
                    }
                    if existing.usage.is_none() || existing.usage == Some(0) {
                        existing.usage = desc.usages.first().copied();
                    }
                }
            } else {
                let report_ids = parsed_desc
                    .as_ref()
                    .map(|d| d.report_ids.clone())
                    .unwrap_or_default();
                let up = parsed_desc
                    .as_ref()
                    .and_then(|d| d.usage_pages.first().copied());
                let u = parsed_desc.as_ref().and_then(|d| d.usages.first().copied());

                entry.info.interfaces.push(InterfaceInfo {
                    interface_number: iface_num,
                    usage_page: up,
                    usage: u,
                    report_ids,
                    endpoints: Vec::new(),
                    hidraw_path: hidraw_node,
                });
            }
        }

        // Sort interfaces by interface_number
        for dev in devices_map.values_mut() {
            dev.info.interfaces.sort_by_key(|i| i.interface_number);
        }

        debug!(
            "Discovered {} devices after sysfs enrichment",
            devices_map.len()
        );
    }
}

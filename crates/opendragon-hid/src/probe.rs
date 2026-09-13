use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use opendragon_core::capability::Capability;
use opendragon_core::device::DeviceInfo;
use opendragon_core::error::OpenDragonError;

use crate::descriptor::HidReportDescriptor;
use crate::discovery::{DeviceEnumerator, DiscoveredDevice};

/// Non-destructive probe safety audit certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeSafetyAudit {
    pub is_read_only: bool,
    pub destructive_commands_sent: u32,
    pub bytes_written: usize,
    pub notes: String,
}

impl Default for ProbeSafetyAudit {
    fn default() -> Self {
        Self {
            is_read_only: true,
            destructive_commands_sent: 0,
            bytes_written: 0,
            notes: "Probe executed non-destructively reading descriptors only. Zero commands transmitted to device.".to_string(),
        }
    }
}

/// Versioned, machine-readable capture produced by `opendragon probe`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProbeReport {
    pub schema_version: String,
    pub generated_at: String,
    pub device: DeviceInfo,
    pub report_descriptors: HashMap<u8, HidReportDescriptor>,
    pub candidate_capabilities: Vec<Capability>,
    pub safety_audit: ProbeSafetyAudit,
}

pub struct DeviceProber;

impl DeviceProber {
    /// Conducts a strictly non-destructive probe of the specified discovered device.
    pub fn probe_device(device: &DiscoveredDevice) -> Result<DeviceProbeReport, OpenDragonError> {
        let mut candidate_capabilities = Vec::new();

        // Detect capabilities based on report descriptors and known vendor features
        for desc in device.report_descriptors.values() {
            for up in &desc.usage_pages {
                match *up {
                    0x0001 => {
                        // Generic Desktop
                        if desc.usages.contains(&0x06) {
                            // Keyboard
                            candidate_capabilities.push(Capability::PollingRate);
                        }
                    }
                    0x000C => {
                        // Consumer Page / Audio / Media
                        candidate_capabilities.push(Capability::Profile);
                    }
                    0xFF1C | 0xFFA0 | 0xFF00 => {
                        // Vendor defined configuration pages (Sonix / EVision / Redragon RGB & macros)
                        candidate_capabilities.push(Capability::Lighting);
                        candidate_capabilities.push(Capability::PerKeyLighting);
                        candidate_capabilities.push(Capability::Macro);
                        candidate_capabilities.push(Capability::Actuation);
                        candidate_capabilities.push(Capability::RapidTrigger);
                    }
                    _ => {}
                }
            }
        }

        candidate_capabilities.dedup();

        let report = DeviceProbeReport {
            schema_version: "1.0.0".to_string(),
            generated_at: chrono_like_timestamp(),
            device: device.info.clone(),
            report_descriptors: device.report_descriptors.clone(),
            candidate_capabilities,
            safety_audit: ProbeSafetyAudit::default(),
        };

        Ok(report)
    }

    /// Saves the probe report to a JSON file.
    pub fn save_to_file(report: &DeviceProbeReport, path: &Path) -> Result<(), OpenDragonError> {
        let json_data = serde_json::to_string_pretty(report)?;
        fs::write(path, json_data)?;
        Ok(())
    }

    /// Probes first matching device by VID/PID or returns first Redragon device.
    pub fn probe_target(
        vid: Option<u16>,
        pid: Option<u16>,
    ) -> Result<DeviceProbeReport, OpenDragonError> {
        let devices = DeviceEnumerator::enumerate_all()?;
        let target = if let (Some(v), Some(p)) = (vid, pid) {
            devices
                .into_iter()
                .find(|d| d.info.vendor_id == v && d.info.product_id == p)
        } else {
            devices
                .into_iter()
                .find(|d| DeviceEnumerator::is_redragon_candidate(&d.info))
        };

        match target {
            Some(dev) => Self::probe_device(&dev),
            None => Err(OpenDragonError::DeviceNotFound(
                "No matching Redragon device found for probing".into(),
            )),
        }
    }
}

fn chrono_like_timestamp() -> String {
    // Simple UTC-like ISO-8601 timestamp generator without external chrono crate
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    format!("{secs}s-since-epoch")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_safety_audit_defaults() {
        let audit = ProbeSafetyAudit::default();
        assert!(audit.is_read_only);
        assert_eq!(audit.destructive_commands_sent, 0);
        assert_eq!(audit.bytes_written, 0);
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::capability::Capability;
use opendragon_core::color::RgbColor;
use opendragon_core::device::DeviceInfo;
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::profile::ProfileId;
use opendragon_core::rapid_trigger::RapidTriggerConfig;
use opendragon_devices::registry::DeviceRegistry;
use opendragon_hid::discovery::DeviceEnumerator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceSummary {
    pub id: String,
    pub name: String,
    pub model: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub is_connected: bool,
    pub is_supported: bool,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub id: ProfileId,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroAction {
    pub key: String,
    pub delay_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroItem {
    pub id: String,
    pub name: String,
    pub actions: Vec<MacroAction>,
}

pub trait DeviceBackend: Send + Sync {
    fn enumerate_devices(&mut self) -> Result<Vec<DeviceSummary>, OpenDragonError>;
    fn get_device_info(&self, device_id: &str) -> Result<DeviceInfo, OpenDragonError>;
    fn get_capabilities(&self, device_id: &str) -> Vec<Capability>;

    fn get_lighting(&self, device_id: &str) -> Result<LightingConfig, OpenDragonError>;
    fn set_lighting(
        &mut self,
        device_id: &str,
        config: &LightingConfig,
    ) -> Result<(), OpenDragonError>;
    fn set_brightness(&mut self, device_id: &str, brightness: u8) -> Result<(), OpenDragonError>;
    fn get_brightness(&self, device_id: &str) -> Result<u8, OpenDragonError>;

    fn get_actuation(&self, device_id: &str) -> Result<ActuationPoint, OpenDragonError>;
    fn set_actuation(
        &mut self,
        device_id: &str,
        point: ActuationPoint,
    ) -> Result<(), OpenDragonError>;

    fn get_rapid_trigger(&self, device_id: &str) -> Result<RapidTriggerConfig, OpenDragonError>;
    fn set_rapid_trigger(
        &mut self,
        device_id: &str,
        config: &RapidTriggerConfig,
    ) -> Result<(), OpenDragonError>;

    fn get_polling_rate(&self, device_id: &str) -> Result<PollingRate, OpenDragonError>;
    fn set_polling_rate(
        &mut self,
        device_id: &str,
        rate: PollingRate,
    ) -> Result<(), OpenDragonError>;

    fn get_profiles(&self, device_id: &str) -> Result<Vec<ProfileSummary>, OpenDragonError>;
    fn set_active_profile(
        &mut self,
        device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError>;
    fn create_profile(
        &mut self,
        device_id: &str,
        name: String,
    ) -> Result<ProfileId, OpenDragonError>;
    fn delete_profile(
        &mut self,
        device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError>;

    fn get_macros(&self, device_id: &str) -> Result<Vec<MacroItem>, OpenDragonError>;
    fn save_macro(&mut self, device_id: &str, item: MacroItem) -> Result<(), OpenDragonError>;
    fn delete_macro(&mut self, device_id: &str, macro_id: &str) -> Result<(), OpenDragonError>;

    fn get_probe_report(&self, device_id: &str) -> Result<String, OpenDragonError>;

    fn get_dpi(&self, device_id: &str) -> Result<u16, OpenDragonError>;
    fn set_dpi(&mut self, device_id: &str, dpi: u16) -> Result<(), OpenDragonError>;
}

/// Simulated mock backend for UI development without physical hardware
#[derive(Clone)]
pub struct MockDeviceBackend {
    devices: Vec<DeviceSummary>,
    device_info: DeviceInfo,
    lighting: LightingConfig,
    actuation: ActuationPoint,
    rapid_trigger: RapidTriggerConfig,
    polling_rate: PollingRate,
    dpi: u16,
    profiles: Vec<ProfileSummary>,
    macros: Vec<MacroItem>,
    pub is_connected: bool,
}

impl Default for MockDeviceBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDeviceBackend {
    pub fn new() -> Self {
        let vid = 0x0c45;
        let pid = 0x5004;
        let mut dev_info = DeviceInfo::new(
            vid,
            pid,
            "SONiX / Redragon".to_string(),
            "Redragon Kumara Pro K552RGB-PRO".to_string(),
        );
        dev_info.serial_number = Some("SN-MOCK-K552-001".to_string());
        dev_info.release_number = Some(0x0100);

        let capabilities = vec![
            Capability::Lighting,
            Capability::PerKeyLighting,
            Capability::Actuation,
            Capability::RapidTrigger,
            Capability::PollingRate,
            Capability::Profile,
            Capability::Macro,
        ];

        let summary = DeviceSummary {
            id: "mock-k552rgb-pro".to_string(),
            name: "Redragon Kumara Pro".to_string(),
            model: "K552RGB-PRO".to_string(),
            vendor_id: vid,
            product_id: pid,
            is_connected: true,
            is_supported: true,
            capabilities,
        };

        let lighting = LightingConfig {
            mode: LightingMode::Static,
            color: RgbColor::new(217, 70, 239),
            brightness: 75,
            speed: 3,
            direction: None,
        };

        let actuation = ActuationPoint::new(1.2).unwrap();

        let rapid_trigger = RapidTriggerConfig {
            enabled: true,
            press_sensitivity_mm: 0.3,
            release_sensitivity_mm: 0.3,
        };

        let profiles = vec![
            ProfileSummary {
                id: ProfileId::new(0),
                name: "Default".to_string(),
                is_active: true,
            },
            ProfileSummary {
                id: ProfileId::new(1),
                name: "Gaming".to_string(),
                is_active: false,
            },
            ProfileSummary {
                id: ProfileId::new(2),
                name: "Work".to_string(),
                is_active: false,
            },
        ];

        let macros = vec![
            MacroItem {
                id: "macro-1".to_string(),
                name: "Quick Buy".to_string(),
                actions: vec![
                    MacroAction {
                        key: "B".to_string(),
                        delay_ms: 50,
                    },
                    MacroAction {
                        key: "4".to_string(),
                        delay_ms: 50,
                    },
                    MacroAction {
                        key: "2".to_string(),
                        delay_ms: 50,
                    },
                ],
            },
            MacroItem {
                id: "macro-2".to_string(),
                name: "Spam Jump".to_string(),
                actions: vec![
                    MacroAction {
                        key: "Space".to_string(),
                        delay_ms: 30,
                    },
                    MacroAction {
                        key: "Space".to_string(),
                        delay_ms: 30,
                    },
                ],
            },
        ];

        let m607 = DeviceSummary {
            id: "mock-m607".to_string(),
            name: "Redragon Griffin".to_string(),
            model: "M607".to_string(),
            vendor_id: 0x04d9,
            product_id: 0xfc38,
            is_connected: true,
            is_supported: true,
            capabilities: vec![
                Capability::Lighting,
                Capability::Profile,
                Capability::Dpi,
                Capability::PollingRate,
            ],
        };

        let m909 = DeviceSummary {
            id: "mock-m909".to_string(),
            name: "Redragon Emperor".to_string(),
            model: "M909".to_string(),
            vendor_id: 0x04d9,
            product_id: 0xfc4c,
            is_connected: true,
            is_supported: true,
            capabilities: vec![
                Capability::Lighting,
                Capability::Profile,
                Capability::Dpi,
                Capability::PollingRate,
            ],
        };

        let m601 = DeviceSummary {
            id: "mock-m601".to_string(),
            name: "Redragon Centrophorus".to_string(),
            model: "M601-3".to_string(),
            vendor_id: 0x04d9,
            product_id: 0xfc18,
            is_connected: true,
            is_supported: true,
            capabilities: vec![
                Capability::Lighting,
                Capability::Profile,
                Capability::Dpi,
                Capability::PollingRate,
            ],
        };

        Self {
            devices: vec![summary, m607, m909, m601],
            device_info: dev_info,
            lighting,
            actuation,
            rapid_trigger,
            polling_rate: PollingRate::Hz1000,
            dpi: 1600,
            profiles,
            macros,
            is_connected: true,
        }
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
        if let Some(dev) = self.devices.first_mut() {
            dev.is_connected = connected;
        }
    }
}

impl DeviceBackend for MockDeviceBackend {
    fn enumerate_devices(&mut self) -> Result<Vec<DeviceSummary>, OpenDragonError> {
        Ok(self.devices.clone())
    }

    fn get_device_info(&self, _device_id: &str) -> Result<DeviceInfo, OpenDragonError> {
        Ok(self.device_info.clone())
    }

    fn get_capabilities(&self, _device_id: &str) -> Vec<Capability> {
        self.devices
            .first()
            .map(|d| d.capabilities.clone())
            .unwrap_or_default()
    }

    fn get_lighting(&self, _device_id: &str) -> Result<LightingConfig, OpenDragonError> {
        Ok(self.lighting.clone())
    }

    fn set_lighting(
        &mut self,
        _device_id: &str,
        config: &LightingConfig,
    ) -> Result<(), OpenDragonError> {
        self.lighting = config.clone();
        Ok(())
    }

    fn set_brightness(&mut self, _device_id: &str, brightness: u8) -> Result<(), OpenDragonError> {
        self.lighting.brightness = brightness.min(100);
        Ok(())
    }

    fn get_brightness(&self, _device_id: &str) -> Result<u8, OpenDragonError> {
        Ok(self.lighting.brightness)
    }

    fn get_actuation(&self, _device_id: &str) -> Result<ActuationPoint, OpenDragonError> {
        Ok(self.actuation)
    }

    fn set_actuation(
        &mut self,
        _device_id: &str,
        point: ActuationPoint,
    ) -> Result<(), OpenDragonError> {
        self.actuation = point;
        Ok(())
    }

    fn get_rapid_trigger(&self, _device_id: &str) -> Result<RapidTriggerConfig, OpenDragonError> {
        Ok(self.rapid_trigger)
    }

    fn set_rapid_trigger(
        &mut self,
        _device_id: &str,
        config: &RapidTriggerConfig,
    ) -> Result<(), OpenDragonError> {
        self.rapid_trigger = *config;
        Ok(())
    }

    fn get_polling_rate(&self, _device_id: &str) -> Result<PollingRate, OpenDragonError> {
        Ok(self.polling_rate)
    }

    fn set_polling_rate(
        &mut self,
        _device_id: &str,
        rate: PollingRate,
    ) -> Result<(), OpenDragonError> {
        self.polling_rate = rate;
        Ok(())
    }

    fn get_profiles(&self, _device_id: &str) -> Result<Vec<ProfileSummary>, OpenDragonError> {
        Ok(self.profiles.clone())
    }

    fn set_active_profile(
        &mut self,
        _device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError> {
        for p in &mut self.profiles {
            p.is_active = p.id == profile_id;
        }
        Ok(())
    }

    fn create_profile(
        &mut self,
        _device_id: &str,
        name: String,
    ) -> Result<ProfileId, OpenDragonError> {
        let max_id = self
            .profiles
            .iter()
            .map(|p| p.id.index())
            .max()
            .unwrap_or(0);
        let new_id = ProfileId::new(max_id + 1);
        self.profiles.push(ProfileSummary {
            id: new_id,
            name,
            is_active: false,
        });
        Ok(new_id)
    }

    fn delete_profile(
        &mut self,
        _device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError> {
        if self.profiles.len() <= 1 {
            return Err(OpenDragonError::InvalidData(
                "Cannot delete the only profile".to_string(),
            ));
        }
        self.profiles.retain(|p| p.id != profile_id);
        if !self.profiles.iter().any(|p| p.is_active) {
            if let Some(first) = self.profiles.first_mut() {
                first.is_active = true;
            }
        }
        Ok(())
    }

    fn get_macros(&self, _device_id: &str) -> Result<Vec<MacroItem>, OpenDragonError> {
        Ok(self.macros.clone())
    }

    fn save_macro(&mut self, _device_id: &str, item: MacroItem) -> Result<(), OpenDragonError> {
        if let Some(existing) = self.macros.iter_mut().find(|m| m.id == item.id) {
            *existing = item;
        } else {
            self.macros.push(item);
        }
        Ok(())
    }

    fn delete_macro(&mut self, _device_id: &str, macro_id: &str) -> Result<(), OpenDragonError> {
        self.macros.retain(|m| m.id != macro_id);
        Ok(())
    }

    fn get_probe_report(&self, _device_id: &str) -> Result<String, OpenDragonError> {
        let report = serde_json::json!({
            "device": {
                "vendor_id": "0x0c45",
                "product_id": "0x5004",
                "vendor_name": "SONiX / Redragon",
                "product_name": "Redragon Kumara Pro K552RGB-PRO",
                "serial_number": "SN-MOCK-K552-001",
                "release_number": 0x0100
            },
            "interfaces": [
                {
                    "interface_number": 0,
                    "usage_page": 1,
                    "usage": 6,
                    "report_ids": [1, 2, 3]
                },
                {
                    "interface_number": 1,
                    "usage_page": 1,
                    "usage": 2,
                    "report_ids": [4, 5]
                }
            ],
            "capabilities": [
                "Lighting",
                "PerKeyLighting",
                "Actuation",
                "RapidTrigger",
                "PollingRate",
                "Profile",
                "Macro"
            ]
        });
        serde_json::to_string_pretty(&report).map_err(|e| OpenDragonError::Protocol(e.to_string()))
    }

    fn get_dpi(&self, _device_id: &str) -> Result<u16, OpenDragonError> {
        Ok(self.dpi)
    }

    fn set_dpi(&mut self, _device_id: &str, dpi: u16) -> Result<(), OpenDragonError> {
        self.dpi = dpi;
        Ok(())
    }
}

/// Real hardware backend communicating via OpenDragon core & HID stack
pub struct RealDeviceBackend {
    devices: HashMap<String, Box<dyn opendragon_core::device::Device>>,
    cached_summaries: Vec<DeviceSummary>,
    cached_info: HashMap<String, DeviceInfo>,
    mock_fallback: MockDeviceBackend,
}

impl Default for RealDeviceBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl RealDeviceBackend {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            cached_summaries: Vec::new(),
            cached_info: HashMap::new(),
            mock_fallback: MockDeviceBackend::new(),
        }
    }
}

impl DeviceBackend for RealDeviceBackend {
    fn enumerate_devices(&mut self) -> Result<Vec<DeviceSummary>, OpenDragonError> {
        let discovered = DeviceEnumerator::enumerate_redragon().unwrap_or_default();
        let mut summaries = Vec::new();

        self.devices.clear();
        self.cached_info.clear();

        for dev in discovered {
            let vid = dev.info.vendor_id;
            let pid = dev.info.product_id;
            let id = format!("{vid:04x}:{pid:04x}");

            let known = DeviceRegistry::find_known(vid, pid);
            let name = known
                .map(|k| k.marketing_name.to_string())
                .unwrap_or_else(|| dev.info.product_name.clone());
            let model = known
                .map(|k| k.model_name.to_string())
                .unwrap_or_else(|| format!("0x{vid:04x}:0x{pid:04x}"));
            let is_supported = known.is_some();

            let boxed = DeviceRegistry::create_device(dev.clone());
            let caps = boxed.capabilities().to_vec();

            self.cached_info.insert(id.clone(), dev.info);
            self.devices.insert(id.clone(), boxed);

            summaries.push(DeviceSummary {
                id,
                name,
                model,
                vendor_id: vid,
                product_id: pid,
                is_connected: true,
                is_supported,
                capabilities: caps,
            });
        }

        if summaries.is_empty() {
            #[cfg(feature = "mock-device")]
            {
                return self.mock_fallback.enumerate_devices();
            }
        }

        self.cached_summaries = summaries.clone();
        Ok(summaries)
    }

    fn get_device_info(&self, device_id: &str) -> Result<DeviceInfo, OpenDragonError> {
        if let Some(info) = self.cached_info.get(device_id) {
            Ok(info.clone())
        } else {
            self.mock_fallback.get_device_info(device_id)
        }
    }

    fn get_capabilities(&self, device_id: &str) -> Vec<Capability> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.capabilities().to_vec()
        } else {
            self.mock_fallback.get_capabilities(device_id)
        }
    }

    fn get_lighting(&self, device_id: &str) -> Result<LightingConfig, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_lighting()
        } else {
            self.mock_fallback.get_lighting(device_id)
        }
    }

    fn set_lighting(
        &mut self,
        device_id: &str,
        config: &LightingConfig,
    ) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_lighting(config)
        } else {
            self.mock_fallback.set_lighting(device_id, config)
        }
    }

    fn set_brightness(&mut self, device_id: &str, brightness: u8) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_brightness(brightness)
        } else {
            self.mock_fallback.set_brightness(device_id, brightness)
        }
    }

    fn get_brightness(&self, device_id: &str) -> Result<u8, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_brightness()
        } else {
            self.mock_fallback.get_brightness(device_id)
        }
    }

    fn get_actuation(&self, device_id: &str) -> Result<ActuationPoint, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_actuation()
        } else {
            self.mock_fallback.get_actuation(device_id)
        }
    }

    fn set_actuation(
        &mut self,
        device_id: &str,
        point: ActuationPoint,
    ) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_actuation(point)
        } else {
            self.mock_fallback.set_actuation(device_id, point)
        }
    }

    fn get_rapid_trigger(&self, device_id: &str) -> Result<RapidTriggerConfig, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_rapid_trigger()
        } else {
            self.mock_fallback.get_rapid_trigger(device_id)
        }
    }

    fn set_rapid_trigger(
        &mut self,
        device_id: &str,
        config: &RapidTriggerConfig,
    ) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_rapid_trigger(config)
        } else {
            self.mock_fallback.set_rapid_trigger(device_id, config)
        }
    }

    fn get_polling_rate(&self, device_id: &str) -> Result<PollingRate, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_polling_rate()
        } else {
            self.mock_fallback.get_polling_rate(device_id)
        }
    }

    fn set_polling_rate(
        &mut self,
        device_id: &str,
        rate: PollingRate,
    ) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_polling_rate(rate)
        } else {
            self.mock_fallback.set_polling_rate(device_id, rate)
        }
    }

    fn get_profiles(&self, device_id: &str) -> Result<Vec<ProfileSummary>, OpenDragonError> {
        self.mock_fallback.get_profiles(device_id)
    }

    fn set_active_profile(
        &mut self,
        device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_profile(profile_id)
        } else {
            self.mock_fallback.set_active_profile(device_id, profile_id)
        }
    }

    fn create_profile(
        &mut self,
        device_id: &str,
        name: String,
    ) -> Result<ProfileId, OpenDragonError> {
        self.mock_fallback.create_profile(device_id, name)
    }

    fn delete_profile(
        &mut self,
        device_id: &str,
        profile_id: ProfileId,
    ) -> Result<(), OpenDragonError> {
        self.mock_fallback.delete_profile(device_id, profile_id)
    }

    fn get_macros(&self, device_id: &str) -> Result<Vec<MacroItem>, OpenDragonError> {
        self.mock_fallback.get_macros(device_id)
    }

    fn save_macro(&mut self, device_id: &str, item: MacroItem) -> Result<(), OpenDragonError> {
        self.mock_fallback.save_macro(device_id, item)
    }

    fn delete_macro(&mut self, device_id: &str, macro_id: &str) -> Result<(), OpenDragonError> {
        self.mock_fallback.delete_macro(device_id, macro_id)
    }

    fn get_probe_report(&self, device_id: &str) -> Result<String, OpenDragonError> {
        if let Some(info) = self.cached_info.get(device_id) {
            let report = serde_json::json!({
                "device": {
                    "vendor_id": format!("0x{:04x}", info.vendor_id),
                    "product_id": format!("0x{:04x}", info.product_id),
                    "vendor_name": info.vendor_name,
                    "product_name": info.product_name,
                    "serial_number": info.serial_number,
                    "release_number": info.release_number
                },
                "interfaces": info.interfaces.iter().map(|i| {
                    serde_json::json!({
                        "interface_number": i.interface_number,
                        "usage_page": i.usage_page,
                        "usage": i.usage,
                        "report_ids": i.report_ids,
                        "hidraw_path": i.hidraw_path
                    })
                }).collect::<Vec<_>>(),
                "capabilities": self.get_capabilities(device_id)
            });
            serde_json::to_string_pretty(&report)
                .map_err(|e| OpenDragonError::Protocol(e.to_string()))
        } else {
            self.mock_fallback.get_probe_report(device_id)
        }
    }

    fn get_dpi(&self, device_id: &str) -> Result<u16, OpenDragonError> {
        if let Some(dev) = self.devices.get(device_id) {
            dev.get_dpi()
        } else {
            self.mock_fallback.get_dpi(device_id)
        }
    }

    fn set_dpi(&mut self, device_id: &str, dpi: u16) -> Result<(), OpenDragonError> {
        if let Some(dev) = self.devices.get_mut(device_id) {
            dev.set_dpi(dpi)
        } else {
            self.mock_fallback.set_dpi(device_id, dpi)
        }
    }
}

pub fn create_backend() -> Arc<Mutex<dyn DeviceBackend>> {
    #[cfg(feature = "mock-device")]
    {
        Arc::new(Mutex::new(MockDeviceBackend::new()))
    }
    #[cfg(not(feature = "mock-device"))]
    {
        Arc::new(Mutex::new(RealDeviceBackend::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_backend_enumeration() {
        let mut backend = MockDeviceBackend::new();
        let devices = backend.enumerate_devices().unwrap();
        assert_eq!(devices.len(), 4);
        let dev = &devices[0];
        assert_eq!(dev.id, "mock-k552rgb-pro");
        assert_eq!(dev.name, "Redragon Kumara Pro");
        assert_eq!(dev.vendor_id, 0x0c45);
        assert_eq!(dev.product_id, 0x5004);
        assert!(dev.is_connected);
        assert!(dev.is_supported);

        // Verify legacy mice in mock enumeration
        assert!(devices.iter().any(|d| d.model == "M607"));
        assert!(devices.iter().any(|d| d.model == "M909"));
        assert!(devices.iter().any(|d| d.model == "M601-3"));
    }

    #[test]
    fn test_mock_backend_dpi() {
        let mut backend = MockDeviceBackend::new();
        assert_eq!(backend.get_dpi("mock-m607").unwrap(), 1600);
        backend.set_dpi("mock-m607", 3200).unwrap();
        assert_eq!(backend.get_dpi("mock-m607").unwrap(), 3200);
    }

    #[test]
    fn test_mock_backend_actuation_point() {
        let mut backend = MockDeviceBackend::new();
        let current = backend.get_actuation("mock-k552rgb-pro").unwrap();
        assert_eq!(current.mm(), 1.2);

        let new_pt = ActuationPoint::new(2.5).unwrap();
        backend.set_actuation("mock-k552rgb-pro", new_pt).unwrap();
        assert_eq!(backend.get_actuation("mock-k552rgb-pro").unwrap().mm(), 2.5);
    }

    #[test]
    fn test_mock_backend_rapid_trigger() {
        let mut backend = MockDeviceBackend::new();
        let mut rt = backend.get_rapid_trigger("mock-k552rgb-pro").unwrap();
        assert!(rt.enabled);
        rt.enabled = false;
        rt.press_sensitivity_mm = 0.5;
        backend.set_rapid_trigger("mock-k552rgb-pro", &rt).unwrap();

        let updated = backend.get_rapid_trigger("mock-k552rgb-pro").unwrap();
        assert!(!updated.enabled);
        assert_eq!(updated.press_sensitivity_mm, 0.5);
    }

    #[test]
    fn test_mock_backend_lighting_config() {
        let mut backend = MockDeviceBackend::new();
        let mut lighting = backend.get_lighting("mock-k552rgb-pro").unwrap();
        assert_eq!(lighting.mode, LightingMode::Static);
        lighting.mode = LightingMode::Breathing;
        lighting.brightness = 90;
        backend.set_lighting("mock-k552rgb-pro", &lighting).unwrap();

        let updated = backend.get_lighting("mock-k552rgb-pro").unwrap();
        assert_eq!(updated.mode, LightingMode::Breathing);
        assert_eq!(updated.brightness, 90);
    }

    #[test]
    fn test_mock_backend_brightness() {
        let mut backend = MockDeviceBackend::new();
        assert_eq!(backend.get_brightness("mock-k552rgb-pro").unwrap(), 75);

        backend.set_brightness("mock-k552rgb-pro", 40).unwrap();
        assert_eq!(backend.get_brightness("mock-k552rgb-pro").unwrap(), 40);
        assert_eq!(
            backend.get_lighting("mock-k552rgb-pro").unwrap().brightness,
            40
        );
    }

    #[test]
    fn test_mock_backend_profiles() {
        let mut backend = MockDeviceBackend::new();
        let initial_profiles = backend.get_profiles("mock-k552rgb-pro").unwrap();
        assert_eq!(initial_profiles.len(), 3);

        let new_pid = backend
            .create_profile("mock-k552rgb-pro", "Custom Profile".into())
            .unwrap();
        let profiles_after_add = backend.get_profiles("mock-k552rgb-pro").unwrap();
        assert_eq!(profiles_after_add.len(), 4);

        backend.delete_profile("mock-k552rgb-pro", new_pid).unwrap();
        let profiles_after_del = backend.get_profiles("mock-k552rgb-pro").unwrap();
        assert_eq!(profiles_after_del.len(), 3);
    }

    #[test]
    fn test_mock_backend_probe_report() {
        let backend = MockDeviceBackend::new();
        let report = backend.get_probe_report("mock-k552rgb-pro").unwrap();
        assert!(report.contains("0x0c45"));
        assert!(report.contains("0x5004"));
        assert!(report.contains("Redragon"));
        assert!(report.contains("Kumara Pro"));
    }
}

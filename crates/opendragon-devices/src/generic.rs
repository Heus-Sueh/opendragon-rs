use opendragon_core::capability::{Capability, SupportStatus};
use opendragon_core::device::{Device, DeviceInfo};

pub struct GenericRedragonDevice {
    info: DeviceInfo,
    capabilities: Vec<Capability>,
}

impl GenericRedragonDevice {
    pub fn new(info: DeviceInfo) -> Self {
        let mut capabilities = Vec::new();
        // Dynamically detect capabilities from interface usages
        for iface in &info.interfaces {
            match iface.usage_page {
                Some(0x0001) => capabilities.push(Capability::PollingRate),
                Some(0x000C) => capabilities.push(Capability::Profile),
                Some(0xFF1C | 0xFFA0 | 0xFF00) => {
                    capabilities.push(Capability::Lighting);
                }
                _ => {}
            }
        }
        capabilities.dedup();

        Self { info, capabilities }
    }
}

impl Device for GenericRedragonDevice {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    fn support_status(&self, capability: Capability) -> SupportStatus {
        if self.capabilities.contains(&capability) {
            SupportStatus::Experimental
        } else {
            SupportStatus::Unsupported
        }
    }
}

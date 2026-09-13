use std::path::PathBuf;
use tracing::info;

use opendragon_core::capability::{Capability, SupportStatus};
use opendragon_core::device::{Device, DeviceInfo};
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::LightingConfig;
use opendragon_core::profile::ProfileId;
use opendragon_hid::transport::HidTransportHandle;
use opendragon_protocol::legacy_mouse::LegacyMousePacketBuilder;

pub const M607_CAPABILITIES: &[Capability] = &[
    Capability::Lighting,
    Capability::Profile,
    Capability::Dpi,
    Capability::PollingRate,
];

pub struct RedragonM607 {
    info: DeviceInfo,
    transport: Option<HidTransportHandle>,
}

impl RedragonM607 {
    pub const VENDOR_ID: u16 = 0x04d9;
    pub const PRODUCT_ID: u16 = 0xfc38;

    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            transport: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), OpenDragonError> {
        let config_path = self
            .info
            .interfaces
            .iter()
            .find(|i| i.interface_number == 2)
            .and_then(|i| i.hidraw_path.as_ref())
            .or_else(|| {
                self.info
                    .interfaces
                    .first()
                    .and_then(|i| i.hidraw_path.as_ref())
            });

        match config_path {
            Some(path) => {
                let transport = HidTransportHandle::open_path(path)?;
                self.transport = Some(transport);
                info!("Connected to Redragon M607 at {}", path);
                Ok(())
            }
            None => Err(OpenDragonError::PermissionDenied {
                path: PathBuf::from("/dev/hidrawX"),
            }),
        }
    }
}

impl Device for RedragonM607 {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn capabilities(&self) -> &[Capability] {
        M607_CAPABILITIES
    }

    fn support_status(&self, capability: Capability) -> SupportStatus {
        match capability {
            Capability::Lighting => SupportStatus::Supported,
            Capability::Profile => SupportStatus::Supported,
            Capability::Dpi => SupportStatus::Wip,
            Capability::PollingRate => SupportStatus::Partial,
            _ => SupportStatus::Unsupported,
        }
    }

    fn set_lighting(&mut self, config: &LightingConfig) -> Result<(), OpenDragonError> {
        if self.transport.is_none() {
            self.connect()?;
        }

        let packets = LegacyMousePacketBuilder::build_lighting_packets(Self::PRODUCT_ID, config)?;
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        for packet in packets {
            transport.send_feature_report(&packet)?;
        }
        info!("Updated Redragon M607 lighting configuration");
        Ok(())
    }

    fn set_profile(&mut self, profile: ProfileId) -> Result<(), OpenDragonError> {
        if self.transport.is_none() {
            self.connect()?;
        }

        let packets = LegacyMousePacketBuilder::build_profile_packets(profile);
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        for packet in packets {
            transport.send_feature_report(&packet)?;
        }
        info!("Updated Redragon M607 active profile to {}", profile);
        Ok(())
    }
}

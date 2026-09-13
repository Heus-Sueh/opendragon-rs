use std::path::PathBuf;
use tracing::info;

use opendragon_core::capability::{Capability, SupportStatus};
use opendragon_core::device::{Device, DeviceInfo};
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::LightingConfig;
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::profile::ProfileId;
use opendragon_hid::transport::HidTransportHandle;
use opendragon_protocol::legacy_mouse::LegacyMousePacketBuilder;

pub const M601_CAPABILITIES: &[Capability] = &[
    Capability::Lighting,
    Capability::Profile,
    Capability::Dpi,
    Capability::PollingRate,
];

pub struct RedragonM601 {
    info: DeviceInfo,
    transport: Option<HidTransportHandle>,
    current_dpi: u16,
    current_polling_rate: PollingRate,
}

impl RedragonM601 {
    pub const VENDOR_ID: u16 = 0x04d9;
    pub const PRODUCT_ID: u16 = 0xfc18;

    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            transport: None,
            current_dpi: 1600,
            current_polling_rate: PollingRate::Hz1000,
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
                info!("Connected to Redragon M601-3 Centrophorus at {}", path);
                Ok(())
            }
            None => Err(OpenDragonError::PermissionDenied {
                path: PathBuf::from("/dev/hidrawX"),
            }),
        }
    }
}

impl Device for RedragonM601 {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn capabilities(&self) -> &[Capability] {
        M601_CAPABILITIES
    }

    fn support_status(&self, capability: Capability) -> SupportStatus {
        match capability {
            Capability::Lighting => SupportStatus::Supported,
            Capability::Profile => SupportStatus::Supported,
            Capability::Dpi => SupportStatus::Supported,
            Capability::PollingRate => SupportStatus::Supported,
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
        info!("Updated Redragon M601 lighting configuration");
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
        info!("Updated Redragon M601 active profile to {}", profile);
        Ok(())
    }

    fn set_dpi(&mut self, dpi: u16) -> Result<(), OpenDragonError> {
        if self.transport.is_none() {
            self.connect()?;
        }

        let packets = LegacyMousePacketBuilder::build_dpi_packets(dpi);
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        for packet in packets {
            transport.send_feature_report(&packet)?;
        }
        self.current_dpi = dpi;
        info!("Updated Redragon M601 sensor DPI to {}", dpi);
        Ok(())
    }

    fn get_dpi(&self) -> Result<u16, OpenDragonError> {
        Ok(self.current_dpi)
    }

    fn set_polling_rate(&mut self, rate: PollingRate) -> Result<(), OpenDragonError> {
        if self.transport.is_none() {
            self.connect()?;
        }

        let packets = LegacyMousePacketBuilder::build_polling_rate_packets(rate)?;
        let transport = self
            .transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        for packet in packets {
            transport.send_feature_report(&packet)?;
        }
        self.current_polling_rate = rate;
        info!("Updated Redragon M601 polling rate to {}", rate);
        Ok(())
    }

    fn get_polling_rate(&self) -> Result<PollingRate, OpenDragonError> {
        Ok(self.current_polling_rate)
    }
}

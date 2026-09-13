use std::path::PathBuf;
use tracing::info;

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::capability::{Capability, SupportStatus};
use opendragon_core::device::{Device, DeviceInfo};
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::LightingConfig;
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::rapid_trigger::RapidTriggerConfig;
use opendragon_hid::transport::HidTransportHandle;
use opendragon_protocol::evision::EvisionPacketBuilder;

use super::capabilities::{get_support_status, K552RGB_PRO_CAPABILITIES};

pub struct RedragonKumaraProK552 {
    info: DeviceInfo,
    config_transport: Option<HidTransportHandle>,
}

impl RedragonKumaraProK552 {
    pub const VENDOR_ID: u16 = 0x0c45;
    pub const PRODUCT_ID: u16 = 0x5004;

    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            config_transport: None,
        }
    }

    /// Attaches the HID transport handle for the vendor configuration interface.
    pub fn attach_transport(&mut self, transport: HidTransportHandle) {
        self.config_transport = Some(transport);
    }

    /// Connects to the configuration interface if a valid path is present.
    pub fn connect(&mut self) -> Result<(), OpenDragonError> {
        // Look for the vendor configuration interface (typically interface 1 with report ID 4)
        let config_path = self
            .info
            .interfaces
            .iter()
            .find(|i| {
                i.report_ids
                    .contains(&EvisionPacketBuilder::REPORT_ID_CONFIG)
                    || i.interface_number == 1
            })
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
                self.config_transport = Some(transport);
                info!("Connected to K552RGB-PRO config interface at {}", path);
                Ok(())
            }
            None => Err(OpenDragonError::PermissionDenied {
                path: PathBuf::from("/dev/hidrawX"),
            }),
        }
    }
}

impl Device for RedragonKumaraProK552 {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn capabilities(&self) -> &[Capability] {
        K552RGB_PRO_CAPABILITIES
    }

    fn support_status(&self, capability: Capability) -> SupportStatus {
        get_support_status(capability)
    }

    fn set_lighting(&mut self, config: &LightingConfig) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_lighting_packet(config)?;
        let transport = self
            .config_transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        // Send output report on vendor configuration interface
        transport.write_output_report(&packet)?;
        info!("Updated K552RGB-PRO lighting configuration");
        Ok(())
    }

    fn set_actuation(&mut self, point: ActuationPoint) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_actuation_packet(point)?;
        let transport = self
            .config_transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        transport.write_output_report(&packet)?;
        info!("Updated K552RGB-PRO actuation point to {}", point);
        Ok(())
    }

    fn set_rapid_trigger(&mut self, config: &RapidTriggerConfig) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_rapid_trigger_packet(config)?;
        let transport = self
            .config_transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        transport.write_output_report(&packet)?;
        info!(
            "Updated K552RGB-PRO Rapid Trigger: enabled={}, press={:.2}mm, release={:.2}mm",
            config.enabled, config.press_sensitivity_mm, config.release_sensitivity_mm
        );
        Ok(())
    }

    fn set_polling_rate(&mut self, rate: PollingRate) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_polling_rate_packet(rate)?;
        let transport = self
            .config_transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        transport.write_output_report(&packet)?;
        info!("Updated K552RGB-PRO polling rate to {}", rate);
        Ok(())
    }
}

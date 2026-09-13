use std::path::PathBuf;
use tracing::info;

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::capability::{Capability, SupportStatus};
use opendragon_core::color::RgbColor;
use opendragon_core::device::{Device, DeviceInfo};
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::profile::ProfileId;
use opendragon_core::rapid_trigger::RapidTriggerConfig;
use opendragon_hid::transport::HidTransportHandle;
use opendragon_protocol::evision::EvisionPacketBuilder;

use super::capabilities::{get_support_status, K552RGB_PRO_CAPABILITIES};

pub struct RedragonKumaraProK552 {
    info: DeviceInfo,
    current_lighting: LightingConfig,
    current_polling_rate: PollingRate,
    current_profile: ProfileId,
    current_actuation: ActuationPoint,
    current_rapid_trigger: RapidTriggerConfig,
    config_transport: Option<HidTransportHandle>,
}

impl RedragonKumaraProK552 {
    pub const VENDOR_ID: u16 = 0x0c45;
    pub const PRODUCT_ID: u16 = 0x5004;

    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            current_lighting: LightingConfig {
                mode: LightingMode::Static,
                color: RgbColor::WHITE,
                brightness: 100,
                speed: 50,
                direction: None,
            },
            current_polling_rate: PollingRate::Hz1000,
            current_profile: ProfileId::new(0),
            current_actuation: ActuationPoint::new(2.0).unwrap(),
            current_rapid_trigger: RapidTriggerConfig::default(),
            config_transport: None,
        }
    }

    /// Attaches the HID transport handle for the vendor configuration interface.
    pub fn attach_transport(&mut self, transport: HidTransportHandle) {
        self.config_transport = Some(transport);
    }

    /// Connects to the configuration interface if a valid path is present.
    pub fn connect(&mut self) -> Result<(), OpenDragonError> {
        // Look for the vendor configuration interface (interface 1 with report ID 4)
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

    /// Sends a standard EVision framing sequence: Begin (0x01), Command Packet, End (0x02).
    fn send_command_sequence(
        &self,
        packet: &[u8; EvisionPacketBuilder::REPORT_LEN],
    ) -> Result<(), OpenDragonError> {
        let transport = self
            .config_transport
            .as_ref()
            .ok_or_else(|| OpenDragonError::HidTransport("No transport handle available".into()))?;

        // 1. Begin configure frame
        let begin_pkt = EvisionPacketBuilder::build_begin_packet();
        transport.write_output_report(&begin_pkt)?;

        // 2. Command payload packet
        transport.write_output_report(packet)?;

        // 3. End configure frame
        let end_pkt = EvisionPacketBuilder::build_end_packet();
        transport.write_output_report(&end_pkt)?;

        Ok(())
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

    fn get_lighting(&self) -> Result<LightingConfig, OpenDragonError> {
        Ok(self.current_lighting.clone())
    }

    fn set_lighting(&mut self, config: &LightingConfig) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let mode_pkt = EvisionPacketBuilder::build_lighting_packet(config)?;
        self.send_command_sequence(&mode_pkt)?;

        // Also update direct brightness register
        let bright_pkt = EvisionPacketBuilder::build_brightness_packet(config.brightness)?;
        self.send_command_sequence(&bright_pkt)?;

        // Also update direct speed register
        let speed_pkt = EvisionPacketBuilder::build_speed_packet(config.speed)?;
        self.send_command_sequence(&speed_pkt)?;

        self.current_lighting = config.clone();
        info!(
            "Updated K552RGB-PRO lighting configuration (mode={}, brightness={}%, speed={}%)",
            config.mode, config.brightness, config.speed
        );
        Ok(())
    }

    fn set_brightness(&mut self, brightness: u8) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        self.current_lighting.brightness = brightness.min(100);

        // 1. Send dedicated PARAM_BRIGHTNESS (0x01)
        let bright_pkt =
            EvisionPacketBuilder::build_brightness_packet(self.current_lighting.brightness)?;
        self.send_command_sequence(&bright_pkt)?;

        // 2. Send PARAM_MODE_EX (0x00) preserving active mode, speed, and color
        let mode_pkt = EvisionPacketBuilder::build_lighting_packet(&self.current_lighting)?;
        self.send_command_sequence(&mode_pkt)?;

        info!(
            "Updated K552RGB-PRO brightness to {}%",
            self.current_lighting.brightness
        );
        Ok(())
    }

    fn get_brightness(&self) -> Result<u8, OpenDragonError> {
        Ok(self.current_lighting.brightness)
    }

    fn set_actuation(&mut self, point: ActuationPoint) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_actuation_packet(point)?;
        self.send_command_sequence(&packet)?;
        self.current_actuation = point;
        info!("Updated K552RGB-PRO actuation point to {}", point);
        Ok(())
    }

    fn get_actuation(&self) -> Result<ActuationPoint, OpenDragonError> {
        Ok(self.current_actuation)
    }

    fn set_rapid_trigger(&mut self, config: &RapidTriggerConfig) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_rapid_trigger_packet(config)?;
        self.send_command_sequence(&packet)?;
        self.current_rapid_trigger = *config;
        info!(
            "Updated K552RGB-PRO Rapid Trigger: enabled={}, press={:.2}mm, release={:.2}mm",
            config.enabled, config.press_sensitivity_mm, config.release_sensitivity_mm
        );
        Ok(())
    }

    fn get_rapid_trigger(&self) -> Result<RapidTriggerConfig, OpenDragonError> {
        Ok(self.current_rapid_trigger)
    }

    fn set_polling_rate(&mut self, rate: PollingRate) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_polling_rate_packet(rate)?;
        self.send_command_sequence(&packet)?;
        self.current_polling_rate = rate;
        info!("Updated K552RGB-PRO polling rate to {}", rate);
        Ok(())
    }

    fn get_polling_rate(&self) -> Result<PollingRate, OpenDragonError> {
        Ok(self.current_polling_rate)
    }

    fn set_profile(&mut self, profile: ProfileId) -> Result<(), OpenDragonError> {
        if self.config_transport.is_none() {
            self.connect()?;
        }

        let packet = EvisionPacketBuilder::build_profile_packet(profile)?;
        self.send_command_sequence(&packet)?;
        self.current_profile = profile;
        info!("Updated K552RGB-PRO profile to {}", profile);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k552_lighting_and_brightness_defaults() {
        let info = DeviceInfo::new(
            RedragonKumaraProK552::VENDOR_ID,
            RedragonKumaraProK552::PRODUCT_ID,
            "SONiX",
            "K552RGB-PRO",
        );
        let mut dev = RedragonKumaraProK552::new(info);

        assert!(dev.capabilities().contains(&Capability::Lighting));
        assert!(dev.capabilities().contains(&Capability::PollingRate));
        assert!(dev.capabilities().contains(&Capability::Actuation));
        assert!(dev.capabilities().contains(&Capability::RapidTrigger));

        let lighting = dev.get_lighting().unwrap();
        assert_eq!(lighting.mode, LightingMode::Static);
        assert_eq!(lighting.color, RgbColor::WHITE);
        assert_eq!(lighting.brightness, 100);
        assert_eq!(dev.get_brightness().unwrap(), 100);
        assert_eq!(dev.get_polling_rate().unwrap(), PollingRate::Hz1000);

        // Update internal state directly for offline/mock test
        let mut new_cfg = lighting.clone();
        new_cfg.brightness = 60;
        new_cfg.mode = LightingMode::Breathing;
        new_cfg.color = RgbColor::new(255, 0, 128);
        dev.current_lighting = new_cfg;

        assert_eq!(dev.get_brightness().unwrap(), 60);
        assert_eq!(dev.get_lighting().unwrap().mode, LightingMode::Breathing);
    }
}

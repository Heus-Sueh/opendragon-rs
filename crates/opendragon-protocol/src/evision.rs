use opendragon_core::actuation::ActuationPoint;
use opendragon_core::color::RgbColor;
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::rapid_trigger::RapidTriggerConfig;

/// Protocol framing for EVision / Sonix Redragon keyboards (Kumara K552 / Mitra K551 / K552RGB-PRO family).
pub struct EvisionPacketBuilder;

impl EvisionPacketBuilder {
    pub const REPORT_ID_CONFIG: u8 = 0x04;
    pub const REPORT_LEN: usize = 65; // 1 byte Report ID + 64 bytes payload

    /// Builds a 64-byte configuration packet with the given command and payload.
    pub fn build_packet(
        cmd: u8,
        sub_cmd: u8,
        data: &[u8],
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        if data.len() > 60 {
            return Err(OpenDragonError::InvalidData(
                "Payload exceeds 60 bytes".into(),
            ));
        }

        let mut packet = [0u8; Self::REPORT_LEN];
        packet[0] = Self::REPORT_ID_CONFIG;
        packet[1] = cmd;
        packet[2] = sub_cmd;
        packet[3] = data.len() as u8;

        packet[4..4 + data.len()].copy_from_slice(data);

        // Calculate simple additive checksum over payload bytes
        let mut sum: u8 = 0;
        for &b in &packet[1..64] {
            sum = sum.wrapping_add(b);
        }
        packet[64] = sum;

        Ok(packet)
    }

    /// Builds an EVision command packet to set static RGB color.
    pub fn build_static_rgb_packet(
        color: RgbColor,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        // Command 0x28 (Lighting), Subcommand 0x01 (Static mode)
        let payload = [color.r, color.g, color.b];
        Self::build_packet(0x28, 0x01, &payload)
    }

    /// Builds an EVision command packet to set lighting effect and parameters.
    pub fn build_lighting_packet(
        config: &LightingConfig,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let mode_code = match config.mode {
            LightingMode::Static => 0x01,
            LightingMode::Breathing => 0x02,
            LightingMode::Rainbow => 0x03,
            LightingMode::Wave => 0x04,
            LightingMode::Reactive => 0x05,
            LightingMode::Ripple => 0x06,
            LightingMode::Flash => 0x07,
            LightingMode::Off => 0x00,
            _ => 0x01,
        };

        // Brightness: 0..4, Speed: 0..4
        let brightness_level = ((config.brightness as u16 * 4) / 100) as u8;
        let speed_level = ((config.speed as u16 * 4) / 100) as u8;

        let payload = [
            mode_code,
            brightness_level,
            speed_level,
            config.color.r,
            config.color.g,
            config.color.b,
        ];

        Self::build_packet(0x28, 0x02, &payload)
    }

    /// Builds an EVision actuation point configuration packet.
    pub fn build_actuation_packet(
        point: ActuationPoint,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        // Distance in 0.1mm units (e.g. 1.2mm = 12 units)
        let units = (point.mm() * 10.0).round() as u8;
        // Command 0x30 (Actuation), Subcommand 0x01
        Self::build_packet(0x30, 0x01, &[units])
    }

    /// Builds a Rapid Trigger configuration packet.
    pub fn build_rapid_trigger_packet(
        config: &RapidTriggerConfig,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let enabled_byte = if config.enabled { 0x01 } else { 0x00 };
        let press_units = (config.press_sensitivity_mm * 10.0).round() as u8;
        let release_units = (config.release_sensitivity_mm * 10.0).round() as u8;

        // Command 0x31 (Rapid Trigger), Subcommand 0x01
        Self::build_packet(0x31, 0x01, &[enabled_byte, press_units, release_units])
    }

    /// Builds a polling rate configuration packet.
    pub fn build_polling_rate_packet(
        rate: PollingRate,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let rate_code = match rate {
            PollingRate::Hz125 => 0x01,
            PollingRate::Hz250 => 0x02,
            PollingRate::Hz500 => 0x03,
            PollingRate::Hz1000 => 0x04,
            PollingRate::Hz2000 => 0x05,
            PollingRate::Hz4000 => 0x06,
            PollingRate::Hz8000 => 0x07,
        };

        // Command 0x22 (Polling Rate)
        Self::build_packet(0x22, 0x01, &[rate_code])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evision_static_rgb_packet() {
        let color = RgbColor::new(0xff, 0xaa, 0x55);
        let packet = EvisionPacketBuilder::build_static_rgb_packet(color).unwrap();

        assert_eq!(packet[0], 0x04); // Report ID
        assert_eq!(packet[1], 0x28); // Command
        assert_eq!(packet[2], 0x01); // Subcommand
        assert_eq!(packet[3], 3); // Payload length
        assert_eq!(packet[4], 0xff); // R
        assert_eq!(packet[5], 0xaa); // G
        assert_eq!(packet[6], 0x55); // B
    }

    #[test]
    fn test_evision_actuation_packet() {
        let point = ActuationPoint::new(2.5).unwrap();
        let packet = EvisionPacketBuilder::build_actuation_packet(point).unwrap();

        assert_eq!(packet[0], 0x04);
        assert_eq!(packet[1], 0x30);
        assert_eq!(packet[4], 25); // 2.5mm * 10
    }
}

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::color::RgbColor;
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{Direction, LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::profile::ProfileId;
use opendragon_core::rapid_trigger::RapidTriggerConfig;

/// Protocol framing for EVision / Sonix Redragon keyboards (Kumara K552 / Mitra K551 / K552RGB-PRO family).
///
/// Packets are 64 bytes total:
/// - Byte 0: Report ID (0x04)
/// - Byte 1..2: 16-bit little-endian additive checksum over bytes 3..63
/// - Byte 3: Command code (0x01: Begin, 0x02: End, 0x06: Set Parameter, 0x11: Write Custom Colors)
/// - Byte 4: Parameter payload length
/// - Byte 5: Parameter ID / offset
/// - Byte 6..7: Additional offset / zeroes
/// - Byte 8..: Parameter data bytes
pub struct EvisionPacketBuilder;

impl EvisionPacketBuilder {
    pub const REPORT_ID_CONFIG: u8 = 0x04;
    pub const REPORT_LEN: usize = 64; // 1 byte Report ID + 63 bytes payload matching report descriptor

    pub const CMD_BEGIN: u8 = 0x01;
    pub const CMD_END: u8 = 0x02;
    pub const CMD_READ_CAPABILITIES: u8 = 0x03;
    pub const CMD_READ_CONFIG: u8 = 0x05;
    pub const CMD_SET_PARAMETER: u8 = 0x06;
    pub const CMD_WRITE_CUSTOM_COLORS: u8 = 0x11;

    pub const PARAM_MODE_EX: u8 = 0x00;
    pub const PARAM_BRIGHTNESS: u8 = 0x01;
    pub const PARAM_SPEED: u8 = 0x02;
    pub const PARAM_DIRECTION: u8 = 0x03;
    pub const PARAM_RANDOM_COLOR_FLAG: u8 = 0x04;
    pub const PARAM_MODE_COLOR: u8 = 0x05;
    pub const PARAM_POLLING_RATE: u8 = 0x0F;

    pub const MODE_WAVE_SHORT: u8 = 0x01;
    pub const MODE_WAVE_LONG: u8 = 0x02;
    pub const MODE_WHEEL: u8 = 0x03;
    pub const MODE_SPECTRUM_CYCLE: u8 = 0x04;
    pub const MODE_BREATHING: u8 = 0x05;
    pub const MODE_STATIC: u8 = 0x06;
    pub const MODE_REACTIVE: u8 = 0x07;
    pub const MODE_RIPPLE: u8 = 0x08;
    pub const MODE_REACTIVE_LINE: u8 = 0x09;
    pub const MODE_STARLIGHT: u8 = 0x0A;
    pub const MODE_BLOOMING: u8 = 0x0B;
    pub const MODE_RAINBOW_WAVE_VERTICAL: u8 = 0x0C;
    pub const MODE_HURRICANE: u8 = 0x0D;
    pub const MODE_ACCUMULATE: u8 = 0x0E;
    pub const MODE_DIGITAL_TIMES: u8 = 0x0F;
    pub const MODE_VISOR: u8 = 0x10;
    pub const MODE_SURMOUNT: u8 = 0x11;
    pub const MODE_RAINBOW_CIRCLE: u8 = 0x12;
    pub const MODE_CUSTOM: u8 = 0x14;

    /// Computes the EVision 16-bit little-endian checksum over bytes 3..64 and inserts it at bytes 1..2.
    pub fn compute_checksum(packet: &mut [u8; Self::REPORT_LEN]) {
        let mut sum: u16 = 0;
        for &b in &packet[3..64] {
            sum = sum.wrapping_add(b as u16);
        }
        packet[1] = (sum & 0xFF) as u8;
        packet[2] = ((sum >> 8) & 0xFF) as u8;
    }

    /// Builds a "Begin Configure" framing packet (0x01).
    pub fn build_begin_packet() -> [u8; Self::REPORT_LEN] {
        let mut packet = [0u8; Self::REPORT_LEN];
        packet[0] = Self::REPORT_ID_CONFIG;
        packet[1] = Self::CMD_BEGIN;
        packet[2] = 0x00;
        packet[3] = Self::CMD_BEGIN;
        packet
    }

    /// Builds an "End Configure" framing packet (0x02).
    pub fn build_end_packet() -> [u8; Self::REPORT_LEN] {
        let mut packet = [0u8; Self::REPORT_LEN];
        packet[0] = Self::REPORT_ID_CONFIG;
        packet[1] = Self::CMD_END;
        packet[2] = 0x00;
        packet[3] = Self::CMD_END;
        packet
    }

    /// Builds an EVision command packet with an arbitrary command, parameter, and payload.
    pub fn build_packet(
        cmd: u8,
        param_or_offset: u8,
        data: &[u8],
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        if data.len() > 56 {
            return Err(OpenDragonError::InvalidData(
                "Payload exceeds 56 data bytes".into(),
            ));
        }

        let mut packet = [0u8; Self::REPORT_LEN];
        packet[0] = Self::REPORT_ID_CONFIG;
        packet[3] = cmd;
        packet[4] = data.len() as u8;
        packet[5] = param_or_offset;

        if !data.is_empty() {
            packet[8..8 + data.len()].copy_from_slice(data);
        }

        Self::compute_checksum(&mut packet);
        Ok(packet)
    }

    /// Builds a dedicated parameter update packet (CMD 0x06).
    pub fn build_parameter_packet(
        param: u8,
        data: &[u8],
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        Self::build_packet(Self::CMD_SET_PARAMETER, param, data)
    }

    /// Maps a 0-100 percentage brightness value to the hardware level (0..4).
    pub fn quantize_brightness(brightness_pct: u8) -> u8 {
        if brightness_pct == 0 {
            0
        } else {
            (((brightness_pct as f32 / 100.0) * 4.0).round() as u8).clamp(0, 4)
        }
    }

    /// Maps speed (either 1..5 level or 0..100 percentage) to EVision firmware delay code (0=Fastest .. 4=Slowest).
    pub fn quantize_speed(speed: u8) -> u8 {
        if speed == 0 {
            4 // Slowest
        } else if speed <= 5 {
            // Direct 1..5 scale (1: Slowest -> 4, 5: Fastest -> 0)
            5u8.saturating_sub(speed).min(4)
        } else {
            // 0..100 percentage
            let inverted = 100u8.saturating_sub(speed.min(100));
            (((inverted as f32 / 100.0) * 4.0).round() as u8).clamp(0, 4)
        }
    }

    /// Builds a dedicated Brightness parameter packet (Param 0x01).
    pub fn build_brightness_packet(
        brightness_pct: u8,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let level = Self::quantize_brightness(brightness_pct);
        Self::build_parameter_packet(Self::PARAM_BRIGHTNESS, &[level])
    }

    /// Builds a dedicated Speed parameter packet (Param 0x02).
    pub fn build_speed_packet(speed: u8) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let level = Self::quantize_speed(speed);
        Self::build_parameter_packet(Self::PARAM_SPEED, &[level])
    }

    /// Builds an EVision command packet to set static RGB color.
    pub fn build_static_rgb_packet(
        color: RgbColor,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        // ModeEx static with maximum brightness (level 4)
        let payload = [
            Self::MODE_STATIC,
            4, // Brightness 100%
            3, // Normal speed
            0, // Left direction
            0, // No random color
            color.r,
            color.g,
            color.b,
        ];
        Self::build_parameter_packet(Self::PARAM_MODE_EX, &payload)
    }

    /// Builds an EVision command packet to set full lighting effect and parameters.
    pub fn build_lighting_packet(
        config: &LightingConfig,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let mode_code = match config.mode {
            LightingMode::Static => Self::MODE_STATIC,
            LightingMode::Breathing => Self::MODE_BREATHING,
            LightingMode::Rainbow => Self::MODE_SPECTRUM_CYCLE,
            LightingMode::Wave => Self::MODE_WAVE_LONG,
            LightingMode::Reactive => Self::MODE_REACTIVE,
            LightingMode::Ripple => Self::MODE_RIPPLE,
            LightingMode::Flash => Self::MODE_STARLIGHT,
            LightingMode::GoWithoutTrace => Self::MODE_REACTIVE_LINE,
            LightingMode::Wheel => Self::MODE_WHEEL,
            LightingMode::Blooming => Self::MODE_BLOOMING,
            LightingMode::VerticalWave => Self::MODE_RAINBOW_WAVE_VERTICAL,
            LightingMode::Whirlpool => Self::MODE_HURRICANE,
            LightingMode::Accumulate => Self::MODE_ACCUMULATE,
            LightingMode::Rain => Self::MODE_DIGITAL_TIMES,
            LightingMode::Visor => Self::MODE_VISOR,
            LightingMode::Surmount => Self::MODE_SURMOUNT,
            LightingMode::RainbowCircle => Self::MODE_RAINBOW_CIRCLE,
            LightingMode::Custom => Self::MODE_CUSTOM,
            LightingMode::Off => Self::MODE_STATIC,
        };

        let brightness_level = if config.mode == LightingMode::Off {
            0
        } else {
            Self::quantize_brightness(config.brightness)
        };

        let speed_level = Self::quantize_speed(config.speed);

        let direction_code = match config.direction {
            Some(Direction::Right) => 1,
            _ => 0,
        };

        let payload = [
            mode_code,
            brightness_level,
            speed_level,
            direction_code,
            0, // Random color flag
            config.color.r,
            config.color.g,
            config.color.b,
        ];

        Self::build_parameter_packet(Self::PARAM_MODE_EX, &payload)
    }

    /// Builds a Polling Rate configuration packet (Param 0x0F).
    pub fn build_polling_rate_packet(
        rate: PollingRate,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let rate_code = match rate {
            PollingRate::Hz125 => 0x00,
            PollingRate::Hz250 => 0x01,
            PollingRate::Hz500 => 0x02,
            PollingRate::Hz1000 => 0x03,
            PollingRate::Hz2000 => 0x03, // Fallback to max supported by EVision
            PollingRate::Hz4000 => 0x03,
            PollingRate::Hz8000 => 0x03,
        };

        Self::build_parameter_packet(Self::PARAM_POLLING_RATE, &[rate_code])
    }

    /// Builds a Profile selection packet.
    pub fn build_profile_packet(
        profile: ProfileId,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let idx = profile.index().min(2);
        Self::build_packet(Self::CMD_SET_PARAMETER, 0x00, &[idx])
    }

    /// Builds an EVision actuation point configuration packet (for Hall Effect / magnetic variants).
    pub fn build_actuation_packet(
        point: ActuationPoint,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let units = (point.mm() * 10.0).round() as u8;
        Self::build_packet(0x30, 0x01, &[units])
    }

    /// Builds a Rapid Trigger configuration packet (for Hall Effect / magnetic variants).
    pub fn build_rapid_trigger_packet(
        config: &RapidTriggerConfig,
    ) -> Result<[u8; Self::REPORT_LEN], OpenDragonError> {
        let enabled_byte = if config.enabled { 0x01 } else { 0x00 };
        let press_units = (config.press_sensitivity_mm * 10.0).round() as u8;
        let release_units = (config.release_sensitivity_mm * 10.0).round() as u8;
        Self::build_packet(0x31, 0x01, &[enabled_byte, press_units, release_units])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evision_framing_and_checksum() {
        let mut packet = [0u8; 64];
        packet[0] = 0x04;
        packet[3] = 0x06;
        packet[4] = 8;
        packet[5] = 0x00;
        packet[8] = 0x06; // Static
        packet[9] = 0x04; // Brightness 4
        packet[10] = 0x03; // Speed 3
        packet[13] = 0x00; // R
        packet[14] = 0xFF; // G
        packet[15] = 0xC8; // B

        EvisionPacketBuilder::compute_checksum(&mut packet);

        let sum: u16 = 0x06 + 8 + 0x06 + 4 + 3 + 0xFF + 0xC8;
        assert_eq!(packet[1], (sum & 0xFF) as u8);
        assert_eq!(packet[2], ((sum >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_evision_begin_end_packets() {
        let begin = EvisionPacketBuilder::build_begin_packet();
        assert_eq!(begin[0], 0x04);
        assert_eq!(begin[1], 0x01);
        assert_eq!(begin[3], 0x01);

        let end = EvisionPacketBuilder::build_end_packet();
        assert_eq!(end[0], 0x04);
        assert_eq!(end[1], 0x02);
        assert_eq!(end[3], 0x02);
    }

    #[test]
    fn test_evision_static_rgb_packet() {
        let color = RgbColor::new(0x00, 0xff, 0xcc);
        let packet = EvisionPacketBuilder::build_static_rgb_packet(color).unwrap();

        assert_eq!(packet[0], 0x04); // Report ID
        assert_eq!(packet[3], EvisionPacketBuilder::CMD_SET_PARAMETER); // 0x06
        assert_eq!(packet[4], 8); // Size 8
        assert_eq!(packet[5], EvisionPacketBuilder::PARAM_MODE_EX); // 0x00
        assert_eq!(packet[8], EvisionPacketBuilder::MODE_STATIC); // 0x06
        assert_eq!(packet[9], 4); // 100% Brightness
        assert_eq!(packet[13], 0x00); // R
        assert_eq!(packet[14], 0xff); // G
        assert_eq!(packet[15], 0xcc); // B
    }

    #[test]
    fn test_evision_brightness_quantization() {
        assert_eq!(EvisionPacketBuilder::quantize_brightness(0), 0);
        assert_eq!(EvisionPacketBuilder::quantize_brightness(15), 1);
        assert_eq!(EvisionPacketBuilder::quantize_brightness(25), 1);
        assert_eq!(EvisionPacketBuilder::quantize_brightness(50), 2);
        assert_eq!(EvisionPacketBuilder::quantize_brightness(75), 3);
        assert_eq!(EvisionPacketBuilder::quantize_brightness(100), 4);

        let pkt = EvisionPacketBuilder::build_brightness_packet(50).unwrap();
        assert_eq!(pkt[0], 0x04);
        assert_eq!(pkt[3], 0x06);
        assert_eq!(pkt[4], 1);
        assert_eq!(pkt[5], 0x01); // Param brightness
        assert_eq!(pkt[8], 2); // Level 2
    }

    #[test]
    fn test_evision_speed_packet_and_quantization() {
        // Direct levels 1..5: 1 is slowest (delay 4), 5 is fastest (delay 0)
        assert_eq!(EvisionPacketBuilder::quantize_speed(1), 4);
        assert_eq!(EvisionPacketBuilder::quantize_speed(2), 3);
        assert_eq!(EvisionPacketBuilder::quantize_speed(3), 2);
        assert_eq!(EvisionPacketBuilder::quantize_speed(4), 1);
        assert_eq!(EvisionPacketBuilder::quantize_speed(5), 0);

        // Percentage: 0% slowest, 100% fastest
        assert_eq!(EvisionPacketBuilder::quantize_speed(0), 4);
        assert_eq!(EvisionPacketBuilder::quantize_speed(100), 0);

        let pkt = EvisionPacketBuilder::build_speed_packet(3).unwrap();
        assert_eq!(pkt[0], 0x04);
        assert_eq!(pkt[3], 0x06);
        assert_eq!(pkt[4], 1);
        assert_eq!(pkt[5], 0x02); // Param speed
        assert_eq!(pkt[8], 2); // Level 3 -> delay 2
    }

    #[test]
    fn test_evision_all_lighting_modes_packet() {
        let all_modes = [
            (LightingMode::Static, EvisionPacketBuilder::MODE_STATIC),
            (
                LightingMode::Breathing,
                EvisionPacketBuilder::MODE_BREATHING,
            ),
            (
                LightingMode::Rainbow,
                EvisionPacketBuilder::MODE_SPECTRUM_CYCLE,
            ),
            (LightingMode::Wave, EvisionPacketBuilder::MODE_WAVE_LONG),
            (LightingMode::Reactive, EvisionPacketBuilder::MODE_REACTIVE),
            (LightingMode::Ripple, EvisionPacketBuilder::MODE_RIPPLE),
            (LightingMode::Flash, EvisionPacketBuilder::MODE_STARLIGHT),
            (
                LightingMode::GoWithoutTrace,
                EvisionPacketBuilder::MODE_REACTIVE_LINE,
            ),
            (LightingMode::Wheel, EvisionPacketBuilder::MODE_WHEEL),
            (LightingMode::Blooming, EvisionPacketBuilder::MODE_BLOOMING),
            (
                LightingMode::VerticalWave,
                EvisionPacketBuilder::MODE_RAINBOW_WAVE_VERTICAL,
            ),
            (
                LightingMode::Whirlpool,
                EvisionPacketBuilder::MODE_HURRICANE,
            ),
            (
                LightingMode::Accumulate,
                EvisionPacketBuilder::MODE_ACCUMULATE,
            ),
            (LightingMode::Rain, EvisionPacketBuilder::MODE_DIGITAL_TIMES),
            (LightingMode::Visor, EvisionPacketBuilder::MODE_VISOR),
            (LightingMode::Surmount, EvisionPacketBuilder::MODE_SURMOUNT),
            (
                LightingMode::RainbowCircle,
                EvisionPacketBuilder::MODE_RAINBOW_CIRCLE,
            ),
            (LightingMode::Custom, EvisionPacketBuilder::MODE_CUSTOM),
        ];

        for (mode, expected_code) in all_modes {
            let cfg = LightingConfig {
                mode,
                color: RgbColor::WHITE,
                brightness: 100,
                speed: 3,
                direction: None,
            };
            let pkt = EvisionPacketBuilder::build_lighting_packet(&cfg).unwrap();
            assert_eq!(
                pkt[8], expected_code,
                "Mode {:?} mapped to wrong opcode",
                mode
            );
        }
    }

    #[test]
    fn test_evision_polling_rate_packet() {
        let p1000 = EvisionPacketBuilder::build_polling_rate_packet(PollingRate::Hz1000).unwrap();
        assert_eq!(p1000[0], 0x04);
        assert_eq!(p1000[3], 0x06);
        assert_eq!(p1000[5], 0x0F); // Param Polling Rate
        assert_eq!(p1000[8], 0x03); // 1000 Hz code

        let p500 = EvisionPacketBuilder::build_polling_rate_packet(PollingRate::Hz500).unwrap();
        assert_eq!(p500[8], 0x02);
    }
}

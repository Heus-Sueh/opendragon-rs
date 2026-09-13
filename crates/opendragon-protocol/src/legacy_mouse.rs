use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::profile::ProfileId;

/// Packet builder preserving the reverse-engineered protocol for Redragon M607 and M909 mice.
pub struct LegacyMousePacketBuilder;

impl LegacyMousePacketBuilder {
    pub const REPORT_ID: u8 = 0x02;
    pub const PACKET_LEN: usize = 16;

    /// Builds the 4-packet sequence (64 bytes total) to configure lighting on Redragon M607 / M909.
    pub fn build_lighting_packets(
        product_id: u16,
        config: &LightingConfig,
    ) -> Result<Vec<[u8; Self::PACKET_LEN]>, OpenDragonError> {
        let (lmode, umode) = Self::resolve_light_modes(product_id, config.mode)?;

        // Map brightness (0..100) to device range (0..3)
        let brightness_val = match config.brightness {
            0..=25 => 0x00,
            26..=50 => 0x01,
            51..=75 => 0x02,
            _ => 0x03,
        };

        // Map speed (0..100) to device range (0..8)
        let speed_val = ((config.speed as u16 * 8) / 100) as u8;

        let color = config.color;

        // Packet 1: Mode initialization
        let p1 = [
            Self::REPORT_ID,
            0xf3,
            0x46,
            0x01,
            0x02,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        // Packet 2: Color, effect, speed, mode
        let p2 = [
            Self::REPORT_ID,
            0xf3,
            0x49,
            0x01,
            0x06,
            0x00,
            0x00,
            0x00,
            color.r,
            color.g,
            color.b,
            lmode,
            speed_val,
            umode,
            0x00,
            0x00,
        ];

        // Packet 3: Brightness
        let p3 = [
            Self::REPORT_ID,
            0xf3,
            0x4f,
            0x01,
            0x01,
            0x00,
            0x00,
            0x00,
            brightness_val,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        // Packet 4: Commit / Apply
        let p4 = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        Ok(vec![p1, p2, p3, p4])
    }

    /// Builds profile switching packet sequence (select packet + 4 commit packets).
    pub fn build_profile_packets(profile: ProfileId) -> Vec<[u8; Self::PACKET_LEN]> {
        let p_select = [
            Self::REPORT_ID,
            0xf3,
            0x2c,
            0x00,
            0x02,
            0x00,
            0x00,
            0x00,
            profile.index(),
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let p_commit1 = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let p_commit2 = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x02,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let p_commit3 = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x04,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let p_commit4 = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x08,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        vec![p_select, p_commit1, p_commit2, p_commit3, p_commit4]
    }

    fn resolve_light_modes(
        product_id: u16,
        mode: LightingMode,
    ) -> Result<(u8, u8), OpenDragonError> {
        match product_id {
            0xfc38 => {
                // Redragon M607 Griffin
                let (lmode, umode) = match mode {
                    LightingMode::Breathing => (0x01, 0x04),
                    LightingMode::Rainbow => (0x01, 0x08),
                    LightingMode::Static => (0x01, 0x02),
                    LightingMode::Wave => (0x02, 0x00),
                    LightingMode::GoWithoutTrace => (0x06, 0x00),
                    LightingMode::Reactive => (0x07, 0x00),
                    LightingMode::Flash => (0x01, 0x10),
                    LightingMode::Off => (0x00, 0x00),
                    _ => (0x01, 0x02), // default to static
                };
                Ok((lmode, umode))
            }
            0xfc4c => {
                // Redragon M909 Emperor
                if matches!(mode, LightingMode::GoWithoutTrace | LightingMode::Reactive) {
                    return Err(OpenDragonError::Protocol(
                        "M909 Emperor does not support GoWithoutTrace or Reactive lighting modes"
                            .into(),
                    ));
                }
                let lmode = match mode {
                    LightingMode::Breathing => 0x02,
                    LightingMode::Rainbow => 0x03,
                    LightingMode::Static => 0x01,
                    LightingMode::Wave => 0x05,
                    LightingMode::Flash => 0x04,
                    LightingMode::Off => 0x00,
                    _ => 0x01,
                };
                Ok((lmode, 0x01))
            }
            _ => Err(OpenDragonError::UnsupportedDevice {
                vendor_id: 0x04d9,
                product_id,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opendragon_core::color::RgbColor;

    #[test]
    fn test_m607_wave_lighting_packets() {
        let config = LightingConfig {
            mode: LightingMode::Wave,
            color: RgbColor::new(0xff, 0x00, 0x00),
            brightness: 100, // 3
            speed: 100,      // 8
            direction: None,
        };

        let packets = LegacyMousePacketBuilder::build_lighting_packets(0xfc38, &config).unwrap();
        assert_eq!(packets.len(), 4);

        // Verify packet 2 contains: color=(255, 0, 0), lmode=0x02, speed=8, umode=0x00
        assert_eq!(
            packets[1],
            [
                0x02, 0xf3, 0x49, 0x01, 0x06, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x02, 0x08, 0x00,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn test_m909_profile_packets() {
        let packets = LegacyMousePacketBuilder::build_profile_packets(ProfileId::new(2));
        assert_eq!(packets.len(), 5);
        assert_eq!(packets[0][8], 0x02); // Profile index 2
        assert_eq!(packets[1][1], 0xf1); // Commit packet 1
    }
}

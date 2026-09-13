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

    /// Builds DPI configuration packet sequence for legacy Holtek mice (M607, M909, M601-3).
    /// Maps DPI to the hardware 50-step units (e.g. 1000 DPI -> 20 units).
    pub fn build_dpi_packets(dpi: u16) -> Vec<[u8; Self::PACKET_LEN]> {
        let units = (dpi / 50).max(1);
        let units_le = units.to_le_bytes();

        let p_dpi = [
            Self::REPORT_ID,
            0xf3,
            0x44, // Primary DPI stage
            0x00,
            0x05, // 5 bytes payload
            0x00,
            0x00,
            0x00,
            0x01, // active flag
            units_le[0],
            units_le[1],
            units_le[0],
            units_le[1],
            0x00,
            0x00,
            0x00,
        ];

        let p_commit = [
            Self::REPORT_ID,
            0xf1,
            0x02,
            0x10,
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

        vec![p_dpi, p_commit]
    }

    /// Builds USB polling rate configuration packet sequence for legacy Holtek mice.
    pub fn build_polling_rate_packets(
        rate: opendragon_core::polling_rate::PollingRate,
    ) -> Result<Vec<[u8; Self::PACKET_LEN]>, OpenDragonError> {
        use opendragon_core::polling_rate::PollingRate;

        let code: u8 = match rate {
            PollingRate::Hz125 => 0x08,
            PollingRate::Hz250 => 0x04,
            PollingRate::Hz500 => 0x02,
            PollingRate::Hz1000 => 0x01,
            _ => {
                return Err(OpenDragonError::Protocol(format!(
                    "Unsupported polling rate {rate} for legacy mouse"
                )))
            }
        };

        let p_rate = [
            Self::REPORT_ID,
            0xf3,
            0x38,
            0x00,
            0x04,
            0x00,
            0x00,
            0x00,
            code,
            0x00,
            code,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        let p_commit = [
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

        Ok(vec![p_rate, p_commit])
    }

    fn resolve_light_modes(
        product_id: u16,
        mode: LightingMode,
    ) -> Result<(u8, u8), OpenDragonError> {
        match product_id {
            0xfc38 | 0xfc18 => {
                // Redragon M607 Griffin / M601-3 Centrophorus
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

    #[test]
    fn test_legacy_mouse_dpi_packets() {
        let packets = LegacyMousePacketBuilder::build_dpi_packets(1000);
        assert_eq!(packets.len(), 2);
        // 1000 / 50 = 20 (0x14)
        assert_eq!(packets[0][0], 0x02);
        assert_eq!(packets[0][1], 0xf3);
        assert_eq!(packets[0][2], 0x44);
        assert_eq!(packets[0][8], 0x01); // active
        assert_eq!(packets[0][9], 20); // 1000 DPI / 50
        assert_eq!(packets[1][1], 0xf1); // commit
    }

    #[test]
    fn test_legacy_mouse_polling_rate_packets() {
        use opendragon_core::polling_rate::PollingRate;
        let packets =
            LegacyMousePacketBuilder::build_polling_rate_packets(PollingRate::Hz1000).unwrap();
        assert_eq!(packets.len(), 2);
        assert_eq!(packets[0][2], 0x38);
        assert_eq!(packets[0][8], 0x01); // 1000Hz code
    }

    #[test]
    fn test_m601_lighting_packets() {
        let config = LightingConfig {
            mode: LightingMode::Breathing,
            color: RgbColor::new(0x00, 0xff, 0x00),
            brightness: 100,
            speed: 50,
            direction: None,
        };
        let packets = LegacyMousePacketBuilder::build_lighting_packets(0xfc18, &config).unwrap();
        assert_eq!(packets.len(), 4);
        assert_eq!(packets[1][11], 0x01); // lmode
        assert_eq!(packets[1][13], 0x04); // umode
    }
}

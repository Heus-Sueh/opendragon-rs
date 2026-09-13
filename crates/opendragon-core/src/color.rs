use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::OpenDragonError;

/// 24-bit RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const YELLOW: Self = Self::new(255, 255, 0);
    pub const CYAN: Self = Self::new(0, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255);

    /// Formats color as `#RRGGBB` hex string.
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl FromStr for RgbColor {
    type Err = OpenDragonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim().trim_start_matches('#');
        if trimmed.len() != 6 {
            return Err(OpenDragonError::InvalidData(format!(
                "Invalid RGB hex string '{s}'. Expected 6 hex characters (e.g. #ff0000 or ff0000)."
            )));
        }

        let r = u8::from_str_radix(&trimmed[0..2], 16).map_err(|e| {
            OpenDragonError::InvalidData(format!("Invalid red channel in '{s}': {e}"))
        })?;
        let g = u8::from_str_radix(&trimmed[2..4], 16).map_err(|e| {
            OpenDragonError::InvalidData(format!("Invalid green channel in '{s}': {e}"))
        })?;
        let b = u8::from_str_radix(&trimmed[4..6], 16).map_err(|e| {
            OpenDragonError::InvalidData(format!("Invalid blue channel in '{s}': {e}"))
        })?;

        Ok(Self::new(r, g, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_hex() {
        let c1: RgbColor = "#ff8800".parse().unwrap();
        assert_eq!(c1, RgbColor::new(0xff, 0x88, 0x00));

        let c2: RgbColor = "00ff22".parse().unwrap();
        assert_eq!(c2, RgbColor::new(0x00, 0xff, 0x22));
    }

    #[test]
    fn format_hex() {
        assert_eq!(RgbColor::new(255, 0, 128).to_hex(), "#ff0080");
    }

    #[test]
    fn parse_invalid_hex() {
        assert!("#fff".parse::<RgbColor>().is_err());
        assert!("invalid".parse::<RgbColor>().is_err());
    }
}

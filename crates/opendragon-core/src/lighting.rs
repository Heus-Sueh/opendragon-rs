use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::color::RgbColor;
use crate::error::OpenDragonError;

/// Predefined or standard RGB lighting effects across Redragon devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightingMode {
    Static,
    Breathing,
    Rainbow,
    Wave,
    Reactive,
    Flash,
    GoWithoutTrace,
    Ripple,
    Wheel,
    Blooming,
    VerticalWave,
    Whirlpool,
    Accumulate,
    Rain,
    Visor,
    Surmount,
    RainbowCircle,
    Off,
    Custom,
}

impl fmt::Display for LightingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Breathing => write!(f, "breathing"),
            Self::Rainbow => write!(f, "rainbow"),
            Self::Wave => write!(f, "wave"),
            Self::Reactive => write!(f, "reactive"),
            Self::Flash => write!(f, "flash"),
            Self::GoWithoutTrace => write!(f, "go_without_trace"),
            Self::Ripple => write!(f, "ripple"),
            Self::Wheel => write!(f, "wheel"),
            Self::Blooming => write!(f, "blooming"),
            Self::VerticalWave => write!(f, "vertical_wave"),
            Self::Whirlpool => write!(f, "whirlpool"),
            Self::Accumulate => write!(f, "accumulate"),
            Self::Rain => write!(f, "rain"),
            Self::Visor => write!(f, "visor"),
            Self::Surmount => write!(f, "surmount"),
            Self::RainbowCircle => write!(f, "rainbow_circle"),
            Self::Off => write!(f, "off"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

impl FromStr for LightingMode {
    type Err = OpenDragonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "static" | "full_lighted" | "steady" => Ok(Self::Static),
            "breathing" | "breath" => Ok(Self::Breathing),
            "rainbow" => Ok(Self::Rainbow),
            "wave" => Ok(Self::Wave),
            "reactive" => Ok(Self::Reactive),
            "flash" | "starlight" | "stars" => Ok(Self::Flash),
            "go_without_trace" | "trace" | "reactive_line" => Ok(Self::GoWithoutTrace),
            "ripple" => Ok(Self::Ripple),
            "wheel" | "color_wheel" => Ok(Self::Wheel),
            "blooming" | "bloom" => Ok(Self::Blooming),
            "vertical_wave" | "wave_vertical" => Ok(Self::VerticalWave),
            "whirlpool" | "hurricane" => Ok(Self::Whirlpool),
            "accumulate" | "cascade" => Ok(Self::Accumulate),
            "rain" | "digital_times" | "matrix" => Ok(Self::Rain),
            "visor" | "scanner" => Ok(Self::Visor),
            "surmount" => Ok(Self::Surmount),
            "rainbow_circle" | "circle" => Ok(Self::RainbowCircle),
            "off" | "none" => Ok(Self::Off),
            "custom" | "per_key" => Ok(Self::Custom),
            _ => Err(OpenDragonError::InvalidData(format!(
                "Unknown lighting mode: '{s}'. Valid modes: static, breathing, rainbow, wave, reactive, flash, ripple, wheel, blooming, vertical_wave, whirlpool, accumulate, rain, visor, surmount, rainbow_circle, off, custom"
            ))),
        }
    }
}

/// Lighting effect direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// Full configuration for device RGB lighting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightingConfig {
    pub mode: LightingMode,
    pub color: RgbColor,
    pub brightness: u8,
    pub speed: u8,
    pub direction: Option<Direction>,
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            mode: LightingMode::Rainbow,
            color: RgbColor::WHITE,
            brightness: 100,
            speed: 50,
            direction: Some(Direction::Right),
        }
    }
}

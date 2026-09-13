use serde::{Deserialize, Serialize};

use crate::error::OpenDragonError;

/// Configuration for Rapid Trigger (dynamic switch reset).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RapidTriggerConfig {
    pub enabled: bool,
    pub press_sensitivity_mm: f32,
    pub release_sensitivity_mm: f32,
}

impl RapidTriggerConfig {
    pub const MIN_SENSITIVITY_MM: f32 = 0.05;
    pub const MAX_SENSITIVITY_MM: f32 = 2.0;

    pub fn new(
        enabled: bool,
        press_sensitivity_mm: f32,
        release_sensitivity_mm: f32,
    ) -> Result<Self, OpenDragonError> {
        if !(Self::MIN_SENSITIVITY_MM..=Self::MAX_SENSITIVITY_MM).contains(&press_sensitivity_mm) {
            return Err(OpenDragonError::InvalidData(format!(
                "Press sensitivity must be between {:.2}mm and {:.2}mm",
                Self::MIN_SENSITIVITY_MM,
                Self::MAX_SENSITIVITY_MM
            )));
        }
        if !(Self::MIN_SENSITIVITY_MM..=Self::MAX_SENSITIVITY_MM).contains(&release_sensitivity_mm)
        {
            return Err(OpenDragonError::InvalidData(format!(
                "Release sensitivity must be between {:.2}mm and {:.2}mm",
                Self::MIN_SENSITIVITY_MM,
                Self::MAX_SENSITIVITY_MM
            )));
        }

        Ok(Self {
            enabled,
            press_sensitivity_mm,
            release_sensitivity_mm,
        })
    }
}

impl Default for RapidTriggerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            press_sensitivity_mm: 0.2,
            release_sensitivity_mm: 0.2,
        }
    }
}

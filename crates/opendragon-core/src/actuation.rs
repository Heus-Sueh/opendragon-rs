use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::OpenDragonError;

/// Switch actuation point setting in millimeters.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ActuationPoint(f32);

impl ActuationPoint {
    pub const MIN_MM: f32 = 0.1;
    pub const MAX_MM: f32 = 4.0;

    pub fn new(distance_mm: f32) -> Result<Self, OpenDragonError> {
        if !(Self::MIN_MM..=Self::MAX_MM).contains(&distance_mm) {
            return Err(OpenDragonError::InvalidData(format!(
                "Actuation point must be between {:.1}mm and {:.1}mm (got {:.2}mm)",
                Self::MIN_MM,
                Self::MAX_MM,
                distance_mm
            )));
        }
        Ok(Self(distance_mm))
    }

    pub fn mm(&self) -> f32 {
        self.0
    }
}

impl fmt::Display for ActuationPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} mm", self.0)
    }
}

impl FromStr for ActuationPoint {
    type Err = OpenDragonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = s.trim().trim_end_matches("mm").trim();
        let val = clean.parse::<f32>().map_err(|e| {
            OpenDragonError::InvalidData(format!("Invalid actuation value '{s}': {e}"))
        })?;
        Self::new(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_actuation() {
        let a = ActuationPoint::new(1.2).unwrap();
        assert_eq!(a.mm(), 1.2);
    }

    #[test]
    fn invalid_bounds() {
        assert!(ActuationPoint::new(0.0).is_err());
        assert!(ActuationPoint::new(4.5).is_err());
    }
}

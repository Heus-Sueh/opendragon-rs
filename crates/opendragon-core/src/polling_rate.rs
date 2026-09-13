use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::OpenDragonError;

/// Supported USB polling rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PollingRate {
    Hz125 = 125,
    Hz250 = 250,
    Hz500 = 500,
    Hz1000 = 1000,
    Hz2000 = 2000,
    Hz4000 = 4000,
    Hz8000 = 8000,
}

impl PollingRate {
    pub const fn hz(&self) -> u32 {
        *self as u32
    }
}

impl fmt::Display for PollingRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Hz", self.hz())
    }
}

impl FromStr for PollingRate {
    type Err = OpenDragonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = s.trim().trim_end_matches("hz").trim_end_matches("Hz");
        match clean {
            "125" => Ok(Self::Hz125),
            "250" => Ok(Self::Hz250),
            "500" => Ok(Self::Hz500),
            "1000" => Ok(Self::Hz1000),
            "2000" => Ok(Self::Hz2000),
            "4000" => Ok(Self::Hz4000),
            "8000" => Ok(Self::Hz8000),
            _ => Err(OpenDragonError::InvalidData(format!(
                "Invalid polling rate '{s}'. Supported rates: 125, 250, 500, 1000, 2000, 4000, 8000 Hz"
            ))),
        }
    }
}

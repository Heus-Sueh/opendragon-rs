use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::OpenDragonError;

/// Identifier for on-board hardware profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProfileId(u8);

impl ProfileId {
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    pub const fn index(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for ProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Profile {}", self.0 + 1)
    }
}

impl FromStr for ProfileId {
    type Err = OpenDragonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = s
            .trim()
            .trim_start_matches("profile")
            .trim_start_matches("Profile")
            .trim();
        let num: u8 = clean.parse().map_err(|e| {
            OpenDragonError::InvalidData(format!("Invalid profile identifier '{s}': {e}"))
        })?;
        // Convert 1-based human input to 0-based index if > 0
        let idx = if num > 0 { num - 1 } else { 0 };
        Ok(Self(idx))
    }
}

use serde::{Deserialize, Serialize};

use opendragon_core::error::OpenDragonError;

/// Command safety classification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSafetyLevel {
    /// Safe, read-only inspection (descriptors, report reading, query states)
    ReadOnly,
    /// Verified, non-destructive configuration changes (static RGB, brightness)
    KnownSafe,
    /// Partially understood or newly mapped features (actuation adjustment, polling rate)
    Experimental,
    /// High-risk commands (firmware writes, bootloader entry, factory resets, flash/EEPROM writes)
    Dangerous,
}

/// Epistemological understanding state of a packet or command sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingState {
    /// Observed in official software captures but not yet verified
    Observed,
    /// Successfully reproduced on device with observable effect
    Reproduced,
    /// Partially understood (some fields known, others uncertain)
    PartiallyUnderstood,
    /// Fully understood (offsets, units, framing, side-effects verified)
    Understood,
    /// Completely unknown sequence
    Unknown,
}

/// Detailed safety classification and risk metadata for a hardware command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSafetyClassification {
    pub level: CommandSafetyLevel,
    pub state: UnderstandingState,
    pub description: String,
    pub allows_fuzzing: bool,
}

impl CommandSafetyClassification {
    pub fn read_only(description: impl Into<String>) -> Self {
        Self {
            level: CommandSafetyLevel::ReadOnly,
            state: UnderstandingState::Understood,
            description: description.into(),
            allows_fuzzing: false,
        }
    }

    pub fn known_safe(description: impl Into<String>) -> Self {
        Self {
            level: CommandSafetyLevel::KnownSafe,
            state: UnderstandingState::Understood,
            description: description.into(),
            allows_fuzzing: false,
        }
    }

    pub fn experimental(description: impl Into<String>) -> Self {
        Self {
            level: CommandSafetyLevel::Experimental,
            state: UnderstandingState::PartiallyUnderstood,
            description: description.into(),
            allows_fuzzing: false,
        }
    }

    pub fn dangerous(description: impl Into<String>) -> Self {
        Self {
            level: CommandSafetyLevel::Dangerous,
            state: UnderstandingState::Unknown,
            description: description.into(),
            allows_fuzzing: false,
        }
    }

    /// Validates execution against safety gate. Rejects dangerous or unconfirmed commands
    /// unless explicitly overridden by `i_know_what_im_doing`.
    pub fn assert_can_execute(&self, i_know_what_im_doing: bool) -> Result<(), OpenDragonError> {
        if self.level == CommandSafetyLevel::Dangerous && !i_know_what_im_doing {
            return Err(OpenDragonError::HardwareSafetyViolation(format!(
                "Command '{}' is classified as Dangerous ({:?}). Potential risk to hardware/firmware.",
                self.description, self.state
            )));
        }
        Ok(())
    }
}

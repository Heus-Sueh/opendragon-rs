use serde::{Deserialize, Serialize};

/// Hardware capabilities that a Redragon peripheral may declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// RGB lighting effects (static, breathing, wave, etc.)
    Lighting,
    /// Per-key individually addressable RGB LEDs
    PerKeyLighting,
    /// On-board macros and key remapping
    Macro,
    /// Adjustable USB polling rate (e.g., 125Hz to 8000Hz)
    PollingRate,
    /// Adjustable switch actuation point (e.g. Hall Effect switches)
    Actuation,
    /// Dynamic key release reset without full return (Rapid Trigger)
    RapidTrigger,
    /// On-board hardware profiles
    Profile,
    /// Mouse sensor DPI settings
    Dpi,
    /// Battery status reporting for wireless devices
    Battery,
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lighting => write!(f, "Lighting"),
            Self::PerKeyLighting => write!(f, "Per-key Lighting"),
            Self::Macro => write!(f, "Macro"),
            Self::PollingRate => write!(f, "Polling Rate"),
            Self::Actuation => write!(f, "Actuation Point"),
            Self::RapidTrigger => write!(f, "Rapid Trigger"),
            Self::Profile => write!(f, "Hardware Profiles"),
            Self::Dpi => write!(f, "DPI Stages"),
            Self::Battery => write!(f, "Battery Reporting"),
        }
    }
}

/// Verification and support status for a device capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportStatus {
    /// Verified working on physical hardware with automated or manual testing
    Supported,
    /// Partially working or subset of features confirmed
    Partial,
    /// Implementation exists but not fully validated on hardware
    Experimental,
    /// Currently being researched or actively developed
    Wip,
    /// Hardware capability is unknown or not yet investigated
    Unknown,
    /// Hardware does not have this capability
    Unsupported,
}

impl std::fmt::Display for SupportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Supported => write!(f, "Supported"),
            Self::Partial => write!(f, "Partial"),
            Self::Experimental => write!(f, "Experimental"),
            Self::Wip => write!(f, "WIP"),
            Self::Unknown => write!(f, "Unknown"),
            Self::Unsupported => write!(f, "Unsupported"),
        }
    }
}

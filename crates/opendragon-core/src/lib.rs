pub mod actuation;
pub mod capability;
pub mod color;
pub mod device;
pub mod error;
pub mod lighting;
pub mod polling_rate;
pub mod profile;
pub mod rapid_trigger;

// Re-exports for convenience
pub use actuation::ActuationPoint;
pub use capability::{Capability, SupportStatus};
pub use color::RgbColor;
pub use device::{Device, DeviceInfo, EndpointDirection, EndpointInfo, InterfaceInfo};
pub use error::OpenDragonError;
pub use lighting::{Direction, LightingConfig, LightingMode};
pub use polling_rate::PollingRate;
pub use profile::ProfileId;
pub use rapid_trigger::RapidTriggerConfig;

use std::path::PathBuf;
use thiserror::Error;

/// Typed errors for the OpenDragon ecosystem.
#[derive(Debug, Error)]
pub enum OpenDragonError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Device matched but configuration interface {interface} was not found")]
    InterfaceNotFound { interface: u8 },

    #[error("Permission denied accessing HID device at {path}. Run with appropriate permissions or configure udev rules.")]
    PermissionDenied { path: PathBuf },

    #[error("Unsupported device (VID: {vendor_id:#06x}, PID: {product_id:#06x})")]
    UnsupportedDevice { vendor_id: u16, product_id: u16 },

    #[error("Device does not support capability: {0:?}")]
    UnsupportedCapability(crate::capability::Capability),

    #[error("HID transport error: {0}")]
    HidTransport(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Hardware safety violation: {0}. Requires --i-know-what-im-doing flag to execute.")]
    HardwareSafetyViolation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

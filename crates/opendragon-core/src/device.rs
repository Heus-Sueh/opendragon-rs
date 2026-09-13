use serde::{Deserialize, Serialize};

use crate::actuation::ActuationPoint;
use crate::capability::{Capability, SupportStatus};
use crate::error::OpenDragonError;
use crate::lighting::LightingConfig;
use crate::polling_rate::PollingRate;
use crate::profile::ProfileId;
use crate::rapid_trigger::RapidTriggerConfig;

/// Endpoint transfer direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointDirection {
    In,
    Out,
}

/// Information about a USB endpoint on a device interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub address: u8,
    pub direction: EndpointDirection,
    pub max_packet_size: u16,
    pub interval_ms: u8,
}

/// Information about an individual HID/USB interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceInfo {
    pub interface_number: u8,
    pub usage_page: Option<u16>,
    pub usage: Option<u16>,
    pub report_ids: Vec<u8>,
    pub endpoints: Vec<EndpointInfo>,
    pub hidraw_path: Option<String>,
}

/// High-level static and discovered device information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub vendor_name: String,
    pub product_name: String,
    pub serial_number: Option<String>,
    pub release_number: Option<u16>,
    pub bus_type: String,
    pub interfaces: Vec<InterfaceInfo>,
}

impl DeviceInfo {
    pub fn new(
        vendor_id: u16,
        product_id: u16,
        vendor_name: impl Into<String>,
        product_name: impl Into<String>,
    ) -> Self {
        Self {
            vendor_id,
            product_id,
            vendor_name: vendor_name.into(),
            product_name: product_name.into(),
            serial_number: None,
            release_number: None,
            bus_type: "USB".to_string(),
            interfaces: Vec::new(),
        }
    }
}

/// Unified trait implemented by all supported Redragon peripheral drivers.
pub trait Device: Send + Sync {
    /// Returns basic hardware and descriptor information for the device.
    fn info(&self) -> &DeviceInfo;

    /// Declares the capabilities supported by this device.
    fn capabilities(&self) -> &[Capability];

    /// Reports the support/verification status for a specific capability.
    fn support_status(&self, capability: Capability) -> SupportStatus {
        if self.capabilities().contains(&capability) {
            SupportStatus::Supported
        } else {
            SupportStatus::Unsupported
        }
    }

    /// Sets device lighting configuration.
    fn set_lighting(&mut self, _config: &LightingConfig) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(Capability::Lighting))
    }

    /// Retrieves current lighting configuration if supported by protocol.
    fn get_lighting(&self) -> Result<LightingConfig, OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(Capability::Lighting))
    }

    /// Sets device brightness (0-100), preserving the current lighting mode and color if supported.
    fn set_brightness(&mut self, brightness: u8) -> Result<(), OpenDragonError> {
        let mut config = self.get_lighting().unwrap_or_default();
        config.brightness = brightness.min(100);
        self.set_lighting(&config)
    }

    /// Retrieves current device brightness level (0-100) if readable.
    fn get_brightness(&self) -> Result<u8, OpenDragonError> {
        self.get_lighting().map(|c| c.brightness)
    }

    /// Sets active hardware profile.
    fn set_profile(&mut self, _profile: ProfileId) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(Capability::Profile))
    }

    /// Sets USB polling rate.
    fn set_polling_rate(&mut self, _rate: PollingRate) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::PollingRate,
        ))
    }

    /// Retrieves current USB polling rate if readable.
    fn get_polling_rate(&self) -> Result<PollingRate, OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::PollingRate,
        ))
    }

    /// Sets switch actuation point in millimeters (magnetic / Hall Effect).
    fn set_actuation(&mut self, _point: ActuationPoint) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::Actuation,
        ))
    }

    /// Gets current switch actuation point if readable.
    fn get_actuation(&self) -> Result<ActuationPoint, OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::Actuation,
        ))
    }

    /// Configures Rapid Trigger dynamic reset.
    fn set_rapid_trigger(&mut self, _config: &RapidTriggerConfig) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::RapidTrigger,
        ))
    }

    /// Gets current Rapid Trigger configuration if readable.
    fn get_rapid_trigger(&self) -> Result<RapidTriggerConfig, OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(
            Capability::RapidTrigger,
        ))
    }

    /// Sets optical/laser sensor DPI sensitivity.
    fn set_dpi(&mut self, _dpi: u16) -> Result<(), OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(Capability::Dpi))
    }

    /// Retrieves current optical/laser sensor DPI sensitivity if readable.
    fn get_dpi(&self) -> Result<u16, OpenDragonError> {
        Err(OpenDragonError::UnsupportedCapability(Capability::Dpi))
    }
}

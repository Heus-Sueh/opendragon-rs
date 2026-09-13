use opendragon_core::error::OpenDragonError;
use std::path::PathBuf;
use std::sync::Mutex;

/// Safe, thread-synchronized wrapper around an open HID device transport handle.
pub struct HidTransportHandle {
    device: Mutex<hidapi::HidDevice>,
    path: PathBuf,
}

impl HidTransportHandle {
    /// Opens a device by its path (e.g., `/dev/hidrawX` on Linux).
    pub fn open_path(path: &str) -> Result<Self, OpenDragonError> {
        let api = hidapi::HidApi::new().map_err(|e| {
            OpenDragonError::HidTransport(format!("Failed to initialize HIDAPI: {e}"))
        })?;

        let c_path = std::ffi::CString::new(path).map_err(|_| {
            OpenDragonError::InvalidData(format!("Path contains null bytes: {path}"))
        })?;

        let device = api.open_path(&c_path).map_err(|e| {
            let pb = PathBuf::from(path);
            let msg = e.to_string();
            if msg.to_lowercase().contains("permission denied")
                || msg.to_lowercase().contains("access denied")
            {
                OpenDragonError::PermissionDenied { path: pb }
            } else {
                OpenDragonError::HidTransport(format!("Failed to open HID device at {path}: {msg}"))
            }
        })?;

        Ok(Self {
            device: Mutex::new(device),
            path: PathBuf::from(path),
        })
    }

    /// Sends a HID Feature Report to the device.
    /// In HIDAPI, the first byte of `data` is the Report ID (or 0x00 if no Report ID).
    pub fn send_feature_report(&self, data: &[u8]) -> Result<(), OpenDragonError> {
        let dev = self
            .device
            .lock()
            .map_err(|_| OpenDragonError::HidTransport("Failed to acquire device lock".into()))?;
        dev.send_feature_report(data).map_err(|e| {
            OpenDragonError::HidTransport(format!(
                "Failed to send feature report to {:?}: {e}",
                self.path
            ))
        })?;
        Ok(())
    }

    /// Reads a HID Feature Report from the device.
    /// `buffer[0]` should contain the desired Report ID.
    pub fn get_feature_report(
        &self,
        report_id: u8,
        buffer: &mut [u8],
    ) -> Result<usize, OpenDragonError> {
        if buffer.is_empty() {
            return Err(OpenDragonError::InvalidData("Buffer is empty".into()));
        }
        buffer[0] = report_id;
        let dev = self
            .device
            .lock()
            .map_err(|_| OpenDragonError::HidTransport("Failed to acquire device lock".into()))?;
        let bytes_read = dev.get_feature_report(buffer).map_err(|e| {
            OpenDragonError::HidTransport(format!(
                "Failed to get feature report (id: {report_id}) from {:?}: {e}",
                self.path
            ))
        })?;
        Ok(bytes_read)
    }

    /// Sends an Output Report to the device (typically via interrupt endpoint or Set_Report).
    pub fn write_output_report(&self, data: &[u8]) -> Result<(), OpenDragonError> {
        let dev = self
            .device
            .lock()
            .map_err(|_| OpenDragonError::HidTransport("Failed to acquire device lock".into()))?;
        dev.write(data).map_err(|e| {
            OpenDragonError::HidTransport(format!(
                "Failed to write output report to {:?}: {e}",
                self.path
            ))
        })?;
        // Drain response ACK if keyboard firmware responds (non-blocking / short timeout)
        let mut ack_buf = [0u8; 64];
        let _ = dev.read_timeout(&mut ack_buf, 40);
        Ok(())
    }

    /// Reads an Input Report from the device with a timeout.
    pub fn read_input_report(
        &self,
        buffer: &mut [u8],
        timeout_ms: i32,
    ) -> Result<usize, OpenDragonError> {
        let dev = self
            .device
            .lock()
            .map_err(|_| OpenDragonError::HidTransport("Failed to acquire device lock".into()))?;
        let bytes_read = dev.read_timeout(buffer, timeout_ms).map_err(|e| {
            OpenDragonError::HidTransport(format!(
                "Failed to read input report from {:?}: {e}",
                self.path
            ))
        })?;
        Ok(bytes_read)
    }

    /// Returns the system path associated with this device.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

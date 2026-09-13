use opendragon_core::device::Device;
use opendragon_hid::discovery::DiscoveredDevice;

use crate::generic::GenericRedragonDevice;
use crate::keyboards::RedragonKumaraProK552;
use crate::mice::{RedragonM607, RedragonM909};

pub struct DeviceEntry {
    pub vendor_id: u16,
    pub product_id: u16,
    pub model_name: &'static str,
    pub marketing_name: &'static str,
}

pub static KNOWN_DEVICES: &[DeviceEntry] = &[
    DeviceEntry {
        vendor_id: 0x0c45,
        product_id: 0x5004,
        model_name: "K552RGB-PRO / K551",
        marketing_name: "Redragon Kumara Pro K552RGB-PRO / Mitra RGB",
    },
    DeviceEntry {
        vendor_id: 0x04d9,
        product_id: 0xfc38,
        model_name: "M607",
        marketing_name: "Redragon M607 Griffin",
    },
    DeviceEntry {
        vendor_id: 0x04d9,
        product_id: 0xfc4c,
        model_name: "M909",
        marketing_name: "Redragon M909 Emperor",
    },
    DeviceEntry {
        vendor_id: 0x04d9,
        product_id: 0xfc18,
        model_name: "M601-3",
        marketing_name: "Redragon M601-3 Centrophorus",
    },
    DeviceEntry {
        vendor_id: 0x1a34,
        product_id: 0xf517,
        model_name: "Wireless 2.4G",
        marketing_name: "Redragon / ACRUX Wireless Receiver",
    },
];

pub struct DeviceRegistry;

impl DeviceRegistry {
    /// Matches a discovered hardware device to its specific driver implementation.
    pub fn create_device(discovered: DiscoveredDevice) -> Box<dyn Device> {
        let vid = discovered.info.vendor_id;
        let pid = discovered.info.product_id;

        match (vid, pid) {
            (RedragonKumaraProK552::VENDOR_ID, RedragonKumaraProK552::PRODUCT_ID) => {
                let mut dev = RedragonKumaraProK552::new(discovered.info);
                // Attempt non-fatal connect
                let _ = dev.connect();
                Box::new(dev)
            }
            (RedragonM607::VENDOR_ID, RedragonM607::PRODUCT_ID) => {
                let mut dev = RedragonM607::new(discovered.info);
                let _ = dev.connect();
                Box::new(dev)
            }
            (RedragonM909::VENDOR_ID, RedragonM909::PRODUCT_ID) => {
                let mut dev = RedragonM909::new(discovered.info);
                let _ = dev.connect();
                Box::new(dev)
            }
            _ => Box::new(GenericRedragonDevice::new(discovered.info)),
        }
    }

    /// Finds entry in known devices table.
    pub fn find_known(vid: u16, pid: u16) -> Option<&'static DeviceEntry> {
        KNOWN_DEVICES
            .iter()
            .find(|d| d.vendor_id == vid && d.product_id == pid)
    }
}

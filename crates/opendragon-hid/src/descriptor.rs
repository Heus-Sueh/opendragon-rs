use serde::{Deserialize, Serialize};

/// High-level parsed item from a HID report descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedItem {
    pub tag: String,
    pub value: u32,
    pub raw_bytes: Vec<u8>,
}

/// Information extracted from a collection in the report descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedCollection {
    pub usage_page: Option<u16>,
    pub usage: Option<u16>,
    pub report_ids: Vec<u8>,
    pub feature_reports: Vec<FeatureReportInfo>,
    pub output_reports: Vec<OutputReportInfo>,
    pub input_reports: Vec<InputReportInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureReportInfo {
    pub report_id: Option<u8>,
    pub total_bits: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputReportInfo {
    pub report_id: Option<u8>,
    pub total_bits: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputReportInfo {
    pub report_id: Option<u8>,
    pub total_bits: u32,
}

/// Parsed HID Report Descriptor structure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HidReportDescriptor {
    pub raw: Vec<u8>,
    pub usage_pages: Vec<u16>,
    pub usages: Vec<u16>,
    pub report_ids: Vec<u8>,
    pub collections: Vec<ParsedCollection>,
}

impl HidReportDescriptor {
    /// Parses a raw HID report descriptor into structured representations.
    pub fn parse(raw: &[u8]) -> Self {
        let mut descriptor = Self {
            raw: raw.to_vec(),
            ..Default::default()
        };

        let mut current_usage_page: Option<u16> = None;
        let mut current_usage: Option<u16> = None;
        let mut current_report_id: Option<u8> = None;
        let mut report_size: u32 = 0;
        let mut report_count: u32 = 0;

        let mut current_collection = ParsedCollection {
            usage_page: None,
            usage: None,
            report_ids: Vec::new(),
            feature_reports: Vec::new(),
            output_reports: Vec::new(),
            input_reports: Vec::new(),
        };

        let mut i = 0;
        while i < raw.len() {
            let prefix = raw[i];
            if prefix == 0xFE {
                // Long item
                if i + 2 >= raw.len() {
                    break;
                }
                let data_len = raw[i + 1] as usize;
                i += 3 + data_len;
                continue;
            }

            let b_size = match prefix & 0x03 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 4,
                _ => unreachable!(),
            };
            let b_type = (prefix >> 2) & 0x03;
            let b_tag = (prefix >> 4) & 0x0F;

            let mut value: u32 = 0;
            for j in 0..b_size {
                if i + 1 + j < raw.len() {
                    value |= (raw[i + 1 + j] as u32) << (j * 8);
                }
            }

            match (b_type, b_tag) {
                // Global Items
                (1, 0) => {
                    // Usage Page
                    let up = value as u16;
                    current_usage_page = Some(up);
                    if !descriptor.usage_pages.contains(&up) {
                        descriptor.usage_pages.push(up);
                    }
                }
                (1, 7) => {
                    // Report Size
                    report_size = value;
                }
                (1, 8) => {
                    // Report ID
                    let rid = value as u8;
                    current_report_id = Some(rid);
                    if !descriptor.report_ids.contains(&rid) {
                        descriptor.report_ids.push(rid);
                    }
                    if !current_collection.report_ids.contains(&rid) {
                        current_collection.report_ids.push(rid);
                    }
                }
                (1, 9) => {
                    // Report Count
                    report_count = value;
                }
                // Local Items
                (2, 0) => {
                    // Usage
                    let u = value as u16;
                    current_usage = Some(u);
                    if !descriptor.usages.contains(&u) {
                        descriptor.usages.push(u);
                    }
                }
                // Main Items
                (0, 8) => {
                    // Input
                    let total_bits = report_size * report_count;
                    current_collection.input_reports.push(InputReportInfo {
                        report_id: current_report_id,
                        total_bits,
                    });
                }
                (0, 9) => {
                    // Output
                    let total_bits = report_size * report_count;
                    current_collection.output_reports.push(OutputReportInfo {
                        report_id: current_report_id,
                        total_bits,
                    });
                }
                (0, 10) => {
                    // Collection
                    current_collection.usage_page = current_usage_page;
                    current_collection.usage = current_usage;
                }
                (0, 11) => {
                    // Feature
                    let total_bits = report_size * report_count;
                    current_collection.feature_reports.push(FeatureReportInfo {
                        report_id: current_report_id,
                        total_bits,
                    });
                }
                (0, 12) => {
                    // End Collection
                    if !current_collection.report_ids.is_empty()
                        || !current_collection.feature_reports.is_empty()
                        || !current_collection.output_reports.is_empty()
                        || !current_collection.input_reports.is_empty()
                    {
                        descriptor.collections.push(current_collection.clone());
                    }
                    current_collection = ParsedCollection {
                        usage_page: current_usage_page,
                        usage: current_usage,
                        report_ids: Vec::new(),
                        feature_reports: Vec::new(),
                        output_reports: Vec::new(),
                        input_reports: Vec::new(),
                    };
                }
                _ => {}
            }

            i += 1 + b_size;
        }

        if !current_collection.report_ids.is_empty()
            || !current_collection.feature_reports.is_empty()
            || !current_collection.output_reports.is_empty()
        {
            descriptor.collections.push(current_collection);
        }

        descriptor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_feature_report_descriptor() {
        // Report descriptor with Usage Page Consumer (0x0C), Usage (0x00), Feature (64 bytes)
        let bytes = [
            0x05, 0x0c, // Usage Page (Consumer)
            0x09, 0x00, // Usage (0)
            0xa1, 0x01, // Collection (Application)
            0x15, 0x80, // Logical Min (-128)
            0x25, 0x7f, // Logical Max (127)
            0x95, 0x40, // Report Count (64)
            0x75, 0x08, // Report Size (8)
            0xb1, 0x02, // Feature (Data, Var, Abs)
            0xc0, // End Collection
        ];
        let parsed = HidReportDescriptor::parse(&bytes);
        assert_eq!(parsed.usage_pages, vec![0x0C]);
        assert_eq!(parsed.usages, vec![0x00]);
        assert_eq!(parsed.collections.len(), 1);
        assert_eq!(parsed.collections[0].feature_reports.len(), 1);
        assert_eq!(parsed.collections[0].feature_reports[0].total_bits, 64 * 8);
    }
}

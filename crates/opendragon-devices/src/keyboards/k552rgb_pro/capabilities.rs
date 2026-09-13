use opendragon_core::capability::{Capability, SupportStatus};

pub const K552RGB_PRO_CAPABILITIES: &[Capability] = &[
    Capability::Lighting,
    Capability::PerKeyLighting,
    Capability::PollingRate,
    Capability::Actuation,
    Capability::RapidTrigger,
    Capability::Profile,
    Capability::Macro,
];

pub fn get_support_status(cap: Capability) -> SupportStatus {
    match cap {
        Capability::Lighting => SupportStatus::Experimental,
        Capability::PerKeyLighting => SupportStatus::Wip,
        Capability::PollingRate => SupportStatus::Experimental,
        Capability::Actuation => SupportStatus::Wip,
        Capability::RapidTrigger => SupportStatus::Wip,
        Capability::Profile => SupportStatus::Wip,
        Capability::Macro => SupportStatus::Unknown,
        _ => SupportStatus::Unsupported,
    }
}

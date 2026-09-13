pub mod device;
pub mod diagnostics;
pub mod home;
pub mod lighting;
pub mod macros;
pub mod performance;
pub mod profiles;
pub mod settings;

pub use device::DeviceOverview;
pub use diagnostics::DiagnosticsView;
pub use home::HomeView;
pub use lighting::LightingView;
pub use macros::MacrosView;
pub use performance::PerformanceView;
pub use profiles::ProfilesView;
pub use settings::SettingsView;

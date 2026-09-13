use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::capability::Capability;
use opendragon_core::color::RgbColor;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::profile::ProfileId;
use opendragon_core::rapid_trigger::RapidTriggerConfig;

use crate::services::devices::{DeviceBackend, DeviceSummary, MacroItem, ProfileSummary};
use crate::services::preferences::Preferences;
use crate::theme::{Theme, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Home,
    Overview,
    Lighting,
    Performance,
    Profiles,
    Macros,
    Settings,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct KeyboardVisualState {
    pub global_lighting: LightingConfig,
    pub per_key_colors: HashMap<u8, RgbColor>,
    pub selected_keys: HashSet<u8>,
}

impl Default for KeyboardVisualState {
    fn default() -> Self {
        Self {
            global_lighting: LightingConfig {
                mode: LightingMode::Static,
                color: RgbColor::new(217, 70, 239),
                brightness: 75,
                speed: 3,
                direction: None,
            },
            per_key_colors: HashMap::new(),
            selected_keys: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceState {
    pub summary: DeviceSummary,
    pub lighting: LightingConfig,
    pub actuation: ActuationPoint,
    pub rapid_trigger: RapidTriggerConfig,
    pub polling_rate: PollingRate,
    pub dpi: u16,
    pub profiles: Vec<ProfileSummary>,
    pub active_profile: ProfileId,
    pub macros: Vec<MacroItem>,
    pub visual_state: KeyboardVisualState,
}

pub struct AppState {
    pub devices: Vec<DeviceSummary>,
    pub selected_device_id: Option<String>,
    pub current_page: Page,
    pub preferences: Preferences,
    pub theme: Theme,
    pub active_device: Option<DeviceState>,
    pub toast: Option<(String, ToastType)>,
    pub is_loading: bool,
    pub backend: Arc<Mutex<dyn DeviceBackend>>,
}

impl AppState {
    pub fn new(backend: Arc<Mutex<dyn DeviceBackend>>) -> Self {
        let prefs = Preferences::load();
        let theme = prefs.theme_mode.resolve();

        let mut app = Self {
            devices: Vec::new(),
            selected_device_id: prefs.last_selected_device.clone(),
            current_page: Page::Home,
            preferences: prefs,
            theme,
            active_device: None,
            toast: None,
            is_loading: false,
            backend,
        };

        app.refresh_devices();
        app
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.preferences.theme_mode = mode;
        self.theme = mode.resolve();
        self.preferences.save();
    }

    pub fn set_toast(&mut self, message: impl Into<String>, toast_type: ToastType) {
        self.toast = Some((message.into(), toast_type));
    }

    pub fn clear_toast(&mut self) {
        self.toast = None;
    }

    pub fn refresh_devices(&mut self) {
        let res = {
            let mut backend = self.backend.lock().unwrap();
            backend.enumerate_devices()
        };

        match res {
            Ok(devs) => {
                self.devices = devs;
                if let Some(selected_id) = &self.selected_device_id {
                    if !self.devices.iter().any(|d| &d.id == selected_id) {
                        if let Some(active) = &mut self.active_device {
                            active.summary.is_connected = false;
                        }
                    }
                }

                if self.selected_device_id.is_none() {
                    let to_select = self
                        .preferences
                        .last_selected_device
                        .as_ref()
                        .and_then(|pref_id| self.devices.iter().find(|d| &d.id == pref_id))
                        .or_else(|| self.devices.iter().find(|d| d.is_supported));

                    if let Some(dev) = to_select {
                        let id = dev.id.clone();
                        self.select_device(&id);
                    }
                }
            }
            Err(e) => {
                self.set_toast(format!("Failed to refresh devices: {e}"), ToastType::Error);
            }
        }
    }

    pub fn select_device(&mut self, device_id: &str) {
        self.selected_device_id = Some(device_id.to_string());
        self.preferences.last_selected_device = Some(device_id.to_string());
        self.preferences.save();

        let summary = match self.devices.iter().find(|d| d.id == device_id) {
            Some(s) => s.clone(),
            None => return,
        };

        let (lighting, actuation, rapid_trigger, polling_rate, profiles, macros, dpi) = {
            let backend = self.backend.lock().unwrap();
            let l = backend
                .get_lighting(device_id)
                .unwrap_or_else(|_| LightingConfig {
                    mode: LightingMode::Static,
                    color: RgbColor::new(217, 70, 239),
                    brightness: 75,
                    speed: 3,
                    direction: None,
                });

            let a = backend
                .get_actuation(device_id)
                .unwrap_or_else(|_| ActuationPoint::new(1.2).unwrap());

            let rt = backend
                .get_rapid_trigger(device_id)
                .unwrap_or(RapidTriggerConfig {
                    enabled: true,
                    press_sensitivity_mm: 0.3,
                    release_sensitivity_mm: 0.3,
                });

            let pr = backend
                .get_polling_rate(device_id)
                .unwrap_or(PollingRate::Hz1000);

            let profs = backend.get_profiles(device_id).unwrap_or_else(|_| {
                vec![ProfileSummary {
                    id: ProfileId::new(0),
                    name: "Default".to_string(),
                    is_active: true,
                }]
            });

            let m = backend.get_macros(device_id).unwrap_or_default();
            let dpi = backend.get_dpi(device_id).unwrap_or(1600);
            (l, a, rt, pr, profs, m, dpi)
        };

        let active_profile = profiles
            .iter()
            .find(|p| p.is_active)
            .map(|p| p.id)
            .unwrap_or(ProfileId::new(0));

        let visual_state = KeyboardVisualState {
            global_lighting: lighting.clone(),
            per_key_colors: HashMap::new(),
            selected_keys: HashSet::new(),
        };

        self.active_device = Some(DeviceState {
            summary,
            lighting,
            actuation,
            rapid_trigger,
            polling_rate,
            dpi,
            profiles,
            active_profile,
            macros,
            visual_state,
        });

        self.current_page = Page::Overview;
    }

    pub fn navigate_to(&mut self, page: Page) {
        self.current_page = page;
    }

    pub fn update_lighting(&mut self, config: LightingConfig) {
        if let Some(active) = &mut self.active_device {
            active.lighting = config.clone();
            active.visual_state.global_lighting = config.clone();
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_lighting(&id, &config);
        }
        self.set_toast("Lighting updated", ToastType::Success);
    }

    pub fn update_actuation(&mut self, point: ActuationPoint) {
        if let Some(active) = &mut self.active_device {
            active.actuation = point;
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_actuation(&id, point);
        }
        self.set_toast(
            format!("Actuation set to {:.1} mm", point.mm()),
            ToastType::Success,
        );
    }

    pub fn update_rapid_trigger(&mut self, config: RapidTriggerConfig) {
        if let Some(active) = &mut self.active_device {
            active.rapid_trigger = config;
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_rapid_trigger(&id, &config);
        }
        let state_str = if config.enabled {
            "enabled"
        } else {
            "disabled"
        };
        self.set_toast(format!("Rapid Trigger {state_str}"), ToastType::Success);
    }

    pub fn update_polling_rate(&mut self, rate: PollingRate) {
        if let Some(active) = &mut self.active_device {
            active.polling_rate = rate;
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_polling_rate(&id, rate);
        }
        self.set_toast(format!("Polling rate set to {rate}"), ToastType::Success);
    }

    pub fn switch_profile(&mut self, profile_id: ProfileId) {
        if let Some(active) = &mut self.active_device {
            for p in &mut active.profiles {
                p.is_active = p.id == profile_id;
            }
            active.active_profile = profile_id;
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_active_profile(&id, profile_id);
        }
        self.set_toast("Profile activated", ToastType::Success);
    }

    pub fn add_profile(&mut self, name: String) {
        let (id, new_id_res) = if let Some(active) = &self.active_device {
            (active.summary.id.clone(), {
                let mut backend = self.backend.lock().unwrap();
                backend.create_profile(&active.summary.id, name.clone())
            })
        } else {
            return;
        };

        if let Ok(new_id) = new_id_res {
            if let Some(active) = &mut self.active_device {
                if active.summary.id == id {
                    active.profiles.push(ProfileSummary {
                        id: new_id,
                        name,
                        is_active: false,
                    });
                }
            }
            self.set_toast("Profile created", ToastType::Success);
        }
    }

    pub fn delete_profile(&mut self, profile_id: ProfileId) {
        let (id, del_res) = if let Some(active) = &self.active_device {
            (active.summary.id.clone(), {
                let mut backend = self.backend.lock().unwrap();
                backend.delete_profile(&active.summary.id, profile_id)
            })
        } else {
            return;
        };

        if del_res.is_ok() {
            if let Some(active) = &mut self.active_device {
                if active.summary.id == id {
                    active.profiles.retain(|p| p.id != profile_id);
                }
            }
            self.set_toast("Profile deleted", ToastType::Success);
        }
    }

    pub fn has_capability(&self, cap: Capability) -> bool {
        self.active_device
            .as_ref()
            .map(|d| d.summary.capabilities.contains(&cap))
            .unwrap_or(false)
    }

    pub fn set_auto_connect(&mut self, enabled: bool) {
        self.preferences.auto_connect = enabled;
        self.preferences.save();
    }

    pub fn toggle_key_selection(&mut self, key_id: u8) {
        if let Some(active) = &mut self.active_device {
            if active.visual_state.selected_keys.contains(&key_id) {
                active.visual_state.selected_keys.remove(&key_id);
            } else {
                active.visual_state.selected_keys.insert(key_id);
            }
        }
    }

    pub fn select_key(&mut self, key_id: u8) {
        if let Some(active) = &mut self.active_device {
            active.visual_state.selected_keys.insert(key_id);
        }
    }

    pub fn clear_key_selection(&mut self) {
        if let Some(active) = &mut self.active_device {
            active.visual_state.selected_keys.clear();
        }
    }

    pub fn update_dpi(&mut self, dpi: u16) {
        if let Some(active) = &mut self.active_device {
            active.dpi = dpi;
            let id = active.summary.id.clone();
            let mut backend = self.backend.lock().unwrap();
            let _ = backend.set_dpi(&id, dpi);
        }
        self.set_toast(format!("DPI updated to {dpi}"), ToastType::Success);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::devices::MockDeviceBackend;

    #[test]
    fn test_app_state_device_selection_and_navigation() {
        let backend = Arc::new(Mutex::new(MockDeviceBackend::new()));
        let mut state = AppState::new(backend);
        assert!(!state.devices.is_empty());

        state.select_device("mock-k552rgb-pro");
        assert_eq!(state.current_page, Page::Overview);
        assert!(state.active_device.is_some());

        assert!(state.has_capability(Capability::Lighting));
        assert!(state.has_capability(Capability::Actuation));
        assert!(state.has_capability(Capability::RapidTrigger));

        state.navigate_to(Page::Lighting);
        assert_eq!(state.current_page, Page::Lighting);
    }

    #[test]
    fn test_app_state_toast_lifecycle() {
        let backend = Arc::new(Mutex::new(MockDeviceBackend::new()));
        let mut state = AppState::new(backend);
        assert!(state.toast.is_none());

        state.set_toast("Device configured", ToastType::Success);
        assert!(state.toast.is_some());
        let (msg, t_type) = state.toast.as_ref().unwrap();
        assert_eq!(msg, "Device configured");
        assert_eq!(*t_type, ToastType::Success);

        state.clear_toast();
        assert!(state.toast.is_none());
    }
}

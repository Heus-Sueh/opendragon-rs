use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::theme::ThemeMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyboardLayout {
    #[default]
    Ansi,
    Abnt2,
    Iso,
}

impl KeyboardLayout {
    pub fn label(&self) -> &'static str {
        match self {
            KeyboardLayout::Ansi => "ANSI (US)",
            KeyboardLayout::Abnt2 => "ABNT2 (BR)",
            KeyboardLayout::Iso => "ISO (EU)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub theme_mode: ThemeMode,
    pub last_selected_device: Option<String>,
    pub start_minimized: bool,
    pub auto_connect: bool,
    #[serde(default)]
    pub keyboard_layout: KeyboardLayout,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            last_selected_device: None,
            start_minimized: false,
            auto_connect: true,
            keyboard_layout: KeyboardLayout::Ansi,
        }
    }
}

impl Preferences {
    fn config_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        Some(
            PathBuf::from(home)
                .join(".config")
                .join("opendragon")
                .join("gui_config.json"),
        )
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(prefs) = serde_json::from_str::<Self>(&data) {
                        return prefs;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(&path, json);
            }
        }
    }
}

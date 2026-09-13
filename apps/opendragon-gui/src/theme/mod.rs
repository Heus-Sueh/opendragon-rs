pub mod colors;
pub mod radius;
pub mod spacing;
pub mod typography;

pub use colors::Theme;
pub use radius::*;
pub use spacing::*;
pub use typography::*;

use gpui::Global;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn resolve(&self) -> Theme {
        match self {
            ThemeMode::Dark | ThemeMode::System => Theme::dark(),
            ThemeMode::Light => Theme::light(),
        }
    }
}

impl Global for Theme {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn test_theme_mode_resolution() {
        let dark_theme = ThemeMode::Dark.resolve();
        assert!(dark_theme.is_dark);

        let light_theme = ThemeMode::Light.resolve();
        assert!(!light_theme.is_dark);

        let system_theme = ThemeMode::System.resolve();
        assert!(system_theme.is_dark);
    }

    #[test]
    fn test_spacing_and_radius_tokens() {
        assert_eq!(SPACE_1, px(4.0));
        assert_eq!(SPACE_2, px(8.0));
        assert_eq!(SPACE_4, px(16.0));
        assert_eq!(RADIUS_MD, px(5.0));
    }
}

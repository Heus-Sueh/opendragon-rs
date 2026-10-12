use gpui::hsla;
use gpui::Hsla;

/// Unified monochrome color scheme matching the gpui-kit / shadcn standard default palette.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Hsla,
    pub surface: Hsla,
    pub surface_elevated: Hsla,
    pub surface_hover: Hsla,
    pub surface_active: Hsla,
    pub border: Hsla,
    pub border_subtle: Hsla,
    pub border_focus: Hsla,

    pub foreground: Hsla,
    pub foreground_muted: Hsla,
    pub foreground_subtle: Hsla,

    // Aliases for compatibility
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_subtle: Hsla,

    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_active: Hsla,
    pub accent_subtle: Hsla,
    pub accent_border: Hsla,
    pub accent_foreground: Hsla,

    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,

    pub is_dark: bool,
}

impl Theme {
    /// Default gpui-kit dark mode: pure black/dark zinc surfaces, crisp white accents & text.
    pub fn dark() -> Self {
        let foreground = hsla(0.0, 0.0, 0.98, 1.0); // Pure crisp white (#FAFAFA)
        let foreground_muted = hsla(240.0 / 360.0, 0.05, 0.65, 1.0); // Neutral zinc-400 (#A1A1AA)
        let foreground_subtle = hsla(240.0 / 360.0, 0.05, 0.45, 1.0); // Neutral zinc-500 (#71717A)

        Self {
            background: hsla(240.0 / 360.0, 0.10, 0.04, 1.0), // Deep zinc-950 (#09090B)
            surface: hsla(240.0 / 360.0, 0.06, 0.09, 1.0),    // Card surface zinc-900 (#141417)
            surface_elevated: hsla(240.0 / 360.0, 0.06, 0.13, 1.0), // Surface elevated zinc-850 (#1E1E22)
            surface_hover: hsla(240.0 / 360.0, 0.05, 0.17, 1.0), // Hover surface zinc-800 (#27272A)
            surface_active: hsla(240.0 / 360.0, 0.05, 0.24, 1.0), // Active surface zinc-700 (#37373C)
            border: hsla(240.0 / 360.0, 0.05, 0.18, 1.0),         // Border zinc-800 (#27272A)
            border_subtle: hsla(240.0 / 360.0, 0.05, 0.12, 1.0),  // Subtle separator (#1C1C1F)
            border_focus: hsla(0.0, 0.0, 0.98, 1.0),              // White focus indicator

            foreground,
            foreground_muted,
            foreground_subtle,

            text: foreground,
            text_muted: foreground_muted,
            text_subtle: foreground_subtle,

            // Pure monochrome accents
            accent: hsla(0.0, 0.0, 0.98, 1.0), // Pure white active element
            accent_hover: hsla(0.0, 0.0, 0.88, 1.0), // Light neutral gray (#E4E4E7)
            accent_active: hsla(0.0, 0.0, 0.80, 1.0), // Mid neutral gray (#D4D4D8)
            accent_subtle: hsla(240.0 / 360.0, 0.05, 0.16, 1.0), // Dark neutral badge (#27272A)
            accent_border: hsla(240.0 / 360.0, 0.05, 0.28, 1.0), // Accent border
            accent_foreground: hsla(240.0 / 360.0, 0.10, 0.04, 1.0), // Dark text on white accent

            success: hsla(142.0 / 360.0, 0.65, 0.45, 1.0),
            warning: hsla(38.0 / 360.0, 0.90, 0.50, 1.0),
            danger: hsla(0.0, 0.75, 0.55, 1.0),

            is_dark: true,
        }
    }

    /// Default gpui-kit light mode: clean white backgrounds, black accents & text.
    pub fn light() -> Self {
        let foreground = hsla(240.0 / 360.0, 0.10, 0.04, 1.0); // Pure dark (#09090B)
        let foreground_muted = hsla(240.0 / 360.0, 0.05, 0.42, 1.0); // Zinc-500
        let foreground_subtle = hsla(240.0 / 360.0, 0.05, 0.60, 1.0); // Zinc-400

        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0), // Pure white (#FFFFFF)
            surface: hsla(0.0, 0.0, 0.98, 1.0),   // Card surface (#FAFAFA)
            surface_elevated: hsla(240.0 / 360.0, 0.05, 0.96, 1.0), // Elevated (#F4F4F5)
            surface_hover: hsla(240.0 / 360.0, 0.05, 0.92, 1.0), // Hover (#E4E4E7)
            surface_active: hsla(240.0 / 360.0, 0.05, 0.86, 1.0), // Active (#D4D4D8)
            border: hsla(240.0 / 360.0, 0.05, 0.88, 1.0), // Border (#E4E4E7)
            border_subtle: hsla(240.0 / 360.0, 0.05, 0.94, 1.0), // Subtle separator (#F4F4F5)
            border_focus: hsla(240.0 / 360.0, 0.10, 0.04, 1.0), // Black focus indicator

            foreground,
            foreground_muted,
            foreground_subtle,

            text: foreground,
            text_muted: foreground_muted,
            text_subtle: foreground_subtle,

            // Pure monochrome accents
            accent: hsla(240.0 / 360.0, 0.10, 0.04, 1.0), // Pure black active element
            accent_hover: hsla(240.0 / 360.0, 0.05, 0.18, 1.0),
            accent_active: hsla(240.0 / 360.0, 0.05, 0.28, 1.0),
            accent_subtle: hsla(240.0 / 360.0, 0.05, 0.94, 1.0),
            accent_border: hsla(240.0 / 360.0, 0.05, 0.84, 1.0),
            accent_foreground: hsla(0.0, 0.0, 1.0, 1.0), // White text on black accent

            success: hsla(142.0 / 360.0, 0.65, 0.40, 1.0),
            warning: hsla(38.0 / 360.0, 0.90, 0.45, 1.0),
            danger: hsla(0.0, 0.75, 0.50, 1.0),

            is_dark: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_dark_and_light() {
        let dark = Theme::dark();
        assert!(dark.is_dark);
        assert_eq!(dark.foreground, hsla(0.0, 0.0, 0.98, 1.0));
        assert_eq!(dark.accent, hsla(0.0, 0.0, 0.98, 1.0));

        let light = Theme::light();
        assert!(!light.is_dark);
        assert_eq!(light.foreground, hsla(240.0 / 360.0, 0.10, 0.04, 1.0));
        assert_eq!(light.accent, hsla(240.0 / 360.0, 0.10, 0.04, 1.0));
    }
}

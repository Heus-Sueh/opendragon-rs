use gpui::hsla;
use gpui::Hsla;

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

    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,

    pub is_dark: bool,
}

impl Theme {
    pub fn dark() -> Self {
        let foreground = hsla(216.0 / 360.0, 0.10, 0.91, 1.0);
        let foreground_muted = hsla(218.0 / 360.0, 0.07, 0.58, 1.0);
        let foreground_subtle = hsla(219.0 / 360.0, 0.08, 0.36, 1.0);

        Self {
            background: hsla(225.0 / 360.0, 0.08, 0.09, 1.0),
            surface: hsla(220.0 / 360.0, 0.09, 0.13, 1.0),
            surface_elevated: hsla(220.0 / 360.0, 0.11, 0.16, 1.0),
            surface_hover: hsla(220.0 / 360.0, 0.11, 0.19, 1.0),
            surface_active: hsla(220.0 / 360.0, 0.12, 0.22, 1.0),
            border: hsla(216.0 / 360.0, 0.10, 0.20, 1.0),
            border_subtle: hsla(220.0 / 360.0, 0.10, 0.15, 1.0),
            border_focus: hsla(356.0 / 360.0, 0.74, 0.53, 1.0),

            foreground,
            foreground_muted,
            foreground_subtle,

            text: foreground,
            text_muted: foreground_muted,
            text_subtle: foreground_subtle,

            accent: hsla(356.0 / 360.0, 0.74, 0.53, 1.0),
            accent_hover: hsla(356.0 / 360.0, 0.79, 0.59, 1.0),
            accent_active: hsla(356.0 / 360.0, 0.70, 0.46, 1.0),
            accent_subtle: hsla(353.0 / 360.0, 0.30, 0.13, 1.0),
            accent_border: hsla(354.0 / 360.0, 0.52, 0.24, 1.0),

            success: hsla(142.0 / 360.0, 0.65, 0.45, 1.0),
            warning: hsla(38.0 / 360.0, 0.90, 0.50, 1.0),
            danger: hsla(0.0, 0.75, 0.55, 1.0),

            is_dark: true,
        }
    }

    pub fn light() -> Self {
        let foreground = hsla(220.0 / 360.0, 0.20, 0.12, 1.0);
        let foreground_muted = hsla(220.0 / 360.0, 0.10, 0.45, 1.0);
        let foreground_subtle = hsla(220.0 / 360.0, 0.10, 0.65, 1.0);

        Self {
            background: hsla(220.0 / 360.0, 0.12, 0.96, 1.0),
            surface: hsla(0.0, 0.0, 1.0, 1.0),
            surface_elevated: hsla(0.0, 0.0, 0.98, 1.0),
            surface_hover: hsla(220.0 / 360.0, 0.10, 0.92, 1.0),
            surface_active: hsla(220.0 / 360.0, 0.10, 0.88, 1.0),
            border: hsla(220.0 / 360.0, 0.10, 0.82, 1.0),
            border_subtle: hsla(220.0 / 360.0, 0.10, 0.90, 1.0),
            border_focus: hsla(356.0 / 360.0, 0.74, 0.48, 1.0),

            foreground,
            foreground_muted,
            foreground_subtle,

            text: foreground,
            text_muted: foreground_muted,
            text_subtle: foreground_subtle,

            accent: hsla(356.0 / 360.0, 0.74, 0.48, 1.0),
            accent_hover: hsla(356.0 / 360.0, 0.74, 0.42, 1.0),
            accent_active: hsla(356.0 / 360.0, 0.70, 0.38, 1.0),
            accent_subtle: hsla(356.0 / 360.0, 0.40, 0.94, 1.0),
            accent_border: hsla(356.0 / 360.0, 0.50, 0.80, 1.0),

            success: hsla(142.0 / 360.0, 0.65, 0.40, 1.0),
            warning: hsla(38.0 / 360.0, 0.90, 0.45, 1.0),
            danger: hsla(0.0, 0.75, 0.50, 1.0),

            is_dark: false,
        }
    }
}

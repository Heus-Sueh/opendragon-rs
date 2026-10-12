use gpui::prelude::*;
use gpui::*;
use gpui_kit::component::badge::Badge as KitBadge;

use crate::theme::{Theme, FONT_SIZE_CAPTION, RADIUS_SM, SPACE_2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStatus {
    Connected,
    Disconnected,
    Supported,
    Experimental,
    Partial,
}

pub struct StatusBadge {
    status: BadgeStatus,
    custom_label: Option<SharedString>,
}

impl StatusBadge {
    pub fn new(status: BadgeStatus) -> Self {
        Self {
            status,
            custom_label: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.custom_label = Some(label.into());
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let (dot_color, bg_color, text_color, default_label) = match self.status {
            BadgeStatus::Connected => (
                theme.success,
                hsla(142.0 / 360.0, 0.65, 0.45, 0.12),
                theme.success,
                "Connected",
            ),
            BadgeStatus::Disconnected => (
                theme.foreground_subtle,
                hsla(220.0 / 360.0, 0.10, 0.50, 0.12),
                theme.foreground_muted,
                "Disconnected",
            ),
            BadgeStatus::Supported => (
                theme.success,
                hsla(142.0 / 360.0, 0.65, 0.45, 0.12),
                theme.success,
                "Supported",
            ),
            BadgeStatus::Experimental => (
                theme.warning,
                hsla(38.0 / 360.0, 0.90, 0.50, 0.12),
                theme.warning,
                "Experimental",
            ),
            BadgeStatus::Partial => (
                theme.warning,
                hsla(38.0 / 360.0, 0.90, 0.50, 0.12),
                theme.warning,
                "Partial",
            ),
        };

        let label = self.custom_label.unwrap_or_else(|| default_label.into());
        let dot = KitBadge::new().dot().color(dot_color);

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(SPACE_2)
            .px(SPACE_2)
            .py(px(3.0))
            .rounded(RADIUS_SM)
            .bg(bg_color)
            .child(dot)
            .child(
                div()
                    .text_color(text_color)
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(FontWeight::MEDIUM)
                    .child(label),
            )
    }
}

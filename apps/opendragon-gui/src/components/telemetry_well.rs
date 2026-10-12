use gpui::prelude::*;
use gpui::*;

use crate::theme::{
    Theme, FONT_SIZE_CAPTION, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_MD, RADIUS_SM, SPACE_1, SPACE_3,
};

pub struct TelemetryWell {
    label: String,
    value: String,
    detail: Option<String>,
    badge: Option<String>,
    accent_value: bool,
}

impl TelemetryWell {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            detail: None,
            badge: None,
            accent_value: false,
        }
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn accent_value(mut self, accent: bool) -> Self {
        self.accent_value = accent;
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let mut well = div()
            .flex()
            .flex_col()
            .justify_between()
            .flex_1()
            .min_w(px(140.0))
            .p(SPACE_3)
            .rounded(RADIUS_MD)
            .bg(theme.surface_elevated)
            .border_1()
            .border_color(theme.border_subtle);

        // Header: label + optional badge
        let mut header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_color(theme.foreground_muted)
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.label),
            );

        if let Some(badge_text) = self.badge {
            header = header.child(
                div()
                    .px(SPACE_1)
                    .py(px(1.0))
                    .rounded(RADIUS_SM)
                    .bg(theme.accent_subtle)
                    .border_1()
                    .border_color(theme.accent_border)
                    .text_color(theme.accent)
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(badge_text),
            );
        }

        well = well.child(header);

        // Value
        let value_color = if self.accent_value {
            theme.accent
        } else {
            theme.foreground
        };

        well = well.child(
            div()
                .my(SPACE_1)
                .text_color(value_color)
                .text_size(FONT_SIZE_TITLE)
                .font_weight(FontWeight::BOLD)
                .child(self.value),
        );

        // Detail
        if let Some(detail_text) = self.detail {
            well = well.child(
                div()
                    .text_color(theme.foreground_subtle)
                    .text_size(FONT_SIZE_SM)
                    .child(detail_text),
            );
        }

        well
    }
}

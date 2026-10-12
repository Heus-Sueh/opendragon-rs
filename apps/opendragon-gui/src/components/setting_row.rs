use gpui::prelude::*;
use gpui::*;

use crate::theme::{Theme, FONT_SIZE_BODY, FONT_SIZE_SM, RADIUS_MD, SPACE_1, SPACE_2, SPACE_3};

/// Modern setting item row (analogous to AdwActionRow or macOS System Settings row)
pub struct SettingRow {
    title: SharedString,
    description: Option<SharedString>,
    control: Option<AnyElement>,
    flat: bool,
}

impl SettingRow {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            control: None,
            flat: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }

    pub fn flat(mut self, flat: bool) -> Self {
        self.flat = flat;
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .gap(SPACE_3);

        if !self.flat {
            row = row
                .px(SPACE_3)
                .py(SPACE_2)
                .rounded(RADIUS_MD)
                .bg(theme.surface)
                .border_1()
                .border_color(theme.border);
        } else {
            row = row.py(SPACE_2);
        }

        let label_col = div()
            .flex()
            .flex_col()
            .gap(SPACE_1)
            .child(
                div()
                    .text_color(theme.foreground)
                    .text_size(FONT_SIZE_BODY)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.title),
            )
            .when_some(self.description, |el, desc| {
                el.child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_SM)
                        .child(desc),
                )
            });

        row = row.child(label_col);

        if let Some(ctrl) = self.control {
            row = row.child(ctrl);
        }

        row
    }
}

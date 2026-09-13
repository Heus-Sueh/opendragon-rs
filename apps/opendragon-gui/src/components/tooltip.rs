use gpui::*;
use gpui_kit::component::tooltip::Tooltip as KitTooltip;

use crate::theme::{Theme, FONT_SIZE_XS, RADIUS_SM, SPACE_1, SPACE_2};

pub struct InfoTooltip {
    text: SharedString,
}

impl InfoTooltip {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }

    pub fn render(self, theme: &Theme) -> Stateful<Div> {
        let tooltip_text = self.text.clone();
        div()
            .id(ElementId::NamedInteger(self.text.clone(), 99))
            .flex()
            .flex_row()
            .items_center()
            .gap(SPACE_1)
            .px(SPACE_2)
            .py(px(2.0))
            .rounded(RADIUS_SM)
            .bg(theme.surface_hover)
            .hover(|s| s.bg(theme.surface_active).border_color(theme.accent))
            .border_1()
            .border_color(theme.border_subtle)
            .cursor_pointer()
            .tooltip(move |window, cx| KitTooltip::new(tooltip_text.clone()).build(window, cx))
            .child(
                div()
                    .text_color(theme.accent)
                    .text_size(FONT_SIZE_XS)
                    .font_weight(FontWeight::BOLD)
                    .child("ℹ"),
            )
            .child(
                div()
                    .text_color(theme.text_muted)
                    .text_size(FONT_SIZE_XS)
                    .child(self.text),
            )
    }
}

use gpui::prelude::*;
use gpui::*;

use crate::theme::{
    Theme, FONT_SIZE_BODY, FONT_SIZE_HEADING, FONT_SIZE_LG, RADIUS_MD, RADIUS_SM, SPACE_2, SPACE_4,
    SPACE_6,
};

pub struct EmptyState {
    icon_text: Option<&'static str>,
    title: SharedString,
    message: SharedString,
    action: Option<AnyElement>,
}

impl EmptyState {
    pub fn new(title: impl Into<SharedString>, message: impl Into<SharedString>) -> Self {
        Self {
            icon_text: None,
            title: title.into(),
            message: message.into(),
            action: None,
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon_text = Some(icon);
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let mut container = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p(SPACE_6)
            .rounded(RADIUS_MD)
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border)
            .w_full()
            .gap(SPACE_4);

        if let Some(icon) = self.icon_text {
            container = container.child(
                div()
                    .size(px(40.0))
                    .rounded(RADIUS_SM)
                    .bg(theme.surface_hover)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(FONT_SIZE_LG)
                    .text_color(theme.foreground_muted)
                    .child(icon),
            );
        }

        container = container.child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(SPACE_2)
                .child(
                    div()
                        .text_color(theme.foreground)
                        .text_size(FONT_SIZE_HEADING)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(self.title),
                )
                .child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_BODY)
                        .child(self.message),
                ),
        );

        if let Some(act) = self.action {
            container = container.child(div().mt(SPACE_2).child(act));
        }

        container
    }
}

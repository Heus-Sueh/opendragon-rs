use gpui::prelude::*;
use gpui::*;

use crate::theme::{Theme, FONT_SIZE_HEADING, FONT_SIZE_SM, SPACE_1, SPACE_3};

pub struct Section {
    title: SharedString,
    description: Option<SharedString>,
    action: Option<AnyElement>,
    content: Vec<AnyElement>,
}

impl Section {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            action: None,
            content: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content.push(child.into_any_element());
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let mut header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_1)
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_HEADING)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(self.title),
                    )
                    .when_some(self.description, |el, desc| {
                        el.child(
                            div()
                                .text_color(theme.foreground_muted)
                                .text_size(FONT_SIZE_SM)
                                .child(desc),
                        )
                    }),
            );

        if let Some(action) = self.action {
            header = header.child(action);
        }

        let mut section_container = div().flex().flex_col().gap(SPACE_3).w_full().child(header);

        for c in self.content {
            section_container = section_container.child(c);
        }

        section_container
    }
}

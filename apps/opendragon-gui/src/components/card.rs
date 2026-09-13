use gpui::prelude::*;
use gpui::*;

use crate::theme::{Theme, RADIUS_MD, SPACE_4};

pub type CardClickHandler = Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>;

pub struct Card {
    content: Vec<AnyElement>,
    is_interactive: bool,
    on_click: Option<CardClickHandler>,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            is_interactive: false,
            on_click: None,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content.push(child.into_any_element());
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.is_interactive = interactive;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self.is_interactive = true;
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let hover_bg = theme.surface_hover;
        let mut el = div()
            .flex()
            .flex_col()
            .p(SPACE_4)
            .rounded(RADIUS_MD)
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border);

        if self.is_interactive {
            el = el.cursor_pointer().hover(move |mut s| {
                s.background = Some(hover_bg.into());
                s
            });
            if let Some(handler) = self.on_click {
                el = el.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    handler(event, window, cx);
                });
            }
        }

        for child in self.content {
            el = el.child(child);
        }

        el
    }
}

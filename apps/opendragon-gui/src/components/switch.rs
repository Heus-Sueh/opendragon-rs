use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_kit::component::switch::Switch as KitSwitch;
use gpui_kit::component::Disableable;

use crate::theme::{
    Theme, FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_SM, RADIUS_MD, SPACE_1, SPACE_2, SPACE_3,
};

pub type SwitchToggleHandler = Arc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

pub struct Switch {
    label: SharedString,
    description: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_toggle: Option<SwitchToggleHandler>,
}

impl Switch {
    pub fn new(label: impl Into<SharedString>, checked: bool) -> Self {
        Self {
            label: label.into(),
            description: None,
            checked,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let is_checked = self.checked;

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(SPACE_3)
            .py(SPACE_2)
            .rounded(RADIUS_MD)
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border);

        let on_toggle_clone = self.on_toggle.clone();

        if self.disabled {
            row = row.opacity(0.4).cursor_not_allowed();
        } else {
            row = row
                .cursor_pointer()
                .hover(move |s| s.bg(theme.surface_hover));

            if let Some(ref handler) = on_toggle_clone {
                let h = handler.clone();
                row = row.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    h(!is_checked, window, cx);
                });
            }
        }

        let label_block = div()
            .flex()
            .flex_col()
            .gap(SPACE_1)
            .child(
                div()
                    .text_color(if self.disabled {
                        theme.foreground_muted
                    } else {
                        theme.foreground
                    })
                    .text_size(FONT_SIZE_BODY)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.label.clone()),
            )
            .when_some(self.description, |el, desc| {
                el.child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_SM)
                        .child(desc),
                )
            });

        let id: ElementId = self.label.into();
        let mut kit_switch = KitSwitch::new(id)
            .checked(is_checked)
            .disabled(self.disabled);

        if let Some(handler) = self.on_toggle {
            kit_switch = kit_switch.on_click(move |val: &bool, window, cx| {
                handler(*val, window, cx);
            });
        }

        let status_text = if is_checked { "ON" } else { "OFF" };
        let status_indicator = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(SPACE_3)
            .child(
                div()
                    .text_color(if is_checked {
                        theme.accent
                    } else {
                        theme.foreground_subtle
                    })
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(status_text),
            )
            .child(kit_switch);

        row.child(label_block).child(status_indicator)
    }
}

use gpui::*;
use gpui_kit::component::button::{Button as KitButton, ButtonVariants};
use gpui_kit::component::{Disableable, Sizable, Size};

use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
}

pub type ButtonClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

pub struct Button {
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    is_disabled: bool,
    on_click: Option<ButtonClickHandler>,
}

impl Button {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Md,
            is_disabled: false,
            on_click: None,
        }
    }

    pub fn primary(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Primary)
    }

    pub fn ghost(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    pub fn danger(label: impl Into<SharedString>) -> Self {
        Self::new(label).variant(ButtonVariant::Danger)
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn render(self, _theme: &Theme) -> KitButton {
        let id: ElementId = self.label.clone().into();
        let mut b = KitButton::new(id).label(self.label);

        b = match self.variant {
            ButtonVariant::Primary => b.primary(),
            ButtonVariant::Secondary => b.secondary(),
            ButtonVariant::Ghost => b.ghost(),
            ButtonVariant::Danger => b.danger(),
        };

        if self.size == ButtonSize::Sm {
            b = b.with_size(Size::Small);
        } else {
            b = b.with_size(Size::Medium);
        }

        if self.is_disabled {
            b = b.disabled(true);
        }

        if let Some(handler) = self.on_click {
            b = b.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        b
    }
}

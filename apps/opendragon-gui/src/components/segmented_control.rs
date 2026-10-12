use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;
use gpui_kit::component::button::{Button as KitButton, ButtonGroup, ButtonVariants};
use gpui_kit::component::Sizable;

use crate::theme::Theme;

pub type SegmentSelectHandler<T> = Arc<dyn Fn(T, &mut Window, &mut App) + 'static>;

pub struct SegmentItem<T: Clone + PartialEq + 'static> {
    pub value: T,
    pub label: SharedString,
}

/// Declarative segmented picker control backed by native gpui-kit ButtonGroup
pub struct SegmentedControl<T: Clone + PartialEq + 'static> {
    items: Vec<SegmentItem<T>>,
    selected: T,
    on_select: Option<SegmentSelectHandler<T>>,
}

impl<T: Clone + PartialEq + 'static> SegmentedControl<T> {
    pub fn new(items: impl IntoIterator<Item = (T, impl Into<SharedString>)>, selected: T) -> Self {
        Self {
            items: items
                .into_iter()
                .map(|(v, l)| SegmentItem {
                    value: v,
                    label: l.into(),
                })
                .collect(),
            selected,
            on_select: None,
        }
    }

    pub fn on_select(mut self, handler: impl Fn(T, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Arc::new(handler));
        self
    }

    pub fn render(self, _theme: &Theme) -> impl IntoElement {
        let mut group = ButtonGroup::new("segmented_ctrl")
            .compact()
            .with_size(gpui_kit::component::Size::Small);

        for item in self.items {
            let is_active = item.value == self.selected;
            let val = item.value.clone();
            let on_sel = self.on_select.clone();

            let mut b = KitButton::new(item.label.clone()).label(item.label);
            if is_active {
                b = b.primary();
            } else {
                b = b.secondary();
            }

            if let Some(cb) = on_sel {
                b = b.on_click(move |_, w, a| {
                    cb(val.clone(), w, a);
                });
            }

            group = group.child(b);
        }

        group
    }
}

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use crate::theme::{
    Theme, FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_SM, RADIUS_MD, RADIUS_SM, SPACE_1, SPACE_2,
    SPACE_3,
};

pub type SliderChangeHandler = Arc<dyn Fn(f32, &mut Window, &mut App) + 'static>;

pub struct Slider {
    label: SharedString,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    unit: &'static str,
    show_stepper: bool,
    presets: Vec<(f32, SharedString)>,
    on_change: Option<SliderChangeHandler>,
}

impl Slider {
    pub fn new(label: impl Into<SharedString>, value: f32, min: f32, max: f32) -> Self {
        Self {
            label: label.into(),
            value: value.clamp(min, max),
            min,
            max,
            step: 0.1,
            unit: "",
            show_stepper: true,
            presets: Vec::new(),
            on_change: None,
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn unit(mut self, unit: &'static str) -> Self {
        self.unit = unit;
        self
    }

    pub fn stepper(mut self, show: bool) -> Self {
        self.show_stepper = show;
        self
    }

    pub fn presets(
        mut self,
        presets: impl IntoIterator<Item = (f32, impl Into<SharedString>)>,
    ) -> Self {
        self.presets = presets.into_iter().map(|(v, l)| (v, l.into())).collect();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Arc::new(handler));
        self
    }

    pub fn render(self, theme: &Theme) -> Div {
        let percent = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        let val_text = if self.step < 1.0 {
            format!("{:.1} {}", self.value, self.unit)
        } else {
            format!("{:.0} {}", self.value, self.unit)
        };

        let min_val = self.min;
        let max_val = self.max;
        let step_val = self.step;
        let cur_val = self.value;

        // Container
        let mut container = div()
            .flex()
            .flex_col()
            .gap(SPACE_2)
            .w_full()
            .p(SPACE_3)
            .rounded(RADIUS_MD)
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border);

        // Header with label and value
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_color(theme.foreground)
                    .text_size(FONT_SIZE_BODY)
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.label),
            )
            .child(
                div()
                    .text_color(theme.accent)
                    .text_size(FONT_SIZE_SM)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(val_text),
            );

        container = container.child(header);

        // Slider track
        let track_bg = theme.surface_active;
        let active_bg = theme.accent;

        let mut track_wrapper = div().flex().flex_row().items_center().w_full().gap(SPACE_2);

        if self.show_stepper {
            let on_change_clone = self.on_change.clone();
            let decr_val = (cur_val - step_val).max(min_val);
            let minus_btn = div()
                .size(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(RADIUS_SM)
                .bg(theme.surface_hover)
                .hover(|s| s.bg(theme.surface_active))
                .text_color(theme.foreground)
                .text_size(FONT_SIZE_SM)
                .font_weight(FontWeight::BOLD)
                .cursor_pointer()
                .child("-");

            let minus_btn = if let Some(cb) = on_change_clone {
                minus_btn.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    cb(decr_val, window, cx);
                })
            } else {
                minus_btn
            };

            track_wrapper = track_wrapper.child(minus_btn);
        }

        // Interactive Track
        const NUM_SEGMENTS: usize = 30;
        let mut segments_row = div().absolute().inset_0().flex().flex_row().items_center();

        for i in 0..NUM_SEGMENTS {
            let frac = i as f32 / (NUM_SEGMENTS - 1) as f32;
            let raw_val = min_val + frac * (max_val - min_val);
            let steps = ((raw_val - min_val) / step_val).round();
            let seg_val = (min_val + steps * step_val).clamp(min_val, max_val);

            let mut seg = div().flex_1().h_full().cursor_pointer();

            if let Some(ref cb) = self.on_change {
                let cb1 = cb.clone();
                let cb2 = cb.clone();
                seg = seg
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        cb1(seg_val, window, cx);
                    })
                    .on_mouse_move(move |event, window, cx| {
                        if event.pressed_button == Some(MouseButton::Left) {
                            cb2(seg_val, window, cx);
                        }
                    });
            }

            segments_row = segments_row.child(seg);
        }

        let thumb_left = (percent * 0.95).max(0.0);

        let track = div()
            .flex_1()
            .h(px(20.0))
            .flex()
            .items_center()
            .relative()
            .child(
                // Visual track bar
                div()
                    .w_full()
                    .h(px(4.0))
                    .rounded(RADIUS_SM)
                    .bg(track_bg)
                    .relative()
                    .child(
                        // Filled portion
                        div()
                            .h_full()
                            .w(relative(percent))
                            .rounded(RADIUS_SM)
                            .bg(active_bg),
                    )
                    .child(
                        // Draggable thumb (14px with 3px radius)
                        div()
                            .absolute()
                            .top(px(-5.0))
                            .left(relative(thumb_left))
                            .size(px(14.0))
                            .rounded(RADIUS_SM)
                            .bg(if theme.is_dark {
                                hsla(0.0, 0.0, 0.95, 1.0)
                            } else {
                                hsla(0.0, 0.0, 1.0, 1.0)
                            })
                            .border_2()
                            .border_color(theme.accent),
                    ),
            )
            .child(segments_row);

        track_wrapper = track_wrapper.child(track);

        if self.show_stepper {
            let on_change_clone2 = self.on_change.clone();
            let incr_val = (cur_val + step_val).min(max_val);
            let plus_btn = div()
                .size(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(RADIUS_SM)
                .bg(theme.surface_hover)
                .hover(|s| s.bg(theme.surface_active))
                .text_color(theme.foreground)
                .text_size(FONT_SIZE_SM)
                .font_weight(FontWeight::BOLD)
                .cursor_pointer()
                .child("+");

            let plus_btn = if let Some(cb) = on_change_clone2 {
                plus_btn.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    cb(incr_val, window, cx);
                })
            } else {
                plus_btn
            };

            track_wrapper = track_wrapper.child(plus_btn);
        }

        container = container.child(track_wrapper);

        // Quick Presets Pills Row (if provided)
        if !self.presets.is_empty() {
            let mut preset_row = div().flex().flex_row().gap(SPACE_2).flex_wrap();
            for (p_val, p_label) in self.presets {
                let is_selected = (cur_val - p_val).abs() < (step_val / 2.0).max(0.01);
                let on_change_p = self.on_change.clone();

                let pill = div()
                    .px(SPACE_2)
                    .py(SPACE_1)
                    .rounded(RADIUS_SM)
                    .bg(if is_selected {
                        theme.accent
                    } else {
                        theme.surface_hover
                    })
                    .hover(move |s| {
                        if !is_selected {
                            s.bg(theme.surface_active)
                        } else {
                            s
                        }
                    })
                    .text_color(if is_selected {
                        hsla(0.0, 0.0, 1.0, 1.0)
                    } else {
                        theme.foreground_muted
                    })
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(if is_selected {
                        FontWeight::SEMIBOLD
                    } else {
                        FontWeight::NORMAL
                    })
                    .cursor_pointer();

                let pill = if let Some(cb) = on_change_p {
                    pill.on_mouse_down(MouseButton::Left, move |_e, window, cx| {
                        cb(p_val, window, cx);
                    })
                } else {
                    pill
                };

                preset_row = preset_row.child(pill.child(p_label));
            }

            container = container.child(preset_row);
        } else {
            // Min / Max markers if no presets
            let markers = div()
                .flex()
                .flex_row()
                .justify_between()
                .w_full()
                .px(SPACE_1)
                .child(
                    div()
                        .text_color(theme.foreground_subtle)
                        .text_size(FONT_SIZE_CAPTION)
                        .child(format!("{:.1} {}", min_val, self.unit)),
                )
                .child(
                    div()
                        .text_color(theme.foreground_subtle)
                        .text_size(FONT_SIZE_CAPTION)
                        .child(format!("{:.1} {}", max_val, self.unit)),
                );

            container = container.child(markers);
        }

        container
    }
}

use gpui::prelude::*;
use gpui::*;

use crate::state::KeyboardVisualState;
use crate::theme::{Theme, RADIUS_MD, RADIUS_SM, SPACE_2, SPACE_3};

#[derive(Debug, Clone)]
struct KeyDef {
    id: u8,
    label: &'static str,
    width_units: f32,
}

impl KeyDef {
    fn new(id: u8, label: &'static str, width_units: f32) -> Self {
        Self {
            id,
            label,
            width_units,
        }
    }
}

pub type KeyClickHandler = std::sync::Arc<dyn Fn(u8, &mut Window, &mut App) + 'static>;

pub struct KeyboardPreview;

fn rgb_to_hsla(color: opendragon_core::color::RgbColor, alpha: f32) -> Hsla {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < 1e-5 {
        return hsla(0.0, 0.0, l, alpha);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < 1e-5 {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < 1e-5 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    hsla(h, s, l, alpha)
}

impl KeyboardPreview {
    fn tkl_layout() -> Vec<Vec<Option<KeyDef>>> {
        vec![
            // Row 0: Function Row
            vec![
                Some(KeyDef::new(1, "Esc", 1.0)),
                None, // Gap
                Some(KeyDef::new(2, "F1", 1.0)),
                Some(KeyDef::new(3, "F2", 1.0)),
                Some(KeyDef::new(4, "F3", 1.0)),
                Some(KeyDef::new(5, "F4", 1.0)),
                None, // Gap
                Some(KeyDef::new(6, "F5", 1.0)),
                Some(KeyDef::new(7, "F6", 1.0)),
                Some(KeyDef::new(8, "F7", 1.0)),
                Some(KeyDef::new(9, "F8", 1.0)),
                None, // Gap
                Some(KeyDef::new(10, "F9", 1.0)),
                Some(KeyDef::new(11, "F10", 1.0)),
                Some(KeyDef::new(12, "F11", 1.0)),
                Some(KeyDef::new(13, "F12", 1.0)),
                None, // Gap to Nav
                Some(KeyDef::new(14, "Prt", 1.0)),
                Some(KeyDef::new(15, "Scr", 1.0)),
                Some(KeyDef::new(16, "Pau", 1.0)),
            ],
            // Row 1: Numbers
            vec![
                Some(KeyDef::new(17, "~", 1.0)),
                Some(KeyDef::new(18, "1", 1.0)),
                Some(KeyDef::new(19, "2", 1.0)),
                Some(KeyDef::new(20, "3", 1.0)),
                Some(KeyDef::new(21, "4", 1.0)),
                Some(KeyDef::new(22, "5", 1.0)),
                Some(KeyDef::new(23, "6", 1.0)),
                Some(KeyDef::new(24, "7", 1.0)),
                Some(KeyDef::new(25, "8", 1.0)),
                Some(KeyDef::new(26, "9", 1.0)),
                Some(KeyDef::new(27, "0", 1.0)),
                Some(KeyDef::new(28, "-", 1.0)),
                Some(KeyDef::new(29, "=", 1.0)),
                Some(KeyDef::new(30, "Bksp", 2.0)),
                None,
                Some(KeyDef::new(31, "Ins", 1.0)),
                Some(KeyDef::new(32, "Hom", 1.0)),
                Some(KeyDef::new(33, "PgU", 1.0)),
            ],
            // Row 2: QWERTY
            vec![
                Some(KeyDef::new(34, "Tab", 1.5)),
                Some(KeyDef::new(35, "Q", 1.0)),
                Some(KeyDef::new(36, "W", 1.0)),
                Some(KeyDef::new(37, "E", 1.0)),
                Some(KeyDef::new(38, "R", 1.0)),
                Some(KeyDef::new(39, "T", 1.0)),
                Some(KeyDef::new(40, "Y", 1.0)),
                Some(KeyDef::new(41, "U", 1.0)),
                Some(KeyDef::new(42, "I", 1.0)),
                Some(KeyDef::new(43, "O", 1.0)),
                Some(KeyDef::new(44, "P", 1.0)),
                Some(KeyDef::new(45, "[", 1.0)),
                Some(KeyDef::new(46, "]", 1.0)),
                Some(KeyDef::new(47, "\\", 1.5)),
                None,
                Some(KeyDef::new(48, "Del", 1.0)),
                Some(KeyDef::new(49, "End", 1.0)),
                Some(KeyDef::new(50, "PgD", 1.0)),
            ],
            // Row 3: Home Row
            vec![
                Some(KeyDef::new(51, "Caps", 1.75)),
                Some(KeyDef::new(52, "A", 1.0)),
                Some(KeyDef::new(53, "S", 1.0)),
                Some(KeyDef::new(54, "D", 1.0)),
                Some(KeyDef::new(55, "F", 1.0)),
                Some(KeyDef::new(56, "G", 1.0)),
                Some(KeyDef::new(57, "H", 1.0)),
                Some(KeyDef::new(58, "J", 1.0)),
                Some(KeyDef::new(59, "K", 1.0)),
                Some(KeyDef::new(60, "L", 1.0)),
                Some(KeyDef::new(61, ";", 1.0)),
                Some(KeyDef::new(62, "'", 1.0)),
                Some(KeyDef::new(63, "Enter", 2.25)),
            ],
            // Row 4: Shift Row
            vec![
                Some(KeyDef::new(64, "Shift", 2.25)),
                Some(KeyDef::new(65, "Z", 1.0)),
                Some(KeyDef::new(66, "X", 1.0)),
                Some(KeyDef::new(67, "C", 1.0)),
                Some(KeyDef::new(68, "V", 1.0)),
                Some(KeyDef::new(69, "B", 1.0)),
                Some(KeyDef::new(70, "N", 1.0)),
                Some(KeyDef::new(71, "M", 1.0)),
                Some(KeyDef::new(72, ",", 1.0)),
                Some(KeyDef::new(73, ".", 1.0)),
                Some(KeyDef::new(74, "/", 1.0)),
                Some(KeyDef::new(75, "Shift", 2.75)),
                None,
                None,
                Some(KeyDef::new(76, "▲", 1.0)),
                None,
            ],
            // Row 5: Bottom Row
            vec![
                Some(KeyDef::new(77, "Ctrl", 1.25)),
                Some(KeyDef::new(78, "Win", 1.25)),
                Some(KeyDef::new(79, "Alt", 1.25)),
                Some(KeyDef::new(80, "Space", 6.25)),
                Some(KeyDef::new(81, "Alt", 1.25)),
                Some(KeyDef::new(82, "Fn", 1.25)),
                Some(KeyDef::new(83, "Menu", 1.25)),
                Some(KeyDef::new(84, "Ctrl", 1.25)),
                None,
                Some(KeyDef::new(85, "◀", 1.0)),
                Some(KeyDef::new(86, "▼", 1.0)),
                Some(KeyDef::new(87, "▶", 1.0)),
            ],
        ]
    }

    pub fn render(
        visual_state: &KeyboardVisualState,
        theme: &Theme,
        on_key_click: Option<KeyClickHandler>,
        on_key_drag: Option<KeyClickHandler>,
    ) -> Div {
        let base_unit = 36.0;
        let global_rgb = visual_state.global_lighting.color;
        let global_hsla = rgb_to_hsla(global_rgb, 0.85);

        let layout = Self::tkl_layout();

        let mut keyboard_chassis = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(SPACE_3)
            .rounded(RADIUS_MD)
            .bg(theme.surface_elevated)
            .border_1()
            .border_color(theme.border);

        for row in layout {
            let mut row_el = div().flex().flex_row().items_center().gap(px(4.0));

            for maybe_key in row {
                match maybe_key {
                    Some(key) => {
                        let key_w = px(key.width_units * base_unit + (key.width_units - 1.0) * 4.0);
                        let is_selected = visual_state.selected_keys.contains(&key.id);

                        let key_color =
                            if let Some(custom) = visual_state.per_key_colors.get(&key.id) {
                                rgb_to_hsla(*custom, 0.9)
                            } else {
                                global_hsla
                            };

                        let key_bg = if is_selected {
                            theme.surface_active
                        } else if theme.is_dark {
                            theme.surface
                        } else {
                            hsla(0.0, 0.0, 1.0, 1.0)
                        };

                        let mut key_el = div()
                            .w(key_w)
                            .h(px(34.0))
                            .rounded(RADIUS_SM)
                            .bg(key_bg)
                            .border_1()
                            .border_color(if is_selected {
                                theme.accent
                            } else {
                                theme.border_subtle
                            })
                            .flex()
                            .flex_col()
                            .justify_between()
                            .p(px(2.0))
                            .cursor_pointer()
                            .child(
                                // RGB backlight LED indicator bar
                                div().w_full().h(px(2.0)).rounded(RADIUS_SM).bg(key_color),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.foreground)
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(key.label),
                            );

                        let kid = key.id;
                        if let Some(handler) = &on_key_click {
                            let cb = handler.clone();
                            key_el = key_el.on_mouse_down(MouseButton::Left, move |_e, w, a| {
                                cb(kid, w, a);
                            });
                        }

                        if let Some(drag_handler) = &on_key_drag {
                            let cb_drag = drag_handler.clone();
                            key_el = key_el.on_mouse_move(move |e, w, a| {
                                if e.pressed_button == Some(MouseButton::Left) {
                                    cb_drag(kid, w, a);
                                }
                            });
                        }

                        row_el = row_el.child(key_el);
                    }
                    None => {
                        // Empty spacer slot (e.g. between F-row groups or nav cluster)
                        row_el = row_el.child(div().w(px(12.0)).h(px(34.0)));
                    }
                }
            }

            keyboard_chassis = keyboard_chassis.child(row_el);
        }

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .p(SPACE_2)
            .child(keyboard_chassis)
    }
}

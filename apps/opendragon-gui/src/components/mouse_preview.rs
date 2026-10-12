use gpui::prelude::*;
use gpui::*;

use crate::state::KeyboardVisualState;
use crate::theme::{Theme, RADIUS_MD, RADIUS_SM, SPACE_2, SPACE_3, SPACE_4};

pub type MouseButtonClickHandler = std::sync::Arc<dyn Fn(u8, &mut Window, &mut App) + 'static>;

pub struct MousePreview;

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

impl MousePreview {
    pub fn render(
        visual_state: &KeyboardVisualState,
        dpi: u16,
        theme: &Theme,
        on_button_click: Option<MouseButtonClickHandler>,
    ) -> Div {
        let global_rgb = visual_state.global_lighting.color;
        let led_hsla = rgb_to_hsla(global_rgb, 0.95);
        let diffuse_glow = rgb_to_hsla(global_rgb, 0.25);

        // Calculate DPI stage index (1 to 5)
        let dpi_stage = if dpi <= 800 {
            1
        } else if dpi <= 1600 {
            2
        } else if dpi <= 2400 {
            3
        } else if dpi <= 3200 {
            4
        } else {
            5
        };

        let is_btn_selected = |btn_id: u8| visual_state.selected_keys.contains(&btn_id);

        let make_click_handler = |btn_id: u8| {
            let handler = on_button_click.clone();
            move |_e: &MouseDownEvent, w: &mut Window, a: &mut App| {
                if let Some(h) = &handler {
                    h(btn_id, w, a);
                }
            }
        };

        // Left Click Button (ID 1)
        let is_left_sel = is_btn_selected(1);
        let left_btn = div()
            .w(px(80.0))
            .h(px(100.0))
            .rounded_tl(px(24.0))
            .rounded_tr(RADIUS_SM)
            .rounded_bl(RADIUS_SM)
            .rounded_br(RADIUS_SM)
            .bg(if is_left_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_left_sel {
                theme.accent
            } else {
                theme.border
            })
            .cursor_pointer()
            .p(SPACE_2)
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .w(px(28.0))
                    .h(px(2.0))
                    .rounded(RADIUS_SM)
                    .bg(if is_left_sel {
                        theme.accent
                    } else {
                        theme.border_subtle
                    }),
            )
            .child(
                div()
                    .text_color(if is_left_sel {
                        theme.accent
                    } else {
                        theme.foreground_muted
                    })
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("M1 Left"),
            )
            .on_mouse_down(MouseButton::Left, make_click_handler(1));

        // Right Click Button (ID 2)
        let is_right_sel = is_btn_selected(2);
        let right_btn = div()
            .w(px(80.0))
            .h(px(100.0))
            .rounded_tr(px(24.0))
            .rounded_tl(RADIUS_SM)
            .rounded_bl(RADIUS_SM)
            .rounded_br(RADIUS_SM)
            .bg(if is_right_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_right_sel {
                theme.accent
            } else {
                theme.border
            })
            .cursor_pointer()
            .p(SPACE_2)
            .flex()
            .flex_col()
            .justify_between()
            .items_end()
            .child(
                div()
                    .w(px(28.0))
                    .h(px(2.0))
                    .rounded(RADIUS_SM)
                    .bg(if is_right_sel {
                        theme.accent
                    } else {
                        theme.border_subtle
                    }),
            )
            .child(
                div()
                    .text_color(if is_right_sel {
                        theme.accent
                    } else {
                        theme.foreground_muted
                    })
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("M2 Right"),
            )
            .on_mouse_down(MouseButton::Left, make_click_handler(2));

        // Middle Spine: Scroll Wheel (ID 3) + DPI Switches (IDs 4 & 5)
        let is_wheel_sel = is_btn_selected(3);
        let scroll_wheel = div()
            .w(px(32.0))
            .h(px(52.0))
            .rounded(px(16.0))
            .bg(if is_wheel_sel {
                theme.surface_active
            } else {
                theme.surface_elevated
            })
            .border_2()
            .border_color(if is_wheel_sel {
                theme.accent
            } else {
                theme.border
            })
            .cursor_pointer()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(3.0))
            .child(
                // Illuminated RGB lightpipe along the center of the scroll wheel
                div().w(px(4.0)).h(px(34.0)).rounded(RADIUS_SM).bg(led_hsla),
            )
            .on_mouse_down(MouseButton::Left, make_click_handler(3));

        let is_dpi_up_sel = is_btn_selected(4);
        let dpi_up_btn = div()
            .w(px(26.0))
            .h(px(18.0))
            .rounded(RADIUS_SM)
            .bg(if is_dpi_up_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_dpi_up_sel {
                theme.accent
            } else {
                theme.border_subtle
            })
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .text_color(if is_dpi_up_sel {
                theme.accent
            } else {
                theme.foreground_muted
            })
            .text_size(px(9.0))
            .font_weight(FontWeight::BOLD)
            .child("▲")
            .on_mouse_down(MouseButton::Left, make_click_handler(4));

        let is_dpi_down_sel = is_btn_selected(5);
        let dpi_down_btn = div()
            .w(px(26.0))
            .h(px(18.0))
            .rounded(RADIUS_SM)
            .bg(if is_dpi_down_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_dpi_down_sel {
                theme.accent
            } else {
                theme.border_subtle
            })
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .text_color(if is_dpi_down_sel {
                theme.accent
            } else {
                theme.foreground_muted
            })
            .text_size(px(9.0))
            .font_weight(FontWeight::BOLD)
            .child("▼")
            .on_mouse_down(MouseButton::Left, make_click_handler(5));

        let center_spine = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(6.0))
            .child(scroll_wheel)
            .child(dpi_up_btn)
            .child(dpi_down_btn);

        let top_cluster = div()
            .flex()
            .flex_row()
            .items_start()
            .justify_center()
            .gap(px(8.0))
            .child(left_btn)
            .child(center_spine)
            .child(right_btn);

        // Ergonomic Side Thumb Rest (IDs 6 & 7)
        let is_side_fwd_sel = is_btn_selected(6);
        let side_fwd_btn = div()
            .w(px(22.0))
            .h(px(32.0))
            .rounded_tl(px(8.0))
            .rounded_bl(RADIUS_SM)
            .bg(if is_side_fwd_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_side_fwd_sel {
                theme.accent
            } else {
                theme.border
            })
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .text_color(if is_side_fwd_sel {
                theme.accent
            } else {
                theme.foreground_muted
            })
            .text_size(px(9.0))
            .font_weight(FontWeight::BOLD)
            .child("M5")
            .on_mouse_down(MouseButton::Left, make_click_handler(6));

        let is_side_back_sel = is_btn_selected(7);
        let side_back_btn = div()
            .w(px(22.0))
            .h(px(32.0))
            .rounded_tl(RADIUS_SM)
            .rounded_bl(px(8.0))
            .bg(if is_side_back_sel {
                theme.surface_active
            } else {
                theme.surface
            })
            .border_1()
            .border_color(if is_side_back_sel {
                theme.accent
            } else {
                theme.border
            })
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .text_color(if is_side_back_sel {
                theme.accent
            } else {
                theme.foreground_muted
            })
            .text_size(px(9.0))
            .font_weight(FontWeight::BOLD)
            .child("M4")
            .on_mouse_down(MouseButton::Left, make_click_handler(7));

        let thumb_wing = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(side_fwd_btn)
            .child(side_back_btn);

        // Palm Rest with illuminated Redragon emblem and DPI stage indicator
        let mut dpi_dots = div().flex().flex_row().items_center().gap(px(4.0));
        for i in 1..=5 {
            let active = i <= dpi_stage;
            dpi_dots = dpi_dots.child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(if active {
                led_hsla
            } else {
                theme.border_subtle
            }));
        }

        let palm_rest = div()
            .w(px(180.0))
            .h(px(140.0))
            .rounded_b(px(36.0))
            .rounded_t(RADIUS_MD)
            .bg(theme.surface)
            .border_1()
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .items_center()
            .justify_between()
            .p(SPACE_3)
            .child(
                // DPI stage micro-LED strip
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_color(theme.foreground_subtle)
                            .text_size(px(8.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("DPI STAGE"),
                    )
                    .child(dpi_dots),
            )
            .child(
                // Illuminated Redragon emblem with diffuse halo
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .w(px(64.0))
                    .h(px(64.0))
                    .rounded_full()
                    .bg(diffuse_glow)
                    .border_1()
                    .border_color(led_hsla)
                    .child(div().text_size(px(22.0)).text_color(led_hsla).child("🐲")),
            )
            .child(
                div()
                    .text_color(theme.foreground_subtle)
                    .text_size(px(9.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(format!("{dpi} DPI")),
            );

        // Assembled chassis with flared thumb wing and underglow rim
        let chassis = div()
            .flex()
            .flex_col()
            .items_center()
            .p(SPACE_4)
            .rounded(px(40.0))
            .bg(theme.surface_elevated)
            .border_2()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(thumb_wing)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(SPACE_2)
                            .child(top_cluster)
                            .child(palm_rest),
                    ),
            )
            .child(
                // Underglow diffused light pipe rim
                div()
                    .mt(SPACE_2)
                    .w(px(140.0))
                    .h(px(3.0))
                    .rounded(RADIUS_SM)
                    .bg(led_hsla),
            );

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .p(SPACE_3)
            .child(chassis)
    }
}

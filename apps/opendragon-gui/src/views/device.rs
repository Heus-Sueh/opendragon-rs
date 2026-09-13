use gpui::prelude::*;
use gpui::*;

use crate::components::{
    BadgeStatus, Card, InfoTooltip, KeyboardPreview, Slider, StatusBadge, Switch,
};
use crate::state::{AppState, Page};
use crate::theme::{
    FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_MD,
    RADIUS_SM, SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_6,
};

pub struct DeviceOverview;

impl DeviceOverview {
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        state: &AppState,
        on_navigate: impl Fn(Page, &mut Window, &mut App) + 'static + Clone,
        on_brightness_change: impl Fn(u8, &mut Window, &mut App) + 'static + Clone,
        on_rapid_trigger_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
        on_key_click: impl Fn(u8, &mut Window, &mut App) + 'static + Clone,
        on_key_drag: impl Fn(u8, &mut Window, &mut App) + 'static + Clone,
        on_clear_selection: impl Fn(&mut Window, &mut App) + 'static + Clone,
        on_dpi_change: impl Fn(u16, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => {
                return div().child("No active device selected");
            }
        };

        let is_mouse = active
            .summary
            .capabilities
            .contains(&opendragon_core::capability::Capability::Dpi)
            && !active
                .summary
                .capabilities
                .contains(&opendragon_core::capability::Capability::Actuation);

        // Header: Marketing Name, Model, Connected Badge
        let header = div()
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
                            .text_size(FONT_SIZE_TITLE)
                            .font_weight(FontWeight::BOLD)
                            .child(active.summary.name.clone()),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child(active.summary.model.clone()),
                    ),
            )
            .child(
                StatusBadge::new(if active.summary.is_connected {
                    BadgeStatus::Connected
                } else {
                    BadgeStatus::Disconnected
                })
                .render(theme),
            );

        // Center preview: Mouse chassis or Keyboard with interactive switch clicks
        let preview_section = if is_mouse {
            let on_dpi = on_dpi_change.clone();
            let mouse_dpi_slider =
                Slider::new("Sensor DPI Sensitivity", active.dpi as f32, 200.0, 10000.0)
                    .step(50.0)
                    .unit(" DPI")
                    .presets([
                        (800.0, "800"),
                        (1600.0, "1600"),
                        (2400.0, "2400"),
                        (3200.0, "3200"),
                        (5000.0, "5000"),
                    ])
                    .on_change(move |val, w, a| {
                        on_dpi(val as u16, w, a);
                    })
                    .render(theme);

            div()
                .flex()
                .flex_col()
                .gap(SPACE_4)
                .w_full()
                .p(SPACE_4)
                .rounded(RADIUS_MD)
                .bg(theme.surface_elevated)
                .border_1()
                .border_color(theme.border)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(SPACE_3)
                                .child(div().text_size(px(24.0)).child("🖱️"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_color(theme.foreground)
                                                .text_size(FONT_SIZE_BODY)
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child(format!("{} Gaming Mouse", active.summary.name)),
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.foreground_muted)
                                                .text_size(FONT_SIZE_SM)
                                                .child("High-precision optical gaming sensor with multi-stage DPI resolution"),
                                        ),
                                ),
                        )
                        .child(
                            StatusBadge::new(BadgeStatus::Connected)
                                .label(format!("{} DPI", active.dpi))
                                .render(theme),
                        ),
                )
                .child(mouse_dpi_slider)
        } else {
            let on_kc = on_key_click.clone();
            let key_click_handler =
                std::sync::Arc::new(move |key_id: u8, w: &mut Window, a: &mut App| {
                    on_kc(key_id, w, a);
                });

            let on_kd = on_key_drag.clone();
            let key_drag_handler =
                std::sync::Arc::new(move |key_id: u8, w: &mut Window, a: &mut App| {
                    on_kd(key_id, w, a);
                });

            let preview = KeyboardPreview::render(
                &active.visual_state,
                theme,
                Some(key_click_handler),
                Some(key_drag_handler),
            );

            // Switch Inspector card when keys are clicked/selected
            let selected_keys = &active.visual_state.selected_keys;
            let switch_inspector = if !selected_keys.is_empty() {
                let on_clr = on_clear_selection.clone();
                let selected_labels = selected_keys
                    .iter()
                    .map(|id| format!("{} (#{id})", key_name_from_id(*id)))
                    .collect::<Vec<_>>()
                    .join(", ");

                Some(
                    Card::new()
                        .child(
                            div()
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
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(SPACE_2)
                                                .child(
                                                    div()
                                                        .text_color(theme.accent)
                                                        .text_size(FONT_SIZE_BODY)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child("Selected Key:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_color(theme.foreground)
                                                        .text_size(FONT_SIZE_BODY)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(selected_labels),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.foreground_muted)
                                                .text_size(FONT_SIZE_SM)
                                                .child(format!(
                                                    "Hall Effect Magnetic Switch • Actuation Point: {:.1} mm • Calibrated",
                                                    active.actuation.mm()
                                                )),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(SPACE_2)
                                        .child(InfoTooltip::new("Mouse Drag Enabled").render(theme))
                                        .child(
                                            div()
                                                .px(SPACE_2)
                                                .py(SPACE_1)
                                                .rounded(RADIUS_SM)
                                                .bg(theme.surface_hover)
                                                .border_1()
                                                .border_color(theme.border)
                                                .text_color(theme.foreground)
                                                .text_size(FONT_SIZE_CAPTION)
                                                .cursor_pointer()
                                                .on_mouse_down(MouseButton::Left, move |_e, w, a| {
                                                    on_clr(w, a);
                                                })
                                                .child("Deselect"),
                                        ),
                                ),
                        )
                        .render(theme),
                )
            } else {
                None
            };

            let mut kb_block = div().flex().flex_col().gap(SPACE_3).w_full().child(preview);
            if let Some(inspector) = switch_inspector {
                kb_block = kb_block.child(inspector);
            }
            kb_block
        };

        // Quick Settings Section
        let quick_header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .child(
                div()
                    .text_color(theme.foreground)
                    .text_size(FONT_SIZE_HEADING)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Quick Adjustments"),
            );

        // Brightness Quick Control
        let on_bright = on_brightness_change.clone();
        let brightness_slider = Slider::new(
            "Lighting Brightness",
            active.lighting.brightness as f32,
            0.0,
            100.0,
        )
        .step(5.0)
        .unit("%")
        .presets([
            (0.0, "0%"),
            (25.0, "25%"),
            (50.0, "50%"),
            (75.0, "75%"),
            (100.0, "100%"),
        ])
        .on_change(move |val, w, a| {
            on_bright(val as u8, w, a);
        })
        .render(theme);

        // Rapid Trigger Quick Control
        let on_rt = on_rapid_trigger_toggle.clone();
        let rapid_trigger_switch = Switch::new("Rapid Trigger", active.rapid_trigger.enabled)
            .description("Responds immediately as the key changes direction")
            .on_toggle(move |enabled, w, a| {
                on_rt(enabled, w, a);
            })
            .render(theme);

        // Quick Cards Row: Active Profile, Actuation, More links
        let on_nav_prof = on_navigate.clone();
        let on_nav_perf = on_navigate.clone();
        let on_nav_light = on_navigate.clone();

        let active_prof_name = active
            .profiles
            .iter()
            .find(|p| p.is_active)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Default".to_string());

        let quick_cards = div()
            .flex()
            .flex_row()
            .gap(SPACE_3)
            .w_full()
            .child(
                Card::new()
                    .interactive(true)
                    .on_click(move |_e, w, a| on_nav_prof(Page::Profiles, w, a))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_1)
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Active Profile"),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_BODY)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(active_prof_name),
                            )
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Manage profiles →"),
                            ),
                    )
                    .render(theme),
            )
            .child(
                Card::new()
                    .interactive(true)
                    .on_click(move |_e, w, a| on_nav_perf(Page::Performance, w, a))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_1)
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Actuation Point"),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_BODY)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format!("{:.1} mm", active.actuation.mm())),
                            )
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Tune actuation →"),
                            ),
                    )
                    .render(theme),
            )
            .child(
                Card::new()
                    .interactive(true)
                    .on_click(move |_e, w, a| on_nav_light(Page::Lighting, w, a))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_1)
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Lighting Mode"),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_BODY)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format!("{}", active.lighting.mode)),
                            )
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Lighting effects →"),
                            ),
                    )
                    .render(theme),
            );

        let mut content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(preview_section)
            .child(quick_header)
            .child(brightness_slider);

        if !is_mouse {
            content = content.child(rapid_trigger_switch);
        }

        content = content.child(quick_cards);

        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .min_h_full()
            .p(SPACE_6)
            .pb(crate::theme::SPACE_12)
            .child(content)
    }
}

fn key_name_from_id(id: u8) -> &'static str {
    match id {
        1 => "Esc",
        2 => "F1",
        3 => "F2",
        4 => "F3",
        5 => "F4",
        6 => "F5",
        7 => "F6",
        8 => "F7",
        9 => "F8",
        10 => "F9",
        11 => "F10",
        12 => "F11",
        13 => "F12",
        14 => "PrtSc",
        15 => "ScrLk",
        16 => "Pause",
        17 => "~",
        18 => "1",
        19 => "2",
        20 => "3",
        21 => "4",
        22 => "5",
        23 => "6",
        24 => "7",
        25 => "8",
        26 => "9",
        27 => "0",
        28 => "-",
        29 => "=",
        30 => "Backspace",
        31 => "Insert",
        32 => "Home",
        33 => "Page Up",
        34 => "Tab",
        35 => "Q",
        36 => "W",
        37 => "E",
        38 => "R",
        39 => "T",
        40 => "Y",
        41 => "U",
        42 => "I",
        43 => "O",
        44 => "P",
        45 => "[",
        46 => "]",
        47 => "\\",
        48 => "Delete",
        49 => "End",
        50 => "Page Down",
        51 => "Caps Lock",
        52 => "A",
        53 => "S",
        54 => "D",
        55 => "F",
        56 => "G",
        57 => "H",
        58 => "J",
        59 => "K",
        60 => "L",
        61 => ";",
        62 => "'",
        63 => "Enter",
        64 => "Shift L",
        65 => "Z",
        66 => "X",
        67 => "C",
        68 => "V",
        69 => "B",
        70 => "N",
        71 => "M",
        72 => ",",
        73 => ".",
        74 => "/",
        75 => "Shift R",
        76 => "Up Arrow",
        77 => "Ctrl L",
        78 => "Win",
        79 => "Alt L",
        80 => "Space",
        81 => "Alt R",
        82 => "Fn",
        83 => "Menu",
        84 => "Ctrl R",
        85 => "Left Arrow",
        86 => "Down Arrow",
        87 => "Right Arrow",
        _ => "Switch",
    }
}

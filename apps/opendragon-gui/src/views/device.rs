use gpui::prelude::*;
use gpui::*;

use crate::components::{
    BadgeStatus, KeyboardPreview, MousePreview, Slider, StatusBadge, Switch, TelemetryWell,
};
use crate::state::{AppState, DeviceAction, Page};
use crate::theme::{
    FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_MD,
    RADIUS_SM, SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_6,
};
use opendragon_core::capability::Capability;
use opendragon_core::color::RgbColor;
use opendragon_core::{ActuationPoint, PollingRate};

pub struct DeviceOverview;

impl DeviceOverview {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => {
                return div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w_full()
                    .h_full()
                    .text_color(theme.foreground_muted)
                    .child("No device selected");
            }
        };

        let is_mouse = active.summary.capabilities.contains(&Capability::Dpi)
            && !active.summary.capabilities.contains(&Capability::Actuation);

        // Header: Device Title, Model, and Connection Status Badge
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .pb(SPACE_4)
            .border_b_1()
            .border_color(theme.border_subtle)
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
                            .child(if is_mouse {
                                format!(
                                    "Model {} • Optical Gaming Mouse • USB HID",
                                    active.summary.model
                                )
                            } else {
                                format!(
                                    "Model {} • Hall-Effect Magnetic Gaming Keyboard",
                                    active.summary.model
                                )
                            }),
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

        // Master Canvas: Keyboard or Mouse
        let hardware_canvas = if is_mouse {
            let on_act_btn = on_action.clone();
            let mouse_click_handler =
                std::sync::Arc::new(move |btn_id: u8, w: &mut Window, a: &mut App| {
                    on_act_btn(DeviceAction::ToggleKeySelection(btn_id), w, a);
                });

            MousePreview::render(
                &active.visual_state,
                active.dpi,
                theme,
                Some(mouse_click_handler),
            )
        } else {
            let on_act_kc = on_action.clone();
            let key_click_handler =
                std::sync::Arc::new(move |key_id: u8, w: &mut Window, a: &mut App| {
                    on_act_kc(DeviceAction::ToggleKeySelection(key_id), w, a);
                });

            let on_act_layout = on_action.clone();
            let layout_change_handler =
                std::sync::Arc::new(move |layout, w: &mut Window, a: &mut App| {
                    on_act_layout(DeviceAction::SetKeyboardLayout(layout), w, a);
                });

            KeyboardPreview::render(
                &active.visual_state,
                theme,
                state.preferences.keyboard_layout,
                Some(key_click_handler),
                Some(layout_change_handler),
            )
        };

        // Telemetry Wells: Edward Tufte High Data-Ink Ratio
        let active_prof_name = active
            .profiles
            .iter()
            .find(|p| p.is_active)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Default".to_string());

        let polling_well = TelemetryWell::new("Polling Rate", format!("{}", active.polling_rate))
            .detail("Response: 1.0 ms")
            .badge("USB HID")
            .render(theme);

        let profile_well = TelemetryWell::new("Active Profile", active_prof_name)
            .detail("Slot 1 Onboard")
            .badge("EEPROM")
            .render(theme);

        let primary_metric_well = if is_mouse {
            TelemetryWell::new("Sensor Sensitivity", format!("{} DPI", active.dpi))
                .detail("PixArt Optical Sensor")
                .accent_value(true)
                .render(theme)
        } else {
            TelemetryWell::new(
                "Actuation Travel",
                format!("{:.1} mm", active.actuation.mm()),
            )
            .detail(if active.rapid_trigger.enabled {
                "Rapid Trigger Active"
            } else {
                "Standard Hall-Effect"
            })
            .accent_value(true)
            .render(theme)
        };

        let lighting_well =
            TelemetryWell::new("Lighting Preset", format!("{}", active.lighting.mode))
                .detail(format!("Brightness {}%", active.lighting.brightness))
                .badge("RGB")
                .render(theme);

        let telemetry_row = div()
            .flex()
            .flex_row()
            .gap(SPACE_3)
            .w_full()
            .child(polling_well)
            .child(profile_well)
            .child(primary_metric_well)
            .child(lighting_well);

        // Master Column (Canvas + Telemetry)
        let master_column = div()
            .flex()
            .flex_col()
            .flex_1()
            .gap(SPACE_4)
            .min_w(px(520.0))
            .child(hardware_canvas)
            .child(telemetry_row);

        // Contextual Inspector Dock (Right Side)
        let selected_keys = &active.visual_state.selected_keys;
        let inspector_dock = if !selected_keys.is_empty() {
            let selected_label = if selected_keys.len() == 1 {
                let id = *selected_keys.iter().next().unwrap();
                format!("{} (#{})", target_name(id, is_mouse), id)
            } else {
                format!("{} Components Selected", selected_keys.len())
            };

            let mut dock = div()
                .flex()
                .flex_col()
                .gap(SPACE_4)
                .w(px(360.0))
                .p(SPACE_4)
                .rounded(RADIUS_MD)
                .bg(theme.surface_elevated)
                .border_1()
                .border_color(theme.border);

            // Inspector Header
            let on_act_clear = on_action.clone();
            dock = dock.child(
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
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_BODY)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(selected_label),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_CAPTION)
                                    .child("Per-component tuning"),
                            ),
                    )
                    .child(
                        div()
                            .px(SPACE_2)
                            .py(SPACE_1)
                            .rounded(RADIUS_SM)
                            .bg(theme.surface_hover)
                            .border_1()
                            .border_color(theme.border)
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_CAPTION)
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, move |_e, w, a| {
                                on_act_clear(DeviceAction::ClearKeySelection, w, a);
                            })
                            .child("Deselect (Esc)"),
                    ),
            );

            if is_mouse {
                // Mouse Button Inspector
                let on_act_dpi = on_action.clone();
                let dpi_slider =
                    Slider::new("Sensor DPI Resolution", active.dpi as f32, 200.0, 10000.0)
                        .flat(true)
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
                            on_act_dpi(DeviceAction::UpdateDpi(val as u16), w, a);
                        })
                        .render(theme);

                dock = dock.child(dpi_slider);

                // Polling Rate Presets
                let on_act_pr = on_action.clone();
                let polling_buttons = div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_2)
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_CAPTION)
                            .font_weight(FontWeight::MEDIUM)
                            .child("Sensor Polling Rate"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(SPACE_2)
                            .child(make_polling_pill(
                                "125 Hz",
                                active.polling_rate == PollingRate::Hz125,
                                {
                                    let on_act = on_act_pr.clone();
                                    move |w, a| {
                                        on_act(
                                            DeviceAction::UpdatePollingRate(PollingRate::Hz125),
                                            w,
                                            a,
                                        )
                                    }
                                },
                                theme,
                            ))
                            .child(make_polling_pill(
                                "250 Hz",
                                active.polling_rate == PollingRate::Hz250,
                                {
                                    let on_act = on_act_pr.clone();
                                    move |w, a| {
                                        on_act(
                                            DeviceAction::UpdatePollingRate(PollingRate::Hz250),
                                            w,
                                            a,
                                        )
                                    }
                                },
                                theme,
                            ))
                            .child(make_polling_pill(
                                "500 Hz",
                                active.polling_rate == PollingRate::Hz500,
                                {
                                    let on_act = on_act_pr.clone();
                                    move |w, a| {
                                        on_act(
                                            DeviceAction::UpdatePollingRate(PollingRate::Hz500),
                                            w,
                                            a,
                                        )
                                    }
                                },
                                theme,
                            ))
                            .child(make_polling_pill(
                                "1000 Hz",
                                active.polling_rate == PollingRate::Hz1000,
                                {
                                    let on_act = on_act_pr.clone();
                                    move |w, a| {
                                        on_act(
                                            DeviceAction::UpdatePollingRate(PollingRate::Hz1000),
                                            w,
                                            a,
                                        )
                                    }
                                },
                                theme,
                            )),
                    );

                dock = dock.child(polling_buttons);
            } else {
                // Keyboard Magnetic Switch Inspector
                let on_act_act = on_action.clone();
                let actuation_slider = Slider::new(
                    "Hall-Effect Actuation Point",
                    active.actuation.mm(),
                    0.1,
                    4.0,
                )
                .flat(true)
                .step(0.1)
                .unit(" mm")
                .presets([
                    (0.8, "0.8 (Speed)"),
                    (2.0, "2.0 (Standard)"),
                    (3.2, "3.2 (Deep)"),
                ])
                .on_change(move |val, w, a| {
                    if let Ok(pt) = ActuationPoint::new(val) {
                        on_act_act(DeviceAction::UpdateActuation(pt), w, a);
                    }
                })
                .render(theme);

                dock = dock.child(actuation_slider);

                // Hall-Effect Travel Depth Visual Scale Gauge
                let travel_percent = (active.actuation.mm() / 4.0).clamp(0.0, 1.0);
                let visual_gauge = div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_1)
                    .p(SPACE_3)
                    .rounded(RADIUS_SM)
                    .bg(theme.surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_CAPTION)
                            .child("0.0 mm (Rest)")
                            .child(format!("Trigger: {:.1} mm", active.actuation.mm()))
                            .child("4.0 mm (Bottom-out)"),
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(8.0))
                            .rounded_full()
                            .bg(theme.surface_hover)
                            .child(
                                div()
                                    .w(px(travel_percent * 320.0))
                                    .h_full()
                                    .rounded_full()
                                    .bg(theme.accent),
                            ),
                    );

                dock = dock.child(visual_gauge);

                // Rapid Trigger Toggle
                let on_act_rt = on_action.clone();
                let rt_switch = Switch::new("Rapid Trigger", active.rapid_trigger.enabled)
                    .flat(true)
                    .description("Immediate switch reset upon key upstroke")
                    .on_toggle(move |val, w, a| {
                        on_act_rt(DeviceAction::ToggleRapidTrigger(val), w, a);
                    })
                    .render(theme);

                dock = dock.child(rt_switch);
            }

            // Per-component RGB Quick Swatches
            let on_act_color = on_action.clone();
            let swatches = div()
                .flex()
                .flex_col()
                .gap(SPACE_2)
                .child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_CAPTION)
                        .font_weight(FontWeight::MEDIUM)
                        .child("Component Backlight Tint"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(SPACE_2)
                        .child(make_swatch_pill(RgbColor::new(239, 68, 68), "Red", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(239, 68, 68);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        }))
                        .child(make_swatch_pill(RgbColor::new(249, 115, 22), "Orange", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(249, 115, 22);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        }))
                        .child(make_swatch_pill(RgbColor::new(16, 185, 129), "Emerald", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(16, 185, 129);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        }))
                        .child(make_swatch_pill(RgbColor::new(6, 182, 212), "Cyan", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(6, 182, 212);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        }))
                        .child(make_swatch_pill(RgbColor::new(217, 70, 239), "Magenta", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(217, 70, 239);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        }))
                        .child(make_swatch_pill(RgbColor::new(255, 255, 255), "White", {
                            let on_act = on_act_color.clone();
                            let mut cfg = active.lighting.clone();
                            cfg.color = RgbColor::new(255, 255, 255);
                            move |w, a| on_act(DeviceAction::UpdateLighting(cfg.clone()), w, a)
                        })),
                );

            dock = dock.child(swatches);

            dock
        } else {
            // Default Idle Inspector: Quick Adjustments & Telemetry
            let mut dock = div()
                .flex()
                .flex_col()
                .gap(SPACE_4)
                .w(px(360.0))
                .p(SPACE_4)
                .rounded(RADIUS_MD)
                .bg(theme.surface_elevated)
                .border_1()
                .border_color(theme.border);

            dock = dock.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_HEADING)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Quick Controls"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Real-time hardware tuning"),
                    ),
            );

            // Lighting Brightness Slider
            let on_act_bright = on_action.clone();
            let brightness_slider = Slider::new(
                "Lighting Brightness",
                active.lighting.brightness as f32,
                0.0,
                100.0,
            )
            .flat(true)
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
                on_act_bright(DeviceAction::SetBrightness(val as u8), w, a);
            })
            .render(theme);

            dock = dock.child(brightness_slider);

            if !is_mouse {
                let on_act_rt = on_action.clone();
                let rt_switch = Switch::new("Rapid Trigger", active.rapid_trigger.enabled)
                    .flat(true)
                    .description("Dynamic reset for competitive gaming")
                    .on_toggle(move |val, w, a| {
                        on_act_rt(DeviceAction::ToggleRapidTrigger(val), w, a);
                    })
                    .render(theme);

                dock = dock.child(rt_switch);
            }

            // Quick Page Jumps
            let on_act_prof = on_action.clone();
            let on_act_perf = on_action.clone();
            let on_act_light = on_action.clone();

            let jump_links = div()
                .flex()
                .flex_col()
                .gap(SPACE_2)
                .child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_CAPTION)
                        .font_weight(FontWeight::MEDIUM)
                        .child("Configuration Modules"),
                )
                .child(make_nav_tile(
                    "Profiles & Memory",
                    "Switch onboard EEPROM profiles",
                    {
                        let on_act = on_act_prof.clone();
                        move |_e, w, a| on_act(DeviceAction::Navigate(Page::Profiles), w, a)
                    },
                    theme,
                ))
                .child(make_nav_tile(
                    if is_mouse {
                        "Sensor & DPI Matrix"
                    } else {
                        "Actuation & Performance"
                    },
                    "Calibrate triggers and curve dynamics",
                    {
                        let on_act = on_act_perf.clone();
                        move |_e, w, a| on_act(DeviceAction::Navigate(Page::Performance), w, a)
                    },
                    theme,
                ))
                .child(make_nav_tile(
                    "Lighting & RGB FX",
                    "Configure animations and color channels",
                    {
                        let on_act = on_act_light.clone();
                        move |_e, w, a| on_act(DeviceAction::Navigate(Page::Lighting), w, a)
                    },
                    theme,
                ));

            dock = dock.child(jump_links);

            // Guidance Hint
            dock = dock.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_1)
                    .p(SPACE_3)
                    .rounded(RADIUS_SM)
                    .bg(theme.surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(SPACE_1)
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_CAPTION)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("ℹ Interactive Inspector"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_subtle)
                            .text_size(FONT_SIZE_CAPTION)
                            .child("Click any key or button on the chassis to inspect magnetic switch travel or rebind."),
                    ),
            );

            dock
        };

        // Master-Detail Split Assembly
        let master_detail = div()
            .flex()
            .flex_row()
            .gap(SPACE_6)
            .w_full()
            .items_start()
            .child(master_column)
            .child(inspector_dock);

        let content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(master_detail);

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

fn make_polling_pill(
    label: &'static str,
    is_active: bool,
    on_click: impl Fn(&mut Window, &mut App) + 'static + Clone,
    theme: &crate::theme::Theme,
) -> Div {
    div()
        .flex_1()
        .py(SPACE_1)
        .rounded(RADIUS_SM)
        .bg(if is_active {
            theme.accent_subtle
        } else {
            theme.surface
        })
        .border_1()
        .border_color(if is_active {
            theme.accent
        } else {
            theme.border_subtle
        })
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .text_color(if is_active {
            theme.accent
        } else {
            theme.foreground_muted
        })
        .text_size(FONT_SIZE_CAPTION)
        .font_weight(if is_active {
            FontWeight::BOLD
        } else {
            FontWeight::MEDIUM
        })
        .child(label)
        .on_mouse_down(MouseButton::Left, move |_e, w, a| {
            on_click(w, a);
        })
}

fn make_swatch_pill(
    color: RgbColor,
    _tooltip: &'static str,
    on_click: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> Div {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let h = if (max - min).abs() < 1e-5 {
        0.0
    } else {
        let d = max - min;
        if (max - r).abs() < 1e-5 {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if (max - g).abs() < 1e-5 {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        }
    };
    let s = if (max - min).abs() < 1e-5 {
        0.0
    } else if l > 0.5 {
        (max - min) / (2.0 - max - min)
    } else {
        (max - min) / (max + min)
    };

    div()
        .w(px(26.0))
        .h(px(26.0))
        .rounded_full()
        .bg(hsla(h, s, l, 1.0))
        .border_2()
        .border_color(hsla(0.0, 0.0, 1.0, 0.2))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |_e, w, a| {
            on_click(w, a);
        })
}

fn make_nav_tile(
    title: &'static str,
    subtitle: &'static str,
    on_click: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    theme: &crate::theme::Theme,
) -> Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .p(SPACE_2)
        .rounded(RADIUS_SM)
        .bg(theme.surface)
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.bg(theme.surface_hover))
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_color(theme.foreground)
                        .text_size(FONT_SIZE_SM)
                        .font_weight(FontWeight::MEDIUM)
                        .child(title),
                )
                .child(
                    div()
                        .text_color(theme.foreground_subtle)
                        .text_size(FONT_SIZE_CAPTION)
                        .child(subtitle),
                ),
        )
        .child(
            div()
                .text_color(theme.foreground_subtle)
                .text_size(FONT_SIZE_BODY)
                .child("›"),
        )
        .on_mouse_down(MouseButton::Left, on_click)
}

fn target_name(id: u8, is_mouse: bool) -> &'static str {
    if is_mouse {
        match id {
            1 => "Left Primary Click",
            2 => "Right Secondary Click",
            3 => "Scroll Wheel Click",
            4 => "DPI Cycle (+)",
            5 => "DPI Cycle (-)",
            6 => "Thumb Forward (Button 5)",
            7 => "Thumb Back (Button 4)",
            _ => "Mouse Button",
        }
    } else {
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
            _ => "Key",
        }
    }
}

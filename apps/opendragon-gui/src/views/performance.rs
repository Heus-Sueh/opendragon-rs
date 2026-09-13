use gpui::prelude::*;
use gpui::*;

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::rapid_trigger::RapidTriggerConfig;

use crate::components::{BadgeStatus, Card, InfoTooltip, Slider, StatusBadge, Switch};
use crate::state::AppState;
use crate::theme::{
    CONTROL_HEIGHT_MD, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE,
    RADIUS_SM, SPACE_1, SPACE_2, SPACE_3, SPACE_6,
};

pub struct PerformanceView;

impl PerformanceView {
    pub fn render(
        state: &AppState,
        on_update_actuation: impl Fn(ActuationPoint, &mut Window, &mut App) + 'static + Clone,
        on_update_rapid_trigger: impl Fn(RapidTriggerConfig, &mut Window, &mut App) + 'static + Clone,
        on_update_polling_rate: impl Fn(PollingRate, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => return div().child("No active device"),
        };

        // Header
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
                            .child("Performance"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Tune Hall Effect magnetic switch actuation, Rapid Trigger, and response rate"),
                    ),
            )
            .child(
                StatusBadge::new(BadgeStatus::Supported)
                    .label("Magnetic Switches")
                    .render(theme),
            );

        // Actuation Point Control (0.1 mm to 4.0 mm, with precise stepper [-] [+] )
        let on_act = on_update_actuation.clone();
        let actuation_slider = Slider::new("Actuation Point", active.actuation.mm(), 0.1, 4.0)
            .step(0.1)
            .unit("mm")
            .presets([
                (0.5, "0.5 mm"),
                (1.0, "1.0 mm"),
                (1.5, "1.5 mm"),
                (2.0, "2.0 mm"),
                (3.0, "3.0 mm"),
                (4.0, "4.0 mm"),
            ])
            .on_change(move |val, w, a| {
                if let Ok(pt) = ActuationPoint::new(val) {
                    on_act(pt, w, a);
                }
            })
            .render(theme);

        // Rapid Trigger Switch + Sensitivities
        let is_rt_enabled = active.rapid_trigger.enabled;
        let on_rt_toggle = on_update_rapid_trigger.clone();
        let rt_cfg_clone = active.rapid_trigger;

        let rt_switch = Switch::new("Rapid Trigger", is_rt_enabled)
            .description("Responds immediately as the key changes direction without returning to reset point")
            .on_toggle(move |enabled, w, a| {
                let mut cfg = rt_cfg_clone;
                cfg.enabled = enabled;
                on_rt_toggle(cfg, w, a);
            })
            .render(theme);

        // When Rapid Trigger is enabled, show Press and Release sensitivities
        let rt_sensitivities = if is_rt_enabled {
            let on_press_change = on_update_rapid_trigger.clone();
            let base_cfg_p = active.rapid_trigger;
            let press_slider = Slider::new(
                "Press Sensitivity",
                active.rapid_trigger.press_sensitivity_mm,
                0.1,
                2.0,
            )
            .step(0.1)
            .unit("mm")
            .presets([
                (0.1, "0.1 mm"),
                (0.2, "0.2 mm"),
                (0.5, "0.5 mm"),
                (1.0, "1.0 mm"),
                (1.5, "1.5 mm"),
                (2.0, "2.0 mm"),
            ])
            .on_change(move |val, w, a| {
                let mut cfg = base_cfg_p;
                cfg.press_sensitivity_mm = val;
                on_press_change(cfg, w, a);
            })
            .render(theme);

            let on_release_change = on_update_rapid_trigger.clone();
            let base_cfg_r = active.rapid_trigger;
            let release_slider = Slider::new(
                "Release Sensitivity",
                active.rapid_trigger.release_sensitivity_mm,
                0.1,
                2.0,
            )
            .step(0.1)
            .unit("mm")
            .presets([
                (0.1, "0.1 mm"),
                (0.2, "0.2 mm"),
                (0.5, "0.5 mm"),
                (1.0, "1.0 mm"),
                (1.5, "1.5 mm"),
                (2.0, "2.0 mm"),
            ])
            .on_change(move |val, w, a| {
                let mut cfg = base_cfg_r;
                cfg.release_sensitivity_mm = val;
                on_release_change(cfg, w, a);
            })
            .render(theme);

            Some(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_3)
                    .w_full()
                    .child(press_slider)
                    .child(release_slider),
            )
        } else {
            None
        };

        // Polling Rate (Supported hardware rates only: 125, 250, 500, 1000)
        let rates = [
            (PollingRate::Hz125, "125 Hz"),
            (PollingRate::Hz250, "250 Hz"),
            (PollingRate::Hz500, "500 Hz"),
            (PollingRate::Hz1000, "1000 Hz"),
        ];

        let mut rate_buttons = div().flex().flex_row().gap(SPACE_2);

        for (rate, label) in rates {
            let is_active = active.polling_rate == rate;
            let on_pr = on_update_polling_rate.clone();

            let btn = div()
                .flex_1()
                .h(CONTROL_HEIGHT_MD)
                .flex()
                .items_center()
                .justify_center()
                .rounded(RADIUS_SM)
                .bg(if is_active {
                    theme.accent
                } else {
                    theme.surface_hover
                })
                .text_color(if is_active {
                    hsla(0.0, 0.0, 1.0, 1.0)
                } else {
                    theme.foreground
                })
                .text_size(FONT_SIZE_SM)
                .font_weight(if is_active {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, move |_e, w, a| {
                    on_pr(rate, w, a);
                })
                .child(label);

            rate_buttons = rate_buttons.child(btn);
        }

        let polling_card = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_3)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("USB Polling Rate"),
                            )
                            .child(InfoTooltip::new("HID Endpoint Interval").render(theme)),
                    )
                    .child(rate_buttons),
            )
            .render(theme);

        // Educational: Magnetic Actuation Explainer Card
        let actuation_explainer = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_2)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Understanding Hall Effect Magnetic Actuation"),
                            )
                            .child(InfoTooltip::new("Linear Magnetic Flux Sensing").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Unlike traditional mechanical switches with fixed metallic contact leaves, magnetic switches continuously measure magnetic flux via Hall Effect sensors. This allows precise per-millimeter actuation depth calibration with zero physical leaf contact wear."),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(SPACE_3)
                            .flex_wrap()
                            .child(
                                div()
                                    .flex_1()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .child(
                                        div()
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("🎯 0.1 mm - 1.2 mm (Esports / Ultra-Fast)"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Instantaneous actuation. Ideal for lightning reaction times in competitive tactical shooters (CS2, Valorant, Apex Legends) and rhythm games."),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .child(
                                        div()
                                            .text_color(theme.foreground)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("✍️ 2.0 mm - 4.0 mm (Typing / Development)"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Deeper and more deliberate keystroke depth. Prevents accidental actuation of adjacent keys during long programming sessions and writing."),
                                    ),
                            ),
                    ),
            )
            .render(theme);

        // Educational: Rapid Trigger Explainer Card
        let rt_explainer = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_2)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("How Rapid Trigger Works (Dynamic Reset)"),
                            )
                            .child(InfoTooltip::new("Continuous Re-Actuation").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("On conventional keyboards, pressing a key again requires releasing it past a fixed physical reset point. With Rapid Trigger enabled, the key resets the exact millisecond your finger begins moving upward, permitting instantaneous re-actuation on the next downward motion without waiting for full key travel return."),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_2)
                            .p(SPACE_3)
                            .rounded(RADIUS_SM)
                            .bg(theme.surface_hover)
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .text_size(FONT_SIZE_SM)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("⚡ Competitive Advantage in Counter-Strafing:"),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_CAPTION)
                                    .child("Releasing 'A' to stop immediately cuts character movement velocity, allowing pinpoint first-bullet accuracy without decelerating drift."),
                            ),
                    ),
            )
            .render(theme);

        // Educational: Polling Rate Explainer Card
        let polling_explainer = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_2)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("USB Polling Frequency (Report Rate)"),
                            )
                            .child(InfoTooltip::new("Kernel Interrupt Rate").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Determines how frequently the keyboard microcontroller reports switch states to the Linux kernel HID subsystem:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(SPACE_2)
                            .flex_wrap()
                            .child(
                                div()
                                    .flex_1()
                                    .p(SPACE_2)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .child(div().text_color(theme.foreground).text_size(FONT_SIZE_SM).font_weight(FontWeight::SEMIBOLD).child("1000 Hz"))
                                    .child(div().text_color(theme.foreground_muted).text_size(FONT_SIZE_CAPTION).child("1.0 ms latency. Maximum competitive performance and lowest input lag.")),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p(SPACE_2)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .child(div().text_color(theme.foreground).text_size(FONT_SIZE_SM).font_weight(FontWeight::SEMIBOLD).child("500 Hz"))
                                    .child(div().text_color(theme.foreground_muted).text_size(FONT_SIZE_CAPTION).child("2.0 ms latency. Rock-solid stability on legacy hardware.")),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p(SPACE_2)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .child(div().text_color(theme.foreground).text_size(FONT_SIZE_SM).font_weight(FontWeight::SEMIBOLD).child("125-250 Hz"))
                                    .child(div().text_color(theme.foreground_muted).text_size(FONT_SIZE_CAPTION).child("4-8 ms latency. Power-saving mode for battery conservation.")),
                            ),
                    ),
            )
            .render(theme);

        let mut content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(actuation_slider)
            .child(actuation_explainer)
            .child(rt_switch);

        if let Some(sens) = rt_sensitivities {
            content = content.child(sens);
        }

        content = content.child(rt_explainer);
        content = content.child(polling_card);
        content = content.child(polling_explainer);

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

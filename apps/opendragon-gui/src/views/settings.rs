use gpui::prelude::*;
use gpui::*;

use crate::components::{Button, Card, Switch};
use crate::state::{AppState, Page};
use crate::theme::{
    ThemeMode, CONTROL_HEIGHT_MD, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_SM,
    SPACE_1, SPACE_2, SPACE_3, SPACE_6,
};

pub struct SettingsView;

impl SettingsView {
    pub fn render(
        state: &AppState,
        on_theme_change: impl Fn(ThemeMode, &mut Window, &mut App) + 'static + Clone,
        on_auto_connect_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
        on_navigate: impl Fn(Page, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let current_theme_mode = state.preferences.theme_mode;

        // Header
        let header = div()
            .flex()
            .flex_col()
            .gap(SPACE_1)
            .child(
                div()
                    .text_color(theme.foreground)
                    .text_size(FONT_SIZE_TITLE)
                    .font_weight(FontWeight::BOLD)
                    .child("Settings"),
            )
            .child(
                div()
                    .text_color(theme.foreground_muted)
                    .text_size(FONT_SIZE_SM)
                    .child("Application preferences, themes, and peripheral runtime options"),
            );

        // Theme Options (Dark, Light, System)
        let theme_options = [
            (ThemeMode::Dark, "Dark"),
            (ThemeMode::Light, "Light"),
            (ThemeMode::System, "System"),
        ];

        let mut theme_buttons = div().flex().flex_row().gap(SPACE_2);

        for (mode, label) in theme_options {
            let is_active = current_theme_mode == mode;
            let on_th = on_theme_change.clone();

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
                    on_th(mode, w, a);
                })
                .child(label);

            theme_buttons = theme_buttons.child(btn);
        }

        let appearance_card = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_3)
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_HEADING)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Appearance"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Select color theme for OpenDragon-rs"),
                    )
                    .child(theme_buttons),
            )
            .render(theme);

        // General Options
        let on_ac = on_auto_connect_toggle.clone();
        let auto_connect_switch = Switch::new(
            "Auto-connect to known devices",
            state.preferences.auto_connect,
        )
        .description(
            "Automatically open the configuration page when exactly one device is detected",
        )
        .on_toggle(move |enabled, w, a| {
            on_ac(enabled, w, a);
        })
        .render(theme);

        let general_card = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_3)
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_HEADING)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("General"),
                    )
                    .child(auto_connect_switch),
            )
            .render(theme);

        // Advanced / Diagnostics Navigation Card
        let on_nav = on_navigate.clone();
        let has_active_device = state.active_device.is_some();

        let diagnostics_card = Card::new()
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
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Device Diagnostics"),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_SM)
                                    .child("Inspect USB descriptors, HID interfaces, usage pages, and export hardware reports"),
                            ),
                    )
                    .child(
                        Button::new(if has_active_device {
                            "Open Diagnostics →"
                        } else {
                            "No device connected"
                        })
                        .size(crate::components::ButtonSize::Sm)
                        .disabled(!has_active_device)
                        .on_click(move |_e, w, a| on_nav(Page::Diagnostics, w, a))
                        .render(theme),
                    ),
            )
            .render(theme);

        // About OpenDragon-rs Card
        let about_card = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_3)
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_HEADING)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("About OpenDragon-rs"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_2)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_SM)
                                            .child("Application Version"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("0.2.0"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_SM)
                                            .child("UI Engine"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("GPUI-CE / gpui-kit"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_SM)
                                            .child("Protocol Backend"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(if cfg!(feature = "mock-device") {
                                                "Mock Hardware Backend"
                                            } else {
                                                "Native Linux HID Transport"
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_SM)
                                            .child("License"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("MIT OR Apache-2.0"),
                                    ),
                            ),
                    ),
            )
            .render(theme);

        let content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(appearance_card)
            .child(general_card)
            .child(diagnostics_card)
            .child(about_card);

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

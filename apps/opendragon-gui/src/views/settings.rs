use gpui::prelude::*;
use gpui::*;

use crate::components::{Button, Card, SegmentedControl, SettingRow, Switch};
use crate::state::{AppState, DeviceAction, KeyboardLayout, Page};
use crate::theme::{
    ThemeMode, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, SPACE_1, SPACE_2, SPACE_3, SPACE_6,
};

pub struct SettingsView;

impl SettingsView {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let on_act_theme = on_action.clone();
        let on_theme_change = move |mode: ThemeMode, w: &mut Window, a: &mut App| {
            on_act_theme(DeviceAction::SetThemeMode(mode), w, a);
        };

        let on_act_auto = on_action.clone();
        let on_auto_connect_toggle = move |enabled: bool, w: &mut Window, a: &mut App| {
            on_act_auto(DeviceAction::SetAutoConnect(enabled), w, a);
        };

        let on_act_layout = on_action.clone();
        let on_layout_change = move |layout: KeyboardLayout, w: &mut Window, a: &mut App| {
            on_act_layout(DeviceAction::SetKeyboardLayout(layout), w, a);
        };

        let on_act_nav = on_action.clone();
        let on_navigate = move |page: Page, w: &mut Window, a: &mut App| {
            on_act_nav(DeviceAction::Navigate(page), w, a);
        };
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

        let theme_control = SegmentedControl::new(theme_options, current_theme_mode)
            .on_select(on_theme_change)
            .render(theme);

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
                    .child(theme_control),
            )
            .render(theme);

        // Hardware & General Options
        let layout_options = [
            (KeyboardLayout::Ansi, "ANSI (US)"),
            (KeyboardLayout::Abnt2, "ABNT2 (Brasil)"),
            (KeyboardLayout::Iso, "ISO (Europe)"),
        ];

        let layout_control =
            SegmentedControl::new(layout_options, state.preferences.keyboard_layout)
                .on_select(on_layout_change)
                .render(theme);

        let layout_row = SettingRow::new("Physical Keyboard Layout")
            .flat(true)
            .description("Select the physical layout standard and keymap used for previews and visual key mapping")
            .control(layout_control)
            .render(theme);

        let on_ac = on_auto_connect_toggle.clone();
        let auto_connect_switch = Switch::new(
            "Auto-connect to known devices",
            state.preferences.auto_connect,
        )
        .flat(true)
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
                            .child("Hardware & General"),
                    )
                    .child(layout_row)
                    .child(div().h_px().w_full().bg(theme.border_subtle))
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
                            "Open diagnostics"
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

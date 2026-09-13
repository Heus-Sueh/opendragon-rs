use gpui::prelude::*;
use gpui::*;

use crate::state::{AppState, Page};
use crate::theme::{
    CONTROL_HEIGHT_MD, FONT_SIZE_BODY, FONT_SIZE_CAPTION, RADIUS_SM, SIDEBAR_WIDTH, SPACE_2,
    SPACE_4,
};

pub struct Sidebar;

impl Sidebar {
    pub fn render(
        state: &AppState,
        on_navigate: impl Fn(Page, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let current_page = state.current_page;
        let has_device = state.active_device.is_some();

        let mut nav_items = Vec::new();

        if has_device {
            nav_items.push((Page::Overview, "Overview", "⊞"));
            if state.has_capability(opendragon_core::capability::Capability::Lighting) {
                nav_items.push((Page::Lighting, "Lighting", "☼"));
            }
            if state.has_capability(opendragon_core::capability::Capability::Actuation)
                || state.has_capability(opendragon_core::capability::Capability::RapidTrigger)
            {
                nav_items.push((Page::Performance, "Performance", "⚡"));
            }
            if state.has_capability(opendragon_core::capability::Capability::Profile) {
                nav_items.push((Page::Profiles, "Profiles", "☰"));
            }
            if state.has_capability(opendragon_core::capability::Capability::Macro) {
                nav_items.push((Page::Macros, "Macros", "⌨"));
            }
        }

        let mut nav_list = div().flex().flex_col().gap(px(2.0)).w_full();

        for (page, label, icon) in nav_items {
            let is_active = current_page == page;
            let bg = if is_active {
                theme.surface_active
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            };
            let text_color = if is_active {
                theme.foreground
            } else {
                theme.foreground_muted
            };
            let border_color = if is_active {
                theme.accent
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            };

            let on_nav = on_navigate.clone();
            let hover_bg = theme.surface_hover;

            let item = div()
                .flex()
                .flex_row()
                .items_center()
                .h(CONTROL_HEIGHT_MD)
                .gap(SPACE_2)
                .px(SPACE_2)
                .rounded(RADIUS_SM)
                .bg(bg)
                .border_l_2()
                .border_color(border_color)
                .text_color(text_color)
                .text_size(FONT_SIZE_BODY)
                .font_weight(if is_active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .cursor_pointer()
                .hover(move |mut s| {
                    if !is_active {
                        s.background = Some(hover_bg.into());
                    }
                    s
                })
                .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    on_nav(page, window, cx);
                })
                .child(
                    div()
                        .size(px(16.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(FONT_SIZE_BODY)
                        .child(icon),
                )
                .child(label);

            nav_list = nav_list.child(item);
        }

        // Bottom items: Devices (Home), Diagnostics, Settings
        let mut bottom_list = div().flex().flex_col().gap(px(2.0)).w_full();

        let mut bottom_items = vec![(Page::Home, "Devices", "❖")];
        if has_device {
            bottom_items.push((Page::Diagnostics, "Diagnostics", "⌥"));
        }
        bottom_items.push((Page::Settings, "Settings", "⚙"));

        for (page, label, icon) in bottom_items {
            let is_active = current_page == page;
            let bg = if is_active {
                theme.surface_active
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            };
            let text_color = if is_active {
                theme.foreground
            } else {
                theme.foreground_muted
            };
            let border_color = if is_active {
                theme.accent
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            };

            let on_nav = on_navigate.clone();
            let hover_bg = theme.surface_hover;

            let item = div()
                .flex()
                .flex_row()
                .items_center()
                .h(CONTROL_HEIGHT_MD)
                .gap(SPACE_2)
                .px(SPACE_2)
                .rounded(RADIUS_SM)
                .bg(bg)
                .border_l_2()
                .border_color(border_color)
                .text_color(text_color)
                .text_size(FONT_SIZE_BODY)
                .font_weight(if is_active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .cursor_pointer()
                .hover(move |mut s| {
                    if !is_active {
                        s.background = Some(hover_bg.into());
                    }
                    s
                })
                .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    on_nav(page, window, cx);
                })
                .child(
                    div()
                        .size(px(16.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(FONT_SIZE_BODY)
                        .child(icon),
                )
                .child(label);

            bottom_list = bottom_list.child(item);
        }

        // Top App Header
        let app_header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(SPACE_2)
            .px(SPACE_2)
            .py(SPACE_2)
            .child(
                div()
                    .size(px(24.0))
                    .rounded(RADIUS_SM)
                    .bg(theme.accent)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    .text_size(FONT_SIZE_CAPTION)
                    .font_weight(FontWeight::BOLD)
                    .child("OD"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_BODY)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("OpenDragon-rs"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_CAPTION)
                            .child("v0.2.0"),
                    ),
            );

        // Sidebar container
        div()
            .w(SIDEBAR_WIDTH)
            .h_full()
            .flex()
            .flex_col()
            .justify_between()
            .p(SPACE_2)
            .bg(theme.surface)
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_4)
                    .w_full()
                    .child(app_header)
                    .child(nav_list),
            )
            .child(bottom_list)
    }
}

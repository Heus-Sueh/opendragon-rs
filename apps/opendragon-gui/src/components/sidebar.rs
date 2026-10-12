use gpui::prelude::*;
use gpui::*;
use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{
    Sidebar as KitSidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader,
    SidebarMenu, SidebarMenuItem,
};

use crate::state::{AppState, DeviceAction, Page};
use crate::theme::{FONT_SIZE_BODY, FONT_SIZE_CAPTION, RADIUS_FULL, RADIUS_SM, SPACE_2};

pub struct Sidebar;

impl Sidebar {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> impl IntoElement {
        let theme = &state.theme;
        let current_page = state.current_page;
        let active_dev = state.active_device.as_ref();
        let has_device = active_dev.is_some();

        // 1. Sidebar Header with App Branding and Connected Device Status
        let mut header = SidebarHeader::new();
        header = header.child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .w_full()
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
                                .flex_row()
                                .items_center()
                                .gap(SPACE_2)
                                .child(
                                    div()
                                        .size(px(18.0))
                                        .rounded(RADIUS_SM)
                                        .bg(theme.accent)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(theme.accent_foreground)
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("🐉"),
                                )
                                .child(
                                    div()
                                        .text_color(theme.foreground)
                                        .text_size(FONT_SIZE_BODY)
                                        .font_weight(FontWeight::BOLD)
                                        .child("OpenDragon"),
                                ),
                        )
                        .child(
                            div()
                                .px(px(6.0))
                                .py(px(1.0))
                                .rounded_full()
                                .bg(theme.surface_hover)
                                .text_color(theme.foreground_muted)
                                .text_size(px(10.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child("v0.2"),
                        ),
                )
                .when_some(active_dev, |el, dev| {
                    el.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(SPACE_2)
                            .pt(px(2.0))
                            .child(div().size(px(6.0)).rounded(RADIUS_FULL).bg(theme.success))
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_CAPTION)
                                    .child(dev.summary.model.clone()),
                            ),
                    )
                }),
        );

        // 2. Hardware Navigation Items
        let mut groups = Vec::new();

        if has_device {
            let mut hw_menu = SidebarMenu::new();

            // Overview
            let on_act = on_action.clone();
            hw_menu = hw_menu.child(
                SidebarMenuItem::new("Overview")
                    .icon(IconName::LayoutDashboard)
                    .active(current_page == Page::Overview)
                    .on_click(move |_, w, a| on_act(DeviceAction::Navigate(Page::Overview), w, a)),
            );

            // Lighting
            if state.has_capability(opendragon_core::capability::Capability::Lighting) {
                let on_act = on_action.clone();
                hw_menu = hw_menu.child(
                    SidebarMenuItem::new("Lighting")
                        .icon(IconName::Sun)
                        .active(current_page == Page::Lighting)
                        .on_click(move |_, w, a| {
                            on_act(DeviceAction::Navigate(Page::Lighting), w, a)
                        }),
                );
            }

            // Performance
            if state.has_capability(opendragon_core::capability::Capability::Actuation)
                || state.has_capability(opendragon_core::capability::Capability::RapidTrigger)
            {
                let on_act = on_action.clone();
                hw_menu = hw_menu.child(
                    SidebarMenuItem::new("Performance")
                        .icon(IconName::Zap)
                        .active(current_page == Page::Performance)
                        .on_click(move |_, w, a| {
                            on_act(DeviceAction::Navigate(Page::Performance), w, a)
                        }),
                );
            }

            // Profiles
            if state.has_capability(opendragon_core::capability::Capability::Profile) {
                let on_act = on_action.clone();
                hw_menu = hw_menu.child(
                    SidebarMenuItem::new("Profiles")
                        .icon(IconName::Layers)
                        .active(current_page == Page::Profiles)
                        .on_click(move |_, w, a| {
                            on_act(DeviceAction::Navigate(Page::Profiles), w, a)
                        }),
                );
            }

            // Macros
            if state.has_capability(opendragon_core::capability::Capability::Macro) {
                let on_act = on_action.clone();
                hw_menu = hw_menu.child(
                    SidebarMenuItem::new("Macros")
                        .icon(IconName::Terminal)
                        .active(current_page == Page::Macros)
                        .on_click(move |_, w, a| {
                            on_act(DeviceAction::Navigate(Page::Macros), w, a)
                        }),
                );
            }

            groups.push(SidebarGroup::new("Hardware").child(hw_menu));
        }

        // 3. System Navigation Items
        let mut sys_menu = SidebarMenu::new();

        // Devices (Home)
        let on_act = on_action.clone();
        sys_menu = sys_menu.child(
            SidebarMenuItem::new("Devices")
                .icon(IconName::Cpu)
                .active(current_page == Page::Home)
                .on_click(move |_, w, a| on_act(DeviceAction::Navigate(Page::Home), w, a)),
        );

        // Diagnostics
        if has_device {
            let on_act = on_action.clone();
            sys_menu = sys_menu.child(
                SidebarMenuItem::new("Diagnostics")
                    .icon(IconName::Activity)
                    .active(current_page == Page::Diagnostics)
                    .on_click(move |_, w, a| {
                        on_act(DeviceAction::Navigate(Page::Diagnostics), w, a)
                    }),
            );
        }

        // Settings
        let on_act = on_action.clone();
        sys_menu = sys_menu.child(
            SidebarMenuItem::new("Settings")
                .icon(IconName::Settings)
                .active(current_page == Page::Settings)
                .on_click(move |_, w, a| on_act(DeviceAction::Navigate(Page::Settings), w, a)),
        );

        groups.push(SidebarGroup::new("System").child(sys_menu));

        // 4. Sidebar Footer
        let on_act_refresh = on_action.clone();
        let footer = SidebarFooter::new().child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w_full()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(SPACE_2)
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, move |_e, w, a| {
                            on_act_refresh(DeviceAction::RefreshDevices, w, a);
                        })
                        .child(
                            div()
                                .text_color(theme.foreground_muted)
                                .text_size(FONT_SIZE_CAPTION)
                                .child("↻ Scan USB"),
                        ),
                )
                .child(
                    div()
                        .text_color(theme.foreground_subtle)
                        .text_size(FONT_SIZE_CAPTION)
                        .child("Ctrl+R"),
                ),
        );

        KitSidebar::new("app_sidebar")
            .collapsible(SidebarCollapsible::None)
            .header(header)
            .children(groups)
            .footer(footer)
    }
}

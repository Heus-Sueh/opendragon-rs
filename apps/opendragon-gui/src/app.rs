use gpui::prelude::*;
use gpui::*;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Root;

use crate::components::Sidebar;
use crate::state::{AppState, DeviceAction, Page, ToastType};
use crate::theme::{RADIUS_MD, SPACE_2, SPACE_3, SPACE_4};
use crate::views::{
    DeviceOverview, DiagnosticsView, HomeView, LightingView, MacrosView, PerformanceView,
    ProfilesView, SettingsView,
};

pub struct OpenDragonApp {
    pub state: Entity<AppState>,
    pub focus_handle: FocusHandle,
    pub scroll_handle: ScrollHandle,
}

impl OpenDragonApp {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_this, _entity, cx| {
            cx.notify();
        })
        .detach();
        let focus_handle = cx.focus_handle();
        let scroll_handle = ScrollHandle::new();
        Self {
            state,
            focus_handle,
            scroll_handle,
        }
    }
}

impl Render for OpenDragonApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let theme = state.theme;
        let current_page = state.current_page;
        let toast = state.toast.clone();

        // Single Unified Action Dispatcher Seam
        let state_dispatch = self.state.clone();
        let on_action = move |action: DeviceAction, _w: &mut Window, cx: &mut App| {
            if let DeviceAction::SetThemeMode(mode) = &action {
                let kit_mode = match mode {
                    crate::theme::ThemeMode::Light => gpui_kit::component::ThemeMode::Light,
                    crate::theme::ThemeMode::Dark | crate::theme::ThemeMode::System => {
                        gpui_kit::component::ThemeMode::Dark
                    }
                };
                gpui_kit::component::Theme::change(kit_mode, None, cx);
            }
            state_dispatch.update(cx, |s, cx| {
                s.dispatch(action);
                cx.notify();
            });
        };

        // Render page content
        let page_content = match current_page {
            Page::Home => HomeView::render(state, on_action.clone()),
            Page::Overview => DeviceOverview::render(state, on_action.clone()),
            Page::Lighting => LightingView::render(state, on_action.clone()),
            Page::Performance => PerformanceView::render(state, on_action.clone()),
            Page::Profiles => ProfilesView::render(state, on_action.clone()),
            Page::Macros => MacrosView::render(state, on_action.clone()),
            Page::Settings => SettingsView::render(state, on_action.clone()),
            Page::Diagnostics => DiagnosticsView::render(state, on_action.clone()),
        };

        // Sidebar
        let sidebar = Sidebar::render(state, on_action.clone());

        // Toast element if present
        let toast_el = toast.map(|(msg, t_type)| {
            let (bg, text_color, icon) = match t_type {
                ToastType::Success => (theme.success, hsla(0.0, 0.0, 1.0, 1.0), "✓"),
                ToastType::Info => (theme.accent, theme.accent_foreground, "ℹ"),
                ToastType::Warning => (theme.warning, hsla(0.0, 0.0, 0.1, 1.0), "⚠"),
                ToastType::Error => (theme.danger, hsla(0.0, 0.0, 1.0, 1.0), "✕"),
            };

            let state_toast_dismiss = self.state.clone();

            div()
                .absolute()
                .bottom(SPACE_4)
                .right(SPACE_4)
                .flex()
                .flex_row()
                .items_center()
                .gap(SPACE_3)
                .px(SPACE_4)
                .py(SPACE_2)
                .rounded(RADIUS_MD)
                .bg(bg)
                .text_color(text_color)
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, move |_e, _w, a| {
                    state_toast_dismiss.update(a, |s, cx| {
                        s.clear_toast();
                        cx.notify();
                    });
                })
                .child(icon)
                .child(msg)
        });

        let on_act_key = on_action.clone();
        let is_dark_theme = theme.is_dark;

        // Main Shell with Global Keyboard Shortcuts
        let mut shell = div()
            .track_focus(&self.focus_handle)
            .key_context("OpenDragonShell")
            .size_full()
            .flex()
            .flex_row()
            .bg(theme.background)
            .relative()
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                let is_ctrl =
                    event.keystroke.modifiers.control || event.keystroke.modifiers.platform;
                if is_ctrl {
                    match event.keystroke.key.as_str() {
                        "1" => on_act_key(DeviceAction::Navigate(Page::Overview), window, cx),
                        "2" => on_act_key(DeviceAction::Navigate(Page::Lighting), window, cx),
                        "3" => on_act_key(DeviceAction::Navigate(Page::Performance), window, cx),
                        "4" => on_act_key(DeviceAction::Navigate(Page::Profiles), window, cx),
                        "5" => on_act_key(DeviceAction::Navigate(Page::Macros), window, cx),
                        "6" => on_act_key(DeviceAction::Navigate(Page::Home), window, cx),
                        "7" => on_act_key(DeviceAction::Navigate(Page::Diagnostics), window, cx),
                        "8" => on_act_key(DeviceAction::Navigate(Page::Settings), window, cx),
                        "r" | "R" => on_act_key(DeviceAction::RefreshDevices, window, cx),
                        "t" | "T" => {
                            let next_mode = if is_dark_theme {
                                crate::theme::ThemeMode::Light
                            } else {
                                crate::theme::ThemeMode::Dark
                            };
                            on_act_key(DeviceAction::SetThemeMode(next_mode), window, cx);
                        }
                        _ => {}
                    }
                } else if event.keystroke.key == "escape" {
                    on_act_key(DeviceAction::ClearKeySelection, window, cx);
                }
            })
            .child(sidebar)
            .child(
                div()
                    .id("page_scroll_area")
                    .flex_1()
                    .h_full()
                    .overflow_y_scrollbar()
                    .child(page_content),
            );

        if let Some(t) = toast_el {
            shell = shell.child(t);
        }

        shell
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}

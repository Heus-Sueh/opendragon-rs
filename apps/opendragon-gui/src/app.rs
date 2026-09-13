use gpui::prelude::*;
use gpui::*;
use gpui_kit::component::Root;

use crate::components::Sidebar;
use crate::state::{AppState, Page, ToastType};
use crate::theme::{RADIUS_MD, SPACE_2, SPACE_3, SPACE_4};
use crate::views::{
    DeviceOverview, DiagnosticsView, HomeView, LightingView, MacrosView, PerformanceView,
    ProfilesView, SettingsView,
};

pub struct OpenDragonApp {
    pub state: Entity<AppState>,
}

impl OpenDragonApp {
    pub fn new(state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_this, _entity, cx| {
            cx.notify();
        })
        .detach();
        Self { state }
    }
}

impl Render for OpenDragonApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let theme = state.theme;
        let current_page = state.current_page;
        let toast = state.toast.clone();

        // Callbacks for subviews
        let state_nav = self.state.clone();
        let on_navigate = move |page: Page, _w: &mut Window, cx: &mut App| {
            state_nav.update(cx, |s, cx| {
                s.navigate_to(page);
                cx.notify();
            });
        };

        let state_sel = self.state.clone();
        let on_select_device = move |dev_id: String, _w: &mut Window, cx: &mut App| {
            state_sel.update(cx, |s, cx| {
                s.select_device(&dev_id);
                cx.notify();
            });
        };

        let state_ref = self.state.clone();
        let on_refresh = move |_w: &mut Window, cx: &mut App| {
            state_ref.update(cx, |s, cx| {
                s.refresh_devices();
                cx.notify();
            });
        };

        let state_exp = self.state.clone();
        let on_export_unsupported = move |dev_id: String, _w: &mut Window, cx: &mut App| {
            state_exp.update(cx, |s, cx| {
                let report = {
                    let b = s.backend.lock().unwrap();
                    b.get_probe_report(&dev_id).unwrap_or_default()
                };
                s.set_toast(
                    format!("Exported diagnostic report for device {dev_id}"),
                    ToastType::Info,
                );
                tracing::info!("Device diagnostic export:\n{report}");
                cx.notify();
            });
        };

        let state_light = self.state.clone();
        let on_update_lighting =
            move |cfg: opendragon_core::lighting::LightingConfig, _w: &mut Window, cx: &mut App| {
                state_light.update(cx, |s, cx| {
                    s.update_lighting(cfg);
                    cx.notify();
                });
            };

        let state_bright = self.state.clone();
        let on_brightness_change = move |bright: u8, _w: &mut Window, cx: &mut App| {
            state_bright.update(cx, |s, cx| {
                if let Some(act) = &s.active_device {
                    let mut cfg = act.lighting.clone();
                    cfg.brightness = bright;
                    s.update_lighting(cfg);
                    cx.notify();
                }
            });
        };

        let state_act = self.state.clone();
        let on_update_actuation =
            move |pt: opendragon_core::actuation::ActuationPoint, _w: &mut Window, cx: &mut App| {
                state_act.update(cx, |s, cx| {
                    s.update_actuation(pt);
                    cx.notify();
                });
            };

        let state_rt = self.state.clone();
        let on_update_rapid_trigger =
            move |cfg: opendragon_core::rapid_trigger::RapidTriggerConfig,
                  _w: &mut Window,
                  cx: &mut App| {
                state_rt.update(cx, |s, cx| {
                    s.update_rapid_trigger(cfg);
                    cx.notify();
                });
            };

        let state_rt_tog = self.state.clone();
        let on_rapid_trigger_toggle = move |enabled: bool, _w: &mut Window, cx: &mut App| {
            state_rt_tog.update(cx, |s, cx| {
                if let Some(act) = &s.active_device {
                    let mut cfg = act.rapid_trigger;
                    cfg.enabled = enabled;
                    s.update_rapid_trigger(cfg);
                    cx.notify();
                }
            });
        };

        let state_pr = self.state.clone();
        let on_update_polling_rate = move |rate: opendragon_core::polling_rate::PollingRate,
                                           _w: &mut Window,
                                           cx: &mut App| {
            state_pr.update(cx, |s, cx| {
                s.update_polling_rate(rate);
                cx.notify();
            });
        };

        let state_prof = self.state.clone();
        let on_switch_profile =
            move |pid: opendragon_core::profile::ProfileId, _w: &mut Window, cx: &mut App| {
                state_prof.update(cx, |s, cx| {
                    s.switch_profile(pid);
                    cx.notify();
                });
            };

        let state_add_prof = self.state.clone();
        let on_add_profile = move |name: String, _w: &mut Window, cx: &mut App| {
            state_add_prof.update(cx, |s, cx| {
                s.add_profile(name);
                cx.notify();
            });
        };

        let state_del_prof = self.state.clone();
        let on_delete_profile =
            move |pid: opendragon_core::profile::ProfileId, _w: &mut Window, cx: &mut App| {
                state_del_prof.update(cx, |s, cx| {
                    s.delete_profile(pid);
                    cx.notify();
                });
            };

        let state_theme = self.state.clone();
        let on_theme_change =
            move |mode: crate::theme::ThemeMode, _w: &mut Window, cx: &mut App| {
                state_theme.update(cx, |s, cx| {
                    s.set_theme_mode(mode);
                    cx.notify();
                });
            };

        let state_ac = self.state.clone();
        let on_auto_connect_toggle = move |enabled: bool, _w: &mut Window, cx: &mut App| {
            state_ac.update(cx, |s, cx| {
                s.set_auto_connect(enabled);
                cx.notify();
            });
        };

        let state_key = self.state.clone();
        let on_key_click = move |key_id: u8, _w: &mut Window, cx: &mut App| {
            state_key.update(cx, |s, cx| {
                s.toggle_key_selection(key_id);
                cx.notify();
            });
        };

        let state_key_drag = self.state.clone();
        let on_key_drag = move |key_id: u8, _w: &mut Window, cx: &mut App| {
            state_key_drag.update(cx, |s, cx| {
                if let Some(act) = &s.active_device {
                    if !act.visual_state.selected_keys.contains(&key_id) {
                        s.select_key(key_id);
                        cx.notify();
                    }
                }
            });
        };

        let state_clr = self.state.clone();
        let on_clear_selection = move |_w: &mut Window, cx: &mut App| {
            state_clr.update(cx, |s, cx| {
                s.clear_key_selection();
                cx.notify();
            });
        };

        let state_dpi = self.state.clone();
        let on_dpi_change = move |dpi: u16, _w: &mut Window, cx: &mut App| {
            state_dpi.update(cx, |s, cx| {
                s.update_dpi(dpi);
                cx.notify();
            });
        };

        let state_cp = self.state.clone();
        let on_copy_info = move |info: String, _w: &mut Window, cx: &mut App| {
            state_cp.update(cx, |s, cx| {
                s.set_toast("Technical details copied to log", ToastType::Info);
                tracing::info!("Technical diagnostic details:\n{info}");
                cx.notify();
            });
        };

        let state_diag_exp = self.state.clone();
        let on_export_report = move |_w: &mut Window, cx: &mut App| {
            state_diag_exp.update(cx, |s, cx| {
                if let Some(act) = &s.active_device {
                    let report = {
                        let b = s.backend.lock().unwrap();
                        b.get_probe_report(&act.summary.id).unwrap_or_default()
                    };
                    s.set_toast("Exported probe report to stdout", ToastType::Success);
                    println!("{report}");
                    cx.notify();
                }
            });
        };

        // Render page content
        let page_content = match current_page {
            Page::Home => {
                HomeView::render(state, on_select_device, on_refresh, on_export_unsupported)
            }
            Page::Overview => DeviceOverview::render(
                state,
                on_navigate.clone(),
                on_brightness_change,
                on_rapid_trigger_toggle,
                on_key_click,
                on_key_drag,
                on_clear_selection,
                on_dpi_change,
            ),
            Page::Lighting => LightingView::render(state, on_update_lighting),
            Page::Performance => PerformanceView::render(
                state,
                on_update_actuation,
                on_update_rapid_trigger,
                on_update_polling_rate,
            ),
            Page::Profiles => {
                ProfilesView::render(state, on_switch_profile, on_add_profile, on_delete_profile)
            }
            Page::Macros => MacrosView::render(state),
            Page::Settings => SettingsView::render(
                state,
                on_theme_change,
                on_auto_connect_toggle,
                on_navigate.clone(),
            ),
            Page::Diagnostics => DiagnosticsView::render(state, on_copy_info, on_export_report),
        };

        // Sidebar
        let sidebar = Sidebar::render(state, on_navigate);

        // Toast element if present
        let toast_el = toast.map(|(msg, t_type)| {
            let (bg, text_color, icon) = match t_type {
                ToastType::Success => (theme.success, hsla(0.0, 0.0, 1.0, 1.0), "✓"),
                ToastType::Info => (theme.accent, hsla(0.0, 0.0, 1.0, 1.0), "ℹ"),
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

        // Main Shell
        let mut shell = div()
            .size_full()
            .flex()
            .flex_row()
            .bg(theme.background)
            .relative()
            .child(sidebar)
            .child(
                div()
                    .id("page_scroll_area")
                    .flex_1()
                    .h_full()
                    .overflow_y_scroll()
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

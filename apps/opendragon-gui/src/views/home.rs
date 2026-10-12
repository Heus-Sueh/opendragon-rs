use gpui::prelude::*;
use gpui::*;

use crate::components::{BadgeStatus, Button, Card, EmptyState, StatusBadge};
use crate::state::{AppState, DeviceAction};
use crate::theme::{
    FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_SM, SPACE_1, SPACE_2,
    SPACE_3, SPACE_4, SPACE_6,
};

pub struct HomeView;

impl HomeView {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;

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
                            .child("Your Devices"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Configure lighting, actuation, and profiles for connected Redragon hardware"),
                    ),
            )
            .child({
                let on_act = on_action.clone();
                Button::new("Refresh Devices")
                    .size(crate::components::ButtonSize::Md)
                    .on_click(move |_e, w, a| on_act(DeviceAction::RefreshDevices, w, a))
                    .render(theme)
            });

        let mut content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header);

        if state.devices.is_empty() {
            let on_act = on_action.clone();
            let empty = EmptyState::new(
                "No Redragon device detected",
                "Connect a supported Redragon keyboard or mouse to configure it.",
            )
            .icon("⌨")
            .action(
                Button::primary("Check for devices")
                    .on_click(move |_e, w, a| on_act(DeviceAction::RefreshDevices, w, a))
                    .render(theme),
            )
            .render(theme);

            content = content.child(empty);
        } else {
            let mut device_grid = div().flex().flex_col().gap(SPACE_3).w_full();

            for dev in &state.devices {
                let dev_id = dev.id.clone();
                let is_supported = dev.is_supported;
                let on_act = on_action.clone();

                if is_supported {
                    let on_act_card = on_act.clone();
                    let card = Card::new()
                        .interactive(true)
                        .on_click(move |_e, w, a| {
                            on_act_card(DeviceAction::SelectDevice(dev_id.clone()), w, a);
                        })
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
                                        .gap(SPACE_3)
                                        .child({
                                            let icon = if dev.capabilities.contains(
                                                &opendragon_core::capability::Capability::Dpi,
                                            ) && !dev.capabilities.contains(
                                                &opendragon_core::capability::Capability::Actuation,
                                            ) {
                                                "🖱"
                                            } else {
                                                "⌨"
                                            };
                                            div()
                                                .size(px(36.0))
                                                .rounded(RADIUS_SM)
                                                .bg(theme.surface_hover)
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_color(theme.accent)
                                                .text_size(px(16.0))
                                                .child(icon)
                                        })
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
                                                        .child(dev.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_color(theme.foreground_muted)
                                                        .text_size(FONT_SIZE_SM)
                                                        .child(dev.model.clone()),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(SPACE_4)
                                        .child(
                                            StatusBadge::new(BadgeStatus::Connected).render(theme),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(SPACE_1)
                                                .text_color(theme.accent)
                                                .text_size(FONT_SIZE_SM)
                                                .font_weight(FontWeight::MEDIUM)
                                                .child("Configure")
                                                .child(
                                                    div()
                                                        .text_color(theme.foreground_subtle)
                                                        .text_size(FONT_SIZE_BODY)
                                                        .child("›"),
                                                ),
                                        ),
                                ),
                        )
                        .render(theme);

                    device_grid = device_grid.child(card);
                } else {
                    let dev_id_exp = dev.id.clone();
                    let card = Card::new()
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
                                        .gap(px(2.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(SPACE_2)
                                                .child(
                                                    div()
                                                        .text_color(theme.foreground)
                                                        .text_size(FONT_SIZE_BODY)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(dev.name.clone()),
                                                )
                                                .child(
                                                    StatusBadge::new(BadgeStatus::Partial)
                                                        .label("Unsupported")
                                                        .render(theme),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.foreground_muted)
                                                .text_size(FONT_SIZE_CAPTION)
                                                .child(format!(
                                                    "This device isn't supported yet. (VID: 0x{:04x}, PID: 0x{:04x})",
                                                    dev.vendor_id, dev.product_id
                                                )),
                                        ),
                                )
                                .child({
                                    let on_act = on_act.clone();
                                    Button::new("Export report")
                                        .size(crate::components::ButtonSize::Sm)
                                        .on_click(move |_e, w, a| {
                                            on_act(
                                                DeviceAction::ExportDeviceDiagnostics(dev_id_exp.clone()),
                                                w,
                                                a,
                                            );
                                        })
                                        .render(theme)
                                }),
                        )
                        .render(theme);

                    device_grid = device_grid.child(card);
                }
            }

            content = content.child(device_grid);
        }

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

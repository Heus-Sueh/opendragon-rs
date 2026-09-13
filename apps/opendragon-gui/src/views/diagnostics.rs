use gpui::prelude::*;
use gpui::*;

use crate::components::{Button, Card};
use crate::state::AppState;
use crate::theme::{
    FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_SM, SPACE_1,
    SPACE_2, SPACE_3, SPACE_6,
};

pub struct DiagnosticsView;

impl DiagnosticsView {
    pub fn render(
        state: &AppState,
        on_copy_info: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
        on_export_report: impl Fn(&mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => return div().child("No active device for diagnostics"),
        };

        let info = {
            let backend = state.backend.lock().unwrap();
            backend.get_device_info(&active.summary.id).ok()
        };

        let (vid_str, pid_str, serial_str, release_str, ifaces) = match &info {
            Some(i) => (
                format!("0x{:04X}", i.vendor_id),
                format!("0x{:04X}", i.product_id),
                i.serial_number.clone().unwrap_or_else(|| "N/A".to_string()),
                i.release_number
                    .map(|r| format!("0x{:04X}", r))
                    .unwrap_or_else(|| "N/A".to_string()),
                i.interfaces.clone(),
            ),
            None => (
                format!("0x{:04X}", active.summary.vendor_id),
                format!("0x{:04X}", active.summary.product_id),
                "N/A".to_string(),
                "N/A".to_string(),
                Vec::new(),
            ),
        };

        let summary_text = format!(
            "Device: {}\nModel: {}\nVID: {}\nPID: {}\nSerial: {}\nFirmware Release: {}\nInterfaces: {}",
            active.summary.name,
            active.summary.model,
            vid_str,
            pid_str,
            serial_str,
            release_str,
            ifaces.len()
        );

        let on_cp = on_copy_info.clone();
        let summary_for_copy = summary_text.clone();
        let on_exp = on_export_report.clone();

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
                            .child("Device Diagnostics"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Low-level hardware descriptors, HID interfaces, and telemetry"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(SPACE_2)
                    .child(
                        Button::new("Copy Info")
                            .size(crate::components::ButtonSize::Sm)
                            .on_click(move |_e, w, a| on_cp(summary_for_copy.clone(), w, a))
                            .render(theme),
                    )
                    .child(
                        Button::primary("Export JSON Report")
                            .size(crate::components::ButtonSize::Sm)
                            .on_click(move |_e, w, a| on_exp(w, a))
                            .render(theme),
                    ),
            );

        // Hardware Properties Card
        let hardware_details = div()
            .flex()
            .flex_col()
            .gap(SPACE_1)
            .w_full()
            .child(render_kv("Vendor ID", &vid_str, theme))
            .child(render_kv("Product ID", &pid_str, theme))
            .child(render_kv("Serial Number", &serial_str, theme))
            .child(render_kv("Firmware / bcdDevice", &release_str, theme))
            .child(render_kv("Transport Bus", "USB", theme));

        let hw_card = Card::new()
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
                            .child("Hardware Identification"),
                    )
                    .child(hardware_details),
            )
            .render(theme);

        // HID Interfaces Table Card
        let mut iface_rows = div().flex().flex_col().gap(SPACE_2).w_full();

        if ifaces.is_empty() {
            iface_rows = iface_rows.child(
                div()
                    .text_color(theme.foreground_muted)
                    .text_size(FONT_SIZE_SM)
                    .child("No interfaces enumerated"),
            );
        } else {
            for iface in &ifaces {
                let up = iface
                    .usage_page
                    .map(|u| format!("0x{:04X}", u))
                    .unwrap_or_else(|| "-".to_string());
                let u = iface
                    .usage
                    .map(|u| format!("0x{:04X}", u))
                    .unwrap_or_else(|| "-".to_string());
                let path = iface.hidraw_path.as_deref().unwrap_or("-");
                let report_ids_str = if iface.report_ids.is_empty() {
                    "-".to_string()
                } else {
                    format!("{:?}", iface.report_ids)
                };

                let row = div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .px(SPACE_3)
                    .py(SPACE_2)
                    .rounded(RADIUS_SM)
                    .bg(theme.surface_hover)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_color(theme.foreground)
                                    .text_size(FONT_SIZE_SM)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(format!("Interface #{}", iface.interface_number)),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_CAPTION)
                                    .child(format!("Node: {path}")),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_color(theme.foreground_muted)
                                    .text_size(FONT_SIZE_SM)
                                    .child(format!("Usage Page: {up} | Usage: {u}")),
                            )
                            .child(
                                div()
                                    .text_color(theme.foreground_subtle)
                                    .text_size(FONT_SIZE_CAPTION)
                                    .child(format!("Report IDs: {report_ids_str}")),
                            ),
                    );

                iface_rows = iface_rows.child(row);
            }
        }

        let iface_card = Card::new()
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
                            .child("HID Interfaces"),
                    )
                    .child(iface_rows),
            )
            .render(theme);

        // Capabilities List Card
        let mut caps_chips = div().flex().flex_row().gap(SPACE_2).flex_wrap();

        for cap in &active.summary.capabilities {
            let chip = div()
                .px(SPACE_2)
                .py(SPACE_1)
                .rounded(RADIUS_SM)
                .bg(theme.surface_hover)
                .border_1()
                .border_color(theme.border)
                .text_color(theme.foreground)
                .text_size(FONT_SIZE_CAPTION)
                .child(format!("{cap}"));

            caps_chips = caps_chips.child(chip);
        }

        let caps_card = Card::new()
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
                            .child("Active Capabilities"),
                    )
                    .child(caps_chips),
            )
            .render(theme);

        let content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(hw_card)
            .child(iface_card)
            .child(caps_card);

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

fn render_kv(key: &'static str, value: &str, theme: &crate::theme::Theme) -> Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .py(SPACE_2)
        .border_b_1()
        .border_color(theme.border_subtle)
        .child(
            div()
                .text_color(theme.foreground_muted)
                .text_size(FONT_SIZE_SM)
                .child(key),
        )
        .child(
            div()
                .text_color(theme.foreground)
                .text_size(FONT_SIZE_SM)
                .font_weight(FontWeight::MEDIUM)
                .child(value.to_string()),
        )
}

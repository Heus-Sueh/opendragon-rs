use gpui::prelude::*;
use gpui::*;

use opendragon_core::profile::ProfileId;

use crate::components::{BadgeStatus, Button, Card, InfoTooltip, StatusBadge};
use crate::state::{AppState, DeviceAction};
use crate::theme::{
    FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_MD,
    RADIUS_SM, SPACE_1, SPACE_2, SPACE_3, SPACE_6,
};

pub struct ProfilesView;

impl ProfilesView {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let on_act_switch = on_action.clone();
        let on_switch_profile = move |id: ProfileId, w: &mut Window, a: &mut App| {
            on_act_switch(DeviceAction::SwitchProfile(id), w, a);
        };

        let on_act_add = on_action.clone();
        let on_add_profile = move |name: String, w: &mut Window, a: &mut App| {
            on_act_add(DeviceAction::AddProfile(name), w, a);
        };

        let on_act_del = on_action.clone();
        let on_delete_profile = move |id: ProfileId, w: &mut Window, a: &mut App| {
            on_act_del(DeviceAction::DeleteProfile(id), w, a);
        };
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => return div().child("No active device"),
        };

        // Header with "+ New Profile"
        let on_add = on_add_profile.clone();
        let new_profile_count = active.profiles.len() + 1;
        let new_profile_name = format!("Profile {new_profile_count}");

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
                            .child("Profiles"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Manage on-board keyboard configuration profiles"),
                    ),
            )
            .child(
                Button::primary("+ New profile")
                    .size(crate::components::ButtonSize::Sm)
                    .on_click(move |_e, w, a| {
                        on_add(new_profile_name.clone(), w, a);
                    })
                    .render(theme),
            );

        let mut profile_list = div().flex().flex_col().gap(SPACE_2).w_full();

        for prof in &active.profiles {
            let pid = prof.id;
            let is_active = prof.is_active;
            let on_sw = on_switch_profile.clone();
            let on_del = on_delete_profile.clone();

            let border_col = if is_active {
                theme.accent
            } else {
                theme.border
            };

            let bg_col = if is_active {
                theme.surface_active
            } else {
                theme.surface
            };

            let row = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w_full()
                .px(SPACE_3)
                .py(SPACE_2)
                .rounded(RADIUS_MD)
                .bg(bg_col)
                .border_1()
                .border_color(border_col)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(SPACE_3)
                        .child(
                            div()
                                .text_color(theme.foreground)
                                .text_size(FONT_SIZE_BODY)
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(prof.name.clone()),
                        )
                        .when(is_active, |el| {
                            el.child(
                                StatusBadge::new(BadgeStatus::Connected)
                                    .label("Active")
                                    .render(theme),
                            )
                        }),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(SPACE_2)
                        .when(!is_active, |el| {
                            let on_sw_click = on_sw.clone();
                            el.child(
                                Button::new("Activate")
                                    .size(crate::components::ButtonSize::Sm)
                                    .on_click(move |_e, w, a| on_sw_click(pid, w, a))
                                    .render(theme),
                            )
                        })
                        .when(!is_active && active.profiles.len() > 1, |el| {
                            let on_del_click = on_del.clone();
                            el.child(
                                Button::danger("Delete")
                                    .size(crate::components::ButtonSize::Sm)
                                    .on_click(move |_e, w, a| on_del_click(pid, w, a))
                                    .render(theme),
                            )
                        }),
                );

            profile_list = profile_list.child(row);
        }

        // Educational: Hardware Profiles Explainer Card
        let profile_guide = Card::new()
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
                                    .child("On-Board Hardware EEPROM Profiles"),
                            )
                            .child(InfoTooltip::new("Non-Volatile Storage").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Profiles are stored directly in non-volatile flash memory inside the peripheral. All customized lighting presets, magnetic switch actuation depths, and keystroke macros remain bound to the hardware and function across Linux, Windows, macOS, and gaming consoles without background software."),
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
                                            .child("⚡ Instant Profile Switching"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Quickly toggle between Esports, Gaming, and Productivity profiles via the GUI or hardware Fn key combos."),
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
                                            .child("🔒 Universal Portability"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Plug your peripheral into any tournament PC, work laptop, or Steam Deck and immediately play with calibrated hardware settings."),
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
            .child(profile_list)
            .child(profile_guide);

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

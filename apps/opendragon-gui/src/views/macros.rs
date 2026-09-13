use gpui::prelude::*;
use gpui::*;

use crate::components::{Button, Card, EmptyState, InfoTooltip};
use crate::state::AppState;
use crate::theme::{
    FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM, FONT_SIZE_TITLE, RADIUS_SM,
    SPACE_1, SPACE_2, SPACE_3, SPACE_6,
};

pub struct MacrosView;

impl MacrosView {
    pub fn render(state: &AppState) -> Div {
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
                            .child("Macros"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Create, configure, and bind custom keystroke automation sequences to hardware switches"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(SPACE_3)
                    .child(InfoTooltip::new("EVision Hardware Macro Engine").render(theme))
                    .child(
                        Button::primary("+ New Macro")
                            .size(crate::components::ButtonSize::Sm)
                            .render(theme),
                    ),
            );

        let mut content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header);

        if active.macros.is_empty() {
            let empty = EmptyState::new(
                "No hardware macros recorded yet",
                "Create a macro to automate repetitive keystrokes, competitive gaming combos, or productivity shortcuts.",
            )
            .icon("⌨")
            .action(
                Button::primary("+ New Macro")
                    .size(crate::components::ButtonSize::Sm)
                    .render(theme),
            )
            .render(theme);

            content = content.child(empty);
        } else {
            let mut macro_list = div().flex().flex_col().gap(SPACE_3).w_full();

            for m in &active.macros {
                let mut action_chips = div().flex().flex_row().gap(SPACE_2).flex_wrap();

                for a in &m.actions {
                    let chip = div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(SPACE_2)
                        .px(SPACE_2)
                        .py(SPACE_1)
                        .rounded(RADIUS_SM)
                        .bg(theme.surface_hover)
                        .border_1()
                        .border_color(theme.border)
                        .child(
                            div()
                                .text_color(theme.foreground)
                                .text_size(FONT_SIZE_CAPTION)
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(format!("Key {}", a.key)),
                        )
                        .child(
                            div()
                                .text_color(theme.foreground_subtle)
                                .text_size(FONT_SIZE_CAPTION)
                                .child(format!("{}ms", a.delay_ms)),
                        );

                    action_chips = action_chips.child(chip);
                }

                let card = Card::new()
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
                                            .text_size(FONT_SIZE_BODY)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(m.name.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child(format!("{} recorded actions", m.actions.len())),
                                    ),
                            )
                            .child(action_chips),
                    )
                    .render(theme);

                macro_list = macro_list.child(card);
            }

            content = content.child(macro_list);
        }

        // Step-by-Step Practical Tutorial: How to Use Macros
        let practical_guide = Card::new()
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
                                    .child("How to Create & Use Macros"),
                            )
                            .child(InfoTooltip::new("Complete Usage Walkthrough").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Follow these 4 practical steps to record, customize timings, and bind macro actions to physical keyboard switches:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(SPACE_2)
                            .child(
                                div()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .flex()
                                    .flex_col()
                                    .gap(SPACE_1)
                                    .child(
                                        div()
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("1. Create Macro & Name Your Sequence"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Click '+ New Macro' to create an empty sequence. Give it an intuitive name reflecting its purpose (e.g., 'CS2_JumpThrow', 'Blender_ExtrudeSpin', 'IDE_FormatCode')."),
                                    ),
                            )
                            .child(
                                div()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .flex()
                                    .flex_col()
                                    .gap(SPACE_1)
                                    .child(
                                        div()
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("2. Record Keystrokes & Fine-Tune Delays"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Type your desired sequence. Each action records a Key Down and Key Up event. Add realistic delay intervals (typically 20ms to 50ms) so game engines and input queues register every key accurately."),
                                    ),
                            )
                            .child(
                                div()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .flex()
                                    .flex_col()
                                    .gap(SPACE_1)
                                    .child(
                                        div()
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("3. Choose Execution Policy"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Select how the hardware fires the macro: Play Once (single execution per actuation), Loop While Held (repeats as key remains pressed), or Toggle Loop (starts looping on first press, halts on second)."),
                                    ),
                            )
                            .child(
                                div()
                                    .p(SPACE_3)
                                    .rounded(RADIUS_SM)
                                    .bg(theme.surface_hover)
                                    .flex()
                                    .flex_col()
                                    .gap(SPACE_1)
                                    .child(
                                        div()
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("4. Bind to Physical Key & Flash to Hardware"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("In the Device Overview tab, select any switch on the interactive keyboard preview and assign your macro. OpenDragon-rs transmits the configuration directly to hardware flash."),
                                    ),
                            ),
                    ),
            )
            .render(theme);

        // Core Concepts Card
        let macro_guide = Card::new()
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
                                    .child("How On-Board Hardware Macros Operate"),
                            )
                            .child(InfoTooltip::new("Zero Software Lag").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("A hardware macro is a firmware-level automation sequence executed directly by the keyboard or mouse microcontroller without requiring host CPU intervention:"),
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
                                            .child("⏱️ Hardware Timers & Delays"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Precise millisecond delays handled by the internal timer of the MCU, preventing dropped frames or anti-cheat flags."),
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
                                            .child("🔄 Flexible Loop Modes"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Play Once, While Held, or Toggle Loop provide versatile control for rhythm gaming, rapid-fire, and complex MMO skill rotations."),
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
                                            .text_color(theme.accent)
                                            .text_size(FONT_SIZE_SM)
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("💾 On-Board Flash Storage"),
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.foreground_muted)
                                            .text_size(FONT_SIZE_CAPTION)
                                            .child("Saved in non-volatile flash memory: works on any PC, Linux workstation, tournament rig, or Steam Deck with no drivers installed."),
                                    ),
                            ),
                    ),
            )
            .render(theme);

        content = content.child(practical_guide);
        content = content.child(macro_guide);

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

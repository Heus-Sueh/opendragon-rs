use gpui::prelude::*;
use gpui::*;

use opendragon_core::color::RgbColor;
use opendragon_core::lighting::{LightingConfig, LightingMode};

use crate::components::{Card, InfoTooltip, KeyboardPreview, MousePreview, Slider};
use crate::state::{AppState, DeviceAction};
use crate::theme::{
    CONTROL_HEIGHT_SM, FONT_SIZE_BODY, FONT_SIZE_CAPTION, FONT_SIZE_HEADING, FONT_SIZE_SM,
    FONT_SIZE_TITLE, RADIUS_MD, RADIUS_SM, SPACE_1, SPACE_12, SPACE_2, SPACE_3, SPACE_4, SPACE_6,
};

pub struct LightingView;

impl LightingView {
    pub fn render(
        state: &AppState,
        on_action: impl Fn(DeviceAction, &mut Window, &mut App) + 'static + Clone,
    ) -> Div {
        let on_act = on_action.clone();
        let on_update_lighting = move |cfg: LightingConfig, w: &mut Window, a: &mut App| {
            on_act(DeviceAction::UpdateLighting(cfg), w, a);
        };
        let theme = &state.theme;
        let active = match &state.active_device {
            Some(dev) => dev,
            None => return div().child("No active device"),
        };

        let current_cfg = active.lighting.clone();

        // Preset Colors (16 Curated Vibrant Palettes)
        let presets = [
            ("#E82A3B", "Redragon", RgbColor::new(232, 42, 59)),
            ("#DC2626", "Crimson", RgbColor::new(220, 38, 38)),
            ("#F97316", "Coral", RgbColor::new(249, 115, 22)),
            ("#F59E0B", "Amber", RgbColor::new(245, 158, 11)),
            ("#EAB308", "Gold", RgbColor::new(234, 179, 8)),
            ("#84CC16", "Neon Lime", RgbColor::new(132, 204, 22)),
            ("#10B981", "Emerald", RgbColor::new(16, 185, 129)),
            ("#06B6D4", "Cyber Cyan", RgbColor::new(6, 182, 212)),
            ("#0EA5E9", "Sky Blue", RgbColor::new(14, 165, 233)),
            ("#3B82F6", "Royal Blue", RgbColor::new(59, 130, 246)),
            ("#6366F1", "Indigo", RgbColor::new(99, 102, 241)),
            ("#8B5CF6", "Deep Purple", RgbColor::new(139, 92, 246)),
            ("#D946EF", "Magenta", RgbColor::new(217, 70, 239)),
            ("#EC4899", "Hot Pink", RgbColor::new(236, 72, 153)),
            ("#E0F2FE", "Ice Blue", RgbColor::new(224, 242, 254)),
            ("#FFFFFF", "Pure White", RgbColor::WHITE),
        ];

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
                            .child("Lighting"),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child("Configure all 18 hardware lighting modes, animation speed, brightness, and custom RGB colors"),
                    ),
            )
            .child(InfoTooltip::new("EVision HID Lighting Protocol").render(theme));

        let is_mouse = active
            .summary
            .capabilities
            .contains(&opendragon_core::capability::Capability::Dpi)
            && !active
                .summary
                .capabilities
                .contains(&opendragon_core::capability::Capability::Actuation);

        let hardware_canvas = if is_mouse {
            let on_act_btn = on_action.clone();
            let mouse_click_handler =
                std::sync::Arc::new(move |btn_id: u8, w: &mut Window, a: &mut App| {
                    on_act_btn(DeviceAction::ToggleKeySelection(btn_id), w, a);
                });
            MousePreview::render(
                &active.visual_state,
                active.dpi,
                theme,
                Some(mouse_click_handler),
            )
        } else {
            let on_act_kc = on_action.clone();
            let key_click_handler =
                std::sync::Arc::new(move |key_id: u8, w: &mut Window, a: &mut App| {
                    on_act_kc(DeviceAction::ToggleKeySelection(key_id), w, a);
                });

            let on_act_layout = on_action.clone();
            let layout_change_handler =
                std::sync::Arc::new(move |layout, w: &mut Window, a: &mut App| {
                    on_act_layout(DeviceAction::SetKeyboardLayout(layout), w, a);
                });

            KeyboardPreview::render(
                &active.visual_state,
                theme,
                state.preferences.keyboard_layout,
                Some(key_click_handler),
                Some(layout_change_handler),
            )
        };

        let canvas_stage = div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .p(SPACE_3)
            .rounded(RADIUS_MD)
            .bg(theme.surface_elevated)
            .border_1()
            .border_color(theme.border_subtle)
            .child(hardware_canvas);

        // Grouped Mode Categories for all 18 K552RGB-PRO Hardware Effects
        let mode_categories = [
            (
                "Dynamic & Flow",
                vec![
                    (LightingMode::Wave, "RGB Wave"),
                    (LightingMode::Rainbow, "Rainbow Cycle"),
                    (LightingMode::Wheel, "Color Wheel"),
                    (LightingMode::VerticalWave, "Vertical Wave"),
                    (LightingMode::RainbowCircle, "Rainbow Circle"),
                ],
            ),
            (
                "Atmospheric & Ambient",
                vec![
                    (LightingMode::Breathing, "Breathing"),
                    (LightingMode::Flash, "Stars / Flash"),
                    (LightingMode::Rain, "Digital Rain"),
                    (LightingMode::Blooming, "Blooming"),
                    (LightingMode::Whirlpool, "Whirlpool"),
                    (LightingMode::Accumulate, "Cascade"),
                    (LightingMode::Visor, "Visor Scanner"),
                    (LightingMode::Surmount, "Collision"),
                ],
            ),
            (
                "Key Reactive",
                vec![
                    (LightingMode::Reactive, "Reactive"),
                    (LightingMode::Ripple, "Ripple Wave"),
                    (LightingMode::GoWithoutTrace, "Line Reactive"),
                ],
            ),
            (
                "Solid & Utilities",
                vec![
                    (LightingMode::Static, "Static Solid"),
                    (LightingMode::Custom, "Per-Key Custom"),
                    (LightingMode::Off, "Backlight Off"),
                ],
            ),
        ];

        let mut mode_card_content = div().flex().flex_col().gap(SPACE_3);

        for (category_title, cat_modes) in mode_categories {
            let mut cat_buttons = div().flex().flex_row().gap(SPACE_2).flex_wrap();

            for (mode, label) in cat_modes {
                let is_active = current_cfg.mode == mode;
                let mut cfg_clone = current_cfg.clone();
                cfg_clone.mode = mode;
                let on_upd = on_update_lighting.clone();

                let btn = div()
                    .h(CONTROL_HEIGHT_SM)
                    .px(SPACE_2)
                    .flex()
                    .items_center()
                    .rounded(RADIUS_SM)
                    .bg(if is_active {
                        theme.accent
                    } else {
                        theme.surface_hover
                    })
                    .hover(move |s| {
                        if !is_active {
                            s.bg(theme.surface_active)
                        } else {
                            s
                        }
                    })
                    .text_color(if is_active {
                        theme.accent_foreground
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
                        on_upd(cfg_clone.clone(), w, a);
                    })
                    .child(label);

                cat_buttons = cat_buttons.child(btn);
            }

            let cat_section = div()
                .flex()
                .flex_col()
                .gap(SPACE_1)
                .child(
                    div()
                        .text_color(theme.foreground_muted)
                        .text_size(FONT_SIZE_CAPTION)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(category_title),
                )
                .child(cat_buttons);

            mode_card_content = mode_card_content.child(cat_section);
        }

        let mode_card = Card::new()
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
                                    .child("Hardware Lighting Modes (18 Built-in Effects)"),
                            )
                            .child(InfoTooltip::new("Direct MCU hardware effect").render(theme)),
                    )
                    .child(mode_card_content),
            )
            .render(theme);

        // Active Effect Explanation Spotlight Card
        let (eff_title, eff_summary, eff_details) = effect_info(current_cfg.mode);
        let spotlight_card = Card::new()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(SPACE_2)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .text_size(FONT_SIZE_HEADING)
                                    .font_weight(FontWeight::BOLD)
                                    .child(eff_title),
                            )
                            .child(InfoTooltip::new("Active Preset Info").render(theme)),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground)
                            .text_size(FONT_SIZE_BODY)
                            .font_weight(FontWeight::MEDIUM)
                            .child(eff_summary),
                    )
                    .child(
                        div()
                            .text_color(theme.foreground_muted)
                            .text_size(FONT_SIZE_SM)
                            .child(eff_details),
                    ),
            )
            .render(theme);

        // Color Section (shown for modes that use configurable colors)
        let show_color = !matches!(
            current_cfg.mode,
            LightingMode::Rainbow
                | LightingMode::Wheel
                | LightingMode::VerticalWave
                | LightingMode::RainbowCircle
                | LightingMode::Off
        );

        let color_section = if show_color {
            let mut swatch_row = div()
                .flex()
                .flex_row()
                .gap(SPACE_2)
                .flex_wrap()
                .items_center();

            for (_hex, _name, rgb) in presets {
                let mut cfg_clone = current_cfg.clone();
                cfg_clone.color = rgb;
                let on_upd = on_update_lighting.clone();
                let is_selected = current_cfg.color == rgb;

                let swatch = div()
                    .size(px(24.0))
                    .rounded(RADIUS_SM)
                    .bg(rgb_to_hsla_local(rgb))
                    .border_1()
                    .border_color(if is_selected {
                        theme.accent
                    } else {
                        theme.border
                    })
                    .cursor_pointer()
                    .hover(|s| s.border_color(theme.accent))
                    .on_mouse_down(MouseButton::Left, move |_e, w, a| {
                        on_upd(cfg_clone.clone(), w, a);
                    });

                swatch_row = swatch_row.child(swatch);
            }

            let hex_value = current_cfg.color.to_hex();

            // Custom RGB Mixer Sliders (Red, Green, Blue 0..255)
            let on_upd_r = on_update_lighting.clone();
            let base_cfg_r = current_cfg.clone();
            let red_slider = Slider::new("Red (R)", current_cfg.color.r as f32, 0.0, 255.0)
                .flat(true)
                .step(1.0)
                .max_width(px(520.0))
                .accent_color(hsla(0.0, 0.85, 0.55, 1.0))
                .presets([
                    (0.0, "0"),
                    (64.0, "64"),
                    (128.0, "128"),
                    (192.0, "192"),
                    (255.0, "255"),
                ])
                .on_change(move |val, w, a| {
                    let mut cfg = base_cfg_r.clone();
                    cfg.color.r = val as u8;
                    on_upd_r(cfg, w, a);
                })
                .render(theme);

            let on_upd_g = on_update_lighting.clone();
            let base_cfg_g = current_cfg.clone();
            let green_slider = Slider::new("Green (G)", current_cfg.color.g as f32, 0.0, 255.0)
                .flat(true)
                .step(1.0)
                .max_width(px(520.0))
                .accent_color(hsla(140.0, 0.70, 0.45, 1.0))
                .presets([
                    (0.0, "0"),
                    (64.0, "64"),
                    (128.0, "128"),
                    (192.0, "192"),
                    (255.0, "255"),
                ])
                .on_change(move |val, w, a| {
                    let mut cfg = base_cfg_g.clone();
                    cfg.color.g = val as u8;
                    on_upd_g(cfg, w, a);
                })
                .render(theme);

            let on_upd_b = on_update_lighting.clone();
            let base_cfg_b = current_cfg.clone();
            let blue_slider = Slider::new("Blue (B)", current_cfg.color.b as f32, 0.0, 255.0)
                .flat(true)
                .step(1.0)
                .max_width(px(520.0))
                .accent_color(hsla(215.0, 0.85, 0.55, 1.0))
                .presets([
                    (0.0, "0"),
                    (64.0, "64"),
                    (128.0, "128"),
                    (192.0, "192"),
                    (255.0, "255"),
                ])
                .on_change(move |val, w, a| {
                    let mut cfg = base_cfg_b.clone();
                    cfg.color.b = val as u8;
                    on_upd_b(cfg, w, a);
                })
                .render(theme);

            let color_card = Card::new()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(SPACE_4)
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
                                        .child("Color Palette & Custom RGB Mixer"),
                                )
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
                                                .bg(rgb_to_hsla_local(current_cfg.color))
                                                .border_1()
                                                .border_color(theme.border),
                                        )
                                        .child(
                                            div()
                                                .text_color(theme.accent)
                                                .text_size(FONT_SIZE_SM)
                                                .font_weight(FontWeight::BOLD)
                                                .child(hex_value),
                                        )
                                        .child(InfoTooltip::new("24-bit True Color").render(theme)),
                                ),
                        )
                        .child(swatch_row)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(SPACE_3)
                                .max_w(px(560.0))
                                .child(
                                    div()
                                        .text_color(theme.foreground_muted)
                                        .text_size(FONT_SIZE_SM)
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("Fine-tune individual RGB channels (0 to 255):"),
                                )
                                .child(red_slider)
                                .child(green_slider)
                                .child(blue_slider),
                        ),
                )
                .render(theme);

            Some(color_card)
        } else {
            None
        };

        // Brightness Slider (shown for any non-off mode)
        let show_brightness = current_cfg.mode != LightingMode::Off;
        let brightness_card = if show_brightness {
            let base_cfg_b = current_cfg.clone();
            let on_upd = on_update_lighting.clone();

            let slider = Slider::new("LED Brightness", current_cfg.brightness as f32, 0.0, 100.0)
                .step(5.0)
                .unit("%")
                .max_width(px(560.0))
                .presets([
                    (0.0, "Off (0%)"),
                    (25.0, "25%"),
                    (50.0, "50%"),
                    (75.0, "75%"),
                    (100.0, "Max (100%)"),
                ])
                .on_change(move |val, w, a| {
                    let mut cfg = base_cfg_b.clone();
                    cfg.brightness = val as u8;
                    on_upd(cfg, w, a);
                })
                .render(theme);

            Some(slider)
        } else {
            None
        };

        // Speed Slider (shown for all animated modes)
        let show_speed = !matches!(
            current_cfg.mode,
            LightingMode::Static | LightingMode::Off | LightingMode::Custom
        );

        let speed_card = if show_speed {
            let base_cfg_s = current_cfg.clone();
            let on_upd = on_update_lighting.clone();

            let speed_level = if current_cfg.speed <= 5 {
                current_cfg.speed.max(1) as f32
            } else {
                (((current_cfg.speed as f32) / 20.0).round() as u8).clamp(1, 5) as f32
            };

            let slider = Slider::new("Effect Animation Speed", speed_level, 1.0, 5.0)
                .step(1.0)
                .unit(" / 5")
                .max_width(px(560.0))
                .presets([
                    (1.0, "1 (Slowest)"),
                    (2.0, "2 (Slow)"),
                    (3.0, "3 (Normal)"),
                    (4.0, "4 (Fast)"),
                    (5.0, "5 (Turbo)"),
                ])
                .on_change(move |val, w, a| {
                    let mut cfg = base_cfg_s.clone();
                    cfg.speed = val as u8;
                    on_upd(cfg, w, a);
                })
                .render(theme);

            Some(slider)
        } else {
            None
        };

        let mut content = div()
            .flex()
            .flex_col()
            .gap(SPACE_6)
            .w_full()
            .max_w(crate::theme::CONTENT_MAX_WIDTH)
            .child(header)
            .child(canvas_stage)
            .child(mode_card)
            .child(spotlight_card);

        if let Some(cc) = color_section {
            content = content.child(cc);
        }
        if let Some(bc) = brightness_card {
            content = content.child(bc);
        }
        if let Some(sc) = speed_card {
            content = content.child(sc);
        }

        // Uses min_h_full() to allow vertical scrolling when content overflows
        div()
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .min_h_full()
            .p(SPACE_6)
            .pb(SPACE_12)
            .child(content)
    }
}

fn effect_info(mode: LightingMode) -> (&'static str, &'static str, &'static str) {
    match mode {
        LightingMode::Static => (
            "💡 Static Solid Mode",
            "Stable continuous illumination in the selected solid color.",
            "Provides maximum visual clarity and ergonomic focus. Excellent for programming, typing, or composing a minimalist setup without distracting animations.",
        ),
        LightingMode::Breathing => (
            "🫁 Breathing Mode",
            "Gentle illumination pulsing that softly fades in and out.",
            "LEDs smoothly increase in brightness and gradually fade to black in rhythmic cycles, simulating deep, calm respiration.",
        ),
        LightingMode::Wave => (
            "🌊 RGB Wave Mode",
            "Dynamic color waves flowing continuously across the entire keyboard chassis.",
            "A classic mechanical keyboard effect where gradient bands travel across key rows. Adjust speed to accelerate or calm the wave flow.",
        ),
        LightingMode::Rainbow => (
            "🌈 Rainbow Spectrum Cycle",
            "Continuous smooth chromatic transition traversing the full RGB spectrum.",
            "Seamless transitions between vivid red, amber, green, cyan, blue, and magenta tones with zero jarring jumps.",
        ),
        LightingMode::Reactive => (
            "⚡ Key Reactive Mode",
            "Keys illuminate instantaneously the exact millisecond they are pressed down.",
            "Each keystroke lights up immediately and fades softly upon key release, providing crisp, immersive physical tactile feedback.",
        ),
        LightingMode::Ripple => (
            "💧 Ripple Wave Mode",
            "Concentric rings of light radiate outwards from every pressed key.",
            "Striking a key sends a circular wave propagating across surrounding switches, much like water droplets impacting a calm pond.",
        ),
        LightingMode::Flash => (
            "✨ Starlight / Flash Mode",
            "Random twinkling light pulses simulating stars in the night sky.",
            "Individual key switches sparkle and pulse at varied coordinates, creating a dynamic, cosmic ambiance.",
        ),
        LightingMode::GoWithoutTrace => (
            "🏃 Line Reactive Mode (Go Without Trace)",
            "Trailing beam reactive lighting that dissipates smoothly along typed keys.",
            "Offers high contrast visual rhythm and responsive tracking for fast touch typists and rhythm gamers.",
        ),
        LightingMode::Wheel => (
            "🎡 Color Wheel Mode",
            "Continuous 360-degree rotational spectrum shifting across the keyboard.",
            "Simulates a circular chromatic wheel rotating around the center of the key matrix.",
        ),
        LightingMode::Blooming => (
            "🌸 Blooming Mode",
            "Radial pulses originating from the center and blooming smoothly outward.",
            "An elegant, harmonious floral effect that fills keys with expanding concentric waves of light.",
        ),
        LightingMode::VerticalWave => (
            "↕️ Vertical Wave Mode",
            "Colored lighting bands cascading vertically from top to bottom.",
            "A fluid vertical alternative to the horizontal wave, conveying the sensation of a luminous waterfall.",
        ),
        LightingMode::Whirlpool => (
            "🌀 Whirlpool / Hurricane Mode",
            "High-energy spiral rotation simulating a dynamic vortex of light.",
            "Energetic pattern with fast chromatic rotations sweeping like a hurricane.",
        ),
        LightingMode::Accumulate => (
            "🪜 Cascade / Accumulate Mode",
            "Light droplets fill keys row by row until the keyboard is saturated, then reset.",
            "A classic stacking animation that progressively fills key rows with luminous color.",
        ),
        LightingMode::Rain => (
            "🌧️ Digital Rain Mode (Matrix Rain)",
            "Luminous droplets descending vertically down key columns.",
            "Inspired by iconic cyberpunk digital rainfall, ideal for dark coding environments.",
        ),
        LightingMode::Visor => (
            "🚨 Visor Scanner Mode (Knight Rider)",
            "Concentrated light beam sweeping back and forth horizontally.",
            "Futuristic retro scanner visual with perpetual, smooth horizontal sweeps.",
        ),
        LightingMode::Surmount => (
            "💥 Collision Mode (Surmount)",
            "Opposing waves rush inward from outer edges, collide at the center, and rebound.",
            "A high-impact, symmetrical kinetic effect with dynamic center dissipation.",
        ),
        LightingMode::RainbowCircle => (
            "⭕ Rainbow Circle Mode",
            "Concentric circular rainbow rings expanding rhythmically.",
            "Hypnotic, smooth visual combining all spectrum hues within geometric expanding rings.",
        ),
        LightingMode::Off => (
            "🌑 Backlight Off",
            "All LED backlighting is completely shut down.",
            "Conserves energy on laptops/portable devices and eliminates all light emission for distraction-free stealth environments.",
        ),
        LightingMode::Custom => (
            "🎨 Per-Key Custom Mode",
            "Individually assigned RGB colors on each switch.",
            "Assign distinct colors to WASD, arrow clusters, modifiers, and function keys independently.",
        ),
    }
}

fn rgb_to_hsla_local(color: RgbColor) -> Hsla {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < 1e-5 {
        return hsla(0.0, 0.0, l, 1.0);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let h = if (max - r).abs() < 1e-5 {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < 1e-5 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };

    hsla(h, s, l, 1.0)
}

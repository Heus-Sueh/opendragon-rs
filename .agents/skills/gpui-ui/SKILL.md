---
name: gpui-ui
description: Best practices, architecture patterns, design system guidelines, and component conventions for developing native desktop user interfaces in OpenDragon using GPUI-CE.
---

# GPUI-CE Desktop UI Guidelines for OpenDragon

This guide instructs AI agents and developers on building and maintaining native desktop GUI features in `apps/opendragon-gui` using `gpui-ce`.

---

## 1. Architectural Principles

1. **Strict Protocol / UI Decoupling**:
   - The GUI must NEVER construct raw USB/HID byte buffers, parse report IDs, or perform low-level packet encoding.
   - All interactions must go through the `DeviceBackend` trait (`services/devices.rs`), using domain types from `opendragon-core` (`LightingConfig`, `ActuationPoint`, `RapidTriggerConfig`, `PollingRate`, `ProfileId`).
2. **Capability-Aware Navigation**:
   - Sidebar navigation and view controls adapt dynamically based on `device.capabilities()`.
   - Never show features that the connected device does not support (e.g., actuation points on a standard mechanical switch keyboard).
   - Contextually hide non-applicable controls (e.g., speed slider when lighting mode is `Static` or `Off`).
3. **Restrained, Modern Aesthetics**:
   - Modern, restrained desktop aesthetic (inspired by clean developer tools like Linear and Zed).
   - Avoid aggressive gamer chrome, neon borders, or harsh contrasts.
   - RGB is visually featured inside the dynamic keyboard preview chassis (`KeyboardPreview`), not as UI decoration.
4. **Mock Backend for Headless Development**:
   - Support development and validation without physical hardware attached using `--features mock-device`.
   - `MockDeviceBackend` emulates real hardware behaviors, bounds, and profile storage.

---

## 2. Design System Tokens (`apps/opendragon-gui/src/theme/`)

Always use semantic tokens rather than magic numbers or hardcoded pixel values:

- **Colors (`theme/colors.rs`)**:
  - `theme.background`: Main window background.
  - `theme.surface`, `theme.surface_hover`, `theme.surface_active`: Card and container levels.
  - `theme.border`, `theme.border_subtle`: Dividers and keycap outlines.
  - `theme.text`, `theme.text_muted`, `theme.text_dim`: Typography hierarchy.
  - `theme.accent`, `theme.accent_hover`: Active selection and interactive highlights.
  - `theme.success`, `theme.warning`, `theme.danger`: Status badges and alerts.
- **Spacing (`theme/spacing.rs`)**:
  - `SPACE_1` (4px), `SPACE_2` (8px), `SPACE_3` (12px), `SPACE_4` (16px), `SPACE_6` (24px), `SPACE_8` (32px), `SPACE_12` (48px).
  - `SIDEBAR_WIDTH`: 220px.
  - `CONTENT_MAX_WIDTH`: 1000px.
- **Radius (`theme/radius.rs`)**:
  - `RADIUS_SM` (4px), `RADIUS_MD` (8px), `RADIUS_LG` (12px), `RADIUS_XL` (16px), `RADIUS_FULL` (9999px).
- **Typography (`theme/typography.rs`)**:
  - `FONT_SIZE_XS` (11px), `FONT_SIZE_SM` (13px), `FONT_SIZE_BASE` (14px), `FONT_SIZE_MD` (16px), `FONT_SIZE_LG` (18px), `FONT_SIZE_XL` (20px), `FONT_SIZE_TITLE` (24px).

---

## 3. GPUI Component Conventions

- **Callback Closures**:
  - GPUI window listeners (`on_mouse_down`) run on the main UI thread and require `'static` closures.
  - Callbacks stored in structs should use `Option<Arc<dyn Fn(...) + 'static>>` or `Option<Box<dyn Fn(...) + 'static>>`.
  - Type aliases should be defined for complex callbacks (e.g., `pub type SliderChangeHandler = Arc<dyn Fn(f32, &mut Window, &mut App) + 'static>;`) to satisfy `clippy::type-complexity`.
- **Scrollable Containers**:
  - GPUI requires stateful divs with `.id("identifier")` before calling `.overflow_y_scroll()`.
  ```rust
  div()
      .id("page_scroll_area")
      .flex_1()
      .h_full()
      .overflow_y_scroll()
      .child(page_content)
  ```
- **State Mutation within `Fn` Callbacks**:
  - When a closure passed to an `Fn` parameter needs to update a struct value and invoke an update handler, clone or copy the initial struct outside, clone or copy inside the closure, mutate the local variable, and pass it to the handler:
  ```rust
  let base_cfg = active.rapid_trigger;
  let on_change = on_update.clone();
  Slider::new(...)
      .on_change(move |val, w, a| {
          let mut cfg = base_cfg;
          cfg.press_sensitivity_mm = val;
          on_change(cfg, w, a);
      })
  ```

---

## 4. Running and Testing the GUI

```bash
# Run GUI with simulated hardware (K552RGB-PRO mock)
cargo run -p opendragon-gui --features mock-device

# Run GUI with real connected hardware
cargo run -p opendragon-gui

# Verification Quality Gates
cargo fmt --check
cargo clippy -p opendragon-gui --all-targets --all-features -- -D warnings
cargo test -p opendragon-gui
```

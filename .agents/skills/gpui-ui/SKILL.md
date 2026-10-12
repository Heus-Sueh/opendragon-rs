---
name: gpui-ui
description: Best practices, architecture patterns, visual design guidelines, and component conventions for developing native desktop user interfaces with GPUI-CE.
---

# GPUI-CE Desktop UI Guidelines

This guide establishes architecture patterns, design system guidelines, component conventions, and visual discipline for developing high-performance native desktop applications using `gpui` / `gpui-ce`.

---

## 1. Architectural Foundations

### 1.1 Strict Business / UI Decoupling
- **Decoupled Backend**: Views must never execute raw transport I/O, construct network/device packets, or execute blocking low-level operations directly.
- **Domain Modeling**: The GUI communicates strictly through typed service traits or state entities using strongly-typed domain structs and enums.
- **Capability-Driven Views**: Elements and sidebar navigation adapt dynamically based on reported capabilities. Never render non-applicable or unsupported controls for the current runtime target.

### 1.2 State Management & Dispatcher Seam
- **Single Action Dispatcher**: Prefer an explicit enum action dispatch pattern (`AppAction` / `DeviceAction`) over scattered ad-hoc state mutations across deep component trees.
- **GPUI Entities**: Keep application state in a central `Entity<AppState>`. Views observe state updates via `cx.observe(&state, ...)` to trigger automatic reactive re-renders.
- **Mock Mode for Headless Development**: Maintain a mock backend feature (e.g., `--features mock-device`) to enable rapid visual iteration, CI runs, and tests without requiring physical hardware or external services.

---

## 2. Desktop-First Ergonomics

GPUI builds native desktop applications, not mobile apps or marketing websites:

1. **Mouse & Keyboard Efficiency**:
   - Provide discoverable keyboard shortcuts (`Ctrl+1`..`Ctrl+8`, `Esc`, `Ctrl+R`) with visible accelerator hints.
   - Use `track_focus(&self.focus_handle)`, `key_context("Shell")`, and `on_key_down` for global navigation.
2. **Compact Control Heights**:
   - Target desktop pointer precision rather than oversized touch targets:
     - Small: `26px` – `28px` (compact table actions, inline badges)
     - Medium: `30px` – `32px` (standard buttons, inputs, segmented pills)
     - Large: `36px` – `38px` (primary page action triggers)
3. **Master-Detail & Split Stages**:
   - Take advantage of wide desktop viewports with responsive master-detail layouts (e.g. stage/canvas on the left, contextual inspector dock on the right).

---

## 3. Visual Aesthetics & Design Discipline

Incorporate principles from modern design studios and precision developer tools (Linear, Zed, Xcode):

### 3.1 Spend Boldness in One Place
- Let **one element** be the memorable centerpiece (e.g., interactive hardware canvas, live audio graph, editor buffer, timeline).
- Keep all surrounding window chrome disciplined, matte, and quiet.
- Do not spray vibrant accents or gradients across standard panels.

### 3.2 Eliminate AI & Generic Web Tells
Reject generic AI-generated interface templates:
- **No SaaS Card Kit**: Never chop every section, slider, and toggle into identical bordered cards. Nested cards inside cards create visual noise. Prefer whitespace, typography, and subtle surface elevation.
- **No ALL-CAPS Eyebrows**: Do not add tracked-out uppercase labels (e.g., `CONTEXTUAL INSPECTOR` or `SETTINGS OVERVIEW`) above section headings. Use clean sentence case.
- **No Trailing Arrows**: Do not append generic `→` to button labels or links (`Save →`, `Configure →`). Use plain verbs or subtle platform chevrons (`›`).
- **No Casual Emojis**: Do not decorate technical cards with emoji bullets (`🎯`, `✍️`, `⚡`). Use precise labels, units, and clear typographic hierarchy.
- **Sentence Case Everywhere**: Use sentence case for buttons, headings, and labels (`+ New profile`, `Sensor polling rate`, `Refresh devices`).

---

## 4. Design System Tokens (`theme/`)

Always use semantic tokens rather than magic numbers or hardcoded pixel values:

### 4.1 Color System & Surface Hierarchy
- `theme.background`: Deepest window base (matte slate / graphite).
- `theme.surface`: Standard container and card level.
- `theme.surface_elevated`: Inspector docks, popovers, and elevated panels.
- `theme.surface_hover` & `theme.surface_active`: Interaction states.
- `theme.border` & `theme.border_subtle`: Hairline dividers and outlines.
- `theme.foreground`: Primary crisp text (high contrast).
- `theme.foreground_muted` & `theme.foreground_subtle`: Secondary descriptions and shortcut hints.
- `theme.accent` & `theme.accent_subtle`: Brand highlight reserved for active selection and key metrics.
- `theme.success`, `theme.warning`, `theme.danger`: Semantic state indicators.

### 4.2 Spacing & Radius Scales
```rust
pub const SPACE_1: Pixels = px(4.0);
pub const SPACE_2: Pixels = px(8.0);
pub const SPACE_3: Pixels = px(12.0);
pub const SPACE_4: Pixels = px(16.0);
pub const SPACE_6: Pixels = px(24.0);
pub const SPACE_8: Pixels = px(32.0);

pub const RADIUS_SM: Pixels = px(3.0);  // Badges, chips, small inputs
pub const RADIUS_MD: Pixels = px(5.0);  // Cards, panels, dialogs
pub const RADIUS_LG: Pixels = px(8.0);  // Canvas frames
pub const RADIUS_FULL: Pixels = px(9999.0); // Pill badges, circular dots
```

### 4.3 Typography Scale
- Title: `20px` Bold (`FONT_SIZE_TITLE`)
- Heading: `15px` – `16px` SemiBold (`FONT_SIZE_HEADING`)
- Body: `13px` – `14px` Regular (`FONT_SIZE_BODY`)
- Caption / Meta: `11px` Medium (`FONT_SIZE_CAPTION`)

---

## 5. GPUI Component Implementation Conventions

### 5.1 Flat vs Bordered Controls
Form controls (`Slider`, `Switch`, `TextInput`) should support a `.flat(true)` mode:
- When rendered standalone: Can display subtle padding and background.
- When nested inside cards, inspector docks, or sections: Render without redundant outer borders or nested card frames.

### 5.2 Callback Closures & Lifetimes
- GPUI window listeners (`on_mouse_down`, `on_click`) run on the main UI thread and require `'static` closures.
- Callbacks stored in component structs should use `Option<Arc<dyn Fn(...) + 'static>>` or `Option<Box<dyn Fn(...) + 'static>>`.
- Use type aliases for complex callbacks (e.g. `pub type ChangeHandler = Arc<dyn Fn(f32, &mut Window, &mut App) + 'static>;`) to satisfy `clippy::type-complexity`.

### 5.3 Local State Mutation within Closures
When a closure passed to an `Fn` parameter needs to update a struct and invoke an update handler, clone outside, clone inside, mutate locally, and invoke the handler:
```rust
let base_cfg = active.settings.clone();
let on_change = on_update.clone();
Slider::new("Sensitivity", base_cfg.sensitivity, 0.1, 4.0)
    .flat(true)
    .on_change(move |val, window, cx| {
        let mut cfg = base_cfg.clone();
        cfg.sensitivity = val;
        on_change(cfg, window, cx);
    })
```

### 5.4 Stateful Scroll Containers
GPUI requires stateful divs with `.id("...")` before calling `.overflow_y_scroll()`:
```rust
div()
    .id("main_scroll_area")
    .flex_1()
    .h_full()
    .overflow_y_scroll()
    .child(content)
```

---

## 6. Verification Quality Gates

Before declaring any GUI feature complete:

```bash
# 1. Code formatting
cargo fmt --check

# 2. Strict linting (zero warnings)
cargo clippy -p <gui-crate> --all-targets --all-features -- -D warnings

# 3. Unit and component tests
cargo test -p <gui-crate>

# 4. Interactive runtime validation with simulated backend
cargo run -p <gui-crate> --features mock-device
```

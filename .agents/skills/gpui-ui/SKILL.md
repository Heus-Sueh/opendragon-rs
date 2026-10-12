---
name: gpui-ui
description: Best practices, architecture patterns, visual design guidelines, and component conventions for developing native desktop user interfaces with GPUI-CE and gpui-kit.
---

# GPUI & gpui-kit Desktop UI Guidelines

This skill provides comprehensive, reusable architectural patterns, design system guidelines, component conventions, and visual discipline for developing high-performance, native desktop applications in Rust using **GPUI** (`gpui` / `gpui-pre`) and the **`gpui-kit`** (`gpui-component`) ecosystem.

---

## 1. Ecosystem Overview

- **GPUI**: A GPU-accelerated native UI framework developed for high-frame-rate desktop applications (originally created for the Zed editor). It uses retained state with immediate-mode declarative rendering, drawing directly to GPU textures via Vulkan, Metal, or DirectX.
- **gpui-kit**: The community standard component library for GPUI (inspired by shadcn/ui and Radix UI). It provides tested, accessible, styled desktop components, eliminating the need to build primitive buttons, menus, cards, and toggles from scratch.

---

## 2. Component Primitives Reference (`gpui-kit`)

**Rule:** Always prefer official `gpui-kit` components over rolling custom implementations with raw `div()`s.

### 2.1 Navigation Sidebar (`gpui_kit::component::sidebar`)
Use for persistent desktop navigation shells:
```rust
use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};

let mut menu = SidebarMenu::new();
menu = menu.child(
    SidebarMenuItem::new("nav_home")
        .label("Overview")
        .icon(IconName::Home)
        .active(is_active)
        .on_click(move |_, window, cx| {
            // handle navigation
        }),
);

let sidebar = Sidebar::new("main_sidebar")
    .header(SidebarHeader::new().child(brand_header))
    .child(SidebarGroup::new("Sections").child(menu))
    .footer(SidebarFooter::new().child(status_footer));
```

### 2.2 Containers & Group Boxes (`gpui_kit::component::group_box`)
Use for structured panels, cards, and grouped sections:
```rust
use gpui_kit::component::group_box::GroupBox;

let card = GroupBox::new()
    .outline() // or .fill()
    .child(content);
```

### 2.3 Buttons & Button Groups (`gpui_kit::component::button`)
Use for standalone triggers or segmented pickers:
```rust
use gpui_kit::component::button::{Button as KitButton, ButtonGroup, ButtonVariants};
use gpui_kit::component::{Sizable, Size};

// Standalone button
let btn = KitButton::new("submit_action")
    .label("Save changes")
    .primary() // or .secondary(), .ghost(), .danger()
    .with_size(Size::Medium)
    .on_click(move |_event, window, cx| {
        // dispatch action
    });

// Compact segmented control group
let mut group = ButtonGroup::new("segmented_picker")
    .compact()
    .with_size(Size::Small);

for (item_id, item_label) in items {
    let mut b = KitButton::new(item_id).label(item_label);
    if item_id == selected_id {
        b = b.primary();
    } else {
        b = b.secondary();
    }
    group = group.child(b);
}
```

### 2.4 Form Toggles (`gpui_kit::component::switch`)
Native spring-animated desktop switches:
```rust
use gpui_kit::component::switch::Switch;

let toggle = Switch::new("Enable hardware acceleration", is_enabled)
    .flat(true)
    .description("Offload shader and compute operations to GPU")
    .on_toggle(move |new_val, window, cx| {
        // dispatch toggle
    });
```

### 2.5 Status Badges (`gpui_kit::component::badge`)
For tags, indicators, and status chips:
```rust
use gpui_kit::component::badge::Badge;

let badge = Badge::new("Connected")
    .dot(); // displays a status dot
```

### 2.6 Desktop Tooltips (`gpui_kit::component::tooltip`)
Attach popover descriptions to any interactive element:
```rust
use gpui_kit::component::tooltip::Tooltip as KitTooltip;

div()
    .child("Help")
    .tooltip(|window, cx| {
        KitTooltip::new("Keyboard shortcut: Ctrl+Shift+P").build(window, cx)
    });
```

### 2.7 Native Scroll Viewports (`gpui_kit::component::scroll`)
Ensure smooth inertia scrolling and rendered scrollbars:
```rust
use gpui_kit::component::scroll::ScrollableElement;

div()
    .id("viewport_scroll")
    .flex_1()
    .h_full()
    .overflow_y_scrollbar()
    .child(content);
```

### 2.8 Vector Icons (`gpui_kit::assets::IconName`)
`gpui-kit` bundles the complete Lucide icon family (`IconName::Settings`, `IconName::Check`, `IconName::AlertTriangle`, `IconName::Cpu`, `IconName::RefreshCw`, etc.).

---

## 3. Visual Aesthetic & Theme Standards

Follow modern desktop design discipline (Linear, Zed, Xcode): clean, matte, high-contrast, and distraction-free.

### 3.1 The Monochrome Standard (Black & White / Zinc)
Default to the shadcn / zinc monochrome palette:

- **Dark Mode**:
  - Background: `#09090B` (zinc-950) — Pitch-black canvas.
  - Surface / Card: `#141417` (zinc-900) — Content containers.
  - Elevated Surface: `#1E1E22` (zinc-850) — Docks, dialogs, dropdowns.
  - Hover / Active: `#27272A` / `#37373C` — Interactive feedback.
  - Borders: `#27272A` / `#1C1C1F` — Hairline borders.
  - Foreground: `#FAFAFA` — High-contrast primary text.
  - Muted Foreground: `#A1A1AA` (zinc-400) / `#71717A` (zinc-500) — Meta, hints, descriptions.
  - Accent: `#FAFAFA` (pure white) — Active buttons and selected states.
  - **Accent Foreground**: `#09090B` (pure dark) — Text on accent backgrounds (**MANDATORY**).

- **Light Mode**:
  - Background: `#FFFFFF` — Pure white canvas.
  - Surface: `#FAFAFA` / `#F4F4F5` — Light neutral panels.
  - Borders: `#E4E4E7` — Subtle dividers.
  - Foreground: `#09090B` — Deep dark text.
  - Accent: `#09090B` (pure dark) — Active buttons and selected states.
  - **Accent Foreground**: `#FFFFFF` (pure white) — Text on accent backgrounds.

### 3.2 The Strict Contrast Rule
**Never** hardcode `hsla(0.0, 0.0, 1.0, 1.0)` (white) for active text color. When an element is selected with `.bg(theme.accent)`, always use `.text_color(theme.accent_foreground)` so the text remains high-contrast in both dark and light modes.

### 3.3 Visual Anti-Patterns to Avoid
- **No Neon Gamer Clutter**: Avoid saturated primary accent fills for window chrome or static cards. Let the user's primary workspace content be the sole focal point.
- **No Generic Card Stacking**: Do not chop every setting and paragraph into individual bordered cards. Group logically and use whitespace, dividers, and typography.
- **No ALL-CAPS Eyebrows**: Avoid small tracked-out uppercase labels (`QUICK ACTIONS`). Use clean sentence-case titles.
- **No Emoji Clutter**: Avoid casual emoji icons (`🔥`, `⚡`, `🎉`) in technical desktop interfaces. Use Lucide icons or clear typography.

---

## 4. Desktop Ergonomics & Layout Architecture

### 4.1 Compact Pointer Targets
Desktop interfaces demand precision. Control heights should be:
- **Small (`24px` – `28px`)**: Table actions, inline badges, stepper buttons.
- **Medium (`30px` – `34px`)**: Standard buttons, inputs, segmented controls.
- **Large (`36px` – `40px`)**: Primary action triggers and hero actions.

### 4.2 Master-Detail Layouts
On desktop viewports, divide screens into:
1. **Primary Stage (`flex_1`)**: The interactive visual workspace, canvas, or data list.
2. **Contextual Inspector (`320px` – `380px`)**: Fixed-width side dock for properties, calibration, or details.
3. **Inspector Vertical Stacking**: Inside narrow inspector sidebars, always stack headers and description text vertically (`flex_col()`) to prevent horizontal text clipping and overflow.

### 4.3 App Shell Structure
Standard application shell hierarchy:
```text
Root Container (flex_row, h_full, w_full, bg: background)
├── Sidebar (w: 240px, border_r, bg: surface)
│   ├── Header (brand / title)
│   ├── Navigation Menu (SidebarMenu)
│   └── Footer (status / version)
└── Main Stage (flex_1, flex_col, h_full)
    ├── Titlebar / Header Bar (breadcrumbs, window actions)
    └── Content Viewport (ScrollableElement, overflow_y_scrollbar)
```

---

## 5. State Management & Architecture

### 5.1 Strict Decoupling
- **Views**: Render UI declaratively based on an immutable snapshot of state and dispatch typed actions. Never perform file I/O, network requests, or hardware transport calls inside view render methods.
- **State Entity**: Keep state in a central `Entity<AppState>`. Views observe state mutations via `cx.observe(&state, |this, _state, cx| cx.notify())`.
- **Action Dispatcher**: Use unidirectional data flow with a central action enum:
  ```rust
  pub enum AppAction {
      Navigate(Page),
      UpdateSetting(SettingKey, Value),
      TriggerSave,
  }
  ```

### 5.2 Callback Closures & Lifetimes
GPUI event handlers require `'static` closures:
- Callbacks in component structs should use `Option<Arc<dyn Fn(...) + 'static>>` or `Option<Box<dyn Fn(...) + 'static>>`.
- Type aliases simplify callback types and satisfy linter complexity limits:
  ```rust
  pub type SelectHandler<T> = Arc<dyn Fn(T, &mut Window, &mut App) + 'static>;
  ```

### 5.3 Local Mutation within Event Listeners
When mutating a value inside an event listener and passing it upstream:
```rust
let base_cfg = state.config.clone();
let on_change = on_action.clone();

slider.on_change(move |new_val, window, cx| {
    let mut cfg = base_cfg.clone();
    cfg.value = new_val;
    on_change(AppAction::UpdateConfig(cfg), window, cx);
});
```

---

## 6. Performance & GPU Rendering Optimization

### 6.1 Dev-Profile Compilation Configuration
GPUI applications execute complex GPU shaders and vector text rendering. In debug mode, unoptimized dependencies cause noticeable frame drops. Always configure package optimization in the workspace root `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 2
```

### 6.2 Streamline the DOM Tree
- **Flatten Nested Divs**: Keep element nesting shallow. Do not wrap single elements in multiple redundant containers.
- **Limit Dynamic Segment Counts**: For sliders, timeline bars, and custom gauges, use bounded step counts (e.g., 10 to 15 segments) rather than generating hundreds of nested `div`s and closures per frame.
- **Static Assets & Icons**: Use `IconName` or cached vector SVGs rather than re-rasterizing complex assets on each render.

---

## 7. Definition of Done & Quality Gates

Every UI change must pass:

```bash
# 1. Format verification
cargo fmt --check

# 2. Strict linting (zero warnings permitted)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Unit and integration test suites
cargo test --workspace

# 4. Release build verification
cargo build --workspace --release
```

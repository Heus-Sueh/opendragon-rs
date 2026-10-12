---
name: gpui-ui
description: Best practices, architecture patterns, visual design guidelines, and component conventions for developing native desktop user interfaces with GPUI-CE and gpui-kit.
---

# GPUI-CE & gpui-kit Desktop UI Guidelines

This guide establishes architecture patterns, design system guidelines, component conventions, and visual discipline for developing high-performance native desktop applications using `gpui` / `gpui-ce` and the official **`gpui-kit`** component ecosystem.

---

## 1. Core Philosophy: Use gpui-kit Primitives First

**Never recreate primitive UI controls from raw `div()`s when `gpui-kit` provides a dedicated component.**

`gpui-kit` provides tested, accessible, GPU-accelerated desktop controls modeled after the modern shadcn / Linear desktop aesthetic:

| Component Type | `gpui-kit` Module | Description & Usage |
|----------------|-------------------|---------------------|
| **Navigation Sidebar** | `gpui_kit::component::sidebar::{Sidebar, SidebarHeader, SidebarGroup, SidebarMenu, SidebarMenuItem, SidebarFooter}` | Collapsible desktop navigation bar with integrated Lucide icons. |
| **Containers & Cards** | `gpui_kit::component::group_box::GroupBox` | Structured surface panels using `.outline()` or `.fill()` variants. |
| **Buttons & Action Groups** | `gpui_kit::component::button::{Button, ButtonGroup, ButtonVariants}` | Primary, secondary, ghost, and danger buttons with `.compact()` grouping. |
| **Form Toggles** | `gpui_kit::component::switch::Switch` | Spring-animated desktop toggles with built-in labels and descriptions. |
| **Status Indicators** | `gpui_kit::component::badge::Badge` | Metric pills and `.dot()` status indicators. |
| **Hover Tooltips** | `gpui_kit::component::tooltip::Tooltip` | Desktop popover tooltips attached via `.tooltip(...)`. |
| **Scroll Containers** | `gpui_kit::component::scroll::ScrollableElement` | Smooth scrollable viewports with native inertia and rendered scrollbars. |
| **Vector Icons** | `gpui_kit::assets::IconName` | Native Lucide icon library (`IconName::Home`, `IconName::Keyboard`, `IconName::Sun`, etc.). |

---

## 2. Default Visual Theme: gpui-kit Monochrome (Black & White)

OpenDragon-rs follows `gpui-kit`'s default **monochrome zinc / shadcn standard**. Avoid tacky saturated "gamer" neon or arbitrary accent colors for chrome.

### 2.1 Color Tokens & Contrast Hierarchy

- **Dark Mode (`Theme::dark`)**:
  - `background`: `#09090B` (`hsla(240°, 10%, 4%)`) — Deep pitch-black base.
  - `surface`: `#141417` (`hsla(240°, 6%, 9%)`) — Standard card/panel background.
  - `surface_elevated`: `#1E1E22` (`hsla(240°, 6%, 13%)`) — Contextual inspector docks and popovers.
  - `surface_hover` / `surface_active`: `#27272A` / `#37373C` — Interactive surfaces.
  - `border` / `border_subtle`: `#27272A` / `#1C1C1F` — Subtle zinc hairline dividers.
  - `foreground`: `#FAFAFA` — Crisp, high-contrast primary text.
  - `foreground_muted`: `#A1A1AA` (zinc-400) — Secondary labels and descriptions.
  - `foreground_subtle`: `#71717A` (zinc-500) — Meta captions and hints.
  - `accent`: `#FAFAFA` (pure white) — Primary active state / selection indicator.
  - `accent_foreground`: `#09090B` (deep black) — Text on active accent backgrounds (**CRITICAL for readability**).
  - `accent_subtle`: `#27272A` — Subtle dark active badges.

- **Light Mode (`Theme::light`)**:
  - `background`: `#FFFFFF` — Pure white window background.
  - `surface`: `#FAFAFA` / `#F4F4F5` — Clean light zinc cards.
  - `border`: `#E4E4E7` — Hairline borders.
  - `foreground`: `#09090B` — Deep dark text.
  - `accent`: `#09090B` (pure dark) — Active buttons/indicators.
  - `accent_foreground`: `#FFFFFF` (pure white) — High-contrast text on active buttons.

### 2.2 Dynamic Contrast Rule (No Invisible Text)
Whenever an element uses `.bg(theme.accent)`, **always** pair it with `.text_color(theme.accent_foreground)`. Never hardcode `hsla(0.0, 0.0, 1.0, 1.0)` for active text, as it becomes invisible white-on-white text in monochrome dark mode.

---

## 3. Desktop Ergonomics & Layout Architecture

### 3.1 Sidebar Navigation Pattern
Construct sidebars declaratively with `gpui-kit::component::sidebar`:

```rust
use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem, SidebarHeader};

let mut menu = SidebarMenu::new();
menu = menu.child(
    SidebarMenuItem::new("nav_home")
        .label("Overview")
        .icon(IconName::Home)
        .active(current_page == Page::Home)
        .on_click(move |_, w, a| on_nav(Page::Home, w, a)),
);

let sidebar = Sidebar::new("app_sidebar")
    .header(header)
    .child(SidebarGroup::new("Main Navigation").child(menu))
    .footer(footer);
```

### 3.2 Master-Detail Layouts
For complex hardware or configuration screens, divide the viewport:
- **Left Stage (Flex-1)**: Visual interactive canvas (e.g. vector keyboard preview, mouse topology, lighting animation canvas).
- **Right Inspector (Fixed width `340px` – `380px`)**: Contextual controls, calibration sliders, and telemetry wells.
- **Guidance & Hint Boxes**: Inside narrow inspector docks, always stack guidance elements vertically (`flex_col()`) with the title on top and wrapping text below to prevent horizontal text clipping.

### 3.3 Multi-Layout Support
For keyboard previews, dynamically render keycap matrices based on the selected layout:
- **ANSI (US)**: Single-row Enter (`2.25u`), horizontal `\` above Enter.
- **ABNT2 (Brasil)**: ISO double-height Enter, dedicated `Ç` key next to `~`, and split right Shift with `/` and `?`.
- **ISO (Europe)**: ISO Enter and short left Shift with `<`/`>` key.

---

## 4. Performance & Rendering Optimization

GPUI renders via Vulkan/Metal/DirectX pipelines. Follow these optimization rules:

### 4.1 Dev Compilation Optimization
Add package optimizations to `Cargo.toml` so shaders, image decoding, and `gpui-kit` run at full 60+ FPS in development mode:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 2
```

### 4.2 Streamline the DOM Tree
- **Avoid deep `div()` nesting**: Flatten keycap hierarchies (e.g. chassis -> key -> LED bar) rather than nesting 4 to 5 container wrappers per key.
- **Bound Interactive Segments**: In sliders and progress bars, use reasonable segment counts (e.g. 10 to 15 segments) rather than dozens of closures, avoiding hundreds of main-thread callback allocations on every render pass.
- **Native Scrollbars**: Use `ScrollableElement::overflow_y_scrollbar()` for native inertia and scrollbar rendering without custom layout thrashing.

---

## 5. Architectural Decoupling & Hardware Safety

### 5.1 Strict Decoupling via Domain Models
- Views must **never** construct raw HID packets, talk directly to transport sockets, or format byte arrays.
- The UI communicates strictly through domain models (`Capability`, `LightingConfig`, `ActuationPoint`, `PollingRate`, `DeviceAction`).
- The application backend (`DeviceBackend` / `DeviceManager`) is responsible for validating commands and dispatching packets.

### 5.2 Mock Backend for Instant Verification
Always maintain a `--features mock-device` mode so UI layout, interactions, layouts, and animations can be tested immediately in headless or hardware-free environments:

```bash
cargo run -p opendragon-gui --features mock-device
```

---

## 6. Definition of Done & Quality Gates

Before declaring any GUI task complete:

```bash
# 1. Code formatting
cargo fmt --check

# 2. Strict linting (zero warnings permitted)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Unit and component tests
cargo test --workspace

# 4. Release build verification
cargo build --workspace --release
```

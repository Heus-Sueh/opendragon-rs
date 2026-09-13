# OpenDragon GUI Design System

The **OpenDragon** desktop application uses a restrained, modern, and ergonomic design system tailored for peripheral management on Linux and cross-platform desktop environments.

---

## 1. Design Principles

- **No Gamer Gimmicks**: Avoid high-saturation backgrounds, harsh contrast lines, or synthetic textures. The interface feels at home on contemporary Linux desktops (GNOME, KDE Plasma, Hyprland/Sway).
- **Physical Feedback Representation**: Lighting and visual customization are expressed through an interactive top-down keyboard rendering rather than garish UI chrome.
- **Progressive Disclosure**: Primary views present immediate, accessible controls (preset effects, brightness slider, actuation slider). Advanced diagnostics and protocol telemetry are grouped under `Settings -> Diagnostics`.
- **Capability-Driven**: Views and sidebar navigation elements render dynamically according to the capabilities reported by the device driver.

---

## 2. Color System

The palette uses HSL-tailored colors balancing subtle elevation layers with accessible contrast:

### Dark Theme (Default)
| Token | Value | Purpose |
|---|---|---|
| `background` | `hsl(222, 24%, 9%)` | Application window background |
| `surface` | `hsl(222, 22%, 13%)` | Cards, panels, input fields |
| `surface_hover` | `hsl(222, 20%, 17%)` | Hovered cards, clickable items |
| `surface_active` | `hsl(222, 18%, 22%)` | Pressed buttons, active tab backgrounds |
| `border` | `hsl(222, 16%, 22%)` | Card borders, subtle dividers |
| `border_subtle` | `hsl(222, 16%, 16%)` | Nested element borders, keycap frames |
| `text` | `hsl(210, 24%, 96%)` | Primary headings, active labels |
| `text_muted` | `hsl(215, 14%, 62%)` | Descriptions, secondary meta text |
| `text_dim` | `hsl(215, 12%, 42%)` | Inactive captions, shortcuts |
| `accent` | `hsl(217, 91%, 60%)` | Interactive highlights, active toggles |
| `accent_hover` | `hsl(217, 91%, 68%)` | Hover state on primary actions |

### Semantic Status Tokens
| Token | Dark Mode Value | Usage |
|---|---|---|
| `success` | `hsl(142, 69%, 45%)` | Connected device, saved profile, valid probe |
| `warning` | `hsl(38, 92%, 50%)` | Partial support, experimental actuation range |
| `danger` | `hsl(0, 84%, 60%)` | Unsupported device, destructive delete, protocol error |

---

## 3. Spacing Scale

Consistent 4px/8px modular scale:

```text
SPACE_1   =  4px   // Tight element padding, status dot gaps
SPACE_2   =  8px   // Button horizontal padding, badge padding
SPACE_3   = 12px   // Form field gaps, list row spacing
SPACE_4   = 16px   // Card interior padding, section item gaps
SPACE_6   = 24px   // Card layout grid gaps, view margins
SPACE_8   = 32px   // Major section separation
SPACE_12  = 48px   // Top-level page padding
```

- **Sidebar Width**: `220px` (Compact, high information density)
- **Content Max Width**: `1000px` (Optimal line length and card grid balance)

---

## 4. Typography Hierarchy

- **Title / Page Header**: 24px SemiBold (`FONT_SIZE_TITLE`)
- **Card Heading / Subheader**: 16px Medium (`FONT_SIZE_MD`)
- **Body / Primary Control**: 14px Regular (`FONT_SIZE_BASE`)
- **Secondary Caption / Meta**: 13px Regular (`FONT_SIZE_SM`)
- **Keycap Label / Badge**: 11px Medium (`FONT_SIZE_XS`)

---

## 5. Component Catalog

- **`Button`**: Variants for `Primary`, `Secondary`, `Ghost`, and `Danger` with consistent hover transitions and disabled states.
- **`Card`**: Surface card with subtle border and optional hover effects.
- **`Slider`**: Precision slider with numeric readout, unit label, and optional `[-]` / `[+]` fine-stepping buttons.
- **`Switch`**: Boolean toggle with smooth thumb transition, title, and optional description.
- **`StatusBadge`**: Pill badge indicating hardware support (`Connected`, `Disconnected`, `Supported`, `Partial`, `Experimental`).
- **`KeyboardPreview`**: Interactive 87-key Tenkeyless (TKL) visual chassis reflecting active global RGB lighting and individual per-key backlight LED status.

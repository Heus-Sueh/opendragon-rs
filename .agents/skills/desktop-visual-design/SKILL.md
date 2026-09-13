---

name: desktop-visual-design
description: >
Create coherent, polished, desktop-first application interfaces with
restrained aesthetics, consistent design tokens, strong typography,
controlled curvature, semantic colors and high information density.
-------------------------------------------------------------------

# Desktop Visual Design

Design desktop applications, not websites or mobile interfaces.

The target visual language is:

* minimal
* restrained
* precise
* technical
* calm
* dense but readable
* desktop-native
* keyboard-friendly
* consistent

Prefer interfaces closer to developer tools, editors and native desktop utilities than SaaS dashboards or landing pages.

## 1. Use a design system

Never choose visual values independently per component.

Define and reuse tokens for:

* colors
* typography
* spacing
* radius
* control sizes
* borders
* shadows
* icons

Avoid arbitrary values.

Example:

```text
spacing: 4 / 8 / 12 / 16 / 24 / 32

radius:
small: 3
medium: 5
large: 8

controls:
compact: 28
normal: 32
large: 38
```

## 2. Colors

Use semantic colors instead of raw values inside components.

Required roles:

```text
background
surface
surface-hover
surface-active

foreground
foreground-muted

border
border-focused

accent
accent-foreground

success
warning
danger
```

Components must reference theme tokens.

Bad:

```text
#79a8ff
```

Good:

```text
theme.colors.accent
```

Use accent colors intentionally, not decoratively.

## 3. Typography

Typography creates most of the hierarchy.

Use a small scale:

```text
caption     11
small       12
body        13
body-large  14
heading     16
title       20
```

Prefer changing weight before dramatically increasing font size.

Use approximately:

```text
regular
medium
semibold
```

Avoid:

* giant headings
* landing-page typography
* many font sizes
* excessive bold text

## 4. Geometry

Desktop controls should use restrained curvature.

Recommended radius scale:

```text
small   3px
medium  5px
large   8px
```

Use:

* small for buttons and compact controls
* medium for inputs, menus and popovers
* large for dialogs and major floating surfaces

Avoid:

* excessive rounding
* pill-shaped controls by default
* 12–24px radii on normal controls
* different arbitrary radii across components

## 5. Density

Desktop interfaces should favor information density.

Typical sizes:

```text
toolbar      36–42px
titlebar     36–44px
sidebar row  30–34px
list row     30–38px
input        30–34px
button       30–34px
icon         14–18px
```

Do not blindly use mobile-sized 44–56px controls.

## 6. Spacing

Spacing must be systematic.

Prefer:

```text
4
8
12
16
24
32
```

Use spacing and alignment to group content before adding borders or containers.

Related elements should be closer together.

Unrelated sections should have more separation.

## 7. Borders and surfaces

Prefer subtle borders and surface differences over shadows.

Typical border:

```text
1px
```

Use elevated surfaces only where hierarchy requires them.

Do not place every section inside a card.

Avoid excessive:

* borders
* shadows
* containers
* nested cards
* opacity layers

## 8. Components

Components with the same purpose must share:

* height
* padding
* typography
* radius
* icon size
* interaction behavior

Typical button:

```text
height: 32
padding-x: 12
gap: 6
radius: small
font: body medium
icon: 16
border: 1
```

Typical input:

```text
height: 32
padding-x: 8
radius: small
font: body
border: 1
```

Typical sidebar item:

```text
height: 32
padding-x: 10
gap: 8
icon: 16
font: body
```

Avoid one-off component dimensions.

## 9. Component states

Interactive components must define:

```text
default
hover
pressed
focused
selected
disabled
```

Applications must also consider:

```text
loading
empty
error
success
```

Focus must always be visible.

## 10. Icons

Use one consistent icon family.

Prefer:

```text
14px compact
16px normal
20px prominent
```

Do not place an icon beside every label.

Icons should improve recognition, not decoration.

## 11. Visual restraint

The default answer to decoration is no.

Do not automatically use:

* gradients
* glassmorphism
* giant shadows
* glowing elements
* excessive accent colors
* rounded cards everywhere
* badges everywhere
* pills everywhere
* giant headings
* decorative icons
* unnecessary animations

Elegance should come from:

* typography
* spacing
* proportion
* alignment
* hierarchy
* consistency

## 12. Avoid generic AI UI

Do not default to:

```text
sidebar
+ giant page title
+ dashboard of rounded cards
+ colorful icons
+ badges
+ gradients
```

Do not make the application resemble a marketing website.

Before adding a visual element, ask:

> Does this improve hierarchy, usability or comprehension?

If not, remove it.

## 13. Theme architecture

Prefer centralized theme objects:

```text
Theme
├── colors
├── typography
├── spacing
├── radius
├── sizing
├── borders
├── shadows
└── icons
```

Components must consume theme tokens instead of defining their own appearance.

Where appropriate, support themes loaded from configuration:

```text
Theme::from_system()
Theme::from_file(...)
Theme::from_palette(...)
```

## 14. Desktop interaction

Take advantage of desktop capabilities:

* keyboard shortcuts
* hover
* focus navigation
* context menus
* tooltips
* resizable layouts
* sidebars
* command palettes
* right click

Do not blindly reproduce mobile interaction patterns.

## 15. Visual review

After implementing the interface, run and inspect it visually.

Review:

* typography
* hierarchy
* alignment
* spacing
* density
* contrast
* radius consistency
* component consistency
* resizing
* hover/focus states
* unnecessary decoration

If screenshots or visual inspection are available, use them.

Iterate instead of accepting the first implementation.

## Final test

Before considering the UI finished, verify:

* Does it look like a desktop application?
* Is the visual hierarchy obvious?
* Are colors semantic?
* Is typography consistent?
* Is curvature restrained?
* Is spacing systematic?
* Are controls appropriately dense?
* Are repeated components visually identical?
* Can anything decorative be removed?
* Does the application still look good when resized?

If not, revise it.

---
name: desktop-ui-ux-design
description: >
  Design and review beautiful, coherent, accessible and desktop-native
  application interfaces. Use when creating, redesigning or reviewing
  desktop UI/UX.
---

# Desktop UI/UX Design

You are designing a real desktop application, not a website.

Your objective is to create interfaces that are:

- visually coherent
- simple
- elegant
- comfortable for long-term use
- discoverable
- accessible
- consistent
- efficient with mouse and keyboard
- appropriate for desktop environments

Do not start implementing UI immediately.

Follow the workflow below.

# 1. Understand the product

Before designing, determine:

- primary purpose of the application
- primary user
- 3–5 most common user actions
- information hierarchy
- navigation model
- destructive actions
- frequent vs infrequent actions
- application states
- empty/loading/error states

Do not invent screens or controls without a user need.

# 2. Define the design direction

Before implementation, establish a small design system.

Define:

- visual personality
- typography hierarchy
- spacing scale
- border radius scale
- color roles
- surface hierarchy
- icon style
- control heights
- interaction states

Avoid arbitrary values.

Prefer reusable design tokens.

Example:

spacing:
4 / 8 / 12 / 16 / 24 / 32

radius:
4 / 8 / 12

Typography:
caption
body
body-strong
heading
title

Colors must have semantic roles:

background
surface
surface-hover
border
text-primary
text-secondary
accent
danger
warning
success

# 3. Establish hierarchy

Every screen must answer within a few seconds:

1. Where am I?
2. What can I do here?
3. What is the primary action?
4. What information matters most?
5. How do I go back or navigate elsewhere?

Use size, weight, spacing and grouping before adding decoration.

Do not solve hierarchy problems by adding more borders.

# 4. Desktop-first interaction

This is a desktop application.

Take advantage of:

- keyboard shortcuts
- hover states
- context menus
- tooltips
- right click
- focus navigation
- resizable layouts
- sidebars
- command palettes when appropriate
- drag and drop when appropriate

Do not blindly reproduce mobile or web patterns.

Avoid oversized touch-oriented controls unless touch is an explicit target.

# 5. Reduce visual noise

Prefer:

- whitespace
- alignment
- grouping
- typography
- subtle surface differences

over:

- excessive borders
- excessive cards
- excessive shadows
- gradients without purpose
- decorative containers
- unnecessary separators

Never put every section inside a card.

# 6. Avoid generic AI-generated UI

Do NOT automatically create:

- a dashboard full of cards
- giant hero headings
- gradients everywhere
- excessive rounded rectangles
- excessive pills
- unnecessary badges
- excessive shadows
- excessive glassmorphism
- huge empty spaces
- marketing-site layouts inside applications
- random accent colors
- icons for every text label

Do not make the interface "fancy" by adding decoration.

Elegance should come from proportion, typography, spacing and hierarchy.

# 7. Progressive disclosure

Show common actions immediately.

Move uncommon or advanced actions into:

- menus
- context menus
- expandable sections
- secondary dialogs
- advanced settings

Do not expose every possible action simultaneously.

# 8. Component consistency

Before creating a new component, check whether an existing component
can express the interaction.

Components with the same purpose must have the same:

- dimensions
- spacing
- typography
- interaction
- states
- visual language

Avoid one-off components.

# 9. States

Every interactive component must consider:

- default
- hover
- pressed
- focused
- selected
- disabled

Relevant asynchronous interfaces must consider:

- loading
- empty
- error
- success
- partial data

Never design only the ideal state.

# 10. Accessibility

Ensure:

- sufficient contrast
- visible keyboard focus
- reasonable target sizes
- understandable labels
- icons are not the only communication mechanism when ambiguous
- color is not the only indicator of state
- keyboard navigation is possible
- destructive actions are clearly distinguished

# 11. Destructive actions

Never visually emphasize destructive actions more than the normal
primary workflow.

For destructive actions:

- use clear wording
- separate them from frequent actions
- require confirmation when consequences are difficult to reverse
- explain what will happen

# 12. Visual review

After implementation, inspect the rendered application.

Do not assume that correct code produces good design.

Review:

- alignment
- spacing
- hierarchy
- density
- typography
- contrast
- component consistency
- unnecessary decoration
- discoverability
- resizing behavior
- empty states
- long content
- keyboard focus

If screenshots or visual inspection tools are available, use them.

Iterate until the UI passes the design review.

# 13. Final quality test

Ask:

- Can anything be removed?
- Is the primary action obvious?
- Are secondary actions visually secondary?
- Are related elements visually grouped?
- Are unrelated elements sufficiently separated?
- Is spacing systematic?
- Are controls consistent?
- Does this look like an application rather than a landing page?
- Does it remain usable at different window sizes?
- Can a new user understand the screen without documentation?
- Can an experienced user operate it efficiently?

If any answer is no, revise the interface before considering it complete.
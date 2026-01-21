# Design System

This is the central design system document for the EVM CLI application. It defines the design principles, patterns, and standards that guide all component development and ensures consistency across the entire interface.

**All specs should reference this design system to maintain visual and interaction consistency.**

---

## Purpose

The design system serves as:
- **Single source of truth** for design decisions across all specs
- **Visual consistency guide** ensuring uniform look and feel
- **Interaction consistency guide** ensuring predictable behavior
- **Component library** documenting all UI elements and their usage
- **Pattern reference** showing reusable interaction flows
- **Contributor guide** helping new team members understand design standards

---

## Design Principles

1. **Clarity**: Information is presented clearly and concisely, avoiding cognitive overload
2. **Consistency**: All components follow established patterns and behave predictably
3. **Accessibility**: All interfaces are accessible via keyboard with clear focus indicators
4. **Efficiency**: Interactions are intuitive and minimize navigation steps
5. **Predictability**: Components behave the same way across contexts
6. **Terminal-native**: Design embraces terminal constraints and capabilities

---

## Core Design Tenets

### 1. Unified Keyboard Vocabulary
Every interactive element uses consistent keyboard bindings:
- **Arrow keys + Vim keys**: `↑`/`k`, `↓`/`j`, `←`/`h`, `→`/`l` for navigation
- **Enter**: Execute/confirm action (consistent across all contexts)
- **Escape**: Dismiss/cancel (modal, menu, or operation)
- **Tab**: Navigate or switch focus (same behavior throughout)

**Benefit**: Users learn controls once, apply everywhere

### 2. Cyan Selection Highlighting
All interactive lists use cyan background to highlight selected item:
- Sidebar contracts and methods
- Output panel cards
- Command Palette commands
- Footer action menus
- Any keyboard-navigable list

**Benefit**: Instant visual feedback, consistent across app

### 3. Symbol + Color Status Indicators
Never use color alone; always pair with symbol:
- ✓ (Green) = Success
- ✗ (Red) = Error
- ⟳ (Yellow) = Loading/Pending
- ℹ (Blue) = Information
- ● (Green) = Connected
- ○ (Gray) = Disconnected

**Benefit**: Accessible to color-blind users, meets WCAG standards

### 4. Auto-Expansion After Actions
After user completes action (deploy, load), automatically expand result:
- New contract auto-expands to show Deploy/Load options
- New instance auto-expands to show methods
- New card auto-selected in output panel

**Benefit**: Minimizes navigation friction

### 5. Real-Time Validation Feedback
Forms provide immediate validation as user types:
- Error appears below field with ✗ icon
- Submit blocked while any field invalid
- All invalid fields highlighted on failed submit
- Cursor moves to first invalid field

**Benefit**: Users fix issues immediately, not after submission

### 6. Modal Centering Pattern
All modals follow identical structure:
- Horizontally and vertically centered
- ~60-80% terminal width
- Close hint (`esc`) top-right
- Title bar, content area, footer instructions
- Focus trapped within modal

**Benefit**: Consistent, predictable modal behavior

---

## Documentation Structure

### [Components](./components.md)
Complete catalog of all UI components:
- Sidebar (tree navigation)
- Output Panel (card display)
- Status Bar (connection info)
- Command Palette (command access)
- Parameter Form (function arguments)
- Interactive elements (fields, buttons, toggles)
- Feedback components (errors, loading, success)

### [Patterns](./patterns.md)
Reusable interaction and layout patterns:
- Navigation patterns (keyboard, focus management)
- Selection patterns (highlighting, auto-select)
- Form patterns (input, validation, submission)
- Async feedback patterns (loading, success, error)
- Modal patterns (centering, focus trap, dismissal)
- Deletion patterns (immediate, no confirmation)
- Tree expansion patterns (auto-expand after action)

### [Colors](./colors.md)
Color palette and semantic usage:
- Primary interactive (Cyan) for selection
- Success (Green) + ✓ symbol
- Error (Red) + ✗ symbol
- Loading (Yellow) + ⟳ symbol
- Information (Blue) + ℹ symbol
- Connection status (● / ○)
- Terminal compatibility guidance
- WCAG AA accessibility compliance

### [Typography](./typography.md)
Text styling and hierarchy:
- Font: Monospace (terminal native)
- Text hierarchy (H1-H4, body, secondary)
- Emphasis techniques for terminal
- Line length and wrapping rules
- Special elements (addresses, hashes, types)
- Component-specific typography guidelines

### [Spacing](./spacing.md)
Margins, padding, and layout measurements:
- Terminal-based units (characters and lines)
- Sidebar hierarchy indentation (2 spaces per level)
- Card spacing and padding (1 space borders, 1 line between)
- Modal inner/outer spacing rules
- Status bar layout and distribution
- Responsive spacing for different terminal sizes

---

## Cross-Spec Design Consistency

### How to Use This Design System

When creating or updating a spec:

1. **Reference Components**: Use component names and descriptions from [components.md](./components.md)
   - "Parameter Input Popup" (not "parameter modal")
   - "Output Card" (not "result display")
   - "Footer Action Menu" (not "action list")

2. **Use Pattern Names**: Reference patterns from [patterns.md](./patterns.md)
   - "Auto-expansion on load" (established pattern)
   - "Escape to dismiss" (consistent behavior)
   - "Cyan highlight selection" (visual consistency)

3. **Apply Colors**: Use semantic colors from [colors.md](./colors.md)
   - Success = Green + ✓
   - Error = Red + ✗
   - Loading = Yellow + ⟳

4. **Follow Typography**: Apply text styles from [typography.md](./typography.md)
   - Use monospace for all text
   - Apply consistent hierarchy for headings
   - Use emphasis techniques appropriate for terminal

5. **Maintain Spacing**: Apply measurements from [spacing.md](./spacing.md)
   - 2 spaces per indentation level
   - 1 line between sections
   - 1 space padding inside components

### Spec Template Reference

Each spec should include sections like:
- **Layout**: Reference [Spacing](./spacing.md) for measurements
- **Colors/Visual**: Reference [Colors](./colors.md) for semantic colors
- **Keyboard Controls**: Reference [Patterns](./patterns.md) for consistent bindings
- **Components**: Reference [Components](./components.md) for element definitions
- **Mockups**: Follow spacing and typography guidelines

---

## Design Consistency Checklist

Use this checklist when designing or reviewing new specs:

### Navigation & Interaction
- [ ] All lists use arrow keys + vim keys for navigation
- [ ] Selection is highlighted with cyan background
- [ ] Enter executes action, Escape cancels
- [ ] Tab switches between major areas
- [ ] First item auto-selected when list displays

### Colors & Feedback
- [ ] Status indicators use symbol + color (never color alone)
- [ ] Success = Green + ✓
- [ ] Error = Red + ✗
- [ ] Loading = Yellow + ⟳
- [ ] Information = Blue + ℹ

### Forms & Input
- [ ] Real-time validation as user types
- [ ] Errors shown below field with ✗
- [ ] Submit blocked if any field invalid
- [ ] Tab moves between fields
- [ ] Escape cancels without submitting

### Modals & Overlays
- [ ] Centered on screen (~60-80% width)
- [ ] Title bar with close hint (`esc`)
- [ ] Focus trapped within modal
- [ ] Escape closes modal
- [ ] Previous focus restored after close

### Accessibility
- [ ] All elements keyboard-accessible
- [ ] Clear focus indicators (cyan highlight)
- [ ] High contrast (WCAG AA minimum)
- [ ] No color-alone indicators
- [ ] Error messages actionable

### Typography
- [ ] Monospace font used throughout
- [ ] Clear heading hierarchy
- [ ] Emphasis via symbols/brackets, not italic
- [ ] Addresses truncated with ellipsis
- [ ] Type names shown with code formatting

### Spacing
- [ ] 2-space indentation per hierarchy level
- [ ] 1-line gaps between sections
- [ ] 1-space padding inside components
- [ ] Consistent alignment and grouping
- [ ] Terminal width considerations

---

## Implementation Guidelines

### For Developers
- Reference [Components](./components.md) for UI element structure
- Follow [Patterns](./patterns.md) for interaction flows
- Use [Colors](./colors.md) for terminal color implementation
- Apply [Spacing](./spacing.md) for layout measurements
- Check typography in [Typography](./typography.md) for text rendering

### For Designers/Spec Writers
- Use [Components](./components.md) to name elements consistently
- Reference [Patterns](./patterns.md) when describing interactions
- Apply [Colors](./colors.md) for visual mockups
- Follow [Typography](./typography.md) for text in specs
- Use [Spacing](./spacing.md) for accurate mockup dimensions

### For Review/Quality Assurance
- Use **Design Consistency Checklist** above
- Reference [Patterns](./patterns.md) for expected behavior
- Check [Colors](./colors.md) for proper semantic colors
- Verify [Components](./components.md) look and act correctly
- Validate [Spacing](./spacing.md) measurements

---

## Related Documentation

- **Feature Specifications**: `/spec/[feature-name]/spec.md`
  - [Contracts Menu](../contracts-menu/spec.md)
  - [Command Palette](../ctrl-p-menu/spec.md)
  - [Transaction & Call Popup](../tx-and-call-popup/spec.md)
  - [Output Panel](../output-panel/spec.md)
  - [Main Interface](../main-interface/spec.md)
  - [General Settings](../general-settings/spec.md)

- **Component Recordings**: `/spec/[feature-name]/recordings/`
  - VHS recordings of components in action

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | Jan 2026 | Comprehensive design system documentation |
| | | Unified components, patterns, colors, typography, spacing |
| | | Cross-spec consistency guidelines |

---

## Contributing to Design System

When adding new components or patterns:

1. **Document in appropriate file**:
   - New component → [Components](./components.md)
   - New interaction → [Patterns](./patterns.md)
   - New color use → [Colors](./colors.md)
   - New text style → [Typography](./typography.md)
   - New spacing rule → [Spacing](./spacing.md)

2. **Reference existing patterns**: Build on established patterns rather than creating variations

3. **Update design consistency checklist**: Add checks for new design decisions

4. **Cross-reference from specs**: Link from feature specs to design system

5. **Test accessibility**: Verify keyboard navigation and color contrast

---

## Design System Philosophy

This design system prioritizes:

- **Consistency over novelty**: Established patterns are preferred over new variations
- **Accessibility first**: Keyboard navigation and color-blind compatibility required
- **Terminal simplicity**: Terminal constraints drive elegant, focused design
- **Predictability**: Users should anticipate component behavior
- **Efficiency**: Minimize navigation and cognitive load
- **Clarity**: Clear visual hierarchy and status indicators

By following this design system, all specs maintain visual and interaction consistency, creating a unified, intuitive user experience.

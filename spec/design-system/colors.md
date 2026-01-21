# Color System

This document defines the color palette and semantic color usage throughout the EVM CLI.

---

## Semantic Color Usage

### Primary Interactive State
**Color**: Cyan (bright cyan from terminal palette)
**Usage**: Selected/focused items, active state
**Components**: Sidebar selected item, output card selection, command palette selection, footer menu selection
**Contrast**: High contrast against default terminal background
**WCAG**: AA compliant when used with proper contrast

### Success State
**Color**: Green (terminal green)
**Symbol**: `✓` (check mark)
**Usage**: Successful operations, valid inputs, confirmations
**Examples**:
- `✓ Transaction successful`
- `✓ Contract deployed`
- `✓ Valid input`
**Never**: Green alone; always paired with symbol

### Error State
**Color**: Red (terminal red)
**Symbol**: `✗` (X mark)
**Usage**: Failed operations, invalid inputs, errors
**Examples**:
- `✗ Invalid address format`
- `✗ Transaction reverted: Insufficient balance`
- `✗ Failed to compile contract`
**Never**: Red alone; always paired with symbol

### Loading/Pending State
**Color**: Yellow (terminal yellow)
**Symbol**: `⟳` (rotating indicator)
**Usage**: Async operations in progress
**Examples**:
- `⟳ Fetching transaction...`
- `⟳ Compiling Contract.sol...`
- `⟳ Waiting for confirmation...`
**Never**: Yellow alone; always paired with symbol

### Information State
**Color**: Blue (terminal blue)
**Symbol**: `ℹ` (info mark)
**Usage**: Informational messages, helpful context
**Examples**:
- `ℹ Transaction pending confirmation`
- `ℹ Method requires parameters`
**Never**: Blue alone; always paired with symbol or text

### Connection Status
**Connected**: Green filled circle `●`
**Disconnected**: Default gray circle `○`
**Location**: Status bar left side
**Usage**: Immediate visual indicator of connection
**Rule**: Symbol + color (never color alone)

### Normal/Default Text
**Color**: Default terminal text color (white on dark, black on light)
**Usage**: Body text, labels, unemphasized content
**Contrast**: High contrast against background
**Notes**: Maintains compatibility with terminal theme

### Secondary/Muted Text
**Color**: Gray (dimmed terminal color)
**Usage**: Less important information, separators, metadata
**Examples**: Helper text, section dividers, timestamps
**Contrast**: Sufficient for readability (AA minimum)

---

## Component Color Mapping

### Sidebar (Contracts Menu)
- **Default item**: Normal text on default background
- **Selected item**: Normal text on cyan background
- **Expanded indicator**: `▾` in default color
- **Collapsed indicator**: `▸` in default color
- **Action item**: `◇` in default color

### Output Panel Cards
- **Card border**: Default gray/neutral color
- **Selected card**: Cyan highlight or border
- **Card title**: Normal text
- **Status Success**: Green `✓`
- **Status Error**: Red `✗`
- **Status Pending**: Yellow `⟳`

### Status Bar
- **Connected indicator**: Green `●`
- **Disconnected indicator**: Gray `○`
- **Background**: Default terminal background
- **Text**: Default terminal text
- **Separator**: Gray `|` or default color

### Command Palette
- **Search field**: Normal text on default background
- **Selected command**: Normal text on cyan background
- **Group header**: Gray or muted text
- **Command description**: Gray or secondary text

### Parameter Input Popup
- **Label**: Default text
- **Valid input**: Default text on default background
- **Invalid input**: Default text on default background
- **Validation error**: Red `✗` + error text in red
- **Field focus**: Cyan border or default with visible cursor

### Footer Action Menu
- **Action prefix**: `◇` in default color
- **Selected action**: Cyan background with text
- **Action shortcut**: Default or secondary text in parentheses
- **Action text**: Default text

---

## Visual Indicators Reference

| State | Symbol | Color | Example |
|-------|--------|-------|---------|
| Success | `✓` | Green | `✓ Deployed` |
| Error | `✗` | Red | `✗ Invalid` |
| Pending | `⟳` | Yellow | `⟳ Loading` |
| Info | `ℹ` | Blue | `ℹ Note` |
| Connected | `●` | Green | `● Connected` |
| Disconnected | `○` | Gray | `○ Offline` |
| Expanded | `▾` | Default | `▾ Contract` |
| Collapsed | `▸` | Default | `▸ Token` |
| Action | `◇` | Default | `◇ Deploy` |
| Load | `+` | Default | `+ Load` |
| Selected | Cyan bg | Cyan | Item highlighted |
| Tree branch | `├` `└` `│` | Default | Hierarchy |

---

## Accessibility Requirements

### Contrast Ratios (WCAG AA Minimum)
- **Normal text**: 4.5:1 ratio
- **Large text**: 3:1 ratio
- **UI components**: 3:1 ratio for active state
- **Cyan on default**: Meets AA requirement
- **Green on dark**: Meets AA requirement
- **Red on dark**: Meets AA requirement
- **Yellow on dark**: Meets AA requirement (with proper shade)

### Color-Blind Accessibility
- **No red/green alone**: Always use symbols
- **Contrast sufficient**: Not dependent on color differentiation
- **Symbol + color**: Ensures accessibility without color
- **Text labels**: Always include text description

### Terminal Theme Compatibility
- **Light terminals**: Adequate contrast maintained
- **Dark terminals**: Adequate contrast maintained
- **Common schemes**: Test with solarized, monokai, gruvbox, etc.
- **User preference**: Respect terminal color settings

---

## Terminal Color Codes

For implementation with terminal color libraries (ratatui, crossterm):

### Base Terminal Colors
- `Black` / `Gray` (0) - For neutral/default
- `Red` (1) - For errors
- `Green` (2) - For success, connected
- `Yellow` (3) - For loading/warning
- `Blue` (4) - For info
- `Cyan` (6) - For selection/focus
- `White` (7) - For emphasis/highlights
- `Default` - For text on theme-respecting background

### Modifier Combinations
- **Bold**: Use with colors for emphasis (e.g., green bold for success)
- **Dim**: Use for secondary/muted text
- **Reverse**: Use for selection highlight (inverse video)

---

## Best Practices

1. **Always pair symbol with color**: Never use color alone to convey status
2. **Maintain contrast**: Ensure WCAG AA minimum contrast ratio
3. **Respect terminal theme**: Don't override user's terminal colors when possible
4. **Use semantic colors consistently**: Same color always means same thing
5. **Test accessibility**: Verify with color-blind simulators
6. **Avoid color overload**: Use color judiciously for emphasis, not decoration

---

## Implementation Guide

When implementing colors in code:

```
Success message: Green text + `✓` symbol
Error message: Red text + `✗` symbol
Selection: Cyan background (reverse video)
Loading: Yellow text + `⟳` symbol
Connected: Green `●` symbol
```

All specs should reference this color system for visual consistency.

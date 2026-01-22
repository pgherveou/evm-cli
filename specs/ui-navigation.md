# UI Navigation & Keyboard Controls

## Overview

This document defines all navigation patterns, keyboard controls, and focus management behaviors used throughout the evm-cli TUI. Individual component specs reference this document to avoid duplication.

---

## Global Navigation

### Focus Areas

The application has two primary focus areas that can be toggled:
1. **Sidebar (Contracts Menu)** - Left panel for contract navigation
2. **Output Panel** - Right panel for viewing results and cards

### Focus Switching

| Key | Action | Available From |
|-----|--------|----------------|
| `Tab` | Toggle focus between sidebar and output panel | Anywhere |
| `Escape` | Close overlay/popup and return focus to previous area | Popups/Modals |

### Global Shortcuts

These shortcuts work regardless of which panel has focus:

| Key | Action | Context | Notes |
|-----|--------|---------|-------|
| `Ctrl+P` | Open Command Palette | Anywhere | Opens command selection overlay |
| `Ctrl+C` | Exit application | Anywhere | Immediate exit, no confirmation |
| `Ctrl+L` | Clear all output cards | Anywhere | Clears output panel content |
| `r` | View Receipt | When Transaction card selected | Works globally |
| `d` | Debug Trace/Call | When Transaction/Call card selected | Works globally |
| `c` | Copy | When Transaction card selected | Copies transaction hash |

---

## Component-Specific Navigation

### Sidebar (Contracts Menu)

**Navigation Keys:**

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `k` | Move up | Navigate to previous item |
| `↓` or `j` | Move down | Navigate to next item |
| `←` or `h` | Collapse | Collapse expanded node |
| `→` or `l` | Expand | Expand collapsed node |
| `Enter` | Select/Execute | Execute item action or expand/collapse |
| `Delete` or `Backspace` | Remove | Delete deployment or contract (no confirmation) |

**Selection Behavior:**
- Current item highlighted with cyan background
- Navigation does NOT wrap (stops at first/last item)
- First item auto-selected when sidebar loads
- Selection persists when switching focus

### Output Panel (Cards)

**Navigation Keys:**

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `k` | Previous card | Select previous output card |
| `↓` or `j` | Next card | Select next output card |
| `Home` or `Ctrl+Home` | First card | Jump to first card |
| `End` or `Ctrl+End` | Last card | Jump to last card |
| `Page Up` | Scroll up | Scroll output view up |
| `Page Down` | Scroll down | Scroll output view down |

**Card-Specific Actions (when card selected):**

| Key | Card Type | Action |
|-----|-----------|--------|
| `c` | Transaction | Copy transaction hash to clipboard |
| `r` | Transaction | View receipt in $EDITOR |
| `d` | Transaction | Open debug trace menu |
| `d` | Call | Debug call |

**Selection Behavior:**
- Selected card has bright white text with cyan left border
- Unselected cards have muted gray text
- Navigation DOES wrap (cycles from last to first)
- Auto-scrolls to keep selected card visible


### Command Palette

**Navigation Keys:**

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `k` | Move up | Navigate to previous command |
| `↓` or `j` | Move down | Navigate to next command |
| `Enter` | Execute | Run selected command |
| `Escape` | Close | Exit command palette |
| Type any character | Search | Filter commands in real-time |
| `Backspace` | Delete char | Remove search character |
| `Ctrl+U` | Clear search | Clear all search text |

**Selection Behavior:**
- Fuzzy search filters commands as you type
- First matching item auto-selected
- Navigation wraps at boundaries

### Parameter Input Popups

**Field Navigation:**

| Key | Action | Behavior |
|-----|--------|----------|
| `Tab` | Next field | Move to next input field |
| `Shift+Tab` | Previous field | Move to previous input field |
| `Enter` | Submit | Validate and submit form |
| `Escape` | Cancel | Close without submitting |

**Special Fields:**

**Boolean Fields:**
| Key | Action |
|-----|--------|
| `↑` or `↓` or `j` or `k` | Toggle between true/false |
| `Enter` | Confirm and move to next field |

**Constructor with Target Selection:**
| Key | Action |
|-----|--------|
| `←` or `→` | Switch between EVM/PVM target |
| `Tab` | Move to first parameter field |

**Input:**
| Key | Action |
|-----|--------|
| Type any character | Enter value |
| `Backspace` | Delete character |
| `Ctrl+A` | Select all (if supported) |
| `Ctrl+U` | Clear field (if supported) |

### File Picker Dialog

**Navigation Keys:**

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `↓` | Navigate | Move through file/directory list |
| `Tab` | Autocomplete | Complete partial path |
| `Enter` | Select/Enter | Open directory or select file |
| `Escape` | Cancel | Close picker without selection |
| Type path | Filter | Filter files/directories |

**Path Display:**
- Shows current directory path
- Updates as you navigate
- Supports relative and absolute paths

### Tracer Config Popup

**Navigation Keys:**

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `Tab` | Previous option | Navigate to previous toggle |
| `↓` or `Shift+Tab` | Next option | Navigate to next toggle |
| `Space` | Toggle | Toggle option ON/OFF |
| `Enter` | Execute | Run trace with current settings |
| `Escape` | Cancel | Close without tracing |

---

## Navigation Patterns

### Focus Retention

When switching between panels or opening/closing overlays:
- Previous selection is remembered and restored
- Scroll position is maintained
- Focus returns to the triggering element after overlay closes

### Visual Feedback

All navigable elements provide clear visual feedback:

| State | Visual Indicator |
|-------|-----------------|
| Selected/Active | Cyan background or border |
| Unselected/Inactive | Default colors (muted) |
| Hovered (if applicable) | Highlighted but not selected |
| Disabled | Grayed out appearance |

### Auto-Scrolling

Components automatically scroll to keep the selected item visible:
- **Sidebar:** Scrolls vertically to keep selected contract/method in view
- **Output Panel:** Scrolls to center selected card when possible
- **Command Palette:** Scrolls to keep selected command visible
- **Parameter Popups:** Scrolls to show active field

### Wrapping Behavior

| Component | Wraps? | Behavior |
|-----------|--------|----------|
| Sidebar | No | Stops at first/last item |
| Output Cards | Yes | Cycles from last to first |
| Command Palette | Yes | Cycles through filtered results |
| Parameter Fields | No | Stops at first/last field |
| File Picker | No | Stops at first/last item |

### Navigation Speed

All navigation is immediate with no animation or delay:
- Instant response to key presses
- No debouncing on navigation keys
- Search/filter updates in real-time as you type

---

## Accessibility Considerations

1. **Keyboard-Only Operation:** All features accessible without mouse
2. **Clear Focus Indicators:** Visible selection at all times
3. **Consistent Patterns:** Same keys work similarly across components
4. **No Hidden Actions:** All available actions visible or discoverable
5. **Status Feedback:** Current state always shown via visual indicators

---

## Platform Differences

The navigation system is designed to work consistently across platforms:
- **Linux/Mac:** All keybindings work as documented
- **Windows:** May require adjusted terminal settings for some Ctrl combinations
- **SSH/Remote:** Ensure terminal properly forwards all key combinations

Note: Some terminal emulators may intercept certain key combinations (e.g., Ctrl+C). Users should configure their terminals accordingly.
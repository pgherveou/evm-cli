# Design Patterns

This document describes reusable interaction and layout patterns used throughout the EVM CLI. These patterns ensure consistency and predictability across the entire interface.

---

## Global Keyboard Navigation Patterns

### Pattern: Escape to Dismiss
**Used in**: Command Palette, Parameter Popup, Card menus, Debug modals
**Behavior**:
1. User presses `Escape` at any time
2. Modal/overlay closes
3. Previous focus restored
4. No action taken

**Implementation**: Consistent across all modal-like components

### Pattern: Arrow Keys + Vim Navigation
**Used in**: Sidebar tree, Output panel cards, Command Palette, Debug menus
**Key bindings**:
- `↑` / `k` = Navigate up
- `↓` / `j` = Navigate down
- `←` / `h` = Collapse/collapse or previous action
- `→` / `l` = Expand/expand or next action

**Benefit**: Supports both arrow-key users and vi/vim power users

### Pattern: Tab to Switch Focus
**Used in**: Main interface
**Behavior**:
1. `Tab` toggles focus between sidebar and output panel
2. Last selected item in each area is remembered
3. Visual indicator shows which area has focus

**Rationale**: Allows quick switching between contract navigation and result viewing

### Pattern: Enter to Execute / Select
**Used in**: Sidebar actions, Command Palette commands, Card footer actions, Forms
**Behavior**:
1. Navigate to item with arrow keys
2. Press `Enter` to execute/select
3. Alternative: Use shortcut key for direct execution

**Consistency**: Same action across all interactive lists

### Pattern: Direct Key Bindings
**Used in**: Output card footer actions, Debug menu options
**Binding**: Action shown as `Name (key)` in UI
**Execution**: Press key directly without `Enter`
**Example**: `c` for copy, `r` for receipt, `d` for debug trace

---

## Selection and Focus Patterns

### Pattern: Cyan Highlight Selection
**Used in**: Sidebar items, Output cards, Command Palette commands, Footer actions
**Visual**: Bright cyan background on currently selected item
**Consistency**: Same color across all contexts
**Benefit**: Clear, immediate visual feedback of selection
**Accessibility**: Never the only indicator; combined with position/symbols

### Pattern: Auto-Select First Item
**Used in**: Sidebar, Output panel, Command Palette
**Trigger**: List displays or switches to view
**Behavior**:
1. First item automatically selected
2. Cyan highlight visible immediately
3. User can navigate away if desired

**Benefit**: Reduces startup friction for keyboard navigation

### Pattern: Selection Persistence
**Used in**: Sidebar contracts/instances, Output cards
**Behavior**:
1. Last selected item remembered per focus area
2. When switching focus back, previous selection restored
3. Visual position scrolled into view

**Benefit**: Maintains context during multi-area interactions

---

## Form and Input Patterns

### Pattern: Real-Time Validation Feedback
**Used in**: Parameter input fields, Command Palette search
**Feedback Loop**:
1. User types character
2. Field validates immediately
3. Error shown below field (if any)
4. Submit remains blocked until valid

**Error Display**:
```
fieldName (type):
invalidInput
✗ Error description explaining the issue
```

**Benefit**: User knows immediately if input is wrong

### Pattern: Field Navigation with Tab
**Used in**: Parameter input popup
**Navigation**:
- `Tab` = Next field
- `Shift+Tab` = Previous field
- `Enter` = Submit form (if all valid)
- `Escape` = Cancel

**Tab Order**: Follows logical top-to-bottom flow

### Pattern: Form Submission Validation
**Trigger**: User presses `Enter`
**Steps**:
1. All fields validated
2. If ANY field invalid:
   - All invalid fields show `✗` errors
   - Cursor moves to first invalid field
   - Form does NOT submit
3. If ALL fields valid:
   - Form submits immediately
   - Popup closes
   - Action executes

**Benefit**: Prevents partial/invalid submissions

### Pattern: Boolean Toggle Input
**Used in**: bool-type parameters, Debug tracer options
**Display**: Current value shown (`true` or `false`)
**Navigation**: `↑`/`↓` or `j`/`k` to toggle value
**Confirmation**: `Enter` to confirm and move to next field
**Constraint**: No text input allowed; only toggle with arrows

**Benefit**: Eliminates typing errors for boolean values

---

## Modal and Overlay Patterns

### Pattern: Centered Overlay Modal
**Used in**: Command Palette, Parameter Input, Deployment Target Selection
**Structure**:
```
┌─ Title ──────────────────────── esc ┐
│ Content area                        │
│ - Form fields                       │
│ - Lists                             │
│ - Options                           │
│                                     │
│ Footer with instructions            │
└─────────────────────────────────────┘
```

**Properties**:
- Horizontally and vertically centered
- Width: ~60-80% of terminal width
- Close hint (`esc`) in top-right
- Title spans full width
- Footer centered below content

**Behavior**:
- Focus trapped within modal
- `Escape` closes modal
- `Tab` moves focus within modal
- Outside area dimmed or disabled

### Pattern: Modal with Search/Filter
**Used in**: Command Palette
**Features**:
1. Search field at top
2. Real-time filtering of results
3. Groups shown only if contain matches
4. Clear search with `Ctrl+U`

**Display**:
```
┌─ Commands ──────────────────────┐
│> searchterm█                    │
│────────────────────────────────│
│ Matching
│ ▌Command 1                      │
│  Command 2                      │
└────────────────────────────────┘
```

---

## Asynchronous Feedback Patterns

### Pattern: Loading State Display
**Used in**: Status bar, Card headers, Modals
**Indicator**: `⟳` (spinning character) in yellow
**Message**: `⟳ [Action description]`
**Examples**:
- `⟳ Fetching transaction...`
- `⟳ Compiling Contract.sol...`
- `⟳ Waiting for confirmation...`

**Behavior**:
- Shows when async operation starts
- Updates with status changes
- Replaced with result when complete

### Pattern: Success State
**Indicator**: `✓` (check mark) in green
**Display**: `Status: Success ✓` or `✓ Operation completed`
**Location**: Card display, status message
**Duration**: Persistent until user navigates away

### Pattern: Error State
**Indicator**: `✗` (X mark) in red
**Display**: `✗ Error description with context`
**Location**: Inline in form, card status, or status bar
**Guidance**: Message includes how to fix or what went wrong

**Examples**:
- `✗ Invalid address format: must be 0x followed by 40 hex characters`
- `✗ Failed to parse ABI for Contract.sol`
- `✗ Transaction reverted: Insufficient balance`

### Pattern: Status Indicator with Icon + Color
**Rule**: Never use color alone to convey meaning
**Implementation**:
- ● (filled circle) + Green = Connected
- ○ (hollow circle) + Default = Disconnected
- ⟳ (rotating) + Yellow = Loading
- ✓ (check) + Green = Success
- ✗ (X) + Red = Error
- ℹ (info) + Blue = Information

---

## Navigation Patterns

### Pattern: Tree Hierarchy Navigation
**Used in**: Sidebar contracts/instances
**Structure**:
```
▾ Contract          ← Expanded
  ├ Item 1          ← Branch
  ├ Item 2          ← Branch
  └ Item 3          ← Final item
▸ Another           ← Collapsed
```

**Controls**:
- `↑`/`↓` = Navigate between items
- `←`/`h` = Collapse node
- `→`/`l` = Expand node
- `Enter` = Execute action

**Visual**:
- Indentation shows hierarchy (2 spaces per level)
- Connectors (`├`, `└`) show relationships
- Icons show node type

### Pattern: Card List Navigation
**Used in**: Output panel
**Structure**: Vertical stack of cards
**Navigation**:
- `↑`/`k` = Previous card
- `↓`/`j` = Next card
- `←`/`h` = Previous footer action
- `→`/`l` = Next footer action

**Behavior**:
- Wrapping enabled: Last card wraps to first
- Auto-scroll: Selected card scrolled into view
- Footer menu appears on selection

### Pattern: List Navigation with Wrapping
**Used in**: Output cards, Command Palette
**Behavior**:
- Down arrow at last item wraps to first
- Up arrow at first item wraps to last
- Visual indication when at end

**User Benefit**: Continuous navigation without stopping

### Pattern: List Navigation without Wrapping
**Used in**: Sidebar items
**Behavior**:
- Navigation stops at first item (can't go up)
- Navigation stops at last item (can't go down)
- No visual wraparound

**User Benefit**: Prevents unexpected jumps

---

## Layout Patterns

### Pattern: Three-Zone Main Interface
**Layout**:
```
┌─ Sidebar (25-30%) ─┬─ Output (70-75%) ─┐
│ Contracts tree     │ Cards display     │
├────────────────────┴───────────────────┤
│ Status Bar (1 line)                   │
└───────────────────────────────────────┘
```

**Zones**:
1. **Sidebar**: Contract/instance navigation (left, narrow)
2. **Output**: Results and interaction cards (right, wide)
3. **Status Bar**: Connection status (bottom, full width)

**Separators**: `─` and `│` characters form box borders

### Pattern: Responsive Terminal Resizing
**Trigger**: Terminal size changed (SIGWINCH signal)
**Behavior**:
1. Detect new terminal dimensions
2. Recalculate panel widths
3. Redraw interface immediately
4. No loss of state

**Minimum Size**: 80 characters wide, 24 lines tall
**Below Minimum**: Alert shown, app pauses until resized

### Pattern: Footer Action Menu
**Display**:
```
◇ Action 1 (key)    ◇ Action 2 (key)    ◇ Action 3 (key)
   ↑ (selected)
```

**Properties**:
- Appears below selected card
- Diamond `◇` prefixes each action
- Shortcut key in parentheses
- One action pre-selected (leftmost)

**Navigation**:
- `←`/`h` = Previous action
- `→`/`l` = Next action
- `Enter` = Execute selected action
- `Escape` = Hide menu (card stays selected)

---

## Tree Expansion Patterns

### Pattern: Auto-Expansion on Load
**Triggers**:
1. **Contract loaded**: Contract auto-expands to show Deploy/Load options
2. **Instance deployed**: Instance auto-expands to show methods
3. **Instance loaded**: Existing instance auto-expands
4. **Card created**: New card auto-selected in output

**Purpose**: Minimize navigation steps after actions complete

**Selection**: Newly expanded item becomes selected (cyan highlight)

---

## Deletion Patterns

### Pattern: Immediate Deletion (No Confirmation)
**Trigger**: `Delete` or `Backspace` on item
**Behavior**:
1. Item immediately removed from display
2. Item removed from configuration
3. Selection moves to next/previous item
4. No undo available

**Recovery**: Item can only be re-added manually

**Applies to**: Contracts, instances

**Alternative (not used in EVM CLI)**: Destructive actions with confirmation typically show confirmation dialog

---

## Accessibility Patterns

### Pattern: Keyboard-First Navigation
**Principle**: Every interactive element accessible via keyboard
**Implementation**:
- All navigation via arrow/vim keys
- All actions via `Enter` or shortcut keys
- No mouse required
- Clear visual focus indicators

### Pattern: Visual + Textual Feedback
**Principle**: Never rely on color alone
**Implementation**:
- Status indicators combine symbol + color
- Selection uses position + highlight + symbols
- Error messages use `✗` icon + red + description

### Pattern: High Contrast for Readability
**Principle**: WCAG AA minimum contrast ratio
**Implementation**:
- Cyan on default background (sufficient contrast)
- Green on dark (readable)
- Red on dark (readable)
- White text on dark backgrounds

---

## Configuration Patterns

### Pattern: Persistent Application State
**Storage**: `.evm-cli/config.json`
**Persisted**:
- RPC URL
- Active account and private key
- Deployed instances (addresses)
- Expanded/collapsed tree state (in app state)
- Last selected item

### Pattern: Clear State (Selective)
**Action**: "Clear State" command
**Clears**: Deployment addresses only
**Preserves**: RPC URL, account, private key
**Confirmation**: No confirmation dialog (immediate)
**Undo**: No undo; user must manually re-load instances

---

## Best Practices

1. **Consistency**: Reuse patterns instead of creating variations
2. **Feedback**: Provide immediate visual feedback for every user action
3. **Navigation**: Always provide clear exit paths (Escape)
4. **Accessibility**: Ensure all patterns work with keyboard-only navigation
5. **Predictability**: Patterns behave identically across contexts
6. **Discoverability**: Show shortcuts and hints in UI
7. **Terminal Awareness**: Account for variable terminal sizes and themes

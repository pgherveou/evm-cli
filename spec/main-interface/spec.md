# Main Interface Specification

## Overview

The TUI uses a full-screen terminal layout built with ratatui, featuring a sidebar for contract navigation and a main output area. The interface is divided into three zones: sidebar (left), output area (center/right), and status bar (bottom).

---

## Layout Structure

```
┌─ Contracts ──────────┬─ Output ─────────────────────────────────────┐
│ + Load new contract  │ balanceOf(addr: 0x123...) @ 0x456...         │
│ ▾ Demo.sol           │ Result: 1000                                 │
│   ◇ Deploy new       │                                              │
│   ◇ Load existing... │ ─────────────────────────────────────────    │
│   ▾ 0x789...         │ increment() @ 0x456...                       │
│     ├ getCount [v]   │ Transaction: 0xabc...                        │
│     ├ increment [pay]│ Status: Success                              │
│     └ setCount [pay] │ Gas used: 43210                              │
│                      │ Logs (1)                                     │
│ ▸ MyToken.sol        │   [0] Incremented @ 0x456...                 │
│                      │       newCount: 43                           │
│                      │                                              │
│                      │ ─────────────────────────────────────────    │
├──────────────────────┴──────────────────────────────────────────────┤
│ ● Connected | Chain: 1 | Account: 0xabc... | Balance: 10.5 ETH      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Zone Breakdown

### Sidebar (Left Panel)
- **Purpose:** Contract navigation and method selection
- **Width:** ~25-30% of terminal width (configurable based on content)
- **Content:** [See Contracts Menu specification](../contracts-menu/spec.md)

### Output Area (Center/Right)
- **Purpose:** Display command results, transaction history, and interactive output cards
- **Width:** ~70-75% of terminal width
- **Content:** [See Output Panel specification](../output-panel/spec.md)

### Status Bar (Bottom)
- **Purpose:** Display connection status and account information
- **Height:** 1 line
- **Content:** [See Status Bar section below](#status-bar)

---

## Status Bar

A thin footer bar spanning the full width of the terminal.

**Format:**
```
● Connected | Chain: 1 | Account: 0xabc... | Balance: 10.5 ETH
```

**Elements:**
- **Connection indicator:** `● Connected` or `○ Disconnected`
  - Green filled circle when connected
  - Hollow circle when disconnected
- **Chain ID:** Currently connected blockchain (e.g., `1` for Ethereum mainnet)
- **Account address:** Currently active account (truncated to last 8 characters: `0xabc...`)
- **Account balance:** ETH balance formatted with appropriate decimals
- **Loading indicator:** Shows during:
  - RPC calls (`⟳ Fetching transaction...`)
  - Contract compilation (`⟳ Compiling Contract.sol...`)
  - Pending transactions (`⟳ Waiting for confirmation...`)

**Status Bar States:**

1. **Connected & Idle**
   ```
   ● Connected | Chain: 1 | Account: 0xf24f... | Balance: 10.5 ETH
   ```

2. **Connected & Loading**
   ```
   ⟳ Fetching... | Chain: 1 | Account: 0xf24f... | Balance: 10.5 ETH
   ```

3. **Disconnected**
   ```
   ○ Disconnected | Chain: N/A | Account: 0xf24f... | Balance: 0 ETH
   ```

---

## Focus Management

The interface supports two focus areas:

1. **Sidebar Focus:** Contract tree is interactive, output area is read-only
   - Navigation with `↑/↓`, `j/k`
   - Expand/collapse with `←/→`, `h/l`
   - Execute with `Enter`

2. **Output Area Focus:** Output cards are navigable and interactive
   - Navigation with `↑/↓`, `j/k`
   - Card selection/menu with `Enter` or `Space`
   - Navigation with `Escape` to dismiss menus

**Switching Focus:**
- `Tab` - Toggle focus between sidebar and output area
- When switching to output, last selected card is re-selected
- When switching to sidebar, last navigated item is re-selected

---

## Visual Design

### Colors & Styling

- **Highlighted/Selected:** Cyan background
- **Success messages:** Green with bold
- **Error messages:** Red with bold
- **Info/Normal:** Default terminal color
- **Muted/Separator:** Dark gray
- **Loading:** Yellow

### Spacing & Layout

- **Padding:** 1 space on left/right of sidebar items
- **Vertical spacing:** 1 line between major sections
- **Indentation:** 2 spaces per tree level
- **Separators:** Use `──────────` for visual breaks

### Typography

- **Monospace font:** All output displayed in monospace for alignment
- **Truncation:** Long addresses/hashes truncated with `...` (last 8 chars shown)
- **Alignment:** Tree indicators aligned left, output content left-aligned

---

## Responsiveness

### Minimum Terminal Size
- **Width:** 80 characters (standard terminal width)
- **Height:** 24 lines (standard terminal height)

### Layout Adaptation
- **Small terminals (< 120 width):** Sidebar width reduced, output area narrowed
- **Very small terminals (< 80 width):** Alert user and suggest larger terminal

### Text Wrapping
- Output panel text wraps at terminal width
- Long contract addresses/hashes truncated with ellipsis
- Method signatures may wrap but remain readable

### Terminal Resize Behavior

**Real-time Adaptation:**
- Layout re-renders immediately when terminal is resized
- Sidebar and output area widths adjust proportionally
- SIGWINCH signal triggers immediate redraw
- No lag or delay in resize response

**Below Minimum Size (<80 width):**
- Alert message displayed: `Terminal too small. Please resize to at least 80 characters wide.`
- Application pauses until terminal is resized
- Layout resumes normal operation when size is acceptable

---

## Focus Indicators

### Sidebar Item Selection
- Cyan background highlighting
- Optional border or arrow indicator (e.g., `▌`)
- Icon prefix indicates item type

### Output Card Selection
- Highlight/border around active card
- Current card position indicator (optional: `Card 3 of 10`)
- Card is scrolled into view when selected

---

## Accessibility Considerations

- All interactive elements accessible via keyboard
- High contrast colors for visibility
- Clear visual indicators for focus and selection
- Status bar always visible for context
- No color-only status indicators (use text + icon)

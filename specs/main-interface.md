# Main Interface Specification

## Overview

The TUI uses a full-screen terminal layout built with ratatui, featuring a sidebar for contract navigation and a main output area. The interface is divided into three zones: sidebar (left), output area (center/right), and status bar (bottom).

---

## Layout Structure

```
┌─ Contracts ──────────┬─ Output ─────────────────────────────────────┐
│ + Load new contract...│ ┃ Connection                                │
│ ▾ Demo               │ ┃   Connected                               │
│   ◇ Deploy new       │ ┃   Account: 0xf24ff3a9cf04c71dbc94d0b566f7a│
│   ◇ Load existing... │ ┃   Balance: 999.999984 ETH                 │
│   ▾ 0x3469e1dac0661..│ ┃   Chain ID: 1337                          │
│     ├ getCount() [view]│ ┃                                          │
│     ├ increment() [send]│                                          │
│     └ setCount(_count: uint256) [send]│ ┃ Transaction         │
│                      │ ┃   Hash: 0xc2fe40cad071d8c6c645720756f6201 │
│ ▸ MyToken            │ ┃   Status: Success                         │
│                      │ ┃   Function: increment()                   │
│                      │ ┃   Contract: Demo                          │
│                      │ ┃   Address: 0x3469e1dac06611030aece8209f07 │
│                      │ ┃   Gas: 43,210                             │
│                      │ ┃                                           │
│                      │ ┃ ◇ Copy (c)  ◇ View Receipt (r)  ◇ Debug Trace (d)│
├──────────────────────┴──────────────────────────────────────────────┤
│ ● Connected | Chain: 1337 | Account: 0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac | Balance: 999.999984 ETH │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Zone Breakdown

### Sidebar (Left Panel)
- **Purpose:** Contract navigation and method selection
- **Width:** ~25-30% of terminal width (configurable based on content)
- **Content:** [See Contracts Menu specification](./contracts-menu.md)

### Output Area (Center/Right)
- **Purpose:** Display command results, transaction history, and interactive output cards
- **Width:** ~70-75% of terminal width
- **Content:** [See Output Panel specification](./output-panel.md)

### Status Bar (Bottom)
- **Purpose:** Display connection status and account information
- **Height:** 1 line
- **Content:** [See Status Bar section below](#status-bar)

---

## Status Bar

A thin footer bar spanning the full width of the terminal.

**Format Examples:**

Wide terminal (>120 chars) - Sidebar focused:
```
● Connected | Chain: 1 | Account: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 | Balance: 10.500000 ETH    Del: remove  Tab: switch tab  Ctrl+P: commands  Ctrl+C: quit
```

Medium terminal (80-120 chars) - Sidebar focused:
```
● Connected | Chain: 1 | Account: 0xf39F...2266 | Balance: 10.5 ETH    Del: remove  Tab: switch  Ctrl+P: cmds  Ctrl+C: quit
```

Narrow terminal (<80 chars) - Sidebar focused:
```
● Connected | Ch: 1 | 0xf3...66 | 10 ETH          Del  Tab  ^P  ^C
```

With output panel focused:
```
● Connected | Chain: 1 | Account: 0xf39F...2266 | Balance: 10.5 ETH    Tab: switch tab  Ctrl+P: commands  Ctrl+C: quit
```

**Elements (Left Side):**
- **Connection indicator:** `● Connected` or `○ Disconnected`
  - Green filled circle when connected
  - Gray hollow circle when disconnected
- **Chain ID:** Currently connected blockchain
  - Wide: `Chain: 1` 
  - Narrow: `Ch: 1`
  - Shows `N/A` when disconnected
- **Account address:** Responsive to terminal width
  - Wide (>120 chars): Full address `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`
  - Medium (80-120): Truncated `0xf39F...2266` (first 6 + last 4)
  - Narrow (<80): Compact `0xf3...66` (first 4 + last 2)
- **Account balance:** Format adapts to space
  - Wide: Full precision `10.500000 ETH`
  - Medium: 3 decimals `10.5 ETH` 
  - Narrow: Integer `10 ETH` or `1K ETH` for large values

**Elements (Right Side):**
- **Context-specific hints:**
  - When sidebar focused: `Del: remove` (shown before global hints)
- **Global hints:** `Tab: switch tab  Ctrl+P: commands  Ctrl+C: quit`

**Status Bar States:**

1. **Connected & Idle (Wide Terminal >120 chars)**
   ```
   ● Connected | Chain: 1 | Account: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 | Balance: 10.500000 ETH
   ```

2. **Connected & Idle (Medium Terminal 80-120 chars)**
   ```
   ● Connected | Chain: 1 | Account: 0xf39F...2266 | Balance: 10.5 ETH
   ```

3. **Connected & Idle (Narrow Terminal <80 chars)**
   ```
   ● Connected | Ch: 1 | 0xf3...66 | 10 ETH
   ```

4. **Disconnected**
   ```
   ○ Disconnected | Chain: N/A | Account: 0xf39F...2266 | Balance: N/A
   ```

**Graceful Disconnected Mode:**
- Application launches even when RPC server is unreachable
- Status bar shows "Disconnected" state
- Background polling attempts reconnection every 5 seconds
- On successful reconnection, status bar updates automatically
- Balance is fetched and displayed once connected

**Config Reload Behavior:**
- When config is edited via `Ctrl+P > Edit config`, the status bar updates to reflect changes
- If the private key (account) changes, the new account address and balance are displayed immediately
- A reconnection attempt is triggered to refresh connection status and balance
- Note: RPC URL changes require application restart to take effect (provider is created at startup)

---

## Focus Management

For detailed focus management and navigation patterns, see [UI Navigation & Keyboard Controls](./ui-navigation.md#focus-areas).

### Quick Overview
The interface has two primary focus areas:
1. **Sidebar (Left):** Contract navigation tree
2. **Output Panel (Right):** Result cards and logs

- **Switch Focus:** Press `Tab` to toggle between panels
- **Global Actions:** Card actions (`r`, `d`, `c`) work from any panel
- **Focus Memory:** Each panel remembers its last selection

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
- **Small terminals (< 120 width):** 
  - Sidebar width reduced
  - Output area narrowed
  - Status bar uses truncated addresses and compact balance format
- **Very small terminals (< 80 width):** 
  - Ultra-compact status bar format
  - Minimal keyboard hints
  - Alert user and suggest larger terminal if below minimum

### Text Wrapping
- Output panel text wraps at terminal width
- Long contract addresses/hashes truncated with ellipsis
- Method signatures may wrap but remain readable
- Status bar automatically adjusts element sizes based on available width

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

- Card is scrolled into view when selected

---

## Accessibility Considerations

- All interactive elements accessible via keyboard
- High contrast colors for visibility
- Clear visual indicators for focus and selection
- Status bar always visible for context
- No color-only status indicators (use text + icon)

---

## Acceptance Criteria

### Layout
- **AC-MI-1**: Application displays 3-zone layout (sidebar, output, status bar)
- **AC-MI-2**: Sidebar occupies ~25-30% of terminal width
- **AC-MI-3**: Output panel occupies ~70-75% of terminal width
- **AC-MI-4**: Status bar displays at bottom, 1 line height

### Status Bar
- **AC-MI-5**: Status bar shows connection indicator (● Connected or ○ Disconnected)
- **AC-MI-6**: Status bar shows chain ID when connected
- **AC-MI-7**: Status bar shows account address (truncated based on terminal width)
- **AC-MI-8**: Status bar shows balance when connected
- **AC-MI-9**: Status bar shows keyboard hints on right side

### Responsiveness
- **AC-MI-10**: Warning displayed when terminal width < 80 characters
- **AC-MI-11**: Layout re-renders immediately on terminal resize
- **AC-MI-12**: Address truncation adapts to terminal width

### Focus
- **AC-MI-13**: Tab key switches focus between sidebar and output panel
- **AC-MI-14**: Current focus indicated by cyan highlighting

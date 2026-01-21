# Output Panel Specification

## Overview

The Output Panel is the primary display area for command results, transaction history, and interactive output cards. It features a card-based interface where each output (transaction, call, or log) is presented as a selectable, navigable item. When a card is selected, the available actions for that card are displayed in the footer as an action menu. This design enables deeper exploration and debugging capabilities while maintaining a clean, organized view.

---

## Layout Structure

The output panel displays a vertical list of cards. Each output (transaction, call, or log) is presented as a card in the list:

```
┌─ Transaction ────────────────────────────────────────────────────┐
│ Hash: 0x1a2b3c4d5e6f...                                          │
│ Status: Success                                                  │
│ Function: swap(address,uint256) @ 0xDEFI...                      │
│ Gas Used: 125,450 / 500,000 (25%)                                │
│ From: 0xUser...                                                  │
│ To: 0xDEFI...                                                    │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

┌─ Call ───────────────────────────────────────────────────────────┐
│ Function: balanceOf(address) @ 0xToken...                        │
│ Parameter: address = 0xUser...                                   │
│ Result: 1000 (type: uint256)                                     │
│                                                                  │
│ ◇ Copy Result    ◇ View as JSON                                  │
└──────────────────────────────────────────────────────────────────┘
```

Cards are displayed in chronological order (newest at bottom). 
The selected card is highlighted with a background color, and its available actions appear in the footer.

---

## Card Types

The output panel displays three types of cards:

### 1. Transaction Card

Represents a state-changing transaction executed on the blockchain.

**Transaction Card States:**

Transaction cards can be in two states:

1. **Pending State** - Transaction submitted but not yet confirmed
   - Yellow ⟳ indicator
   - Card automatically polls `eth_getTransactionReceipt` for updates
   - Actions disabled until transaction is confirmed
   - Displays estimated gas and transaction hash

2. **Finalized State** - Transaction confirmed on blockchain
   - Green ✓ (success) or Red ✗ (failed) indicator
   - All actions available
   - Displays actual gas used, block number, and full transaction details

**When selected (finalized), shows:**
```
┌─ Transaction ──────────────────────────────────────────────────────┐
│ Hash: 0x1a2b3c4d5e6f...                                            │
│ Status: Success ✓                                                  │
│ Function: swap(address,uint256) @ 0xDEFI...                        │
│ Gas Used: 125,450 / 500,000 (25%)                                  │
│ Gas Price: 25 gwei                                                 │
│ Block: #12345678                                                   │
│ From: 0xUser...                                                    │
│ To: 0xDEFI...                                                      │
│ Value: 0 ETH                                                       │
│                                                                    │
│ Events (2):                                                        │
│   [0] Transfer @ 0xToken...                                        │
│       from: 0xUser... → to: 0xDEFI... → value: 1000                │
│   [1] Swap @ 0xDEFI...                                             │
│       amountIn: 1000 → amountOut: 2.5 ETH                          │
└────────────────────────────────────────────────────────────────────┘

Footer: ◇ View Receipt (r)   ◇ Debug Trace (d)   ◇ View Logs (l)
                ↑ (selected)
```

**When selected (pending), shows:**
```
┌─ Transaction ──────────────────────────────────────────────────────┐
│ Hash: 0x5a6b7c...                                                  │
│ Status: Pending ⟳                                                  │
│ Function: constructor() @ (deploying...)                           │
│ Gas Estimate: 123,456                                              │
│ Waiting for confirmation...                                        │
└────────────────────────────────────────────────────────────────────┘

Footer: (Actions available after confirmation)
```

### 2. Call Card

Represents a read-only function call (view/pure).

**When selected, shows:**
```
┌─ Call ────────────────────────────────────────────────────────────┐
│ Function: balanceOf(address) @ 0xToken...                         │
│ From: 0xUser...                                                   │
│                                                                   │
│ Parameters:                                                       │
│   account: 0xUser1234567890abcdef1234567890abcdef1234             │
│                                                                   │
│ Result: 1000000000000000000 (uint256)                             │
│ Result (readable): 1.0 tokens (assuming 18 decimals)              │
│                                                                   │
│ Block: #12345678                                                  │
│ Called at: 2024-01-15 14:30:45                                    │
└───────────────────────────────────────────────────────────────────┘

Footer: ◇ Copy Result (c)   ◇ View as JSON (j)   ◇ Copy Call Data (d)
                ↑ (selected)
```

### 3. Log Card

Represents informational output (compilation results, status messages, errors). Log cards are navigable but pressing Enter has no effect.

Eg

┌─ Log ────────────────────────────────────────────────────────────┐
│ Command: forge build Demo.sol                                    │
│ Result: success                                                  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘

---

## Card Navigation

### Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `k` | Select previous card (dismisses footer menu if open) |
| `↓` / `j` | Select next card (dismisses footer menu if open) |
| `←` / `h` | Navigate to previous action in footer menu |
| `→` / `l` | Navigate to next action in footer menu |
| `Enter` / `Space` | Execute selected footer action |
| `Escape` | Dismiss footer menu, keep card selected |
| Action key binding | Execute action directly (e.g., `r` for View Receipt) |

### Navigation Behavior

- **Selection highlighting:** Current card highlighted with cyan background or border
- **Wrapping:** Navigation wraps from last to first card and vice versa (always enabled)
- **Auto-scroll:** Selected card scrolled into view automatically
- **Position indicator:** Optional "Card N of M" display
- **Action Menu:** When an interactive card is selected, available actions appear in the footer

### Footer Menu Navigation

When a card is selected and the footer shows actions:

1. **Default Selection:** The leftmost action is pre-selected (highlighted)
2. **Navigation:** Use `←`/`→` or `h`/`l` to move between actions
3. **Execution:** Press `Enter` or the action's key binding to execute
4. **Dismissal:** Press `Escape` to hide footer (card remains selected)
5. **Quick Access:** Press action key binding directly (e.g., `r` for View Receipt)

**Key Bindings:**
- Actions in footer include single-letter shortcuts shown in parentheses
- Example: `◇ View Receipt (r)` - press `r` to execute directly
- Key bindings work whether footer is visible or not (if card supports that action)

**Visual:**
```
Footer: ◇ Copy Result (c)    ◇ View as JSON (j)    ◇ Copy Call Data (d)
                ↑ (selected)
```

The second card (CALL) is selected, shown with background highlighting, and its actions appear in the footer with the leftmost action pre-selected.

---

## Card Interactions

### Transaction Cards

When a transaction card is selected:

**Action Menu appears in footer:**
```
◇ View Receipt (r)   ◇ Debug Trace (d)   ◇ View Logs (l)
         ↑ (selected)
```

User can:
- Press `Enter` to execute the selected action (View Receipt)
- Press `←`/`→` or `h`/`l` to change selection
- Press `r`, `d`, or `l` directly to execute that action
- Press `j`/`k` to navigate to a different card (dismisses footer)

#### View Receipt

1. Fetches full transaction receipt via `eth_getTransactionReceipt`
2. Formats receipt as JSON with proper indentation
3. Opens result in `$EDITOR`
4. Returns to card view when editor closes
5. Card selection preserved

**Example Receipt Output:**
```json
{
  "transactionHash": "0x1a2b3c...",
  "blockNumber": 12345678,
  "status": 1,
  "gasUsed": "0x1e9d6",
  "logs": [...]
}
```

#### Debug Trace

1. Opens submenu with tracer options:
   - Call Tracer
   - Prestate Tracer
   - Oplog Tracer
   - FlatCallTracer

2. On tracer selection, configuration menu appears

3. User can modify tracer options (tracer-specific):

   **Tracer Configuration Options:**

   | Tracer | Option | Default | Description |
   |--------|--------|---------|-------------|
   | Call Tracer | `onlyTopCall` | `false` | Capture all nested calls (true = only top-level call) |
   | Call Tracer | `withLog` | `true` | Include log output in trace |
   | Prestate Tracer | `diffMode` | `true` | Show only state changes (true = diff mode, false = full state) |
   | Oplog Tracer | (none) | N/A | No configuration options available |
   | FlatCall Tracer | `includePrecompiles` | `false` | Exclude precompile calls (true = include them) |

4. On confirmation:
   - Executes `debug_traceTransaction` with selected tracer and config
   - Results formatted as JSON
   - Opened in `$EDITOR`
   - Returns to card view when done

**Configuration Menu Visual:**
```
Debug Configuration (Call Tracer)

onlyTopCall: true
  ↑/↓ to toggle

withLog: false
  ↑/↓ to toggle

[Confirm] [Cancel]
```

#### View Logs

1. Displays all event logs emitted by transaction
2. Logs automatically decoded if ABI available
3. Opens in `$EDITOR` for detailed viewing

**Decoded Log Example:**
```
Logs (2)
  [0] Transfer @ 0x5678...
      from: 0xaaaa...
      to: 0xbbbb...
      value: 100
  [1] Approval @ 0x5678...
      owner: 0xaaaa...
      spender: 0xcccc...
      value: 1000
```

### Call Cards

When a call card is selected:

**Action Menu appears in footer:**
```
◇ Copy Result (c)   ◇ View as JSON (j)   ◇ Copy Call Data (d)
         ↑ (selected)
```

User can:
- Press `Enter` to execute the selected action (Copy Result)
- Press `←`/`→` or `h`/`l` to change selection
- Press `c`, `j`, or `d` directly to execute that action
- Press up/down to navigate to a different card (dismisses footer)

#### Copy Result
- Copies the function result to clipboard using `arboard` crate
- Shows confirmation in status bar: "Copied result to clipboard"
- If clipboard unavailable: Display error message "Clipboard unavailable"

#### View as JSON
- Formats full call details as JSON
- Opens in `$EDITOR`
- Returns to card view on close

**JSON Example:**
```json
{
  "type": "call",
  "function": "balanceOf",
  "address": "0xToken...",
  "parameters": {
    "account": "0xUser..."
  },
  "result": "1000",
  "resultType": "uint256"
}
```

#### Copy Call Data
- Copies encoded function call data (hex) using `arboard` crate
- Useful for debugging and transaction replay
- Shows confirmation in status bar: "Copied call data to clipboard"
- If clipboard unavailable: Display error message "Clipboard unavailable"

### Log Cards

Log cards are non-interactive:

- Navigable with j/k or arrow keys
- Pressing Enter has no effect (no menu appears)
- No footer actions displayed when selected
- Visual styling differs from Transaction/Call cards to indicate non-interactive nature

---

## Output Panel Behavior

### Scrolling

- **Auto-scroll:** Panel scrolls to show newly added cards
- **Manual scroll:** User can scroll with Page Up/Page Down or mouse wheel
- **Scroll to bottom:** When new card added, automatically scroll to show it
- **Scroll to top:** Ctrl+Home shortcut

### Card Addition

When a new output is generated:

1. **Card Creation:** Full card is constructed and added to the list
2. **Auto-Selection:** The new card is automatically selected (highlighted)
3. **Auto-scroll:** List scrolls to show the new card
4. **State Updates:**
   - Transaction cards start in "Pending" state
   - Pending transactions automatically poll `eth_getTransactionReceipt`
   - Card updates to "Finalized" state when receipt is received
   - Card re-renders with full transaction details and event logs

**Example Flow:**
```
1. User calls contract method
2. Pending transaction card created → automatically selected
3. Card displays: "Status: Pending ⟳"
4. Background polling for receipt begins
5. Receipt received → card updates to "Status: Success ✓"
6. Full details (gas used, block number, events) now displayed
7. Footer actions become available
```

### Card Clearing

**Clear All Cards:**
- Keyboard shortcut: `Ctrl+L`
- Shows confirmation dialog before clearing
- All cards removed from output panel
- Cannot be undone

**Confirmation Dialog:**
```
┌─────────────────────────────────────────────┐
│ Clear all output cards?                     │
│                                             │
│ This will remove 12 cards from view.        │
│ This action cannot be undone.               │
│                                             │
│ [Yes (y)]  [No (n)]                         │
└─────────────────────────────────────────────┘
```

- Press `y` or `Enter` to confirm
- Press `n` or `Escape` to cancel
- Default selection: "No" (safe default)

**Session Persistence:**
- Cards kept in memory during session
- Cards cleared on application exit
- No persistent storage of card history

---

## Focus & Interactivity

### Focus States

The output panel operates with a single focus state where all navigation is available:

**When Output Panel Has Focus:**
- `j`/`k` or `↑`/`↓` - Navigate between cards (dismisses footer if showing)
- `h`/`l` or `←`/`→` - Navigate between footer menu actions (if footer visible)
- `Enter` - Execute currently selected footer action
- `Escape` - Dismiss footer menu (keeps card selected) or return focus to sidebar
- Action key bindings (e.g., `r`, `d`, `l`) - Execute actions directly

**Switching to Output Panel Focus:**
- Press `Tab` when sidebar has focus
- Last selected card is re-selected
- If card is interactive, footer menu automatically appears

**Switching from Output Panel:**
- Press `Tab` to return focus to sidebar
- Press `Escape` to dismiss footer and return to sidebar
- Card selection is preserved

### Footer Menu Behavior

**Automatic Display:**
- Footer appears automatically when an interactive card (Transaction/Call) is selected
- Leftmost action is pre-selected by default

**Dismissal:**
- Pressing `j`/`k` or `↑`/`↓` navigates to another card and dismisses footer
- Pressing `Escape` dismisses footer but keeps current card selected
- Selecting a non-interactive card (Log) shows no footer

**Action Execution:**
- Action executes (e.g., opening `$EDITOR` for receipt viewing)
- When action completes and user returns to app:
  - Original card remains selected
  - Footer menu automatically re-appears
  - Same action that was executed remains highlighted

---

## Error Handling

### Failed Network Requests

If a card interaction requires a network call and fails:

- Error shown as a new log card in the output panel
- Error message: `✗ Failed to fetch receipt: Connection timeout`
- Additional guidance: `ℹ Please check your RPC connection`
- User can retry the action by selecting the card again
- Network issue also reported in status bar

### Invalid Configuration

If debug trace configuration is invalid:

- Validation error shown in status bar or inline in config menu
- Error message: `✗ Invalid configuration: Please use true or false`
- User returned to config menu to correct values
- Cannot proceed until valid configuration provided

---

## Performance Considerations

### Large Number of Cards

- Cards kept in memory throughout session
- Scrolling remains responsive even with 100+ cards
- Optional: Implement card history limit or archiving
- Optional: Pagination mode for very long sessions

### Rendering Optimization

- Only visible cards fully rendered
- Off-screen cards stored efficiently
- Quick search/filter of cards (future enhancement)

---

## Accessibility

- Full keyboard navigation (no mouse required)
- Clear visual indicators for card types
- Color + icon indicators (not color alone)
- Consistent menu patterns across card types
- Tab navigation to other UI areas
- Help information accessible

---

## Visual Styling

### Card Appearance

- **Unselected:** Normal background
- **Selected**: Background highlighted (cyan)

Available actions for interactive cards (Transaction/Call) are displayed in the footer when selected.

### Status Colors

- **Success:** Green text
- **Failed:** Red text
- **Pending:** Yellow text
- **Info:** Blue text

### Icon Legend

- `✓` - Success/complete (green)
- `✗` - Error/failed (red)
- `⟳` - Loading/pending (yellow)
- `ℹ` - Info message (blue)
- `◇` - Action menu item

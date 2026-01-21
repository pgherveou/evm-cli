# Output Panel Specification

## Overview

The Output Panel is the primary display area for command results, transaction history, and interactive output cards. It features a card-based interface where each output (transaction, call, or log) is presented as a selectable, navigable item. This design enables deeper exploration and debugging capabilities while maintaining a clean, organized view.

---

## Layout Structure

The output panel occupies the right side of the main interface:

```
┌─ Output ──────────────────────────────────────────────────────────┐
│ ▌TX: 0x1a2b3c... | Success | swap(USDC→ETH) | Gas: 125,450 / 500k │
│  CALL: 0xabc123... → 0xdef456... | balanceOf(0x...) | Value: 0 ETH   │
│  ✓ Compilation succeeded                                            │
│                                                                      │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Card Types

The output panel displays three types of cards:

### 1. Transaction Card

Represents a state-changing transaction executed on the blockchain.

**Format:**
```
TX: 0x1a2b3c... | Success | swap(USDC→ETH) | Gas: 125,450 / 500,000
```

**Components:**
- **Prefix:** `TX:` indicator
- **Hash:** Transaction hash (truncated: first 6 + `...`)
- **Status:** `Success` (green) or `Failed` (red)
- **Function:** Method name and parameter types
- **Gas:** Used gas / Estimated gas

**Expanded View (when selected):**
```
┌─ Transaction ────────────────────────────────────────────────────────┐
│ Hash: 0x1a2b3c4d5e6f...                                              │
│ Status: Success                                                      │
│ Function: swap(address,uint256) @ 0xDEFI...                          │
│ Gas Used: 125,450 / 500,000 (25%)                                    │
│ From: 0xUser...                                                      │
│ To: 0xDEFI...                                                        │
│                                                                       │
│ ◇ View Receipt    ◇ Debug Trace   ◇ View Logs                        │
└────────────────────────────────────────────────────────────────────────┘
```

### 2. Call Card

Represents a read-only function call (view/pure).

**Format:**
```
CALL: 0xabc123...→0xdef456... | balanceOf(0x...) | Result: 1000
```

**Components:**
- **Prefix:** `CALL:` indicator
- **From/To:** Caller and target addresses (truncated)
- **Function:** Method name with truncated parameters
- **Result:** Return value

**Expanded View (when selected):**
```
┌─ Call ────────────────────────────────────────────────────────────┐
│ Function: balanceOf(address) @ 0xToken...                         │
│ Parameter: address = 0xUser...                                    │
│ Result: 1000 (type: uint256)                                      │
│                                                                    │
│ ◇ Copy Result    ◇ View as JSON                                   │
└────────────────────────────────────────────────────────────────────┘
```

### 3. Log Card

Represents informational output (compilation results, status messages, errors).

**Format:**
```
✓ Compilation succeeded
✗ Error: Address validation failed
⟳ Fetching transaction receipt...
```

**Components:**
- **Icon:** Status indicator
  - `✓` - Success (green)
  - `✗` - Error (red)
  - `⟳` - Loading (yellow)
  - `ℹ` - Info (blue)
- **Message:** Text content

**Non-interactive:** Log cards are navigable but pressing Enter has no effect.

---

## Card Navigation

### Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `k` | Select previous card |
| `↓` / `j` | Select next card |
| `Enter` / `Space` | Open card menu/details |
| `Escape` | Close card menu and return to card list |
| `q` | Close expanded view and return to list |
| `c` | Copy card content (context-dependent) |

### Navigation Behavior

- **Selection highlighting:** Current card highlighted with cyan background or border
- **Wrapping:** Navigation wraps from last to first card and vice versa
- **Auto-scroll:** Selected card scrolled into view
- **Position indicator:** Optional "Card N of M" display

**Visual:**
```
  TX: 0x1a2b3c... | Success | swap() | Gas: 125,450 / 500k
▌CALL: 0xabc123...→0xdef456... | balanceOf() | Result: 1000
  ✓ Compilation succeeded
```

---

## Card Interactions

### Transaction Cards

When a transaction card is selected and Enter/Space is pressed:

**Menu appears:**
```
▌View Receipt
 Debug Trace
 View Logs
```

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

   **Call Tracer options:**
   - `onlyTopCall` (true/false)
   - `withLog` (true/false)

   **Prestate Tracer options:**
   - `diffMode` (true/false)

   **Oplog Tracer options:**
   - (no options)

   **Flatcall Tracer options:**
   - `includePrecompiles` (true/false)

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
3. Shows in card details or new expanded view

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

**Menu appears:**
```
▌Copy Result
 View as JSON
 Copy Call Data
```

#### Copy Result
- Copies the function result to clipboard
- Shows confirmation in status bar

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
- Copies encoded function call data (hex)
- Useful for debugging

### Log Cards

Log cards are non-interactive:

- Navigable with j/k or arrow keys
- Pressing Enter has no effect (no menu appears)
- Visual styling differs from TX/CALL cards to indicate non-interactive nature

**Visual Difference:**
```
▌TX: 0x1a2b3c... | Success | swap() | Gas: 125,450 / 500k
 CALL: 0xabc...→0xdef... | balanceOf() | Result: 1000
 ✓ Compilation succeeded                               (no highlight change)
```

---

## Card Display Formats

### Inline Format (List View)

Cards displayed as single-line items in vertical list:

```
TX: 0x1a2b3c... | Success | swap(USDC→ETH) | Gas: 125,450 / 500,000
CALL: 0xabc123...→0xdef456... | balanceOf(0x...) | Result: 1000
✓ Compilation succeeded
```

### Expanded Format

When a card is focused and full details shown:

**Transaction Expanded:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Transaction
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hash: 0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b
Status: Success ✓
Function: swap(address indexed, uint256 amountIn, uint256 amountOutMin)
Address: 0xDEFI1234567890abcdef1234567890abcdef1234

From: 0xUser1234567890abcdef1234567890abcdef1234
To: 0xDEFI1234567890abcdef1234567890abcdef1234
Value: 0 ETH
Data: 0x38ed3740000...

Gas Used: 125,450 / 500,000 (25%)
Gas Price: 25 gwei
Nonce: 42

Block Number: 12345678
Block Hash: 0xblock...
Transaction Index: 5

Events (2):
  [0] Transfer @ 0xToken...
      from: 0xUser...
      to: 0xDEFI...
      value: 1000
  [1] Swap @ 0xDEFI...
      tokenIn: 0xToken...
      amountIn: 1000
      amountOut: 2.5 ETH

Press 'u' for receipt, 'd' for debug, 'q' to close
```

**Call Expanded:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Call
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Function: balanceOf(address)
Address: 0xToken1234567890abcdef1234567890abcdef1234

Parameters:
  account: 0xUser1234567890abcdef1234567890abcdef1234

Result: 1000000000000000000 (uint256)
Result (readable): 1.0 (assuming 18 decimals)

Called at: 2024-01-15 14:30:45
Block Number: 12345678
```

---

## Output Panel Behavior

### Scrolling

- **Auto-scroll:** Panel scrolls to show newly added cards
- **Manual scroll:** User can scroll with Page Up/Page Down or mouse wheel
- **Scroll to bottom:** When new card added, automatically scroll to show it
- **Scroll to top:** Ctrl+Home shortcut

### Card Addition

When a new output is generated:

1. **During execution:** Status message shown ("Deploying...", "Executing...", etc.)
2. **On completion:** Full card added to list
3. **Auto-scroll:** List scrolls to show new card
4. **Selection:** New card optionally selected (configurable)

### Card Clearing

- **Clear all:** Ctrl+L clears all cards (with confirmation)
- **Clear type:** Optional submenu to clear only specific card types
- **Session persistence:** Cards kept in memory during session, cleared on app exit

---

## Integration with Compilation & Deployment

### Compilation Output

When a contract is compiled:

```
▌⟳ Compiling Counter.sol...
 ✓ Compilation succeeded
 ℹ ABI extracted: 3 methods, 0 events
```

### Deployment Output

When a contract is deployed:

```
▌⟳ Deploying Counter.sol...
 TX: 0x5a6b7c... | Pending | constructor() | Gas: 123,456 / 500,000
 ✓ Deployment confirmed
 ℹ Contract deployed to: 0xDeployed1234567890abcdef1234567890abcdef12
```

### Method Call Output

When a method is called:

```
▌CALL: 0xUser...→0xDeployed... | getCount() | Result: 42
 TX: 0x1a2b3c... | Success | setCount(uint256) | Gas: 43,210 / 500,000
 ✓ Transaction confirmed
```

---

## Focus & Interactivity

### Focus States

The output panel can be in two states:

1. **Card Navigation Mode:** j/k or arrow keys navigate between cards
2. **Menu Mode:** Up/down keys navigate menu options, Enter selects

**Switching to Output Panel Focus:**
- Press `Tab` when sidebar focused
- Last selected card is re-selected

**Switching from Output Panel:**
- Press `Tab` or `Escape` returns focus to sidebar
- Card selection preserved

### Menu Dismissal

Pressing `Escape` when a menu is open:
- Menu closes
- Returns to card navigation mode
- Selected card remains highlighted

---

## Error Handling

### Failed Network Requests

If a card interaction requires a network call and fails:

```
▌TX: 0x1a2b3c... | Success | swap() | Gas: 125,450 / 500k
 ✗ Failed to fetch receipt: Connection timeout
 ℹ Please check your RPC connection
```

- Error shown as log card
- User can retry the action
- Network issue reported in status bar

### Invalid Configuration

If debug trace configuration is invalid:

```
▌Call Config: onlyTopCall = unknown value
 ✗ Invalid configuration: Please use true or false
```

- Validation error shown
- User returned to config menu to correct

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

**Unselected:**
```
  TX: 0x1a2b3c... | Success | swap() | Gas: 125,450 / 500k
```

**Selected (Highlighted):**
```
▌TX: 0x1a2b3c... | Success | swap() | Gas: 125,450 / 500k
```

### Status Colors

- **Success:** Green text
- **Failed:** Red text
- **Pending:** Yellow text
- **Info:** Blue text

### Icon Legend

- `▌` - Selected card indicator
- `TX:` - Transaction card
- `CALL:` - Call card
- `✓` - Success/complete
- `✗` - Error/failed
- `⟳` - Loading/pending
- `ℹ` - Info message

---

## Keyboard Shortcuts (Output Panel Focus)

| Key | Action |
|-----|--------|
| `↑` / `k` | Select previous card |
| `↓` / `j` | Select next card |
| `Enter` / `Space` | Open card menu |
| `Escape` | Close menu / Return to sidebar |
| `q` | Close expanded view |
| `c` | Copy card content |
| `Ctrl+L` | Clear all cards (with confirmation) |
| `Home` / `Ctrl+Home` | Jump to first card |
| `End` / `Ctrl+End` | Jump to last card |
| `Page Up` | Scroll up |
| `Page Down` | Scroll down |
| `Tab` | Switch focus to sidebar |

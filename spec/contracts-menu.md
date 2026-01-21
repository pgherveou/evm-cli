# Contracts Menu Specification

## Overview

The Contracts Menu is a persistent sidebar displaying all loaded contracts and their deployed instances in a hierarchical tree structure. It provides keyboard-driven navigation for managing contracts, deployments, and accessing contract methods.

---

## Tree Structure

### Hierarchy Levels

```
+ Load new contract        ← Level 0: File picker action
▾ Counter.sol              ← Level 1: Loaded contract source
  ◇ Deploy new instance    ← Level 2: Deploy action
  ◇ Load existing...       ← Level 2: Load existing instance action
  ▾ 0x12...ab              ← Level 2: Deployed instance (address)
    ├ increment() [pay]    ← Level 3: Payable method
    ├ retrieve() [v]       ← Level 3: View method
    └ store(uint256) [pay] ← Level 3: Method with parameters
▸ MyToken.sol              ← Level 1: Collapsed contract
```

### Item Types and Visual Indicators

| Item Type | Indicator | Example | Description |
|-----------|-----------|---------|-------------|
| Load Action | `+` | `+ Load new contract` | Opens file picker to load `.sol` file |
| Contract (Expanded) | `▾` | `▾ Counter.sol` | Loaded contract, expanded to show instances |
| Contract (Collapsed) | `▸` | `▸ MyToken.sol` | Loaded contract, collapsed |
| Deploy Action | `◇` | `◇ Deploy new instance` | Deploy new instance with constructor params |
| Load Existing | `◇` | `◇ Load existing...` | Load already-deployed instance by address |
| Deployed Instance | Address | `▾ 0x12...ab` | Deployed instance (address truncated) |
| Tree Branch | `├` | │ | Connector for non-final items |
| Tree Final | `└` | │ | Connector for final item |
| Method Indicator | `│` | │ | Vertical line in tree |

### Method Type Indicators

Methods display their characteristics with tags:

| Tag | Type | Behavior | Gas Cost |
|-----|------|----------|----------|
| `[view]` | View | Read-only, no state changes | None (local call) |
| `[pay]` | Payable | Accepts ETH value | Yes (state-changing) |
| (none) | State-changing | Modifies contract state | Yes (gas required) |

**Example Methods:**
```
├ balanceOf(address) [view]        ← View function, read-only
├ transfer(address,uint256) [pay]   ← Payable function
├ approve(address,uint256)      ← State-changing (not view/payable)
```

---

## Auto-Expansion Behavior

The tree automatically expands in specific scenarios to improve UX:

### On Contract Load
- When a contract is loaded via "Load new contract"
- The contract node automatically expands
- Shows "Deploy new instance" and "Load existing..." options
- The contract becomes the selected item

### On Instance Deployment
- When a new instance is deployed successfully
- The deployed instance node automatically expands
- All methods are immediately visible
- The instance becomes the selected item

### On Instance Load
- When an existing instance is loaded
- The instance node automatically expands
- All methods are immediately visible
- The instance becomes the selected item

---

## Navigation

### Keyboard Controls

| Key | Action | Behavior |
|-----|--------|----------|
| `↑` or `k` | Move up | Navigate to previous item |
| `↓` or `j` | Move down | Navigate to next item |
| `←` or `h` | Collapse | Collapse expanded node |
| `→` or `l` | Expand | Expand collapsed node |
| `Enter` | Select | Execute item action or expand |
| `Delete` / `Backspace` | Remove | Delete deployment or contract |

### Selection & Focus

- **Visual Feedback:** Selected item highlighted with cyan background
- **Navigation Wrapping:** Optional wrapping from last to first item
- **Auto-Select:** First item automatically selected when sidebar loads
- **Persistence:** Last selected item persists within a session

---

## Interactions

### Load New Contract

**Action:** Press `Enter` on "Load new contract"

**Behavior:**
1. Opens file picker dialog
2. Filters for `.sol` files
3. Supports autocomplete/search
4. On selection:
   - Contract loaded and compiled
   - Contract added to sidebar as new node
   - Contract automatically expanded
   - Becomes the selected item
   - Methods displayed under contract

**Error Handling:**
- File not found: Show error in output panel
- Compilation failed: Display error details
- Invalid contract file: Reject and prompt user

### Deploy New Instance

**Action:** Press `Enter` on "Deploy new instance"

**Behavior:**
1. Shows [Parameter Input Popup](./tx-and-call-popup.md) for constructor args
2. On parameter submission:
   - Optionally shows [Deployment Target Selection](./tx-and-call-popup.md#deployment-target-selection)
   - Transaction sent to blockchain
   - Shows transaction hash in output panel
   - Waits for confirmation
   - Deployed instance added to sidebar
   - Instance automatically expanded
   - Becomes the selected item
   - Methods immediately visible

**Feedback:**
- Status shown in output panel: "Deploying...", "Deployed to 0x...", etc.
- Gas estimation shown before sending
- Transaction hash displayed during pending

### Load Existing Instance

**Action:** Press `Enter` on "Load existing..."

**Behavior:**
1. Prompts for contract address (in output panel or as popup)
2. On address submission:
   - Validates address format
   - Connects to instance on blockchain
   - Retrieves instance ABI from contract
   - Instance added to sidebar
   - Instance automatically expanded
   - Becomes the selected item
   - Methods immediately visible

**Validation:**
- Address format: `0x` + 40 hex characters
- Address validity: Check against network

### Execute Contract Method

**Action:** Press `Enter` on a method name

**Behavior:**

#### For View/Pure Methods:
1. If no parameters: Execute immediately as `eth_call`
2. If parameters: Show [Parameter Input Popup](./tx-and-call-popup.md)
   - User enters parameter values
   - On submission: Execute as `eth_call`
3. Result displayed in [Output Panel](./output-panel.md)

#### For State-Changing Methods:
1. If no parameters: Prompt for execution confirmation
2. If parameters: Show [Parameter Input Popup](./tx-and-call-popup.md)
   - User enters parameter values
   - On submission: Show transaction confirmation
3. Transaction sent with gas estimation
4. Hash displayed in output panel
5. After confirmation:
   - Full receipt displayed
   - Decoded logs shown
   - Result added as output card

#### For Payable Methods:
1. Parameter popup includes optional ETH value field
2. User can specify amount of ETH to send
3. Transaction includes value in addition to parameters

### Delete Deployment or Contract

**Action:** Press `Delete` or `Backspace` on deployed instance or contract

**Behavior:**

#### Deleting Deployed Instance:
1. Instance removed from sidebar
2. Address removed from deployment store
3. Session persists (state saved)
4. Selection moves to next item

#### Deleting Contract:
1. Contract and all instances removed from sidebar
2. All deployments for contract removed from store
3. Session persists (state saved)
4. Selection moves to next item

**Confirmation:**
- Optional confirmation prompt before deletion
- Message shown in output panel confirming removal

---

## Visual State Examples

### Loading a Contract

**Before:**
```
+ Load new contract
▸ Counter.sol
```

**After (during load):**
```
+ Load new contract
⟳ Counter.sol
```

**After (completed):**
```
+ Load new contract
▾ Counter.sol ← Selected & Expanded
  ◇ Deploy new instance
  ◇ Load existing...
```

### Deploying an Instance

**After deployment:**
```
▾ Counter.sol
  ◇ Deploy new instance
  ◇ Load existing...
  ▾ 0x1234567890abcdef1234567890abcdef12345678 ← Selected & Expanded
    ├ increment() [pay]
    ├ decrement() [pay]
    ├ retrieve() [view]
    └ store(uint256) [pay]
```

### Collapsed Contract

```
+ Load new contract
▸ Counter.sol ← Can expand with → or l
  ◇ Deploy new instance
▾ MyToken.sol
  ◇ Deploy new instance
```

---

## State Management

### Contract Tree State
- **Expanded/Collapsed:** Persisted per session (stored in app state)
- **Selection:** Last selected item restored on app restart
- **Deployments:** Persisted to `.evm-cli/config.json`

### Performance Considerations
- Tree lazily loads method lists on instance expansion
- Large contracts with many methods: Consider pagination (future)
- Efficient updates: Minimal re-renders on state change

---

## Accessibility

- Full keyboard navigation (no mouse required)
- Clear visual indicators for item types
- Color + symbol indicators (not color alone)
- Status messages in output panel for all actions
- Tab navigation to other UI areas

# Contracts Menu Specification

## Overview

The Contracts Menu is a persistent sidebar displaying all loaded contracts and their deployed instances in a hierarchical tree structure. It provides keyboard-driven navigation for managing contracts, deployments, and accessing contract methods.

## Tree Structure

### Hierarchy Levels

```
+ Load new contract...     ← Level 0: File picker action
▾ Counter                  ← Level 1: Loaded contract (without .sol extension)
  ◇ Deploy new instance    ← Level 2: Deploy action
  ◇ Load existing instance... ← Level 2: Load existing instance action
  ▾ 0x12...ab              ← Level 2: Deployed instance (address)
    ├ increment() [send]   ← Level 3: State-changing method
    ├ retrieve() [view]    ← Level 3: View method (no params)
    └ store(_value: uint256) [send] ← Level 3: Method with parameter name and type
▸ MyToken                  ← Level 1: Collapsed contract
```

### Item Types and Visual Indicators

| Item Type | Indicator | Example | Description |
|-----------|-----------|---------|-------------|
| Load Action | `+` | `+ Load new contract...` | Opens file picker to load `.sol` file |
| Contract (Expanded) | `▾` | `▾ Counter` | Loaded contract, expanded to show instances |
| Contract (Collapsed) | `▸` | `▸ MyToken` | Loaded contract, collapsed |
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
| `[send]` | State-changing | Modifies contract state, may accept ETH | Yes (transaction required) |

**Example Methods:**
```
├ balanceOf(_account: address) [view]     ← View function, read-only
├ transfer(_to: address, _value: uint256) [send]  ← State-changing function
├ approve(_spender: address, _value: uint256) [send]  ← State-changing function
```

**Method Name Display:**
- Methods shown with parameter names and types inline
- Full signature visible directly in the sidebar
- Examples:
  - `balanceOf(_account: address)` 
  - `transfer(_to: address, _value: uint256)`
  - `increment()` for parameterless methods
  - `setCount(_count: uint256)`

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

## Navigation

For detailed keyboard controls and navigation behavior, see [UI Navigation & Keyboard Controls](./ui-navigation.md#sidebar-contracts-menu).

### Quick Reference
- **Move:** `↑`/`↓` or `j`/`k` to navigate items
- **Expand/Collapse:** `←`/`→` or `h`/`l`  
- **Select:** `Enter` to execute action
- **Delete:** `Delete` or `Backspace` to remove (no confirmation)
- **Switch Focus:** `Tab` to move to output panel

### Delete Behavior

**Immediate Deletion (No Confirmation):**

**Deleting a Deployed Instance:**
- Pressing `Delete` or `Backspace` on an instance address (e.g., `0x789...`)
- Instance immediately removed from tree
- Address removed from `.evm-cli/config.json` deployments array
- No confirmation dialog

**Deleting a Contract:**
- Pressing `Delete` or `Backspace` on a contract (e.g., `Demo.sol`)
- Contract and ALL its deployed instances removed from tree
- All instance addresses removed from config.json
- No confirmation dialog

## Interactions

### Load New Contract

**Action:** Press `Enter` on "Load new contract"

**Behavior:**
1. Opens file picker dialog
2. Filters for `.sol` files
3. Supports autocomplete/search
4. On selection:
   - Creates a "Loading contract card" in the output panel (See [output-panel.md](./output-panel.md) for card details)
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
   - Displays a deployment card in the output panel (See [output-panel.md](./output-panel.md) for card details)
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
3. Displays a call card in the output panel (See [output-panel.md](./output-panel.md) for card details)

#### For State-Changing Methods:
1. If no parameters: Prompt for execution confirmation
2. If parameters: Show [Parameter Input Popup](./tx-and-call-popup.md)
   - User enters parameter values
   - On submission: Show transaction confirmation
3. Transaction sent with gas estimation
4. Displays a transaction card in the output panel (See [output-panel.md](./output-panel.md) for card details)
5. After confirmation:
   - Full receipt displayed
   - Decoded logs shown

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
- No confirmation prompt (immediate deletion)
- Message shown in output panel confirming removal

### ABI Parsing Errors

**When ABI Cannot Be Extracted:**

If contract compilation succeeds but ABI parsing fails:
- Error message shown in output panel: `✗ Failed to parse ABI for Contract.sol`
- Contract is NOT loaded into sidebar
- User must fix contract and recompile
- Common causes: Malformed JSON, invalid ABI structure

---

## Visual State Examples

### Loading a Contract

**Before:**
```
+ Load new contract...
▸ Counter
```

**After (during load):**
```
+ Load new contract...
⟳ Counter
```

**After (completed):**
```
+ Load new contract...
▾ Counter ← Selected & Expanded
  ◇ Deploy new instance
  ◇ Load existing instance...
```

### Deploying an Instance

**After deployment:**
```
▾ Counter
  ◇ Deploy new instance
  ◇ Load existing instance...
  ▾ 0x1234567890abcdef1234567890abcdef12345678 ← Selected & Expanded
    ├ getCount() [view]
    ├ decrement() [send]
    ├ increment() [send]
    └ setCount(_count: uint256) [send]
```

### Collapsed Contract

```
+ Load new contract...
▸ Counter ← Can expand with → or l
  ◇ Deploy new instance
▾ MyToken
  ◇ Deploy new instance
```

---

## State Management

### Contract Tree State
- **Expanded/Collapsed:** Persisted per session (stored in app state)
- **Selection:** Last selected item restored on app restart
- **Deployments:** Persisted to `~/.evm-cli/config.json`

---

## Accessibility

- Full keyboard navigation (no mouse required)
- Clear visual indicators for item types
- Color + symbol indicators (not color alone)
- Status messages in output panel for all actions
- Tab navigation to other UI areas

---

## Acceptance Criteria

### Tree Structure
- **AC-CM-1**: "Load new contract" appears at top of sidebar
- **AC-CM-2**: Contracts display with ▾/▸ expansion indicators
- **AC-CM-3**: Methods show type tags [view] or [send]
- **AC-CM-4**: Methods display parameter names and types inline

### Navigation
- **AC-CM-5**: j/k or arrow keys navigate items
- **AC-CM-6**: h/l or arrow keys collapse/expand nodes
- **AC-CM-7**: Enter executes the selected action
- **AC-CM-8**: Navigation does NOT wrap at boundaries

### Auto-Expansion
- **AC-CM-9**: Contract auto-expands when loaded
- **AC-CM-10**: Instance auto-expands when deployed
- **AC-CM-11**: Instance auto-expands when loaded by address

### Load Contract
- **AC-CM-12**: Enter on "Load new contract" opens file picker
- **AC-CM-13**: File picker filters for .sol files
- **AC-CM-14**: Tab autocompletes path in file picker

### Deploy Instance
- **AC-CM-15**: Enter on "Deploy new instance" shows parameter popup
- **AC-CM-16**: Parameter popup includes EVM/PVM target selection
- **AC-CM-17**: Deployed instance address appears in tree after deployment

### Load Existing
- **AC-CM-18**: Enter on "Load existing..." prompts for address
- **AC-CM-19**: Valid address adds instance to tree

### Delete
- **AC-CM-20**: Delete/Backspace removes item immediately (no confirmation)
- **AC-CM-21**: Deleting contract removes all its instances
- **AC-CM-22**: Deletion persists to config.json

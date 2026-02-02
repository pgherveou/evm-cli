# Contracts Menu Specification

## Overview

The Contracts Menu is a persistent sidebar displaying all loaded contracts and their deployed instances in a hierarchical tree structure. It provides keyboard-driven navigation for managing contracts, deployments, and accessing contract methods.

## Tree Structure

### Hierarchy Levels

```
▸ Load contract...         ← Level 0: Contract loader menu (expandable)
  ├ Solidity (.sol)        ← Level 1: Solidity file picker
  └ Rust (Cargo project)   ← Level 1: Rust project picker
▾ Counter                  ← Level 0: Loaded Solidity contract
  ◇ Deploy new instance    ← Level 1: Deploy action
  ◇ Load existing instance... ← Level 1: Load existing instance action
  ▾ 0x12...ab              ← Level 1: Deployed instance (address)
    ├ increment() [send]   ← Level 2: State-changing method
    ├ retrieve() [view]    ← Level 2: View method (no params)
    └ store(_value: uint256) [send] ← Level 2: Method with parameter name and type
▸ MyToken                  ← Level 0: Collapsed contract
▾ my-token [Rust/PVM]      ← Level 0: Loaded Rust contract (tagged)
  ◇ Deploy new instance    ← Level 1: Deploy action (PVM only)
  ◇ Load existing instance... ← Level 1: Load existing instance action
```

### Item Types and Visual Indicators

| Item Type | Indicator | Example | Description |
|-----------|-----------|---------|-------------|
| Load Menu (Collapsed) | `▸` | `▸ Load contract...` | Expandable menu for loading contracts |
| Load Menu (Expanded) | `▾` | `▾ Load contract...` | Shows Solidity and Rust options |
| Load Solidity | `├` | `├ Solidity (.sol)` | Opens file picker for .sol files |
| Load Rust | `└` | `└ Rust (Cargo project)` | Opens folder picker for Cargo.toml projects |
| Contract (Expanded) | `▾` | `▾ Counter` | Loaded contract, expanded to show instances |
| Contract (Collapsed) | `▸` | `▸ MyToken` | Loaded contract, collapsed |
| Rust Contract | `▾` + tag | `▾ my-token [Rust/PVM]` | Loaded Rust contract (PVM-only indicator) |
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

### Load Contract Menu

The "Load contract..." item is an expandable menu that provides two options for loading contracts.

**Action:** Press `Enter` or `l` on "Load contract..."

**Behavior:**
1. Menu expands to show contract type options
2. Use `j`/`k` to navigate between options
3. Press `Enter` to select an option
4. Press `h` or `Escape` to collapse menu

**Visual States:**

Collapsed:
```
▸ Load contract...
▾ Counter
```

Expanded:
```
▾ Load contract...
  ├ Solidity (.sol)          ← Select to load Solidity contract
  └ Rust (Cargo project)     ← Select to load Rust contract
▾ Counter
```

---

### Load Solidity Contract

**Action:** Press `Enter` on "Solidity (.sol)"

**Behavior:**
1. Opens file picker dialog
2. Filters for `.sol` files
3. Supports autocomplete/search with Tab
4. On file selection:
   - Creates "Contract Loading" card in output panel (status: Compiling)
   - Runs `forge build` to compile contract
   - On success: Card updates to "Contract Loaded", contract added to sidebar
   - On failure: Card updates to "Contract Load Failed" with error details
5. Contract automatically expanded with "Deploy new instance" visible

**Contract Loading Card shows:**
- Source type: Solidity
- File path
- Build command executed
- Bytecode locations (EVM and PVM if available)

**Error Handling:**
- File not found: Show error in output panel
- Compilation failed: Display error details in card
- Invalid contract file: Reject and prompt user

---

### Load Rust Contract

**Action:** Press `Enter` on "Rust (Cargo project)"

**Behavior:**
1. Opens folder picker dialog
2. User enters path to folder containing `Cargo.toml`
3. System validates `Cargo.toml` exists and parses it
4. **Rust Contract Configuration popup** appears with:
   - Binary target selection (from `[[bin]]` entries)
   - Solidity interface selection (auto-matched by name)
   - Build mode toggle (Release/Debug)
5. On confirmation:
   - Creates "Contract Loading" card in output panel (status: Building)
   - Runs `cargo build [--release] --bin <target>`
   - Runs `forge build` on selected `.sol` interface for ABI
   - On success: Card updates, contract added to sidebar
   - On failure: Card updates with cargo error output

**Important:** Rust contracts only support PVM deployment. The EVM target option is disabled when deploying Rust contracts.

**Error Handling:**
- No Cargo.toml found: Show error in output panel
- No `[[bin]]` targets: Show error "No binary targets found in Cargo.toml"
- No .sol files: Show error "No Solidity interface files found"
- Build failed: Display cargo error output in card

---

### Rust Contract Configuration Popup

When loading a Rust contract, a configuration popup appears to select build options.

**Visual:**
```
┌─────────────────────────────────────────────────────────┐
│ Load Rust Contract                                      │
│                                                         │
│ Project: /tmp/my-token                                  │
│                                                         │
│ Binary Target:                                          │
│ > ● my-token                                            │
│   ○ other-bin                                           │
│                                                         │
│ Solidity Interface:                                     │
│   ● MyToken.sol  (matched)                              │
│   ○ OtherInterface.sol                                  │
│                                                         │
│ Build Mode:                                             │
│   ● Release                                             │
│   ○ Debug                                               │
│                                                         │
│                        [Load]  [Cancel]                 │
└─────────────────────────────────────────────────────────┘
```

**Fields:**

| Field | Description | Auto-selection |
|-------|-------------|----------------|
| Binary Target | `[[bin]]` entries from Cargo.toml | First if single, else requires selection |
| Solidity Interface | `.sol` files found in project | Auto-matched by name (see below) |
| Build Mode | Release or Debug compilation | Default: Release |

**Auto-matching Solidity Interface:**

The system attempts to match the Solidity interface file to the binary target name:

| Binary Name | Matched .sol File |
|-------------|-------------------|
| `my-token` | `MyToken.sol` (kebab-case → PascalCase) |
| `my_token` | `MyToken.sol` (snake_case → PascalCase) |
| `mytoken` | `Mytoken.sol` or `MyToken.sol` (case-insensitive) |

If no match found, user must manually select.

**Navigation:**
- `Tab` or `j`/`k`: Move between fields
- `Space` or `Enter` on radio: Toggle selection
- `Enter` on [Load]: Confirm and start loading
- `Escape` or `Enter` on [Cancel]: Cancel and close

**Validation:**
- Binary target must be selected
- Solidity interface must be selected
- If Cargo.toml has no `[[bin]]` entries: Show error, close popup

---

### Deploy Rust Contract Instance

**Action:** Press `Enter` on "Deploy new instance" under a Rust contract

**Behavior:**
1. Shows Parameter Input Popup for constructor args
2. **Target selection is locked to PVM** (EVM greyed out)
3. Bytecode loaded from `target/<name>.polkavm` (or debug variant based on build mode)
4. Deployment proceeds as normal PVM transaction

**Visual (Parameter Popup for Rust contract):**
```
┌─────────────────────────────────────────────────────────┐
│ Deploy my-token                                         │
│                                                         │
│ Target:                                                 │
│   ○ EVM  (not available for Rust contracts)             │
│   ● PVM                                                 │
│                                                         │
│ Constructor Parameters:                                 │
│   initialSupply: [1000000_______________]               │
│                                                         │
│                       [Deploy]  [Cancel]                │
└─────────────────────────────────────────────────────────┘
```

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
- **AC-CM-1**: "Load contract..." appears at top of sidebar as expandable menu
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

### Load Contract Menu
- **AC-CM-12**: "Load contract..." expands with Enter or l
- **AC-CM-13**: Menu shows "Solidity (.sol)" and "Rust (Cargo project)" options
- **AC-CM-14**: Menu collapses with h or Escape

### Load Solidity Contract
- **AC-CM-15**: "Solidity (.sol)" opens file picker filtered for .sol files
- **AC-CM-16**: Tab autocompletes path in file picker
- **AC-CM-17**: Contract Loading card created immediately on file selection
- **AC-CM-18**: Card updates to success/failure state after compilation

### Load Rust Contract
- **AC-CM-19**: "Rust (Cargo project)" opens folder picker
- **AC-CM-20**: Configuration popup appears after folder selection
- **AC-CM-21**: Binary targets parsed from Cargo.toml [[bin]] entries
- **AC-CM-22**: Solidity interface auto-matched by name when possible
- **AC-CM-23**: Build mode defaults to Release
- **AC-CM-24**: Contract Loading card shows cargo build progress
- **AC-CM-25**: Rust contracts tagged with [Rust/PVM] in sidebar

### Deploy Instance
- **AC-CM-26**: Enter on "Deploy new instance" shows parameter popup
- **AC-CM-27**: Solidity contracts: Parameter popup includes EVM/PVM target selection
- **AC-CM-28**: Rust contracts: EVM target disabled, PVM auto-selected
- **AC-CM-29**: Deployed instance address appears in tree after deployment

### Load Existing
- **AC-CM-30**: Enter on "Load existing..." prompts for address
- **AC-CM-31**: Valid address adds instance to tree

### Delete
- **AC-CM-32**: Delete/Backspace removes item immediately (no confirmation)
- **AC-CM-33**: Deleting contract removes all its instances
- **AC-CM-34**: Deletion persists to config.json

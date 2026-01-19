# evm-cli Product Requirements Document

## Overview

`evm-cli` is an interactive command-line tool for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains. It provides a streamlined developer experience with real-time fuzzy filtering, session persistence, and automatic state management.

---

## Table of Contents

1. [User Interface](#user-interface)
2. [Core Features](#core-features)
3. [Contract Interaction](#contract-interaction)
4. [Configuration](#configuration)
5. [Keyboard Shortcuts](#keyboard-shortcuts)
6. [Error Handling](#error-handling)
7. [File Structure](#file-structure)
8. [Dependencies](#dependencies)
9. [Future Considerations](#future-considerations)

---

## User Interface

### Layout Overview

The CLI uses a full-screen terminal layout with a sidebar and main content area:

```
┌──────────────────┬──────────────────────────────────────────────────────┐
│ Contracts        │                                                      │
│                  │                SCROLLABLE OUTPUT AREA                │
│ ▸ New contract   │                                                      │
│                  │  Results, transaction logs, and command history      │
│ ▾ Counter        │                                                      │
│   ├ Constructor  │                                                      │
│   └ 0x12...ab    │                                                      │
│     ├ increment  │                                                      │
│     ├ retrieve   │                                                      │
│     └ store      │                                                      │
│                  │                                                      │
│ ▸ MyToken        │                                                      │
│   └ Constructor  │                                                      │
├─────────────────────────────────────────────────────────────────────────┤
│ ● Connected                                   ctrl+p commands           │
└─────────────────────────────────────────────────────────────────────────┘
```

### Contract Sidebar (Left Panel)

A persistent sidebar displaying all loaded contracts and their deployed instances in a tree structure.

**Tree Structure:**
```
▸ New contract              ← Always at top, opens file picker
▾ Counter                   ← Contract name (collapsible)
  ├ Constructor(..)         ← Deploy new instance
  └ 0x12...ab               ← Deployed instance address
    ├ increment()           ← Method
    ├ retrieve()            ← Method
    └ store(uint256)        ← Method with parameters
▸ MyToken                   ← Another contract (collapsed)
```

**Hierarchy Levels:**
1. **New contract** - Special item at top to load a new .sol file
2. **Contract name** - Loaded contract source (e.g., `Counter`, `MyToken`)
3. **Constructor** - First item under each contract, used to deploy new instances
4. **Deployed address** - Each deployed instance shown as `0x...` (truncated)
5. **Methods** - Contract methods listed under each deployed address

**Interaction:**
- Use `↑/↓` arrow keys to navigate the tree
- `Enter` on "New contract" opens file picker
- `Enter` on Constructor opens parameter popup for deployment
- `Enter` on a method opens parameter popup (or executes if no params)
- `←/→` or `Enter` to collapse/expand tree nodes

**Visual Indicators:**
- `▸` Collapsed node (has children)
- `▾` Expanded node
- `├` Tree branch connector
- `└` Last item connector
- Selected item highlighted with accent background

### Scrollable Output Area

The main content area displays command results and transaction history.

- Occupies the right side of the screen
- Scrolls independently from the sidebar
- Shows separator lines between command outputs for clarity
- Default terminal background color

**Example Output:**
```
│ Calling retrieve()
│ Result: 42
│ ──────────────────────────────────────────────────────────────────
│
│ Sending store(100)
│ Transaction: 0xabc123...
│ Status: Success
│ Gas Used: 21000
│ ──────────────────────────────────────────────────────────────────
```

#### Output Formatting
- Each command followed by separator line
- Error messages highlighted distinctly
- Clear visual distinction between consecutive operations

#### Transaction Errors
- Failed transactions show revert reason
- Gas estimation errors displayed before sending
- Network errors show connection status

### Status Bar

A thin footer bar spanning the full width of the terminal.

**Left Side (below sidebar):**
- Connection status indicator (`● Connected` / `○ Disconnected`)
- Loading indicator during transactions

**Right Side (below output area):**
- Keyboard shortcut hints (`ctrl+p commands`)
- Context-sensitive hints (e.g., `esc cancel` during operations)

---

## Core Features

### 1. Command Palette (Ctrl+P)

A centered overlay menu for accessing application commands and actions.

**Mockup:**
```
                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │                                             │
                    │ Suggested                                   │
                    │ ▌Load Contract                              │ ← selected (highlighted)
                    │  Switch Contract              ctrl+space s  │
                    │  Clear State                  ctrl+space c  │
                    │                                             │
                    │ Settings                                    │
                    │  Open Config                  ctrl+space e  │
                    │  Change RPC URL                             │
                    │  Change Account                             │
                    └─────────────────────────────────────────────┘
```

**Features:**
- Centered modal overlay
- Search input with fuzzy filtering
- Grouped categories (Suggested, Settings, etc.)
- Keyboard shortcuts displayed on the right
- Selected item highlighted with accent background
- Press `esc` to close

**Available Commands:**
- Load new contract (.sol file)
- Switch to previously loaded contracts
- Clear state/reset
- Open `.evm-cli/config.json` settings in editor

### 2. Contract Sidebar Navigation

The sidebar provides a tree-based navigation for all contracts and their deployed instances.

**Features:**
- Always visible on the left side of the screen
- Tree structure shows contracts → deployments → methods
- Keyboard-driven navigation with arrow keys
- Visual feedback for selected/focused item
- Collapsible nodes for cleaner organization

**Method Type Indicators:**
Methods in the tree show their type:
- `view` - Read-only calls (no gas)
- `pure` - Pure computation (no gas)
- `send` - State-changing transactions
- `payable` - Transactions accepting ETH

**Workflow:**
1. Navigate to desired method using `↑/↓`
2. Press `Enter` to select
3. If method has parameters, popup appears
4. If no parameters, method executes immediately

### 3. Parameter Input Popup

When a contract method is selected, a centered popup collects the required parameters.

**Mockup:**
```
                    ┌─────────────────────────────────────────────┐
                    │ transfer(address,uint256)              esc  │
                    │                                             │
                    │ to (address):                               │
                    │ 0x742d35Cc6634C0532925a3b844Bc9e7595f█      │
                    │                                             │
                    │ amount (uint256):                           │
                    │                                             │
                    │                                             │
                    │ Press return to confirm, tab to next field  │
                    └─────────────────────────────────────────────┘
```

**Features:**
- Centered modal overlay (similar to Command Palette)
- Method signature as title
- Sequential field input with type hints
- Tab to move between fields
- Enter to submit, Escape to cancel

### 4. Session Persistence

**Automatic State Saving:**
- Last used contract path saved automatically
- Last used contract address saved automatically
- State persists in `.evm-cli/config.json`

**Auto-Load on Startup:**
- If previous session exists, automatically loads:
  - Last used contract (compiles if source exists)
  - Last used address
- Otherwise, opens Command Palette for contract selection

---

## Contract Interaction

### Command Categories

Methods are organized by type:

| Type | Description | Gas |
|------|-------------|-----|
| `[view]` | Read-only calls | No gas |
| `[pure]` | Pure computation | No gas |
| `[send]` | State-changing transactions | Gas required |
| `[payable]` | Transactions accepting ETH | Gas required |
| `[deploy]` | Constructor (only when no address set) | Gas required |

### Deployment Flow

1. Select `[deploy]` constructor command from autocomplete
2. Enter constructor arguments via Parameter Input Popup
3. Transaction hash displayed, waits for confirmation
4. Deployed address saved to history
5. Address automatically set as current contract

### Method Calls

**View/Pure Functions:**
- Executed as `eth_call` (no gas, immediate result)
- Result displayed in output area

**State-Changing Functions:**
- Sent as transaction
- Displays transaction hash
- Waits for receipt and shows status
- Gas usage displayed

### Supported Parameter Types

| Type | Input Format |
|------|--------------|
| `address` | `0x` + 40 hex characters |
| `bool` | `true` / `false` selection |
| `uint256`, `int256` | Numeric input (supports other bit sizes) |
| `bytes`, `bytes32` | Hex string (supports other fixed sizes) |
| `string` | Free text input |
| `array` | Dynamic input with "add more" option |
| `tuple` | Struct with field-by-field input |

---

## Configuration

### Config File Location

`.evm-cli/config.json`

### Schema

```json
{
  "config": {
    "rpc_url": "http://localhost:8545",
    "address": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
    "private_key": "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"
  },
  "deployments": {
    "/path/to/Contract.sol": ["0x123...", "0x456..."]
  },
  "last_contract": "/path/to/Contract.sol",
  "last_address": "0x123..."
}
```

the following are the default if no state exist:

  - "rpc_url": "http://localhost:8545",
  - "address": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
  - "private_key": "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"


### Solidity Compilation

Uses `forge build` for contract compilation. Build artifacts stored in `.evm-cli/build`.

---

## Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `Ctrl+P` | Open Command Palette |
| `Ctrl+C` | Exit application (when input is empty) |
| `Escape` | Close overlay / Cancel operation |

### Command Palette

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate through items |
| `Enter` | Execute selected command |
| `Escape` | Close palette |
| Type | Filter commands in real-time |

### Contract Sidebar

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate tree items |
| `←` | Collapse current node / Go to parent |
| `→` | Expand current node |
| `Enter` | Select item (execute method / expand node) |

### Parameter Input Popup

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Enter` | Submit form |
| `Escape` | Cancel and close |


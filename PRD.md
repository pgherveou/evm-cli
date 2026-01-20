# evm-cli Product Requirements Document

## Overview

`evm-cli` is an interactive terminal UI (TUI) for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains. It provides a streamlined developer experience with a sidebar-based layout, automatic contract expansion, event log decoding, and session persistence.

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

The TUI uses a full-screen terminal layout built with ratatui, featuring a sidebar and output panel:

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

### Contract Sidebar (Left Panel)

A persistent sidebar displaying all loaded contracts and their deployed instances in a tree structure.

**Tree Structure:**
```
+ Load new contract        ← File picker to load .sol file
▾ Counter.sol              ← Contract (expanded, auto-selected on load)
  ◇ Deploy new instance    ← Deploy with constructor params
  ◇ Load existing...       ← Load instance by address
  ▾ 0x12...ab              ← Deployed instance (expanded, auto-selected)
    ├ increment() [pay]    ← Payable method
    ├ retrieve() [v]       ← View method
    └ store(uint256) [pay] ← Method with parameters
▸ MyToken.sol              ← Another contract (collapsed)
```

**Hierarchy Levels:**
1. **Load new contract** - Always at top, opens file picker
2. **Contract name** - Loaded contract source (e.g., `Counter.sol`, `MyToken.sol`)
3. **Deploy new instance** - Deploy a new instance with constructor params
4. **Load existing instance** - Load an already-deployed instance by address
5. **Deployed address** - Each deployed instance shown as `0x...` (truncated)
6. **Methods** - Contract methods listed under each deployed address

**Auto-Expansion Behavior:**
- When a contract is loaded, it's automatically expanded and selected
- When a contract is deployed, the new instance is automatically expanded and selected
- All methods are immediately visible after deployment

**Interaction:**
- Use `↑/↓` or `j/k` (vim-style) to navigate the tree
- `Enter` on "Load new contract" opens file picker with autocomplete
- `Enter` on "Deploy new instance" opens parameter popup for deployment
- `Enter` on "Load existing..." prompts for address input
- `Enter` on a method opens parameter popup (or executes if no params)
- `←/→` or `h/l` to collapse/expand tree nodes
- `Delete` or `Backspace` to remove selected deployment or contract

**Visual Indicators:**
- `+` Load new contract action
- `◇` Action items (Deploy, Load)
- `▸` Collapsed node (has children)
- `▾` Expanded node
- `├` Tree branch connector
- `└` Last item connector
- `[v]` View function (read-only)
- `[pay]` Payable function (can send ETH)
- Selected item highlighted with cyan background

### Output Area (Right Panel)

The main content area displays command results, transaction history, and logs.

**Features:**
- Occupies the right side of the screen
- Auto-scrolls to bottom on new output
- Shows separator lines between operations

**Output Format:**
```
balanceOf(addr: 0xf24ff...) @ 0x1234...
Result: 1000

─────────────────────────────────────────────────

increment() @ 0x1234...
Transaction: 0xabc123...
Status: Success
Gas used: 43210
Logs (1)
  [0] Incremented @ 0x1234...
      newCount: 43

─────────────────────────────────────────────────
```

**Output Styling:**
- Function calls: `methodName(params) @ contractAddress` (cyan/highlight)
- Success messages: Green with bold
- Error messages: Red with bold
- Info messages: Dark gray
- Waiting/pending: Yellow
- Separators: Dark gray horizontal lines

**Event Log Decoding:**
When transactions emit events, logs are automatically decoded using the contract ABI:
- Event name displayed with contract address
- All parameters shown with names and decoded values
- Falls back to raw topics/data if decoding fails
- Works for all known deployed instances

**Example Decoded Log:**
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

### Status Bar

A thin footer bar spanning the full width of the terminal.

**Format:**
```
● Connected | Chain: 1 | Account: 0xabc... | Balance: 10.5 ETH
```

**Elements:**
- Connection status indicator (`● Connected` / `○ Disconnected`)
- Chain ID
- Current account address (truncated)
- Account balance
- Loading indicator during transactions

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
                    │ ▌Load Contract                              │ ← selected
                    │  Load Existing Instance                     │
                    │  Clear State                                │
                    │                                             │
                    │ Settings                                    │
                    │  Open Config                                │
                    │  Change RPC URL                             │
                    │  Change Account                             │
                    └─────────────────────────────────────────────┘
```

**Features:**
- Centered modal overlay
- Search input with real-time filtering
- Grouped categories (Suggested, Settings, etc.)
- Selected item highlighted with accent background
- Press `Esc` to close

**Available Commands:**
- Load new contract (.sol file)
- Load existing instance (by address)
- Clear state/reset (clears all deployments and session)
- Open `.evm-cli/config.json` settings

### 2. Contract Sidebar Navigation

The sidebar provides tree-based navigation with automatic state management.

**Auto-Expansion:**
- Contracts expand automatically when selected/loaded
- Deployed instances expand automatically after deployment
- Methods are immediately visible and ready to call

**Navigation:**
- Keyboard-driven with arrow keys or vim-style (hjkl)
- Visual feedback for selected/focused item
- Collapsible nodes for organization
- Delete key to remove deployments or contracts

**Method Type Indicators:**
Methods show their type with tags:
- `[v]` - View (read-only, no gas)
- `[pure]` - Pure computation (no gas)
- `[pay]` - Payable (accepts ETH)
- Methods without tags are state-changing (require gas)

### 3. Parameter Input Popup

When a contract method or constructor is selected, a centered popup collects parameters.

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
- Centered modal overlay
- Method/constructor signature as title
- Sequential field input with type hints
- Tab to move between fields
- Enter to submit, Escape to cancel
- Support for all Solidity types

### 4. Deployment Target Selection

For deployments, user selects bytecode target:

**Popup:**
```
                    ┌─────────────────────────────────────────────┐
                    │ Select Deployment Target                    │
                    │                                             │
                    │ ▌EVM                                        │ ← selected
                    │  PVM (PolkaVM)                              │
                    │                                             │
                    └─────────────────────────────────────────────┘
```

- **EVM** - Standard Ethereum Virtual Machine (default)
- **PVM** - PolkaVM bytecode (requires polkavm-enabled solc)

### 5. Session Persistence

**Automatic State Saving:**
- All deployed contract addresses saved per contract
- Persists to `.evm-cli/config.json`
- Survives application restarts

**Auto-Load on Startup:**
- Previously loaded contracts appear in sidebar
- Previously deployed instances are available
- Last used contract and instance state restored

**Clear State:**
- Command palette → "Clear State"
- Removes all deployments from store
- Clears sidebar state
- Persists the cleared state to disk

---

## Contract Interaction

### Compilation

Uses `solc` directly for contract compilation:
- Reads ABI from compiled artifacts
- Supports multi-contract .sol files
- Compiles on-demand during deployment
- Caches ABIs for performance

### Deployment Flow

1. Navigate to contract → "Deploy new instance"
2. Enter constructor arguments via popup
3. Select deployment target (EVM/PVM)
4. Transaction sent and hash displayed
5. Waits for confirmation
6. Deployed address saved to deployment store
7. **Instance automatically expanded and selected**
8. Methods immediately visible and ready to call

### Method Calls

**View/Pure Functions:**
- Executed as `eth_call` (no gas, immediate result)
- Result displayed in output area
- Format: `methodName(params) @ contractAddress`

**State-Changing Functions:**
- Sent as transaction with gas
- Shows transaction hash
- Waits for receipt and displays:
  - Status (Success/Reverted)
  - Gas used
  - **Decoded event logs** (if any)
- Format: `methodName(params) @ contractAddress`

### Event Log Decoding

When transactions emit events:
1. Logs are matched against contract ABI
2. Event signatures compared with log topics
3. Parameters decoded and displayed with names
4. Works for all deployed instances in the store
5. Falls back to raw display if ABI not found

### Supported Parameter Types

| Type | Input Format |
|------|--------------|
| `address` | `0x` + 40 hex characters |
| `bool` | `true` / `false` selection |
| `uint256`, `int256` | Numeric input (supports all bit sizes) |
| `bytes`, `bytes32` | Hex string (supports all fixed sizes) |
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
    "/absolute/path/to/Contract.sol": [
      "0x1234567890abcdef1234567890abcdef12345678",
      "0xabcdef1234567890abcdef1234567890abcdef12"
    ]
  }
}
```

**Default Values** (if no config exists):
- `rpc_url`: `http://localhost:8545`
- `address`: `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`
- `private_key`: `5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133`

**Deployment Storage:**
- Contract paths are canonicalized (absolute paths)
- Each contract can have multiple deployed instances
- Addresses stored as hex strings with `0x` prefix
- Cleared when "Clear State" command is executed

### Environment Variables

Override config with `.env` file:

```bash
PRIVATE_KEY=your_private_key_here
ETH_RPC_URL=http://localhost:8545
```

---

## Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `Ctrl+P` | Open Command Palette |
| `Ctrl+C` | Exit application |
| `Tab` | Switch focus between sidebar and output |
| `Escape` | Close overlay / Cancel operation |

### Contract Sidebar

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate tree items |
| `←/→` or `h/l` | Collapse/expand nodes |
| `Enter` | Select item (execute/expand) |
| `Delete` or `Backspace` | Remove deployment or contract |

### Command Palette

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate through items |
| `Enter` | Execute selected command |
| `Escape` | Close palette |
| Type | Filter commands in real-time |

### Parameter Input Popup

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Enter` | Submit form |
| `Escape` | Cancel and close |

---

## Error Handling

### Transaction Errors
- Revert reasons displayed when available
- Gas estimation errors shown before sending
- Network errors with connection status

### Compilation Errors
- Solc errors displayed in output panel
- Contract not loaded if compilation fails

### Input Validation
- Address format validated (0x + 40 hex chars)
- Numeric types validated for range
- Type mismatches caught before transaction

---

## File Structure

```
evm-cli/
├── src/
│   ├── main.rs              # Entry point, CLI args
│   ├── app.rs               # Main application logic
│   ├── compile.rs           # Solidity compilation
│   ├── store.rs             # Session persistence
│   ├── provider.rs          # Ethereum provider/signer
│   ├── method_list.rs       # ABI method parsing
│   ├── prompts.rs           # Parameter formatting utilities
│   ├── tui/                 # Terminal UI components
│   │   ├── mod.rs
│   │   ├── event.rs         # Event handling
│   │   ├── state.rs         # Application state
│   │   ├── layout.rs        # Layout management
│   │   └── widgets/         # UI widgets
│   │       ├── contract_tree.rs    # Sidebar contract tree
│   │       ├── output_area.rs      # Output panel
│   │       ├── command_palette.rs  # Command palette popup
│   │       ├── parameter_popup.rs  # Parameter input
│   │       └── status_bar.rs       # Status bar
│   └── tree_node.rs         # Tree node definitions
├── examples/
│   └── Demo.sol             # Demo contract for testing
├── .evm-cli/
│   └── config.json          # User config and state
├── Cargo.toml
├── PRD.md                   # This document
└── README.md
```

---

## Dependencies

### Core
- **alloy** - Ethereum library (RPC, signing, ABI)
- **ratatui** - Terminal UI framework
- **crossterm** - Terminal manipulation
- **tokio** - Async runtime

### Utilities
- **serde** / **serde_json** - Serialization
- **anyhow** - Error handling
- **inquire** - Interactive prompts (file picker, etc.)

### External
- **solc** - Solidity compiler (required in PATH)
- **anvil** / **hardhat** - Local blockchain for testing (optional)

---

## Future Considerations

### Planned Features
- [ ] Multi-chain support with chain selector
- [ ] Transaction history view
- [ ] Contract verification integration
- [ ] Custom event filtering
- [ ] Batch transaction support
- [ ] Contract watch mode (auto-reload on file changes)
- [ ] Export transaction history to CSV

### Nice to Have
- [ ] ENS name resolution
- [ ] ABI auto-fetching from Etherscan
- [ ] Gas price recommendations
- [ ] Transaction simulation/preview
- [ ] Contract favorites/bookmarks
- [ ] Theme customization

---

## Design Principles

1. **Keyboard First** - All actions accessible via keyboard
2. **Auto-Expansion** - Reduce clicks by auto-expanding on load/deploy
3. **Visual Clarity** - Padding, colors, and spacing for readability
4. **Immediate Feedback** - Show results instantly in output panel
5. **Smart Defaults** - Auto-select newly deployed instances
6. **Decode Everything** - Event logs decoded automatically when possible
7. **Session Continuity** - State persists across restarts
8. **Progressive Disclosure** - Tree structure hides complexity until needed

---

## Version History

- **v0.2.0** - Ratatui TUI with sidebar layout, auto-expansion, event decoding
- **v0.1.0** - Initial release with inquire-based prompts

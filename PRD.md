# evm-cli Product Requirements Document

## Overview

`evm-cli` is an interactive command-line tool for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains. It provides a streamlined developer experience with real-time fuzzy filtering, session persistence, and automatic state management.

## Core Features

### 1. Interactive Fuzzy Filter UI

The CLI uses a real-time fuzzy filter interface inspired by tools like fzf.

**Method Selection (`/`)**
- Press `/` to immediately display all available contract methods
- Type to filter methods in real-time (e.g., `/bal` filters to `balanceOf`)
- Methods are categorized by type:
  - `[view]` - Read-only calls (no gas)
  - `[send]` - State-changing transactions
  - `[payable]` - Transactions that accept ETH
  - `[deploy]` - Constructor (only shown when no address is set)
- Arrow keys to navigate, Enter to select, Escape to cancel

**Context Menu (`@`)**
- Press `@` to access context options
- Available actions:
  - Load new contract (.sol file)
  - Enter address manually
  - Reset (clear saved state)
  - Switch to previously loaded contracts
  - Switch to previously deployed addresses

**Mode Switching**
- If filter input is empty, typing `/` or `@` switches between modes
- Backspace on empty input is silently ignored (no action)

**Menu Display**
- Shows up to 5 matching items at a time
- If more matches exist, displays "... N more" indicator
- Items are rendered bottom-up, closest to the input line
- Separator line between command outputs for clarity

### 2. Session Persistence

**Automatic State Saving**
- Last used contract path is saved automatically
- Last used contract address is saved automatically
- State persists in `.evm-cli` file (JSON format)

**Auto-Load on Startup**
- If a previous session exists, automatically loads:
  - Last used contract (compiles if source exists)
  - Last used address
- No prompts required - goes straight to interactive mode

**Reset State**
- Use `@` → "Reset (clear saved state)" to clear saved contract/address
- After reset, next startup will not auto-load anything

### 3. Contract Interaction

**Deployment**
- Select constructor from method list (shown as `[deploy]`)
- Prompts for constructor arguments if any
- Displays transaction hash and waits for confirmation
- Saves deployed address to history

**Method Calls**
- View/Pure functions: Executed as `eth_call` (no gas, immediate result)
- State-changing functions: Sent as transaction, waits for receipt
- Automatic ABI encoding/decoding of parameters and return values

**Supported Parameter Types**
- `address` - Ethereum addresses (0x + 40 hex chars)
- `bool` - true/false selection
- `uint256`, `int256` (and other bit sizes)
- `bytes`, `bytes32` (and other fixed sizes)
- `string`
- `array` - Dynamic arrays with "add more" prompt
- `tuple` - Structs with field-by-field input

### 4. Configuration

**Environment Variables**
- `PRIVATE_KEY` - Wallet private key (required)
- `ETH_RPC_URL` - RPC endpoint (defaults to http://localhost:8545)

**dotenv Support**
- Automatically loads `.env` file from current directory
- Example `.env`:
  ```
  PRIVATE_KEY=your_private_key_here
  ETH_RPC_URL=http://localhost:8545
  ```

**Persistent Config (`.evm-cli`)**
```json
{
  "config": {
    "rpc_url": "http://localhost:8545"
  },
  "deployments": {
    "/path/to/Contract.sol": ["0x123...", "0x456..."]
  },
  "last_contract": "/path/to/Contract.sol",
  "last_address": "0x123..."
}
```

### 5. Solidity Compilation

**Built-in solc Integration**
- Requires `solc` (Solidity compiler) to be installed and available in PATH
- Compiles `.sol` files using the command:
  ```
  solc --combined-json abi,bin <path/to/Contract.sol>
  ```
- Parses the JSON output to extract:
  - ABI (Application Binary Interface) for method signatures
  - Bytecode (compiled EVM bytecode for deployment)
- Handles multiple contracts per file:
  - If file contains one contract, uses it directly
  - If file contains multiple contracts, prompts user to select which one
- Error handling:
  - Compilation errors are displayed and abort the operation
  - Warnings are ignored (only errors with severity "error" are fatal)

**Custom Bytecode Support**
- `-b, --bytecode <FILE>` flag to use pre-compiled bytecode
- Useful for PolkaVM or other custom compilation targets
- When provided:
  - ABI is still extracted from the Solidity source via `solc`
  - Bytecode is read from the specified file instead of using `solc` output
- Bytecode file should contain raw binary (not hex-encoded)

## User Interface

### Startup Display
```
Connected with account: 0x...
Balance: 100.000000 ETH
```

The app immediately enters interactive mode with the filter UI at the bottom of the screen.

### Filter UI Display
The filter menu renders at the bottom of the terminal, preserving command output above.

```
Result: 32
─────────────────────────────────────────────────────────────────

Calling store...
Transaction: 0x123...
Status: Success
─────────────────────────────────────────────────────────────────

> balanceOf(addr: address) -> uint256                       [view]
  transfer(to: address, amount: uint256)                    [send]
  approve(spender: address, amount: uint256)                [send]
  ... 7 more
─────────────────────────────────────────────────────────────────
/bal█
─────────────────────────────────────────────────────────────────
Contract: MyContract | Address: 0x12...ab | Chain: 31337

  / methods    @ context
```

### Status Bar (Footer)
- Positioned at bottom of screen, below the input line
- Shows current contract name
- Shows current address (or "not deployed")
- Shows chain ID
- Help hints for `/` and `@` shortcuts

## Command Line Arguments

```
evm-cli [OPTIONS]

Options:
  -c, --contract <CONTRACT>  Path to a .sol file to load on startup
  -b, --bytecode <BYTECODE>  Path to pre-compiled bytecode file
  -a, --address <ADDRESS>    Contract address to interact with
  -h, --help                 Print help
  -V, --version              Print version
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `/` | Open method filter (or switch to method mode if in context menu) |
| `@` | Open context menu (or switch to context mode if in method filter) |
| `↑/↓` | Navigate list |
| `Enter` | Confirm selection |
| `Escape` | Cancel and return to filter |
| `Backspace` | Delete character (ignored if input empty) |
| `Ctrl+C` | Exit application |

## Error Handling

**Graceful Cancellation**
- Pressing Escape or Ctrl+C during parameter input returns to main screen
- No error messages shown for user-initiated cancellations

**Transaction Errors**
- Failed transactions show revert status
- Gas usage displayed for all transactions

**Output Formatting**
- Each completed command is followed by a separator line
- Command outputs remain visible above the filter UI
- Clear visual distinction between consecutive operations

## File Structure

```
crates/evm-cli/
├── Cargo.toml
├── PRD.md
└── src/
    ├── main.rs          # Entry point, CLI args, startup logic
    ├── app.rs           # Main application state and interaction loop
    ├── filter_ui.rs     # Fuzzy filter UI component
    ├── method_list.rs   # ABI method parsing and display
    ├── context_menu.rs  # File path prompts
    ├── prompts.rs       # Parameter input prompts
    ├── provider.rs      # Ethereum provider/signer setup
    ├── solc.rs          # Solidity compilation
    ├── store.rs         # Persistence (.evm-cli file)
    └── ui.rs            # Terminal UI helpers
```

## Dependencies

- `alloy` - Ethereum interaction (provider, signer, ABI encoding)
- `crossterm` - Terminal raw mode and styling
- `inquire` - Interactive prompts for parameter input
- `dotenvy` - Environment variable loading
- `clap` - Command line argument parsing
- `tokio` - Async runtime

## Future Considerations

- Event log decoding and display
- Transaction history viewer
- Gas estimation before sending
- Multiple wallet support
- Network switching
- Contract verification integration

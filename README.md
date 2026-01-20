# evm-cli

An interactive terminal UI for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains.

[![asciicast](https://asciinema.org/a/tbS6u4RAYBPmO0sY.svg)](https://asciinema.org/a/tbS6u4RAYBPmO0sY)

![evm-cli recording](docs/recording.gif)

## Features

- **Interactive Sidebar UI** - Browse contracts, deployments, and methods in a tree view
- **Smart Contract Compilation** - Built-in Solidity compiler integration
- **Multi-Target Support** - Deploy to EVM or PolkaVM
- **Event Log Decoding** - Automatically decode contract events using ABI
- **Session Persistence** - Save deployment history and contract state
- **Auto-complete** - File path suggestions for contract loading
- **Real-time Updates** - See transaction status and decoded results instantly

## Installation

### Prerequisites

- Rust toolchain (1.70+)
- Solidity compiler (`solc`) in PATH
- EVM-compatible blockchain RPC endpoint

### Install from source

```bash
git clone https://github.com/paritytech/evm-cli.git
cd evm-cli
cargo install --path .
```

## Quick Start

1. **Create a `.env` file** with your credentials:

```bash
PRIVATE_KEY=your_private_key_here
ETH_RPC_URL=http://localhost:8545
```

2. **Run evm-cli**:

```bash
evm-cli
```

3. **Load a contract**:
   - Press Enter on "Load new contract..."
   - Type or autocomplete the path to your `.sol` file
   - Press Enter

4. **Deploy or interact**:
   - Navigate with arrow keys or `j/k`
   - Press Enter to select actions
   - View methods by expanding deployed instances

## Usage

### Keyboard Controls

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate up/down |
| `←/→` or `h/l` | Collapse/expand nodes |
| `Enter` | Execute selected item |
| `Tab` | Switch focus between panels |
| `Ctrl+P` | Open command palette |
| `Delete/Backspace` | Remove selected deployment or contract |
| `Ctrl+C` | Exit application |

### Interface Layout

```
┌─ Contracts ────────────┬─ Output ─────────────────┐
│ + Load new contract    │ balanceOf(...) @ 0x1234  │
│ ▾ MyToken.sol          │ Result: 1000             │
│   ◇ Deploy             │                          │
│   ◇ Load existing...   │ ─────────────────────    │
│   ▾ 0x5678...          │ increment() @ 0x1234     │
│     ├ transfer() [pay] │ Transaction: 0xabcd...   │
│     ├ balanceOf() [v]  │ Status: Success          │
│     ├ approve() [pay]  │ Logs (1)                 │
│                        │   [0] Transfer @ 0x1234  │
│                        │       from: 0xaaaa...    │
│                        │       to: 0xbbbb...      │
│                        │       value: 100         │
└────────────────────────┴──────────────────────────┘
 Connected | Chain: 1 | Account: 0xabc... | Balance: 10 ETH
```

### Command Line Options

```
evm-cli [OPTIONS]

Options:
  -c, --contract <CONTRACT>  Path to a .sol file to load on startup
  -a, --address <ADDRESS>    Contract address to interact with
  -h, --help                 Print help
  -V, --version              Print version
```

### Environment Variables

- `PRIVATE_KEY` (required) - Private key for signing transactions
- `ETH_RPC_URL` (optional) - RPC endpoint (defaults to http://localhost:8545)

## Examples

### Deploy a Contract

1. Start evm-cli
2. Select "Load new contract..."
3. Enter path: `examples/Demo.sol`
4. Select "Deploy new instance"
5. Enter constructor parameters
6. Select target (EVM/PVM)
7. Wait for deployment confirmation

### Call View Functions

1. Navigate to a deployed instance
2. Press Right arrow to expand methods
3. Select a method marked `[view]`
4. Press Enter
5. See results instantly in Output panel

### Execute Transactions

1. Navigate to a payable method
2. Press Enter
3. Enter parameters if required
4. Wait for transaction confirmation
5. View decoded event logs automatically

## Features in Detail

### Event Log Decoding

When contracts emit events, evm-cli automatically:
- Matches event signatures with the ABI
- Decodes indexed and non-indexed parameters
- Displays human-readable parameter names and values

Example output:
```
Logs (1)
  [0] Transfer @ 0x1234...
      from: 0xaaaa...
      to: 0xbbbb...
      value: 100
```

### Session Persistence

evm-cli saves state to `~/.evm-cli.json`:
- Contract paths and ABIs
- Deployment addresses per contract
- Last selected contract and instance

Use Ctrl+P → "Reset" to clear saved state.

### Multi-Target Compilation

Deploy the same Solidity contract to:
- **EVM** - Standard Ethereum Virtual Machine
- **PVM** - PolkaVM (requires polkavm-enabled solc)

Select target during deployment.

## Development

### Build

```bash
cargo build --release
```

### Run locally

```bash
cargo run -- --contract examples/Demo.sol
```

### Run tests

```bash
cargo test
```

## Creating a Demo

See [DEMO.md](DEMO.md) for instructions on creating terminal casts and screenshots.

## Architecture

Built with:
- [alloy](https://github.com/alloy-rs/alloy) - Ethereum library for Rust
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation

### File Structure

```
evm-cli/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Main application logic
│   ├── compile.rs           # Solidity compilation
│   ├── store.rs             # Session persistence
│   ├── provider.rs          # Ethereum provider
│   ├── tui/                 # Terminal UI components
│   │   ├── widgets/         # UI widgets
│   │   │   ├── contract_tree.rs
│   │   │   ├── output_area.rs
│   │   │   ├── command_palette.rs
│   │   │   └── ...
│   │   ├── state.rs         # UI state
│   │   ├── event.rs         # Event handling
│   │   └── layout.rs        # Layout management
│   └── ...
├── examples/
│   └── Demo.sol             # Demo contract
├── Cargo.toml
├── DEMO.md                  # Demo recording guide
└── README.md
```

## Security Considerations

- **Never commit `.env` files** - They contain private keys
- **Use dedicated test accounts** - Don't use mainnet keys for development
- **Verify transaction details** - Review all parameters before confirming
- **Local testing** - Use local blockchains (Anvil, Hardhat) for development

## License

MIT License - see LICENSE for details.

## Contributing

Contributions welcome! Please open an issue or PR.

## Credits

Originally developed as part of [cargo-pvm-contract](https://github.com/paritytech/cargo-pvm-contract).

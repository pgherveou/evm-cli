# evm-cli

An interactive command-line tool for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains.

## Features

- **Interactive Fuzzy Filter UI** - Real-time method filtering with `/` command (inspired by fzf)
- **Context Menu** - Quick access to contract management with `@` command
- **Session Persistence** - Automatically saves and reloads last used contract and address
- **Automatic Compilation** - Built-in Solidity compiler integration (requires `solc`)
- **Full Type Support** - Handle addresses, arrays, tuples, and all Solidity types
- **Transaction Management** - View calls, state-changing transactions, and payable methods
- **.env Support** - Secure credential management via environment variables

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

3. **Load a contract** by pressing `@` and selecting "Load new contract"

4. **Deploy or interact** by pressing `/` to browse available methods

## Usage

### Command Line Options

```
evm-cli [OPTIONS]

Options:
  -c, --contract <CONTRACT>  Path to a .sol file to load on startup
  -b, --bytecode <BYTECODE>  Path to pre-compiled bytecode file
  -a, --address <ADDRESS>    Contract address to interact with
  -h, --help                 Print help
  -V, --version              Print version
```

### Interactive Commands

| Key | Action |
|-----|--------|
| `/` | Open method filter |
| `@` | Open context menu |
| `↑/↓` | Navigate list |
| `Enter` | Confirm selection |
| `Escape` | Cancel |
| `Ctrl+C` | Exit application |

### Environment Variables

- `PRIVATE_KEY` (required) - Private key for signing transactions
- `ETH_RPC_URL` (optional) - RPC endpoint (defaults to http://localhost:8545)

### Session Persistence

evm-cli automatically saves your session state to `.evm-cli` (JSON format):
- Last used contract path
- Last used contract address
- Deployment history per contract

Use `@` → "Reset (clear saved state)" to clear saved data.

## Examples

### Deploy a new contract

```bash
# Start with a specific contract
evm-cli --contract MyToken.sol

# Press / to see methods
# Select the constructor (shown as [deploy])
# Enter constructor arguments
# Contract deploys automatically
```

### Interact with existing contract

```bash
# Start with contract and address
evm-cli --contract MyToken.sol --address 0x123...

# Press / and type "balance" to filter
# Select balanceOf method
# Enter address parameter
# View result immediately
```

### Custom bytecode (e.g., PolkaVM)

```bash
# Use pre-compiled bytecode with Solidity ABI
evm-cli --contract MyContract.sol --bytecode contract.polkavm
```

## Architecture

For detailed product requirements and architecture, see [PRD.md](PRD.md).

### File Structure

```
evm-cli/
├── src/
│   ├── main.rs          # Entry point, CLI args, startup
│   ├── app.rs           # Main application loop
│   ├── filter_ui.rs     # Fuzzy filter UI component
│   ├── method_list.rs   # ABI method parsing
│   ├── context_menu.rs  # Context menu handlers
│   ├── prompts.rs       # Parameter input prompts
│   ├── provider.rs      # Ethereum provider/signer
│   ├── solc.rs          # Solidity compilation
│   ├── store.rs         # Session persistence
│   └── ui.rs            # Terminal UI helpers
├── Cargo.toml
├── PRD.md               # Product requirements
└── README.md
```

## Development

### Build

```bash
cargo build --release
```

### Run locally

```bash
cargo run -- --contract examples/MyToken.sol
```

### Run tests

```bash
cargo test
```

## Security Considerations

- **Never commit `.env` files** - They contain private keys
- **Use dedicated test accounts** - Don't use mainnet keys for development
- **Verify transaction details** - Review all parameters before confirming transactions

## License

MIT License - see workspace for details.

## Contributing

Contributions welcome! Please open an issue or PR.

## Credits

Originally developed as part of [cargo-pvm-contract](https://github.com/paritytech/cargo-pvm-contract).

Built with:
- [alloy](https://github.com/alloy-rs/alloy) - Ethereum library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [inquire](https://github.com/mikaelmello/inquire) - Interactive prompts

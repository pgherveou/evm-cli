# evm-cli

Interactive terminal UI for Solidity smart contracts on EVM chains.

![evm-cli demo](demo/demo.gif)

## Install

```bash
# Prerequisites: Foundry (https://github.com/paritytech/foundry-polkadot/)
git clone https://github.com/paritytech/evm-cli.git
cd evm-cli
cargo install --path .
```

## Usage

```bash
evm-cli
```

| Key | Action |
|-----|--------|
| `Enter` | Load contract / Deploy / Execute |
| `j/k` | Navigate up/down |
| `h/l` | Collapse/Expand tree nodes |
| `Tab` | Switch panels |
| `Ctrl+P` | Command palette |
| `Del/Backspace` | Delete selected item |
| `Ctrl+C` | Quit |

## Features

- **Load** `.sol` files with auto-compilation via Foundry
- **Deploy** to EVM or PVM targets
- **Call** view functions and send transactions
- **Inspect** transactions with debug traces
- **Vim-style** keyboard navigation

## Development

```bash
cargo build --release
cargo test
cargo clippy
```

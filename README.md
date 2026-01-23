# evm-cli

Interactive terminal UI for Solidity smart contracts on EVM chains.

[![asciicast](https://asciinema.org/a/uL5P6PpMVWVdGLKF.svg)](https://asciinema.org/a/uL5P6PpMVWVdGLKF)

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
| `h/l` | Collapse/Expand |
| `Tab` | Switch panels |
| `Ctrl+P` | Command palette |
| `Backspace` | Delete item |

## Features

- **Load** `.sol` files with auto-compilation
- **Deploy** to EVM or PVM targets
- **Call** view and state-changing methods
- **Inspect** transactions with debug traces
- **Vim-style** keyboard navigation

## Development

```bash
cargo build --release
cargo test
```

# evm-cli

An interactive terminal UI for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains.

[![asciicast](https://asciinema.org/a/uL5P6PpMVWVdGLKF.svg)](https://asciinema.org/a/uL5P6PpMVWVdGLKF)

## Installation

### Prerequisites

- [Foundry](http://github.com/paritytech/foundry-polkadot/)

### Install from source

```bash
git clone https://github.com/paritytech/evm-cli.git
cd evm-cli
cargo install --path .
```

## Quick Start

**Run evm-cli**:

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

## Development

### Build

```bash
cargo build --release
```

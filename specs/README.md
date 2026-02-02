# evm-cli Product Specification

## Overview

**evm-cli** is an interactive terminal UI (TUI) for deploying and interacting with smart contracts on EVM-compatible blockchains. It supports both Solidity and Rust contracts (via cargo-pvm-contract), providing a keyboard-driven interface for contract development workflows without leaving the terminal.

## Target Users

- Solidity developers working with local development nodes (Anvil, Hardhat)
- Rust developers building PolkaVM smart contracts
- Developers who prefer terminal-based workflows
- Teams testing smart contracts during development

## Supported Contract Types

evm-cli supports two types of smart contracts:

### Solidity Contracts
- **Source:** `.sol` files
- **Compilation:** `forge build` (Foundry)
- **Targets:** EVM and PVM (via resolc)
- **ABI:** Extracted from forge artifacts

### Rust Contracts (cargo-pvm-contract)
- **Source:** Cargo projects with `[[bin]]` targets
- **Compilation:** `cargo build` with PolkaVM target
- **Targets:** PVM only (EVM not supported)
- **ABI:** Extracted from accompanying Solidity interface file
- **Reference:** [cargo-pvm-contract](https://github.com/parity-tech/cargo-pvm-contract)

Rust contracts require a Solidity interface file (`.sol`) in the project that defines the contract's public API. This interface is used to generate the ABI for deployment and interaction.

## Core User Journey

```
1. Launch evm-cli
   └── App starts, connects to RPC (or gracefully shows disconnected state)

2. Load Contract
   └── Select "Load contract..." → Expands menu
       ├── Solidity (.sol): File picker → Select .sol file
       │   └── Contract compiles via forge → ABI extracted
       └── Rust (Cargo project): Folder picker → Select project
           └── Configure binary/interface/mode → Contract builds via cargo
   └── Contract Loading card shows real-time progress
   └── Contract added to sidebar on success

3. Deploy Instance
   └── Select "Deploy new instance"
       ├── Solidity contracts: Choose target (EVM or PVM)
       └── Rust contracts: PVM only (EVM not available)
   └── Enter constructor parameters → Transaction sent
   └── Instance appears in tree with all methods

4. Interact with Contract
   └── Expand deployed instance → View methods
   └── Select method → Enter parameters (if any) → Execute
   └── View results in output panel (Transaction/Call cards)

5. Debug & Inspect
   └── Select transaction card → View receipt, debug trace
   └── Copy transaction hash or contract address
```

## Architecture

### Layout Structure

```
┌─ Contracts ──────────┬─ Output ─────────────────────────────────────┐
│ Sidebar              │ Output Panel                                 │
│ (25-30% width)       │ (70-75% width)                              │
│                      │                                              │
│ - Contract tree      │ - Card-based output                         │
│ - Methods list       │ - Transaction/Call/Log cards                │
│ - Deploy actions     │ - Interactive actions (r, d, c)             │
├──────────────────────┴──────────────────────────────────────────────┤
│ Status Bar (1 line) - Connection status, chain, account, balance   │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Components

| Component | Purpose | Spec File |
|-----------|---------|-----------|
| Main Interface | Overall layout and zones | [main-interface.md](./main-interface.md) |
| Contracts Menu | Sidebar tree navigation | [contracts-menu.md](./contracts-menu.md) |
| Output Panel | Card-based results display | [output-panel.md](./output-panel.md) |
| Parameter Popup | Input forms for method calls | [tx-and-call-popup.md](./tx-and-call-popup.md) |
| Command Palette | Quick actions via Ctrl+P | [ctrl-p-menu.md](./ctrl-p-menu.md) |
| Navigation | Keyboard controls | [ui-navigation.md](./ui-navigation.md) |
| Settings | Configuration and shortcuts | [general-settings.md](./general-settings.md) |

## Design System

### Colors

| Purpose | Color | Usage |
|---------|-------|-------|
| Selected/Active | Cyan | Highlighted items, borders, focus indicators |
| Success | Green | Successful transactions, connected status |
| Error | Red | Failed transactions, validation errors |
| Warning/Loading | Yellow | Pending states, disconnected status |
| Muted/Inactive | Dark Gray | Unselected items, separators |
| Info | Blue | Informational messages |

### Visual Indicators

| Indicator | Meaning |
|-----------|---------|
| `+` | Add action (Load new contract) |
| `▾` / `▸` | Expanded / Collapsed node |
| `◇` | Action item (Deploy, Load existing) |
| `├` / `└` | Tree branch connectors |
| `┃` | Card left border |
| `●` / `○` | Connected / Disconnected |
| `✓` / `✗` | Success / Error |
| `⟳` | Loading/Pending |

### Keyboard Patterns

| Key | Global Behavior |
|-----|-----------------|
| `Tab` | Switch focus between sidebar and output panel |
| `Ctrl+P` | Open command palette |
| `Ctrl+C` | Exit application |
| `j` / `k` | Navigate down / up (vim-style) |
| `h` / `l` | Collapse / Expand (vim-style) |
| `Enter` | Execute action |
| `Escape` | Close popup / Cancel |

### Typography

- Monospace font throughout
- Address truncation: `0xf39F...2266` (first 6 + last 4 chars)
- Number formatting with thousand separators (e.g., `43,210`)

## Configuration

**Location**: `~/.evm-cli/config.json`

```json
{
  "config": {
    "rpc_url": "http://localhost:8545",
    "address": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
    "private_key": "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"
  },
  "deployments": {
    "/path/to/Contract.sol:ContractName": ["0x..."]
  }
}
```

## Dependencies

- **Foundry**: Required for `forge build` (Solidity compilation)
- **Local RPC**: Anvil, Hardhat, or any EVM-compatible node

## Code Quality Requirements

All code changes must pass the following checks with zero warnings:

```bash
cargo build    # Must produce no warnings
cargo clippy   # Must produce no warnings
cargo test     # All tests must pass
```

Use `#[allow(dead_code)]` sparingly and only for intentionally unused API surface that will be used in future iterations. Prefer removing truly unused code over suppressing warnings.

## Spec Index

| Spec | Description | Priority |
|------|-------------|----------|
| [main-interface.md](./main-interface.md) | Layout, zones, status bar, responsiveness | Core |
| [contracts-menu.md](./contracts-menu.md) | Sidebar tree, contract/instance management | Core |
| [output-panel.md](./output-panel.md) | Card types, actions, scrolling | Core |
| [ui-navigation.md](./ui-navigation.md) | All keyboard controls and navigation | Core |
| [tx-and-call-popup.md](./tx-and-call-popup.md) | Parameter input, validation | Core |
| [ctrl-p-menu.md](./ctrl-p-menu.md) | Command palette | Secondary |
| [general-settings.md](./general-settings.md) | Config, error handling, reference | Reference |

## Version

Current: v0.1.0

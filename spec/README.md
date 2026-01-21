# evm-cli Product Requirements Document

## Overview

`evm-cli` is an interactive terminal UI (TUI) for deploying and interacting with Solidity smart contracts on EVM-compatible blockchains. It provides a streamlined developer experience with a sidebar-based layout, automatic contract expansion, event log decoding, and session persistence.

---

## Features

The evm-cli application is organized into the following key feature areas:

### 1. [Main Interface](./main-interface.md)
The overall layout and visual design of the TUI, including the sidebar, output area, and status bar that form the core structure of the application.

### 2. [Contracts Menu](./contracts-menu.md)
The persistent sidebar navigation tree for managing loaded contracts, deployed instances, and method selection with keyboard-driven interaction.

### 3. [Ctrl+P Menu (Command Palette)](./ctrl-p-menu.md)
A centered overlay menu for accessing application-wide commands and configuration options via keyboard search and selection.

### 4. [Transaction & Call Popup](./tx-and-call-popup.md)
Parameter input popups for contract methods and constructors, enabling users to input arguments before execution.

### 5. [Output Panel](./output-panel.md)
The interactive card-based output display system for viewing results, transaction history, decoded logs, and accessing debugging tools.

---

## Core Concepts

### Auto-Expansion Behavior
- Contracts expand automatically when selected/loaded
- Deployed instances expand automatically after deployment
- Methods are immediately visible and ready to call

### Event Log Decoding
Event logs are automatically decoded using the contract ABI:
- Event name displayed with contract address
- All parameters shown with names and decoded values
- Falls back to raw topics/data if decoding fails
- Works for all known deployed instances

### Session Persistence
- All deployed contract addresses saved per contract
- Persists to `.evm-cli/config.json`
- Survives application restarts
- Can be cleared via Command Palette → "Clear State"

---

## General Features

### [Configuration](./general-settings.md#configuration)
User settings stored in `.evm-cli/config.json` with support for environment variable overrides.

### [Keyboard Shortcuts](./general-settings.md#keyboard-shortcuts)
Complete list of keyboard bindings organized by context (Global, Sidebar, Command Palette, etc.).

### [Error Handling](./general-settings.md#error-handling)
Transaction, compilation, and input validation error handling strategies.

### [Supported Parameter Types](./general-settings.md#supported-parameter-types)
Complete listing of Solidity types and their input formats.

---

## File Structure

```
evm-cli/
├── spec/
│   ├── README.md                    # This document
│   ├── main-interface.md            # Layout, visual design
│   ├── contracts-menu.md            # Sidebar navigation
│   ├── ctrl-p-menu.md               # Command palette
│   ├── tx-and-call-popup.md         # Parameter input
│   ├── output-panel.md              # Output display & cards
│   └── general-settings.md          # Config, shortcuts, errors
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
└── README.md
```

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

## Version History

- **v0.2.0** - Ratatui TUI with sidebar layout, auto-expansion, event decoding, output cards
- **v0.1.0** - Initial release with inquire-based prompts

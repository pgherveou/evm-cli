# General Settings & Reference

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

### Configuration Fields

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `rpc_url` | string | Ethereum RPC endpoint URL | `http://localhost:8545` |
| `address` | string | Active account address (0x + 40 hex chars) | `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac` |
| `private_key` | string | Private key for signing transactions | `5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133` |
| `deployments` | object | Map of contract paths to deployed addresses (cleared by "Clear State" command) | `{}` |

### Deployment Storage

- **Contract paths:** Canonicalized as absolute paths
- **Multiple instances:** Each contract can have multiple deployed addresses
- **Address format:** Hex strings with `0x` prefix
- **Clearing:** Removed when "Clear State" command is executed

### Default Values

If no config file exists, defaults are used:
- `rpc_url`: `http://localhost:8545`
- `address`: `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`
- `private_key`: `5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133`

### Environment Variables

Override config file with `.env` or system environment variables:

```bash
PRIVATE_KEY=your_private_key_here
ETH_RPC_URL=http://localhost:8545
ETH_ACCOUNT=0xyour_account_here
```

**Priority:** Environment variables > Config file > Defaults

### Config File Creation

The `.evm-cli/config.json` file is created automatically on first application run with default values:

```json
{
  "config": {
    "rpc_url": "http://localhost:8545",
    "address": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
    "private_key": "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"
  },
  "deployments": {}
}
```

Users can then modify this file manually or override with environment variables.

### Clear State Behavior

The "Clear State" command:
- **Clears:** Only the `deployments` object
- **Preserves:** RPC URL, account address, private key
- **Effect:** All deployed instance addresses removed from config.json
- **UI:** All instances removed from sidebar tree
- **Irreversible:** No undo mechanism (instances can be manually re-loaded)

---

## Keyboard Shortcuts

### Global Shortcuts

| Key | Action | Context |
|-----|--------|---------|
| `Ctrl+P` | Open Command Palette | Anywhere |
| `Ctrl+C` | Exit application | Anywhere |
| `Ctrl+L` | Clear all output cards | Output panel |
| `Tab` | Switch focus (sidebar ↔ output) | Anywhere |
| `Escape` | Close overlay / Cancel operation | Modals/Menus |

### Contract Sidebar

| Key | Action | Effect |
|-----|--------|--------|
| `↑` or `k` | Navigate up | Move to previous item |
| `↓` or `j` | Navigate down | Move to next item |
| `←` or `h` | Collapse | Collapse expanded node |
| `→` or `l` | Expand | Expand collapsed node |
| `Enter` | Select | Execute item action or expand |
| `Delete` or `Backspace` | Remove | Delete deployment or contract |

### Command Palette

| Key | Action | Effect |
|-----|--------|--------|
| `↑` or `k` | Move up | Navigate to previous command |
| `↓` or `j` | Move down | Navigate to next command |
| `Enter` | Execute | Run selected command |
| `Escape` | Close | Exit command palette |
| Type | Search | Filter commands in real-time |
| `Backspace` | Delete | Remove search character |
| `Ctrl+U` | Clear search | Clear all search text |

### Parameter Input Popup

| Key | Action | Effect |
|-----|--------|--------|
| `Tab` | Next field | Move to next input field |
| `Shift+Tab` | Previous field | Move to previous input field |
| `Enter` | Submit | Confirm and execute |
| `Escape` | Cancel | Close without submitting |
| Type | Input | Enter parameter value |
| `Backspace` | Delete | Remove character |

### Output Panel (Card View)

| Key | Action | Effect |
|-----|--------|--------|
| `↑` or `k` | Previous card | Navigate to previous output card |
| `↓` or `j` | Next card | Navigate to next output card |
| `Enter` or `Space` | Open menu | Show card action menu in footer |
| `Escape` | Close menu | Dismiss footer menu, keep card selected |
| `c` | Copy | Copy card content |
| `Home` or `Ctrl+Home` | First card | Jump to first card |
| `End` or `Ctrl+End` | Last card | Jump to last card |
| `Page Up` | Scroll up | Scroll output up |
| `Page Down` | Scroll down | Scroll output down |

### Debug Menu (Inside Card)

| Key | Action | Effect |
|-----|--------|--------|
| `↑` or `k` | Previous option | Navigate up in menu |
| `↓` or `j` | Next option | Navigate down in menu |
| `Enter` | Select | Execute selected option |
| `Escape` | Cancel | Close menu without action |

---

## Error Handling

### Transaction Errors

**Revert Errors:**
- Revert reason displayed when available
- Format: `Revert: <reason>`
- Example: `Revert: Insufficient balance`

**Gas Errors:**
- Gas estimation failures shown before sending
- Format: `Gas Error: <details>`
- Example: `Gas Error: Transaction would always revert`

**Network Errors:**
- Connection issues reported in status bar
- Format: `Error: <message>`
- Example: `Error: Connection to RPC failed`
- Suggested action: Check config, retry, or change endpoint

### Compilation Errors

- Solc compiler errors displayed in output panel as log card
- Error details shown with line numbers
- Contract not loaded if compilation fails
- User prompted to fix code and retry

**Format:**
```
✗ Compilation failed
  Counter.sol:5:5: Error: Invalid syntax
    error here ↑
```

### Input Validation Errors

**Address Format:**
- Must be `0x` + 40 hexadecimal characters
- Checksum validation (optional ERC-55 compliance)
- Error: `Invalid address: must be 0x + 40 hex characters`

**Numeric Types:**
- Must be valid decimal number
- Range validation per type (uint8: 0-255, uint256: 0-2^256-1, etc.)
- Error: `Invalid uint256: value too large`

**Type Mismatches:**
- Caught before transaction submission
- Error: `Type mismatch: expected address, got string`

**Array Validation:**
- Each element validated per type
- Error: `Invalid array element at index 2: <error>`

### User Confirmation

For potentially risky operations:

1. **Deployments:**
   - Show constructor args before sending
   - Display gas estimate
   - Ask for confirmation

2. **State-changing calls:**
   - Show gas estimate
   - Display transaction details
   - Ask for confirmation

3. **Clear State:**
   - Confirm intent: "Clear all deployments? (y/n)"
   - Show what will be cleared

---

## Supported Parameter Types

### Primitive Types

| Type | Input Format | Range/Constraint | Example |
|------|--------------|------------------|---------|
| `address` | `0x` + 40 hex | Valid Ethereum address | `0x742d35Cc6634C0532925a3b844Bc9e7595f7a` |
| `bool` | `true` / `false` | Boolean value | `true` |
| `uint256` | Decimal number | 0 to 2^256 - 1 | `1000000000000000000` |
| `int256` | Decimal number | -(2^255) to 2^255 - 1 | `-5000000000000000000` |
| `bytes` | Hex string with `0x` | Even length hex | `0xdeadbeef` |
| `bytes32` | Hex string with `0x` | Exactly 32 bytes | `0xabcd...ef01` |
| `string` | Free text | No length restriction | `Hello, EVM!` |
| `uint8` | Decimal number | 0 to 255 | `42` |
| `uint16` | Decimal number | 0 to 65,535 | `1000` |
| `uint64` | Decimal number | 0 to 2^64 - 1 | `9000000000000000000` |
| `int8` | Decimal number | -128 to 127 | `-42` |
| `int16` | Decimal number | -32,768 to 32,767 | `-1000` |

### Complex Types

| Type | Input Format | Example |
|------|--------------|---------|
| `address[]` | Comma or newline separated | `0xaaa...,0xbbb...,0xccc...` |
| `uint256[]` | Comma or newline separated | `1,2,3,100` |
| `bytes[]` | Comma or newline separated | `0xaa,0xbb,0xcc` |
| `string[]` | Comma or newline separated | `apple,banana,cherry` |
| `(address,uint256)` | Nested field input | Multi-field popup |
| `struct` / Tuple | Nested field input | Field-by-field form |

### Special Handling

**Arrays:**
- Delimiter: Comma or newline
- Optional "Add item" button for UI
- Each element validated per type

**Structs/Tuples:**
- Displayed as nested form
- Field names shown with types
- Same validation rules per field

---

## Dependencies

### Core Dependencies

| Package | Purpose | Version Constraint |
|---------|---------|-------------------|
| `alloy` | Ethereum library (RPC, signing, ABI) | Latest |
| `ratatui` | Terminal UI framework | Latest |
| `crossterm` | Terminal manipulation | Latest |
| `tokio` | Async runtime | 1.x |

### Utility Dependencies

| Package | Purpose | Version Constraint |
|---------|---------|-------------------|
| `serde` | Serialization framework | 1.x |
| `serde_json` | JSON serialization | 1.x |
| `anyhow` | Error handling | 1.x |
| `inquire` | Interactive prompts (file picker) | Latest |
| `strum` | Enum utilities | Latest |
| `tracing` | Logging and diagnostics | Latest |

### External Dependencies

| Tool | Purpose | Installation |
|------|---------|--------------|
| `solc` | Solidity compiler | Required in `$PATH` |
| `anvil` | Local EVM blockchain | Optional (for testing) |
| `hardhat` | Blockchain development | Optional (for testing) |

### Optional Tools

- **Editor integration:** Respects `$EDITOR` environment variable
- **Git:** Optional for version control of contracts

---

## Version History

### v0.2.0
- Ratatui TUI with sidebar layout
- Auto-expansion behavior
- Event log decoding
- **Output card feature:** Card-based navigation, transaction debugging, trace support
- Interactive output panel with menu-driven actions

### v0.1.0
- Initial release
- Inquire-based prompts
- Basic contract compilation
- Simple method execution

---

## Future Considerations

### Planned Features
- [ ] Multi-chain support with chain selector
- [ ] Transaction history export to CSV
- [ ] Contract verification integration
- [ ] Custom event filtering
- [ ] Batch transaction support
- [ ] Contract watch mode (auto-reload on file changes)
- [ ] Card search/filter capability

### Nice to Have
- [ ] ENS name resolution
- [ ] ABI auto-fetching from Etherscan
- [ ] Gas price recommendations
- [ ] Transaction simulation/preview
- [ ] Contract favorites/bookmarks
- [ ] Theme customization
- [ ] Multi-account support with switching
- [ ] Persistent card history

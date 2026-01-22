# General Settings & Reference

## Configuration

### Config File Location

`~/.evm-cli/config.json` (in user's home directory)

### Schema

```json
{
  "config": {
    "rpc_url": "http://localhost:8545",
    "address": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
    "private_key": "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133"
  },
  "deployments": {
    "/home/user/project/Demo.sol:Demo": [
      "0x3469e1dac06611030aece8209f07501e9a7acc69"
    ],
    "/home/user/project/Token.sol:MyToken": [
      "0x1234567890abcdef1234567890abcdef12345678",
      "0xabcdef1234567890abcdef1234567890abcdef12"
    ],
    "/home/user/project/Empty.sol:EmptyContract": []
  }
}
```

**Note:** 
- `Demo` has one deployed instance
- `MyToken` has two deployed instances
- `EmptyContract` is loaded but not deployed yet (empty array)

### Configuration Fields

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `config.rpc_url` | string | Ethereum RPC endpoint URL | `http://localhost:8545` |
| `config.address` | string | Account address derived from private key (0x + 40 hex chars) | `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac` |
| `config.private_key` | string | Private key for signing transactions (64 hex chars, optional 0x prefix) | `5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133` |
| `deployments` | object | Map of contract IDs to deployed addresses (format: "path:name" → addresses[]) | `{}` |

### Deployment Storage

- **Key format:** `"/absolute/path/to/Contract.sol:ContractName"` 
  - Combines absolute file path with contract name using colon separator
  - Example: `"/home/user/project/Demo.sol:Demo"`
- **Multiple instances:** Each contract can have multiple deployed addresses (array of addresses)
- **Address format:** Hex strings with `0x` prefix
- **Persistence:** Contract entries preserved even when all deployments removed (empty array)
- **Clearing:** Removed when "Clear State" command is executed

### Default Values

If no config file exists, defaults are used:
- `rpc_url`: `http://localhost:8545`
- `address`: `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`
- `private_key`: `5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133`

**Note:** These are well-known test credentials used by local Ethereum development nodes (e.g., Anvil, Hardhat). They are publicly known and have no real value. NEVER use these credentials in production.

### Config File Creation

The `~/.evm-cli/config.json` file is created automatically on first application run with default values:

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

Users can modify this file manually or through the "Edit config" command (`Ctrl+P` > `Edit config`).

### Config Reload Behavior

When config is edited via the "Edit config" command:
- Opens config file in user's `$EDITOR` (or `vim` if not set)
- After saving and closing the editor, the config is automatically reloaded
- Account address is updated if private key changes
- Connection status is refreshed with new credentials
- **Note:** RPC URL changes require application restart to take effect

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
| `r` | View Receipt | When Transaction card selected |
| `d` | Debug Trace / Debug Call | When Transaction or Call card selected |

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
| `c` | Copy | Copy card content |
| `r` | View Receipt | Show receipt for Transaction cards |
| `d` | Debug | Debug Trace for Transaction cards, Debug Call for Call cards |
| `Home` or `Ctrl+Home` | First card | Jump to first card |
| `End` or `Ctrl+End` | Last card | Jump to last card |
| `Page Up` | Scroll up | Scroll output up |
| `Page Down` | Scroll down | Scroll output down |

### Tracer Config Popup

| Key | Action | Effect |
|-----|--------|--------|
| `↑` or `Tab` | Previous field | Navigate to previous toggle |
| `↓` or `Shift+Tab` | Next field | Navigate to next toggle |
| `Space` | Toggle | Toggle the selected option ON/OFF |
| `Enter` | Execute | Run debug trace with current settings |
| `Escape` | Cancel | Close popup without action |

---

## Connection Behavior

### Graceful Disconnected Mode

The application launches even when the RPC server is unreachable:

- **Startup:** App starts without blocking on RPC connection
- **Connection Card:** Displays connection status, account, balance, chain ID
- **Background Polling:** When disconnected, retries connection every 5 seconds
- **Auto-Recovery:** On successful reconnection, updates UI automatically

### Disconnected State Restrictions

When disconnected, certain operations are blocked:

| Operation | Behavior When Disconnected |
|-----------|---------------------------|
| Deploy contract | Blocked with error message in Log card |
| State-changing calls | Blocked with error message in Log card |
| View-only calls | Blocked (requires RPC) |
| Browse sidebar | Allowed |
| Navigate cards | Allowed |
| Edit config | Allowed |

### Status Bar Display

Format: `● Connected | Chain: <id> | Account: <address> | Balance: <amount> ETH`

- **Connected:** Green dot with "Connected"
- **Disconnected:** Yellow dot with "Disconnected"
- **Account:** Address display is responsive to terminal width:
  - **Wide terminals (>120 chars):** Full 42-character address
  - **Medium terminals (80-120 chars):** Truncated format `0xf24f...2266` (first 6 + last 4 chars)
  - **Narrow terminals (<80 chars):** Ultra-compact `0xf2...66` (first 4 + last 2 chars)
- **Balance:** Format adapts to available space:
  - **Wide terminals:** Full precision `999.999984 ETH`
  - **Medium terminals:** 3 decimal places `999.999 ETH`
  - **Narrow terminals:** Integer only `999 ETH` or compact notation `1K ETH` for large values
- **Updates:** Balance refreshed after each transaction

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

- **Editor integration:** Respects `$EDITOR` or `$VISUAL` environment variable (defaults to `vim`)
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

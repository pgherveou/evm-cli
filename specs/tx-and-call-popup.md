# Transaction & Call Popup Specification

## Overview

Parameter input popups collect user arguments for contract methods, constructors, and other operations. These centered modal overlays provide sequential field input with type hints and validation.

---

## Parameter Input Popup

### Mockup

```
                    ┌─────────────────────────────────────────────┐
                    │ transfer(address,uint256)              esc  │
                    │                                             │
                    │ to (address):                               │
                    │ 0x742d35Cc6634C0532925a3b844Bc9e7595f█      │
                    │                                             │
                    │ amount (uint256):                           │
                    │                                             │
                    │                                             │
                    │ Press return to confirm, tab to next field  │
                    └─────────────────────────────────────────────┘
```

### Layout

- **Title:** Method/constructor signature (e.g., `transfer(address,uint256)`)
- **Close hint:** `esc` indicator in top-right
- **Fields:** Vertical stack of input fields
- **Footer:** Helper text for navigation
- **Width:** ~60-80% of terminal width (centered)
- **Height:** Dynamic based on number of fields

### Components

#### Title Bar
- Method signature with full parameter types
- Right-aligned close indicator (`esc`)
- Example: `transfer(address,uint256)`

#### Input Field

Each field includes:
- **Label:** Parameter name with type hint
- **Input area:** Text input with cursor
- **Placeholder:** Optional example format
- **Status:** Optional validation state

**Example Field:**
```
to (address):
0x742d35Cc6634C0532925a3b844Bc9e7595f█

amount (uint256):
█
```

#### Footer
- Navigation instructions: "Press return to confirm, tab to next field"
- Alternative shortcut hints (if applicable)

---

## Supported Parameter Types

### Primitive Types

| Type | Input Format | Example | Validation |
|------|--------------|---------|-----------|
| `address` | `0x` + 40 hex chars | `0x742d35Cc6634C0532925a3b844Bc9e7595f` | Must be valid hex, 40 chars |
| `bool` | `true` or `false` | `true` | Case-insensitive dropdown |
| `uint256` | Numeric (decimal) | `1000000000000000000` | Non-negative, within uint range |
| `int256` | Numeric (decimal, can be negative) | `-5000000000000000000` | Within int range |
| `bytes` | Hex string (0x prefix) | `0xdeadbeef` | Must be valid hex, even length |
| `bytes32` | Hex string (0x prefix, 32 bytes) | `0xabcd...ef01` | Exactly 32 bytes when decoded |
| `string` | Free text | `Hello, EVM!` | No length restrictions |
| `uint*` / `int*` | Numeric (bit-specific) | `42` (for uint8) | Type-specific range validation |

### Complex Types

| Type | Input Format | Example |
|------|--------------|---------|
| `address[]` | Comma-separated in single field | `0xaaa...,0xbbb...,0xccc...` |
| `uint256[]` | Comma-separated in single field | `1,2,3,100` |
| `bytes[]` | Comma-separated in single field | `0xaa,0xbb,0xcc` |
| `tuple` / `struct` | Dot notation fields | `recipient.address`, `recipient.amount` as separate fields |
| `(address,uint256)` | Dot notation fields | For parameter named `recipient`, creates `recipient.address` and `recipient.amount` fields |

### Tuple/Struct Input Example

For a function `transfer((address,uint256) recipient)`:

```
┌─────────────────────────────────────────────┐
│ transfer((address,uint256))           esc  │
│                                             │
│ recipient.address (address):                │
│ 0x742d35Cc6634C0532925a3b844Bc9e7595f█      │
│                                             │
│ recipient.amount (uint256):                 │
│ 1000█                                       │
│                                             │
│ Press return to confirm, tab to next field  │
└─────────────────────────────────────────────┘
```

---

## Keyboard Controls

For detailed input controls and field navigation, see [UI Navigation & Keyboard Controls](./ui-navigation.md#parameter-input-popups).

### Quick Reference
- **Navigate Fields:** `Tab` (next), `Shift+Tab` (previous)
- **Submit:** `Enter` (validates all fields first)
- **Cancel:** `Escape`
- **Boolean Fields:** `↑`/`↓` to toggle true/false
- **Constructor Target:** `←`/`→` to switch EVM/PVM
- **Input:** Type to enter, `Backspace` to delete

---

## Validation

### Input Validation

Validation occurs on each field as user types or after field completion:

#### Address Validation
- Format check: `0x` followed by 40 hexadecimal characters
- Checksum validation (optional ERC-55 compliance)
- User feedback: Show valid/invalid indicator

#### Numeric Validation
- Type check: Decimal number only (no hex for decimal fields)
- Range check: Within type-specific bounds
- Non-negative check (for uint types)
- User feedback: Show valid/invalid indicator

#### Bytes Validation
- Format check: `0x` followed by even number of hex chars
- Length check (for fixed-size types like `bytes32`)
- User feedback: Show valid/invalid indicator

#### String Validation
- No strict validation (accept any text)
- Optional: Warn if string exceeds certain length

### Validation Feedback

Validation occurs in real-time as user types. Invalid fields show an inline error message below the input:

**Example - Invalid Address:**
```
to (address):
0x742d35Cc_INVALID_CHARACTER
✗ Invalid address format: must be 0x followed by 40 hex characters
```

**Example - Out of Range:**
```
amount (uint256):
-500
✗ Invalid uint256: value must be non-negative
```

**Validation Rules:**
- Errors appear immediately after invalid input
- Red ✗ icon precedes error message
- Error message describes what's wrong and how to fix it
- Field cannot be submitted while invalid

### Form Submission

**Validation on Submit:**
- When user presses `Enter`, all fields are validated
- If ANY field is invalid:
  - Form does NOT submit
  - All invalid fields show error messages
  - Cursor moves to first invalid field
  - User must fix errors before resubmitting

**Success Flow:**
- All fields valid → Form submits immediately
- Popup closes
- Transaction/call is executed
- Result appears as new card in output panel

**Cancellation:**
- Press `Escape` at any time to cancel
- No data is submitted
- Popup closes, returns to previous view

---

## Array Input

### Dynamic Arrays

For types like `address[]` or `uint256[]`:

**Input Format:**
```
addresses (address[]):
0xaaa...,0xbbb...,0xccc...

Or enter one item per line:
0xaaa...
0xbbb...
0xccc...
```

**Add Item Interaction:**
- "Add more" button/option to add additional items
- Each item validated individually
- Remove items with indicator

### Struct/Tuple Input

For complex types like `(address,uint256)` or custom structs:

**Mockup:**
```
┌──────────────────────────────────┐
│ swapParams((address,uint256)) esc│
│                                  │
│ recipient (address):             │
│ 0x742d35Cc6634C0532925a3b844█    │
│                                  │
│ amount (uint256):                │
│ 1000█                            │
│                                  │
│ Press return to confirm           │
└──────────────────────────────────┘
```

- Nested fields displayed in order
- Field labels include component name and type
- Same validation rules applied per field

---

## Constructor Parameters

When deploying a new contract, constructor parameters are collected similarly:

### Mockup

```
                     ┌─────────────────────────────────────────────┐
                     │ constructor - Enter Parameters         esc  │
                     │                                             │
                     │ Target:  EVM   PVM   (←/→ to switch)        │
                     │                                             │
                     │ _initial (uint256):                         │
                     │ 42█                                         │
                     │                                             │
                     │ _owner (address):                           │
                     │ 0x742d35Cc6634C0532925a3b844Bc9e7595f█      │
                     │                                             │
                     │ ←/→: target  Tab: next  Shift+Tab: prev     │
                     │ Enter: submit  Esc: cancel                  │
                     └─────────────────────────────────────────────┘
```

- Title shows "constructor - Enter Parameters"
- Includes deployment target selection (EVM/PVM) at the top of the dialog
- Same field layout and validation as method parameters
- Footer shows navigation instructions: `←/→: target  Tab: next  Shift+Tab: prev  Enter: submit  Esc: cancel`
- Submission: User selects target → enters parameters → confirm → proceed to deployment

---

## Deployment Target Selection

For deployments, the bytecode target is selected within the constructor parameter dialog:

### Integration

The deployment target selection is integrated at the top of the constructor parameter dialog as shown:
```
Target:  EVM   PVM   (←/→ to switch)
```

### Options

1. **EVM** (Default)
   - Standard Ethereum Virtual Machine bytecode
   - Supported on all EVM-compatible chains
   - Selected by default

2. **PVM** (PolkaVM)
   - PolkaVM bytecode (requires polkavm-enabled solc)
   - Special compilation mode

### Navigation

| Key | Action |
|-----|--------|
| `←` / `→` | Switch between EVM and PVM targets |
| `Tab` | Move to first parameter field |

---

## ETH Value Input

For payable methods, an optional ETH value field appears:

### Mockup

```
                    ┌─────────────────────────────────────────────┐
                    │ transfer(address,uint256)              esc  │
                    │                                             │
                    │ to (address):                               │
                    │ 0x742d35Cc6634C0532925a3b844Bc9e7595f█      │
                    │                                             │
                    │ amount (uint256):                           │
                    │ 1000█                                       │
                    │                                             │
                    │ ETH Value (optional):                       │
                    │ 0.5█                                        │
                    │                                             │
                    │ Press return to confirm, tab to next field  │
                    └─────────────────────────────────────────────┘
```

- **Label:** "ETH Value (optional)"
- **Input format:** Decimal number (e.g., `0.5`, `1`, `2.5`)
- **Validation:** Non-negative decimal number
- **Conversion:** Input converted to wei (multiplied by 10^18)
- **Default:** 0 if not specified

---

## Error States

### Field Validation Error

**Visual:**
```
to (address):
0x123  ✗ Invalid address format

amount (uint256):
abc    ✗ Expected numeric value
```

- Field marked with `✗` or colored red
- Error message shown below field
- Submit button disabled

### Network Error

If parameter input requires network call (e.g., loading contract info):

**Visual:**
```
Loading contract ABI...
```

- Loading indicator shown
- Fields disabled during fetch
- Timeout handling with retry option

---

## Accessibility

- All fields accessible via keyboard
- Clear labels for each input
- Type hints provided
- Error messages descriptive and actionable
- Validation feedback immediate
- Tab order follows logical flow

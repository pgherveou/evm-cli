# Typography Guidelines

This document defines typography standards for the EVM CLI, including font choices, sizing, and hierarchy.

---

## Font Family

### Primary Font
**Font**: Monospace (terminal default)
**Examples**: Monaco, Menlo, Consolas, Courier New, DejaVu Sans Mono
**Usage**: All text in application
**Rationale**: Terminal requires monospace for alignment and proper character spacing

### Characteristics
- Fixed-width: Every character occupies same horizontal space
- Clear distinction: Characters like `0`, `O`, `1`, `l`, `I` easily differentiated
- Terminal native: No custom font loading required
- Readable at terminal sizes: Typically 8-12pt in terminal settings

---

## Text Hierarchy

### Primary Title (H1)
**Usage**: Main interface titles, modal titles
**Style**: Bold (if supported), uppercase emphasis
**Examples**:
- `# Transaction & Call Popup Specification`
- Modal title: `transfer(address,uint256)`
**Line height**: Compact (1.0-1.2)

### Section Title (H2)
**Usage**: Major section headings
**Style**: Normal weight, clear emphasis through positioning
**Examples**:
- `## Supported Parameter Types`
- `## Keyboard Controls`
**Line height**: Normal (1.2)

### Subsection Title (H3)
**Usage**: Secondary section headings, groupings
**Style**: Normal weight, indentation/position shows hierarchy
**Examples**:
- `### Navigation`
- `### Success State`
**Line height**: Normal (1.2)

### Component Label (H4)
**Usage**: Field labels, component names
**Style**: Normal weight, displayed inline with content
**Examples**:
- `to (address):`
- `recipient.address (address):`
**Line height**: Compact (1.0)

### Body Text
**Usage**: Descriptions, instructions, content
**Style**: Regular weight
**Line height**: Relaxed (1.2-1.4) for readability
**Examples**: Status messages, error text, helper text

### Secondary Text
**Usage**: Less important information, helpers, metadata
**Style**: Regular weight, gray/dimmed color
**Line height**: Normal (1.0-1.2)
**Examples**: Shortcut hints, context info, timestamps

---

## Text Formatting for Terminal

Since terminal doesn't support italic/underline well:

### Emphasis Techniques

#### Bold (if supported)
**Syntax**: Bold text in output
**Usage**: Important information, success status
**Example**: `Status: **Success**`

#### Brackets for Hints
**Syntax**: `[text]`
**Usage**: Keyboard shortcuts, hints
**Examples**:
- `[Escape to close]`
- `[Press return to confirm, tab to next field]`

#### Symbols for Visual Markers
**Syntax**: Unicode symbols at line start
**Usage**: Status indicators, list markers
**Examples**:
- `✓ Success`
- `✗ Error`
- `→ Next item`
- `• List item`

#### UPPERCASE for Keys/Commands
**Syntax**: UPPERCASE text
**Usage**: Key names, command names
**Examples**:
- `Press ESCAPE to cancel`
- `CTRL+P to open command palette`
- `TAB to switch focus`

#### Monospace Code Snippets
**Syntax**: Backticks for inline code
**Usage**: Addresses, hashes, type names
**Examples**:
- `0x742d35Cc6634C0532925a3b844Bc9e7595f`
- `uint256`
- `.evm-cli/config.json`

#### Quotation for Exact Text
**Syntax**: Quote marks for user-entered or system text
**Usage**: Messages, values, configuration
**Examples**:
- Input value: `"0xABC..."`
- Config file: `".evm-cli/config.json"`
- Message: `"Terminal too small"`

---

## Line Length and Wrapping

### Optimal Width
- **Standard**: 80 characters (terminal standard)
- **Comfortable**: 80-120 characters
- **Maximum**: Terminal width without horizontal scroll

### Wrapping Strategy
- **Addresses/Hashes**: Truncate with ellipsis (`0xabc...`) if too long
- **Descriptions**: Wrap at word boundary
- **Code/JSON**: Preserve formatting, allow horizontal scroll if needed
- **Status messages**: Single line, truncate if necessary

### Line Length Examples
```
Short line (under 80 chars):
✓ Transaction successful

Medium line (80-100 chars):
Method: transfer(address recipient, uint256 amount)

Long content wraps:
This is a long error message that explains what went
wrong and how to fix it by providing clear guidance
```

---

## Typography in Components

### Sidebar Tree Labels
**Style**: Normal weight, default color
**Alignment**: Left aligned, hierarchical indentation
**Line spacing**: No blank lines between items in tree
**Group spacing**: 1 blank line between groups

### Card Headers
**Style**: Normal or bold, concise
**Content**: Card type + status
**Examples**:
- `Transaction: transfer(address,uint256)`
- `Call: balanceOf(address)`

### Card Content
**Style**: Regular monospace
**Content**: Key-value pairs or results
**Alignment**: Left aligned, indentation for hierarchy
**Spacing**: 1 line between logical sections

### Status Messages
**Style**: Symbol + color + text
**Format**: `[Symbol] Message text with context`
**Examples**:
- `✓ Deployed to 0x1234...`
- `✗ Invalid address format`
- `⟳ Fetching transaction...`

### Input Field Labels
**Style**: Normal weight, followed by colon and type
**Format**: `labelName (type):`
**Examples**:
- `to (address):`
- `amount (uint256):`
- `recipient.address (address):`

### Error Messages
**Style**: Red, with `✗` symbol, actionable description
**Format**: `✗ Error description explaining what's wrong and how to fix`
**Examples**:
- `✗ Invalid address format: must be 0x followed by 40 hex characters`
- `✗ Invalid uint256: value must be non-negative`

### Helper Text
**Style**: Gray/dimmed, small size
**Format**: Text describing behavior or shortcuts
**Examples**:
- `Press return to confirm, tab to next field`
- `[Escape to cancel]`
- `[c] Copy  [r] Receipt  [d] Debug`

---

## Special Text Elements

### Addresses (Shortened)
**Format**: `0x` + first part + `...` + last 8 characters
**Example**: `0x742d35Cc...e7595f` (from `0x742d35Cc6634C0532925a3b844Bc9e7595f`)
**Usage**: Sidebar instances, status bar, card headers
**Rationale**: Saves space while maintaining uniqueness

### Transaction Hashes (Shortened)
**Format**: `0x` + first part + `...` (variable length)
**Example**: `0x1a2b3c4d...9z8y7x6w`
**Usage**: Card content, transaction details
**Rationale**: Provides recognition without consuming full line width

### Method Signatures
**Format**: `methodName(type1,type2,...)`
**Example**: `transfer(address,uint256)`
**Usage**: Card headers, popup titles, sidebar methods
**Display**: Parameter types shown in signatures (unlike sidebar where only `...` used)

### Type Names
**Format**: Monospace, exact Solidity type
**Examples**: `address`, `uint256`, `bytes32`, `bool`, `address[]`, `(address,uint256)`
**Usage**: Field labels, documentation, error messages

### Status Indicators
**Format**: Word + symbol
**Examples**:
- `Success ✓`
- `Error ✗`
- `Pending ⟳`
- `Connected ●`

---

## Line Height and Spacing

### Single-Line Elements
**Line height**: 1.0 (no extra space)
**Examples**: Sidebar tree items, action menu items
**Spacing**: No blank line between items

### Multi-Line Content
**Line height**: 1.0 for each line
**Spacing**: 1 blank line between logical sections
**Rationale**: Clear grouping without wasting space

### Content with Labels
**Line height**: 1.0
**Label**: On same line or line before
**Examples**:
```
to (address):
0x742d35Cc6634C0532925a3b844Bc9e7595f█

Result: 1000
```

---

## Best Practices

1. **Consistency**: Use same format for same content type everywhere
2. **Clarity**: Clear hierarchy through position and formatting
3. **Scanability**: Ample whitespace between sections
4. **Accessibility**: Never rely on formatting alone (e.g., no italic for emphasis)
5. **Terminal compatibility**: Monospace only, no special fonts
6. **Abbreviation rules**: Keep abbreviated text meaningful
7. **Symbol usage**: Consistent symbols across application
8. **Case sensitivity**: Match exact casing for commands, types, addresses

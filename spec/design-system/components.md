# Component Catalog

This document outlines all UI components used throughout the EVM CLI application, their purpose, and usage guidelines.

## Layout Components

### Sidebar (Contracts Menu)
**Purpose**: Contract and instance navigation with hierarchical tree display
**Width**: ~25-30% of terminal width
**Content**: Contracts, deployed instances, and methods in collapsible tree structure
**Key feature**: Auto-expansion on load/deploy

### Output Panel
**Purpose**: Display results, transaction history, and interactive cards
**Width**: ~70-75% of terminal width
**Content**: Transaction cards, call cards, log cards, and debug information
**Key feature**: Card-based navigation with footer action menu

### Status Bar
**Purpose**: Display connection status, chain, account, and balance
**Height**: 1 line at bottom
**Content**: Connection status + context information
**Key feature**: Real-time updates during async operations

### Command Palette
**Purpose**: Quick access to application commands
**Style**: Centered overlay modal (~60-80% width)
**Content**: Searchable command list organized by category
**Key feature**: Real-time filtering with group organization

### Parameter Input Popup
**Purpose**: Collect user arguments for functions and constructors
**Style**: Centered overlay modal (~60-80% width)
**Content**: Form fields with labels, types, and validation feedback
**Key feature**: Real-time validation with inline error messages

---

## Interactive Components

### Tree Node
**Indicators**:
- `▾` = Expanded node
- `▸` = Collapsed node
- `+` = Action to load/add
- `◇` = Action item (deploy, load existing)

**Behavior**:
- `Enter` executes action or toggles expand/collapse
- `←`/`h` collapses, `→`/`l` expands
- Cyan background highlights selected node

### Input Field
**Anatomy**:
- Label with type hint: `fieldName (type):`
- Input area with visible cursor: `█`
- Optional placeholder text
- Real-time validation feedback below

**Validation Display**:
```
fieldName (type):
invalidValue  ✗ Error description
```

**Types Supported**:
- **Primitives**: address, bool, uint256, int256, bytes, bytes32, string
- **Fixed-size**: uint8-uint256, int8-int256, bytes1-bytes32
- **Arrays**: address[], uint256[], bytes[], etc. (comma or newline separated)
- **Tuples/Structs**: Nested fields with dot notation (e.g., `recipient.address`)
- **Special**: bool uses toggle with ↑/↓ instead of text input

### Output Card
**Structure**:
```
┌─ Card Type (State) ──────┐
│ Key: Value               │
│ Multiple rows of data    │
│ Additional information   │
│                          │
│ ◇ Action 1 (key)         │
│ ◇ Action 2 (key)         │
└──────────────────────────┘
```

**Card Types**:
1. **Transaction Card**: Deployment or state-changing operations
   - Header: Method name, address, state (Pending/Success/Failed)
   - Content: Gas estimate, transaction hash, receipt data
   - Actions: View Receipt (r), Debug Trace (d), View Logs (l)

2. **Call Card**: Read-only function calls
   - Header: Method name, address, result
   - Content: Return value(s), formatted output
   - Actions: Copy Result (c), View as JSON (j)

3. **Log Card**: Informational messages or compilation output
   - Header: Message type or status
   - Content: Details, errors, or warnings
   - No interactive actions

**Properties**:
- 1-line spacing between cards
- Selected card highlighted with border/positioning
- Auto-selected on creation
- Cyan selection highlight
- Scrollable within panel

### Footer Action Menu
**Display**:
```
◇ Copy Result (c)    ◇ View as JSON (j)    ◇ Copy Call Data (d)
```

**Behavior**:
- Appears when card is selected
- Diamond `◇` prefixes each action
- Shortcut key shown in parentheses
- Navigate with `←`/`→`, execute with `Enter` or shortcut
- Dismiss with `Escape` (card stays selected)
- Re-appears when selecting another card

### Status Indicator
**Display**:
- `●` (filled green) = Connected
- `○` (hollow) = Disconnected
- `⟳` (yellow) = Loading/Pending
- `✓` (green) = Success
- `✗` (red) = Error
- `ℹ` (blue) = Info

**Rule**: Always paired with status text; never use color alone

### Boolean Toggle
**Display**: Current value shown (`true` or `false`)
**Navigation**: `↑`/`↓` or `j`/`k` to toggle
**Confirmation**: `Enter` to move to next field
**Behavior**: No text input allowed

---

## Form Components

### Parameter Form
**Structure**:
- Title with method/constructor signature
- Close hint (`esc`) in top-right
- Vertical stack of input fields
- Footer with navigation instructions

**Navigation**:
- `Tab` = Next field
- `Shift+Tab` = Previous field
- `Enter` = Submit (if all fields valid)
- `Escape` = Cancel

**Validation**:
- Real-time as user types
- Error appears below field with `✗` icon
- Form cannot submit if any field invalid
- All invalid fields show errors on submit attempt
- Cursor moves to first invalid field on failed submit

---

## Feedback Components

### Loading Indicator
**Display**: `⟳ [Action description]`
**Context**: Status bar, card header, or modal message
**Color**: Yellow
**Pattern**: Shows during async operations (deployment, compilation, RPC calls)

### Success Feedback
**Display**: `✓ Status: Success` or `Status: Complete ✓`
**Color**: Green
**Location**: Card, status bar, or inline message
**Duration**: Persistent in card; temporary in status bar

### Error Feedback
**Display**: `✗ Error description with actionable guidance`
**Color**: Red
**Location**: Inline in form, card, or status bar
**Example**: `✗ Invalid address format: must be 0x followed by 40 hex characters`

### Validation Error
**Display**: Field label + invalid value + `✗` error message
**Location**: Below input field
**Timing**: Real-time as user types or on field blur
**Actionability**: Message explains what's wrong and how to fix

---

## Guidelines

### Component Consistency
- All components must follow [color system](./colors.md)
- Typography follows [typography guidelines](./typography.md)
- Spacing follows [spacing rules](./spacing.md)
- All patterns follow [interaction patterns](./patterns.md)

### Keyboard Accessibility
- All interactive elements keyboard accessible
- Consistent navigation keys across components
- Clear visual focus indicators (cyan background or border)
- Tab order follows logical flow

### Visual Feedback
- State changes immediately visible
- Color + symbol (never color alone)
- High contrast for readability (WCAG AA minimum)
- Loading states distinguish from complete states

### Component Reusability
- Components self-contained and modular
- Consistent behavior across application
- Documented interaction patterns
- Clear visual indicators for all states

---

## Component Reference by Spec

| Component | Spec | Usage |
|-----------|------|-------|
| Sidebar | contracts-menu | Contract/instance navigation |
| Output Panel | output-panel | Card display and interaction |
| Status Bar | main-interface | Connection and account info |
| Command Palette | ctrl-p-menu | Global command access |
| Parameter Form | tx-and-call-popup | Function/constructor parameters |
| Tree Node | contracts-menu | Hierarchical menu items |
| Card | output-panel | Result display container |
| Footer Menu | output-panel | Card action navigation |
| Input Field | tx-and-call-popup | Form field input |
| Toggle | tx-and-call-popup | Boolean parameter input |

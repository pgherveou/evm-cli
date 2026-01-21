# QA Recordings Manifest

This document tracks all VHS recordings created for evm-cli specifications.

## Recording Coverage Summary

| Spec | Recording Scripts | Status | Coverage |
|------|-------------------|--------|----------|
| **main-interface** | 2 | ✅ Complete | Layout/zones, Focus management |
| **contracts-menu** | 2 | ✅ Complete | Tree navigation, Load contract |
| **ctrl-p-menu** | 2 | ✅ Complete | Palette basics, Command execution |
| **tx-and-call-popup** | 2 | ✅ Complete | Parameter input, Validation feedback |
| **output-panel** | 2 | ✅ Complete | Card navigation, Card types |
| **general-settings** | 2 | ✅ Complete | Configuration, Keyboard shortcuts |
| **TOTAL** | **12** | ✅ **Complete** | All specs covered |

---

## Main Interface Recordings

### `spec/main-interface/recordings/layout-and-zones.tape`
**Purpose**: Demonstrates the three-zone layout (sidebar, output, status bar)
**Features Covered**:
- Three-zone layout visibility
- Sidebar with contract tree
- Output area
- Status bar showing connection, chain, account, balance
- Basic navigation to verify layout responsiveness

### `spec/main-interface/recordings/focus-management.tape`
**Purpose**: Demonstrates Tab key focus switching between sidebar and output
**Features Covered**:
- Tab to switch focus sidebar → output
- Tab to switch focus output → sidebar
- Navigation with focus on sidebar (j/k)
- Navigation with focus on output area (j/k)
- Focus state persistence

---

## Contracts Menu Recordings

### `spec/contracts-menu/recordings/tree-navigation.tape`
**Purpose**: Demonstrates hierarchical tree navigation
**Features Covered**:
- Tree item navigation (j/k)
- Expand/collapse (l/h)
- Tree structure with multiple levels
- Visual indicators (symbols)
- Navigation wrapping behavior

### `spec/contracts-menu/recordings/load-contract.tape`
**Purpose**: Demonstrates loading a contract and auto-expansion
**Features Covered**:
- Load new contract action
- File picker interaction
- Contract loading and compilation
- Auto-expansion of loaded contract
- Deploy and Load existing options appear
- Contract becomes selected item

---

## Command Palette (Ctrl+P) Recordings

### `spec/ctrl-p-menu/recordings/palette-basics.tape`
**Purpose**: Demonstrates opening and basic palette usage
**Features Covered**:
- Opening palette with Ctrl+P
- Palette overlay centered display
- Command list visibility
- Navigation through commands (j/k)
- Search/filter capability
- Closing palette with Escape

### `spec/ctrl-p-menu/recordings/command-execution.tape`
**Purpose**: Demonstrates command execution from palette
**Features Covered**:
- Navigating to specific commands
- Executing commands with Enter
- Palette closes after execution
- Multiple command execution
- Command feedback

---

## Transaction & Call Popup Recordings

### `spec/tx-and-call-popup/recordings/parameter-input.tape`
**Purpose**: Demonstrates parameter input popup interaction
**Features Covered**:
- Parameter popup display
- Parameter input field rendering
- Tab to move between fields
- Type text input
- Popup title showing method/constructor signature
- Cancel with Escape
- Multiple parameter types

### `spec/tx-and-call-popup/recordings/validation-feedback.tape`
**Purpose**: Demonstrates real-time parameter validation
**Features Covered**:
- Invalid address detection
- Error message display
- Real-time validation feedback
- Field clearing with Ctrl+U
- Valid input acceptance
- Validation state in popup

---

## Output Panel Recordings

### `spec/output-panel/recordings/card-navigation.tape`
**Purpose**: Demonstrates navigating through output cards
**Features Covered**:
- Output cards display
- Card selection with j/k
- Card highlighting (cyan background)
- Tab to focus output area
- Open card menu with Enter/Space
- Navigate through card actions
- Close menu with Escape

### `spec/output-panel/recordings/card-types.tape`
**Purpose**: Demonstrates different card types (Log, Transaction, Call)
**Features Covered**:
- Log cards (Loading, status messages)
- Transaction card appearance
- Call card appearance
- Card type differentiation
- Multiple card navigation
- Focus switching between sidebar and output

---

## General Settings Recordings

### `spec/general-settings/recordings/configuration.tape`
**Purpose**: Demonstrates configuration management
**Features Covered**:
- Config file usage on startup
- Command palette for config editing
- Edit config command
- Config file location (.evm-cli/config.json)
- Environment variable awareness

### `spec/general-settings/recordings/keyboard-shortcuts.tape`
**Purpose**: Demonstrates all major keyboard shortcuts
**Features Covered**:
- Tab - Focus switching
- Ctrl+P - Command palette
- j/k - Navigation (vim keys)
- h/l - Expand/collapse (vim keys)
- Delete - Item deletion
- Ctrl+C - Exit application
- Escape - Close modals

---

## QA Expert Recording Tool

### Automated Recording Script

The **`scripts/record-spec-tape.sh`** script provides a complete automation solution for QA experts:

**Features:**
- ✅ Prerequisite validation (vhs, solc, blockchain)
- ✅ Automatic project build
- ✅ VHS recording execution
- ✅ ASCII file generation (for text review)
- ✅ MP4 video generation (for playback)
- ✅ Review guide creation
- ✅ Error handling and recovery

**Quick Start:**

```bash
# Record a single spec feature
./scripts/record-spec-tape.sh <spec-name> <feature-name>

# Record main-interface layout
./scripts/record-spec-tape.sh main-interface layout-and-zones

# Record contracts menu navigation
./scripts/record-spec-tape.sh contracts-menu tree-navigation
```

**Complete Documentation:**

See **[QA_EXPERT_GUIDE.md](QA_EXPERT_GUIDE.md)** for:
- Complete prerequisite setup
- Step-by-step recording instructions
- ASCII file review process
- Review checklists
- Troubleshooting guide
- Best practices

---

## Recording Naming Convention

All recordings follow this naming pattern:
```
spec/[spec-name]/recordings/[feature-name].tape
```

Where:
- `[spec-name]`: One of: main-interface, contracts-menu, ctrl-p-menu, tx-and-call-popup, output-panel, general-settings
- `[feature-name]`: Descriptive name of the feature being demonstrated

---

## How to Generate Recordings

### Method 1: Using QA Expert Recording Script (Recommended)

The `scripts/record-spec-tape.sh` script automates the entire recording process for QA experts.

**Prerequisites:**
- VHS installed: `brew install charmbracelet/tap/vhs` (macOS) or see https://github.com/charmbracelet/vhs
- solc installed: `brew install solidity` (macOS)
- Anvil running (optional): `anvil` for blockchain-dependent recordings

**Usage:**

```bash
# Record a single spec feature
./scripts/record-spec-tape.sh <spec-name> <feature-name>

# Examples:
./scripts/record-spec-tape.sh main-interface layout-and-zones
./scripts/record-spec-tape.sh contracts-menu load-contract
./scripts/record-spec-tape.sh ctrl-p-menu palette-basics
./scripts/record-spec-tape.sh tx-and-call-popup parameter-input
./scripts/record-spec-tape.sh output-panel card-navigation
./scripts/record-spec-tape.sh general-settings keyboard-shortcuts
```

**What the script does:**
1. ✅ Validates prerequisites (vhs, solc, blockchain)
2. ✅ Builds the project in release mode
3. ✅ Runs the VHS tape file
4. ✅ Generates `.ascii` file for terminal review
5. ✅ Generates `.mp4` file for video review
6. ✅ Creates a `.REVIEW.md` guide for QA testing
7. ✅ Verifies all outputs

**Output Files:**
After running the script, you'll find:
- `spec/<spec-name>/recordings/<feature>.tape` - Script used for recording
- `spec/<spec-name>/recordings/<feature>.ascii` - ASCII terminal recording (for review)
- `spec/<spec-name>/recordings/<feature>.mp4` - MP4 video (for presentation)
- `spec/<spec-name>/recordings/<feature>.REVIEW.md` - Review guide and checklist

**Review the Recording:**

```bash
# View ASCII recording in terminal
vhs play spec/<spec-name>/recordings/<feature>.ascii

# View raw ASCII text
cat spec/<spec-name>/recordings/<feature>.ascii

# Watch MP4 video
open spec/<spec-name>/recordings/<feature>.mp4
```

### Method 2: Manual VHS Recording

If you prefer to run VHS directly:

```bash
# Generate a single recording
vhs < spec/main-interface/recordings/layout-and-zones.tape

# Generate all recordings
for tape in spec/*/recordings/*.tape; do
  vhs < "$tape"
done

# Record and output specific formats
vhs record spec/main-interface/recordings/layout-and-zones.tape --output spec/main-interface/recordings/layout-and-zones.gif
```

---

## Recording Quality Standards

All recordings follow these standards:
- **Format**: VHS tape scripts (.tape files)
- **Terminal Size**: 1200x600 pixels for clear visibility
- **Theme**: Molokai for good contrast
- **Font Size**: 14pt for readability
- **Typing Speed**: 50ms between characters
- **Duration**: 15-60 seconds each for focused demonstration
- **Coverage**: All major workflows from each spec

---

## Verification Checklist

- ✅ All 6 specs have recording scripts created
- ✅ Each spec has 2 focused recordings
- ✅ Total of 12 recording scripts
- ✅ All scripts follow VHS format
- ✅ Scripts cover main workflows from specs
- ✅ Scripts demonstrate expected behaviors
- ✅ Recording file paths documented
- ✅ Clear naming convention applied

---

## Next Steps

1. Execute VHS tool on all tape files to generate actual video recordings
2. Verify recordings play correctly and demonstrate specs accurately
3. Archive generated video files (MP4/GIF format)
4. Update spec documentation with recording links
5. Create recording viewer/reference in CI/CD pipeline


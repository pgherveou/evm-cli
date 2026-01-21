# QA Expert Guide: Recording & Reviewing Spec Tapes

This guide provides comprehensive instructions for QA experts to record VHS tapes and generate review files (.ascii) for evm-cli specifications.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Prerequisites](#prerequisites)
3. [Recording Scripts](#recording-scripts)
4. [Understanding ASCII Recordings](#understanding-ascii-recordings)
5. [Reviewing Recordings](#reviewing-recordings)
6. [Review Checklist](#review-checklist)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Record a Single Spec

```bash
./scripts/record-spec-tape.sh <spec-name> <feature-name>
```

**Example:**
```bash
./scripts/record-spec-tape.sh main-interface layout-and-zones
```

### Review the Recording

```bash
# View ASCII recording
vhs play spec/main-interface/recordings/layout-and-zones.ascii

# View raw text
cat spec/main-interface/recordings/layout-and-zones.ascii | less
```

### Record All Specs

```bash
# Record all specs in batch
for spec in main-interface contracts-menu ctrl-p-menu tx-and-call-popup output-panel general-settings; do
  for feature in spec/$spec/recordings/*.tape; do
    feature_name=$(basename "$feature" .tape)
    ./scripts/record-spec-tape.sh "$spec" "$feature_name"
  done
done
```

---

## Prerequisites

### Required Software

1. **VHS** - Terminal recording tool
   ```bash
   # macOS
   brew install charmbracelet/tap/vhs

   # Linux - Download from:
   # https://github.com/charmbracelet/vhs/releases

   # Windows (WSL)
   wget https://github.com/charmbracelet/vhs/releases/download/.../vhs_Linux_x86_64.tar.gz
   ```

2. **Solc** - Solidity Compiler
   ```bash
   # macOS
   brew install solidity

   # Linux (Arch)
   sudo pacman -S solidity

   # Linux (Ubuntu/Debian)
   sudo apt-get install solc
   ```

3. **Rust & Cargo** - For building the project
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

4. **Anvil** - Local Ethereum test node (optional but recommended)
   ```bash
   # Via Foundry
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   anvil
   ```

### Verify Prerequisites

```bash
# Check all tools are available
vhs --version
solc --version
cargo --version
rustc --version
```

---

## Recording Scripts

### Built-in Recording Tool

The `scripts/record-spec-tape.sh` script automates the entire recording process:

**Features:**
- ✅ Validates all prerequisites
- ✅ Builds the project if needed
- ✅ Runs VHS on tape files
- ✅ Generates ASCII recordings
- ✅ Generates MP4 videos
- ✅ Creates review guides
- ✅ Backs up and restores configs

### Available Specs

```
main-interface        - Main TUI layout and zones
contracts-menu        - Sidebar contract tree
ctrl-p-menu          - Command palette
tx-and-call-popup    - Parameter input forms
output-panel         - Card-based output display
general-settings     - Configuration and shortcuts
```

### Usage Examples

**Record main-interface layout:**
```bash
./scripts/record-spec-tape.sh main-interface layout-and-zones
```

**Record contracts-menu tree navigation:**
```bash
./scripts/record-spec-tape.sh contracts-menu tree-navigation
```

**Record all features of a spec:**
```bash
./scripts/record-spec-tape.sh main-interface focus-management
./scripts/record-spec-tape.sh ctrl-p-menu palette-basics
./scripts/record-spec-tape.sh tx-and-call-popup parameter-input
```

### Recording Output

After running the script, you'll find these files in `spec/<spec-name>/recordings/`:

```
<feature>.tape         - VHS script used for recording
<feature>.ascii        - ASCII terminal recording (for text review)
<feature>.mp4          - MP4 video (for playback)
<feature>.REVIEW.md    - Review checklist and guide
```

---

## Understanding ASCII Recordings

### What is ASCII?

ASCII recordings are text-based terminal captures that show:
- All terminal commands executed
- All output displayed
- Timing information
- Color codes (represented as escape sequences)

### ASCII File Format

```
% ASCII Recording Format:
# Command line: vhs record demo.tape
# Recording started at: 2026-01-21T11:00:00Z
# Terminal: 1200x600

▌ evm-cli Application
├─ Loading...
└─ Ready

Key bindings:
> Exit with Ctrl+C
```

### Reviewing ASCII Files

#### View in Terminal

```bash
# Direct view (will show color codes)
cat spec/main-interface/recordings/layout-and-zones.ascii

# View with pager (allows scrolling)
less -R spec/main-interface/recordings/layout-and-zones.ascii

# View line count
wc -l spec/main-interface/recordings/layout-and-zones.ascii
```

#### Playback with VHS

```bash
# Play the recording at normal speed
vhs play spec/main-interface/recordings/layout-and-zones.ascii

# Play at custom speed
vhs play --speed 1.5 spec/main-interface/recordings/layout-and-zones.ascii
```

#### Extract Specific Sections

```bash
# View first 50 lines
head -50 spec/main-interface/recordings/layout-and-zones.ascii

# View last 50 lines
tail -50 spec/main-interface/recordings/layout-and-zones.ascii

# Search for specific text
grep -n "Connected" spec/main-interface/recordings/layout-and-zones.ascii
```

### Interpreting ASCII Recording Content

**Key sections to look for:**

1. **Startup Phase**
   ```
   ▌ Load new contract
   ▌ Connected | Chain: 1 | Account: 0x...
   ```

2. **Navigation**
   ```
   Type "j"
   Down
   ```

3. **User Input**
   ```
   Type "examples/Demo.sol"
   Enter
   ```

4. **Output/Results**
   ```
   Status: Success ✓
   Gas Used: 21000
   ```

5. **Shutdown**
   ```
   Ctrl+C
   (application exits)
   ```

---

## Reviewing Recordings

### Step-by-Step Review Process

#### 1. Prepare for Review

```bash
# Set up your review workspace
cd /home/pg/github/evm-cli

# Open three terminals/windows:
# Terminal 1: Review ASCII file
# Terminal 2: Read spec documentation
# Terminal 3: Check tape script
```

#### 2. Read the Tape Script

The tape script shows exactly what actions were recorded:

```bash
cat spec/main-interface/recordings/layout-and-zones.tape
```

**Look for:**
- All `Type` commands (what was typed)
- All `Enter` presses (when actions executed)
- All `Wait` statements (expected output)
- Timing (`Sleep` durations)

#### 3. Review the Specification

Open the corresponding spec to understand what should be demonstrated:

```bash
cat spec/main-interface/spec.md
```

**Check:**
- Layout description (3-zone layout)
- Visual design (colors, styling)
- Keyboard controls
- Expected behaviors
- Status bar information

#### 4. Play the ASCII Recording

Review the actual terminal recording:

```bash
vhs play spec/main-interface/recordings/layout-and-zones.ascii
```

**Observe:**
- Is the layout correct? (sidebar, output, status bar visible)
- Are colors appropriate? (cyan selection, status colors)
- Does navigation work smoothly?
- Are all elements responsive?
- Does timing seem realistic?

#### 5. Compare Recording to Spec

Use the review checklist (below) to verify:
- ✅ Visual elements match mockups
- ✅ Keyboard navigation works
- ✅ Features behave as documented
- ✅ Errors display correctly
- ✅ Transitions are smooth

#### 6. Document Findings

Create a review report:

```bash
# Use the generated review guide
cat spec/main-interface/recordings/layout-and-zones.REVIEW.md

# Or create your own
cat > spec/main-interface/recordings/layout-and-zones-QA-REVIEW.txt << 'EOF'
REVIEW: main-interface / layout-and-zones
Date: 2026-01-21
Reviewer: QA Expert Name
Status: PASS / FAIL / NEEDS-WORK

Findings:
- [] Layout matches spec
- [ ] Sidebar visible and functional
- [ ] Output area responsive
- [ ] Status bar shows connection info
- [ ] Focusing between areas works
- [ ] No rendering glitches

Issues Found:
(List any issues here)

Recommendations:
(Suggest improvements)

EOF
```

---

## Review Checklist

### Visual Design Checklist

#### Layout & Structure
- [ ] Three-zone layout clearly visible (sidebar, output, status bar)
- [ ] Sidebar width proportional (~25-30% of terminal)
- [ ] Output area width proportional (~70-75% of terminal)
- [ ] Status bar height is 1 line
- [ ] All zones clearly separated

#### Colors & Styling
- [ ] Cyan selection highlighting visible and correct
- [ ] Green `✓` for success messages
- [ ] Red `✗` for error messages
- [ ] Yellow `⟳` for loading states
- [ ] Blue `ℹ` for info messages
- [ ] Text is readable with good contrast

#### Typography
- [ ] Monospace font used throughout
- [ ] Clear hierarchy in headings
- [ ] Status indicators visible
- [ ] Addresses truncated appropriately (e.g., `0x742d...e7595f`)

### Keyboard Navigation Checklist

- [ ] Arrow keys (↑↓←→) navigate correctly
- [ ] Vim keys (hjkl) work as alternatives
- [ ] `j` moves down, `k` moves up
- [ ] `l` expands, `h` collapses
- [ ] `Enter` executes selected item
- [ ] `Tab` switches focus between areas
- [ ] `Escape` cancels/closes modals

### Feature-Specific Checklist

#### main-interface
- [ ] Three zones visible and functional
- [ ] Tab switches focus between sidebar and output
- [ ] Status bar displays connection, chain, account, balance
- [ ] Status bar updates in real-time

#### contracts-menu
- [ ] Tree structure displays correctly
- [ ] `▾` and `▸` indicators show expand/collapse state
- [ ] `◇` shows deploy/load actions
- [ ] Methods display with type tags `[view]`, `[pay]`
- [ ] Auto-expansion after contract load works
- [ ] Selection persists when navigating

#### ctrl-p-menu
- [ ] Palette opens with `Ctrl+P`
- [ ] Palette is centered on screen
- [ ] Search/filter works in real-time
- [ ] Commands display correctly
- [ ] Navigation through commands works
- [ ] `Enter` executes selected command
- [ ] `Escape` closes palette

#### tx-and-call-popup
- [ ] Parameter input form appears
- [ ] Form title shows method/constructor signature
- [ ] Field labels display with type hints
- [ ] Cursor visible in input fields
- [ ] `Tab` moves between fields
- [ ] Validation feedback appears in real-time
- [ ] Invalid fields show `✗` error messages
- [ ] Submit blocked while fields invalid

#### output-panel
- [ ] Cards display in output area
- [ ] Cards are selectable with navigation
- [ ] Selected card highlighted with cyan background
- [ ] Transaction cards show correct info
- [ ] Call cards show correct result
- [ ] Log cards show status messages
- [ ] Footer action menu appears when card selected
- [ ] Actions are executable

#### general-settings
- [ ] Configuration file accessible
- [ ] Keyboard shortcuts work correctly
- [ ] `Ctrl+P` opens command palette
- [ ] `Ctrl+L` clears output
- [ ] `Ctrl+C` exits application
- [ ] `Tab` switches focus
- [ ] Status messages display clearly

### Quality Checklist

- [ ] Recording is clear and readable
- [ ] No glitches or rendering issues
- [ ] Timing is realistic (not too fast/slow)
- [ ] All important actions visible
- [ ] Transitions are smooth
- [ ] No unexpected pauses or delays
- [ ] Audio (if applicable) is clear

---

## Troubleshooting

### Common Issues

#### "vhs: command not found"
**Solution:**
```bash
# Install VHS
brew install charmbracelet/tap/vhs

# Or download from GitHub:
# https://github.com/charmbracelet/vhs/releases
```

#### "solc: command not found"
**Solution:**
```bash
# Install Solidity compiler
brew install solidity
# Or: sudo pacman -S solidity
```

#### "Blockchain not accessible"
**Solution:**
```bash
# Start Anvil in another terminal
anvil

# Or use existing RPC
export BLOCKCHAIN_RPC=https://eth.llamarpc.com
./scripts/record-spec-tape.sh main-interface layout-and-zones
```

#### "Build failed"
**Solution:**
```bash
# Clean build
cargo clean
cargo build --release

# Or let the script handle it:
./scripts/record-spec-tape.sh main-interface layout-and-zones
```

#### "ASCII file empty or too small"
**Solution:**
1. Check tape file syntax (invalid VHS syntax)
2. Verify the application ran successfully
3. Check for timeout issues (increase Sleep durations)
4. Re-run recording:
   ```bash
   ./scripts/record-spec-tape.sh main-interface layout-and-zones
   ```

#### "Recording stopped unexpectedly"
**Solution:**
1. Check terminal size: Must be at least 80x24
2. Verify app didn't crash (check for error messages)
3. Increase Sleep timings in tape file
4. Check VHS logs: `vhs record --verbose spec/.../feature.tape`

### Debugging Tips

**Enable verbose VHS output:**
```bash
vhs --verbose record spec/main-interface/recordings/layout-and-zones.tape
```

**Test tape syntax:**
```bash
# Run tape and check for errors
vhs < spec/main-interface/recordings/layout-and-zones.tape 2>&1 | head -20
```

**Capture terminal output:**
```bash
# Record your own test run and compare
script recording.log
./target/release/evm-cli
exit  # Exit script
cat recording.log
```

---

## Best Practices

### Recording Best Practices

1. **Clean Environment**
   - Clear terminal history: `clear`
   - Remove old config: `rm ~/.evm-cli/config.json`
   - Fresh database: `anvil` (new process each time)

2. **Realistic Timing**
   - Use `Sleep 0.5s` between rapid actions
   - Use `Sleep 1s-2s` after significant operations
   - Use `Wait /pattern/` instead of fixed sleeps when possible

3. **Clear Actions**
   - Type slowly enough to be readable
   - Pause after entering values
   - Show results before moving to next action

4. **Error Scenarios**
   - Show both happy path and error cases
   - Display validation errors
   - Show recovery from errors

### Review Best Practices

1. **Comprehensive Review**
   - Always compare against spec
   - Check all keyboard shortcuts
   - Test error scenarios if recorded
   - Verify visual consistency

2. **Document Issues**
   - Be specific about what's wrong
   - Include steps to reproduce
   - Note expected vs actual behavior
   - Reference spec sections

3. **Approve or Re-record**
   - Approve only if matches spec 100%
   - Re-record if any issues found
   - Don't merge recordings with issues
   - Add review approval to commit message

---

## Integration with CI/CD

### Automated Recording Verification

Add to your CI pipeline:

```bash
#!/bin/bash
# ci/verify-recordings.sh

set -e

echo "Verifying all spec recordings..."

specs=(
  "main-interface:layout-and-zones"
  "contracts-menu:tree-navigation"
  "ctrl-p-menu:palette-basics"
  # ... etc
)

for spec_feature in "${specs[@]}"; do
  IFS=':' read -r spec feature <<< "$spec_feature"

  echo "Recording: $spec / $feature"
  ./scripts/record-spec-tape.sh "$spec" "$feature"

  # Verify files created
  [[ -f "spec/$spec/recordings/$feature.ascii" ]] || exit 1
  [[ -f "spec/$spec/recordings/$feature.mp4" ]] || exit 1
done

echo "✓ All recordings verified successfully"
```

---

## Support & Contact

For issues with recording or reviewing:

1. Check this guide's [Troubleshooting](#troubleshooting) section
2. Review VHS documentation: https://github.com/charmbracelet/vhs
3. File an issue with:
   - Spec name and feature
   - Error message
   - Steps attempted
   - Output of `./scripts/record-spec-tape.sh --help`

---

*QA Expert Guide - Last Updated: January 2026*

# QA Recording Setup Guide

Complete setup and usage guide for recording and reviewing evm-cli specification tapes.

---

## Overview

The QA recording system allows QA experts to:
1. **Record** terminal sessions as VHS tapes
2. **Generate** ASCII files for text-based review
3. **Export** MP4 videos for presentation
4. **Document** feature demonstrations with automated guides

---

## Quick Start (5 Minutes)

### 1. Install Prerequisites

```bash
# macOS
brew install charmbracelet/tap/vhs
brew install solidity
brew install rust  # or use rustup

# Linux (Ubuntu/Debian)
sudo apt-get install solc
# VHS: https://github.com/charmbracelet/vhs/releases
# Rust: https://rustup.rs/

# Start local blockchain (optional but recommended)
brew install foundry  # or npm install -g @nomicfoundation/hardhat
anvil  # in separate terminal
```

### 2. Build the Project

```bash
cd /home/pg/github/evm-cli
cargo build --release
```

### 3. Record Your First Spec

```bash
# Record main-interface layout demonstration
./scripts/record-spec-tape.sh main-interface layout-and-zones

# Or contracts menu navigation
./scripts/record-spec-tape.sh contracts-menu tree-navigation
```

### 4. Review the Recording

```bash
# Play ASCII recording
vhs play spec/main-interface/recordings/layout-and-zones.ascii

# View review checklist
cat spec/main-interface/recordings/layout-and-zones.REVIEW.md
```

---

## Detailed Setup Instructions

### Step 1: Install VHS

VHS is a terminal session recorder by Charm Bracelet.

**macOS:**
```bash
brew install charmbracelet/tap/vhs
vhs --version  # Verify installation
```

**Linux (Ubuntu/Debian):**
```bash
# Download from releases
wget https://github.com/charmbracelet/vhs/releases/download/v0.10.0/vhs_Linux_x86_64.tar.gz
tar xzf vhs_Linux_x86_64.tar.gz
sudo mv vhs /usr/local/bin/
vhs --version
```

**Linux (Arch):**
```bash
yay -S vhs  # or pacman if available
```

**Windows (WSL):**
```bash
# Same as Linux instructions above
```

### Step 2: Install Solidity Compiler

The tape files require `solc` for contract compilation.

**macOS:**
```bash
brew install solidity
solc --version
```

**Linux (Ubuntu/Debian):**
```bash
sudo add-apt-repository ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc
solc --version
```

**Linux (Arch):**
```bash
sudo pacman -S solidity
solc --version
```

### Step 3: Install Rust & Cargo

Needed for building evm-cli.

```bash
# Install Rust (one-liner)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or use package manager
brew install rust  # macOS
sudo apt-get install cargo  # Linux
```

### Step 4: Setup Local Blockchain (Optional)

Some recordings benefit from a running blockchain node.

```bash
# Via Foundry (recommended)
curl -L https://foundry.paradigm.xyz | bash
foundryup
anvil  # Runs local node at http://localhost:8545

# Or via Hardhat
npm install -g @nomicfoundation/hardhat
hardhat node
```

### Step 5: Verify Setup

```bash
# Check all prerequisites
echo "Checking prerequisites..."
command -v vhs && echo "✓ VHS" || echo "✗ VHS"
command -v solc && echo "✓ solc" || echo "✗ solc"
command -v cargo && echo "✓ Cargo" || echo "✗ Cargo"
command -v rustc && echo "✓ Rustc" || echo "✗ Rustc"

# Build the project
cd /home/pg/github/evm-cli
cargo build --release

# Verify script is executable
ls -lh scripts/record-spec-tape.sh
```

---

## Recording Workflow

### Using the Automated Recording Script

#### Basic Usage

```bash
./scripts/record-spec-tape.sh <spec-name> <feature-name>
```

#### Available Specs

```
main-interface        - Main TUI layout and zones
contracts-menu        - Sidebar contract tree navigation
ctrl-p-menu          - Command palette (Ctrl+P)
tx-and-call-popup    - Parameter input forms
output-panel         - Card-based output display
general-settings     - Configuration and shortcuts
```

#### Recording Examples

```bash
# Record main interface
./scripts/record-spec-tape.sh main-interface layout-and-zones
./scripts/record-spec-tape.sh main-interface focus-management

# Record contracts menu
./scripts/record-spec-tape.sh contracts-menu tree-navigation
./scripts/record-spec-tape.sh contracts-menu load-contract

# Record command palette
./scripts/record-spec-tape.sh ctrl-p-menu palette-basics
./scripts/record-spec-tape.sh ctrl-p-menu command-execution

# Record parameter popups
./scripts/record-spec-tape.sh tx-and-call-popup parameter-input
./scripts/record-spec-tape.sh tx-and-call-popup validation-feedback

# Record output panel
./scripts/record-spec-tape.sh output-panel card-navigation
./scripts/record-spec-tape.sh output-panel card-types

# Record settings
./scripts/record-spec-tape.sh general-settings configuration
./scripts/record-spec-tape.sh general-settings keyboard-shortcuts
```

#### Script Output

The script will:

1. **Validate prerequisites**
   ```
   ✓ vhs found
   ✓ solc found
   ✓ Blockchain accessible
   ```

2. **Build project**
   ```
   → Building evm-cli...
   ✓ Build successful
   ```

3. **Record tape**
   ```
   → Recording tape...
   → Running VHS recording...
   ✓ Recording completed!
   ```

4. **Generate outputs**
   ```
   ✓ ASCII file created: spec/main-interface/recordings/layout-and-zones.ascii (125K)
   ✓ MP4 file created: spec/main-interface/recordings/layout-and-zones.mp4 (1.2M)
   ✓ Review guide created: spec/main-interface/recordings/layout-and-zones.REVIEW.md
   ```

#### Next Steps After Recording

```bash
# 1. Play the ASCII recording
vhs play spec/main-interface/recordings/layout-and-zones.ascii

# 2. Read the review guide
cat spec/main-interface/recordings/layout-and-zones.REVIEW.md

# 3. Compare with specification
cat spec/main-interface/spec.md

# 4. Check the tape script
cat spec/main-interface/recordings/layout-and-zones.tape
```

---

## Reviewing ASCII Files

### What is ASCII?

ASCII recordings are text-based terminal captures that show:
- All commands executed
- Terminal output displayed
- Timing and delays
- Color codes (as escape sequences)

### How to Review

#### 1. View in Terminal

```bash
# Direct view
cat spec/main-interface/recordings/layout-and-zones.ascii

# With color codes preserved
cat -A spec/main-interface/recordings/layout-and-zones.ascii

# Page through file
less -R spec/main-interface/recordings/layout-and-zones.ascii

# Count lines
wc -l spec/main-interface/recordings/layout-and-zones.ascii
```

#### 2. Play Recording

```bash
# Play at normal speed
vhs play spec/main-interface/recordings/layout-and-zones.ascii

# Play at custom speed
vhs play --speed 0.5 spec/main-interface/recordings/layout-and-zones.ascii  # Slow
vhs play --speed 2.0 spec/main-interface/recordings/layout-and-zones.ascii  # Fast

# Play and save to file
vhs play spec/main-interface/recordings/layout-and-zones.ascii > /tmp/playback.log
```

#### 3. Inspect Content

```bash
# Search for specific text
grep -n "Connected" spec/main-interface/recordings/layout-and-zones.ascii

# View first 100 lines (start of recording)
head -100 spec/main-interface/recordings/layout-and-zones.ascii

# View last 100 lines (end of recording)
tail -100 spec/main-interface/recordings/layout-and-zones.ascii

# Extract specific section
sed -n '100,200p' spec/main-interface/recordings/layout-and-zones.ascii
```

#### 4. Compare with Specification

```bash
# Open in your editor
vim spec/main-interface/spec.md &
vhs play spec/main-interface/recordings/layout-and-zones.ascii
```

---

## Review Checklist

### Before Starting

- [ ] Tape file exists at `spec/<spec-name>/recordings/<feature>.tape`
- [ ] ASCII file generated successfully
- [ ] Review guide available at `spec/<spec-name>/recordings/<feature>.REVIEW.md`
- [ ] Specification document available at `spec/<spec-name>/spec.md`

### Visual Elements

- [ ] Layout matches spec mockups
- [ ] Colors correct (cyan selection, status colors)
- [ ] Spacing follows design system
- [ ] Typography readable and consistent
- [ ] All interactive elements visible
- [ ] No rendering glitches

### Keyboard Navigation

- [ ] Arrow keys work correctly
- [ ] Vim keys (hjkl) functional
- [ ] Tab switches focus
- [ ] Enter executes actions
- [ ] Escape cancels/closes modals
- [ ] All shortcuts from spec work

### Feature-Specific Behavior

- [ ] Feature behaves as documented
- [ ] Error handling displays correctly
- [ ] Status feedback is clear
- [ ] Loading states visible
- [ ] Transitions smooth
- [ ] Auto-expansion works

### Quality Assessment

- [ ] Recording is clear and readable
- [ ] No terminal glitches
- [ ] Timing is realistic
- [ ] All steps visible
- [ ] Can be used for documentation
- [ ] Suitable for demo/presentation

---

## Batch Recording (All Specs)

Record all specs at once:

```bash
#!/bin/bash
# Record all spec features

cd /home/pg/github/evm-cli

specs=(
  "main-interface:layout-and-zones"
  "main-interface:focus-management"
  "contracts-menu:tree-navigation"
  "contracts-menu:load-contract"
  "ctrl-p-menu:palette-basics"
  "ctrl-p-menu:command-execution"
  "tx-and-call-popup:parameter-input"
  "tx-and-call-popup:validation-feedback"
  "output-panel:card-navigation"
  "output-panel:card-types"
  "general-settings:configuration"
  "general-settings:keyboard-shortcuts"
)

for spec_feature in "${specs[@]}"; do
  IFS=':' read -r spec feature <<< "$spec_feature"
  echo "Recording: $spec / $feature"
  ./scripts/record-spec-tape.sh "$spec" "$feature"
  echo ""
done

echo "✓ All recordings complete!"
```

Save as `scripts/record-all-specs.sh` and run:

```bash
chmod +x scripts/record-all-specs.sh
./scripts/record-all-specs.sh
```

---

## Troubleshooting

### VHS Not Found

```bash
# macOS
brew install charmbracelet/tap/vhs

# Linux - Download from GitHub
wget https://github.com/charmbracelet/vhs/releases/download/v0.10.0/vhs_Linux_x86_64.tar.gz
tar xzf vhs_Linux_x86_64.tar.gz
sudo mv vhs /usr/local/bin/
```

### Solc Not Found

```bash
# macOS
brew install solidity

# Linux (Ubuntu)
sudo apt-get update
sudo apt-get install solc

# Linux (Arch)
sudo pacman -S solidity
```

### Build Fails

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Or let the script handle it
./scripts/record-spec-tape.sh main-interface layout-and-zones
```

### Recording Timeouts

**Issue:** "timeout waiting for pattern"

**Solutions:**
1. Increase Sleep durations in tape file
2. Check that app actually runs successfully
3. Verify terminal size (must be 80x24 minimum)
4. Run with blockchain if tape requires it

### Empty or Small ASCII File

**Issue:** ASCII file too small or empty

**Solutions:**
1. Check tape syntax (invalid VHS commands)
2. Verify app ran without crashing
3. Check system resources (CPU, memory)
4. Re-run with verbose output:
   ```bash
   vhs --verbose record spec/.../feature.tape
   ```

### Blockchain Connection Issues

**Issue:** "Blockchain not accessible"

**Solutions:**
```bash
# Start blockchain
anvil

# Or skip blockchain-dependent features
# Modify tape file to remove blockchain-dependent Wait statements
```

---

## Documentation Files

### Files Created/Updated

1. **scripts/record-spec-tape.sh** (11KB)
   - Automated recording tool for QA experts
   - Handles prerequisites, build, recording, output
   - Comprehensive error handling

2. **scripts/README.md** (2KB)
   - Script directory documentation
   - Quick reference for tools

3. **spec/QA_EXPERT_GUIDE.md** (15KB)
   - Complete QA guide
   - Setup instructions
   - Review procedures
   - Troubleshooting

4. **spec/RECORDINGS_MANIFEST.md** (5KB)
   - Recording manifest and index
   - Recording file locations
   - Usage instructions

5. **QA_RECORDING_SETUP.md** (this file)
   - Complete setup and workflow guide
   - Quick start instructions
   - Review processes

### All Tape Files Updated

All 12 tape files now include:
- `.ascii` output (text-based recording)
- `.mp4` output (video recording)
- Proper VHS configuration

---

## Next Steps

1. **Setup Environment**
   ```bash
   # Install all prerequisites
   brew install vhs solidity rust
   ```

2. **Build Project**
   ```bash
   cd /home/pg/github/evm-cli
   cargo build --release
   ```

3. **Record Your First Spec**
   ```bash
   ./scripts/record-spec-tape.sh main-interface layout-and-zones
   ```

4. **Review the Recording**
   ```bash
   vhs play spec/main-interface/recordings/layout-and-zones.ascii
   cat spec/main-interface/recordings/layout-and-zones.REVIEW.md
   ```

5. **Document Your Findings**
   - Use the review guide
   - Compare with specification
   - Note any issues or improvements
   - Approve or re-record as needed

---

## Support

For issues or questions:

1. Check **[QA_EXPERT_GUIDE.md](spec/QA_EXPERT_GUIDE.md)** - Comprehensive guide
2. Check **[scripts/README.md](scripts/README.md)** - Script documentation
3. Review **[RECORDINGS_MANIFEST.md](spec/RECORDINGS_MANIFEST.md)** - Recording reference
4. File an issue with:
   - Error messages
   - Steps attempted
   - Spec and feature name
   - System information

---

*QA Recording Setup Guide - January 2026*

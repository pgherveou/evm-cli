#!/bin/bash
# QA Expert Spec Recording Script
# Records VHS tapes for evm-cli specs and generates .ascii review files
#
# Usage:
#   ./scripts/record-spec-tape.sh <spec-name> <feature-name>
#
# Examples:
#   ./scripts/record-spec-tape.sh main-interface layout-and-zones
#   ./scripts/record-spec-tape.sh contracts-menu load-contract
#   ./scripts/record-spec-tape.sh ctrl-p-menu palette-basics

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SPECS_DIR="$PROJECT_ROOT/spec"
BLOCKCHAIN_RPC="${BLOCKCHAIN_RPC:-http://localhost:8545}"

# Supported specs
declare -A SPECS=(
  ["main-interface"]="Main Interface Spec"
  ["contracts-menu"]="Contracts Menu Spec"
  ["ctrl-p-menu"]="Command Palette Spec"
  ["tx-and-call-popup"]="Transaction & Call Popup Spec"
  ["output-panel"]="Output Panel Spec"
  ["general-settings"]="General Settings Spec"
)

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
  echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
  echo -e "${BLUE}$1${NC}"
  echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
}

print_success() {
  echo -e "${GREEN}✓${NC} $1"
}

print_error() {
  echo -e "${RED}✗${NC} $1"
}

print_info() {
  echo -e "${CYAN}ℹ${NC} $1"
}

print_step() {
  echo -e "${YELLOW}→${NC} $1"
}

# ============================================================================
# Argument Validation
# ============================================================================

if [[ $# -lt 2 ]]; then
  echo -e "${RED}Usage: $0 <spec-name> <feature-name>${NC}"
  echo ""
  echo "Available specs:"
  for spec in "${!SPECS[@]}"; do
    echo "  - $spec: ${SPECS[$spec]}"
  done
  echo ""
  echo "Examples:"
  echo "  $0 main-interface layout-and-zones"
  echo "  $0 contracts-menu load-contract"
  exit 1
fi

SPEC_NAME="$1"
FEATURE_NAME="$2"

# Validate spec name
if [[ ! -v SPECS[$SPEC_NAME] ]]; then
  print_error "Unknown spec: $SPEC_NAME"
  echo "Available specs: ${!SPECS[@]}"
  exit 1
fi

# ============================================================================
# Print Banner
# ============================================================================

print_header "evm-cli QA Recording Tool"
print_info "Spec: ${SPECS[$SPEC_NAME]} ($SPEC_NAME)"
print_info "Feature: $FEATURE_NAME"
echo ""

# ============================================================================
# Setup Paths
# ============================================================================

SPEC_DIR="$SPECS_DIR/$SPEC_NAME"
RECORDINGS_DIR="$SPEC_DIR/recordings"
TAPE_FILE="$RECORDINGS_DIR/$FEATURE_NAME.tape"
ASCII_FILE="$RECORDINGS_DIR/${FEATURE_NAME}.ascii"
MP4_FILE="$RECORDINGS_DIR/${FEATURE_NAME}.mp4"

# Verify tape file exists
if [[ ! -f "$TAPE_FILE" ]]; then
  print_error "Tape file not found: $TAPE_FILE"
  echo "Please create the tape file at: $TAPE_FILE"
  exit 1
fi

print_success "Tape file found: $TAPE_FILE"

# ============================================================================
# Prerequisite Checks
# ============================================================================

print_step "Checking prerequisites..."
echo ""

# Check for VHS
if ! command -v vhs >/dev/null 2>&1; then
  print_error "vhs not found"
  echo "  Install with:"
  echo "    macOS: brew install charmbracelet/tap/vhs"
  echo "    Linux: https://github.com/charmbracelet/vhs/releases"
  exit 1
fi
print_success "vhs found: $(vhs --version)"

# Check for solc
if ! command -v solc >/dev/null 2>&1; then
  print_error "solc not found"
  echo "  Install with:"
  echo "    macOS: brew install solidity"
  echo "    Linux: sudo pacman -S solidity"
  exit 1
fi
print_success "solc found: $(solc --version | head -1)"

# Check blockchain (optional - only if tape needs it)
if grep -q "Type.*./target/release/evm-cli" "$TAPE_FILE"; then
  print_step "Checking blockchain connection..."
  if curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
    "$BLOCKCHAIN_RPC" >/dev/null 2>&1; then
    print_success "Blockchain accessible at $BLOCKCHAIN_RPC"
  else
    print_info "Blockchain not accessible (optional for some recordings)"
    print_info "Start with: anvil"
  fi
fi

echo ""

# ============================================================================
# Build Project
# ============================================================================

print_step "Building evm-cli..."
cd "$PROJECT_ROOT"

if cargo build --release 2>&1 | grep -E "(Finished|Compiling|warning)" | tail -5; then
  print_success "Build successful"
else
  print_error "Build failed"
  exit 1
fi

echo ""

# ============================================================================
# Record Tape
# ============================================================================

print_step "Recording tape..."
echo "  Spec: ${SPECS[$SPEC_NAME]}"
echo "  Feature: $FEATURE_NAME"
echo "  Tape File: $TAPE_FILE"
echo "  Output ASCII: $ASCII_FILE"
echo "  Output MP4: $MP4_FILE"
echo ""

# Clean config to start fresh for recording
CONFIG_FILE="$HOME/.evm-cli/config.json"
if [[ -f "$CONFIG_FILE" ]]; then
  print_info "Backing up config: $CONFIG_FILE.bak"
  cp "$CONFIG_FILE" "$CONFIG_FILE.bak"
fi

# Change to recordings directory for VHS output
cd "$RECORDINGS_DIR"

# Run VHS
print_step "Running VHS recording..."
echo ""

if vhs "$TAPE_FILE"; then
  print_success "Recording completed!"
else
  print_error "Recording failed"
  exit 1
fi

echo ""

# ============================================================================
# Verify Outputs
# ============================================================================

print_step "Verifying outputs..."

if [[ -f "$ASCII_FILE" ]]; then
  ASCII_SIZE=$(du -h "$ASCII_FILE" | cut -f1)
  print_success "ASCII file created: $ASCII_FILE ($ASCII_SIZE)"
else
  print_error "ASCII file not found: $ASCII_FILE"
  exit 1
fi

if [[ -f "$MP4_FILE" ]]; then
  MP4_SIZE=$(du -h "$MP4_FILE" | cut -f1)
  print_success "MP4 file created: $MP4_FILE ($MP4_SIZE)"
else
  print_info "MP4 file not created (check tape file for Output directive)"
fi

echo ""

# ============================================================================
# Create Review Instructions
# ============================================================================

print_step "Creating review instructions..."

REVIEW_FILE="$RECORDINGS_DIR/${FEATURE_NAME}.REVIEW.md"
cat > "$REVIEW_FILE" << 'REVIEW_EOF'
# Recording Review Guide

## How to Review This Recording

### 1. View ASCII Recording
The `.ascii` file contains a text-based recording of the terminal session. You can:

**Option A: Direct View**
```bash
cat spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii
```

**Option B: VHS Playback**
```bash
vhs play spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii
```

### 2. Review Tape Script
Check the `.tape` file to understand the recorded interactions:
```bash
cat spec/$SPEC_NAME/recordings/$FEATURE_NAME.tape
```

### 3. Compare to Specification
1. Open `spec/$SPEC_NAME/spec.md`
2. Verify the recording demonstrates all key features from the spec
3. Check that keyboard shortcuts work as documented
4. Verify visual styling matches specification mockups

## Review Checklist

### Visual Elements
- [ ] UI layout matches spec mockups
- [ ] Colors and highlighting correct (cyan selection, status colors)
- [ ] Spacing and alignment follow design system
- [ ] All interactive elements visible and responsive

### Keyboard Navigation
- [ ] Arrow keys work as documented
- [ ] Vim keys (hjkl) work as documented
- [ ] Tab switching works between areas
- [ ] Enter executes actions
- [ ] Escape cancels/closes modals

### Feature-Specific
- [ ] Feature behavior matches spec description
- [ ] Error messages display correctly
- [ ] Status feedback is clear
- [ ] Loading states show properly
- [ ] Transitions are smooth

### Quality
- [ ] Recording is clear and readable
- [ ] No glitches or rendering issues
- [ ] Timing is realistic (not too fast/slow)
- [ ] All steps are visible and understandable

## Issues Found

If you find issues with the recording or implementation:

1. **Recording Issues**: Re-run the recording script
   ```bash
   ./scripts/record-spec-tape.sh $SPEC_NAME $FEATURE_NAME
   ```

2. **Implementation Issues**: File a bug report with:
   - Spec name and feature
   - Description of the issue
   - Steps to reproduce
   - Expected vs actual behavior

## Recording Regeneration

To regenerate this recording:
```bash
./scripts/record-spec-tape.sh $SPEC_NAME $FEATURE_NAME
```

## Related Files
- Spec: `spec/$SPEC_NAME/spec.md`
- Tape: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.tape`
- ASCII: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.ascii`
- Video: `spec/$SPEC_NAME/recordings/$FEATURE_NAME.mp4`

REVIEW_EOF

print_success "Review guide created: $REVIEW_FILE"

echo ""

# ============================================================================
# Summary
# ============================================================================

print_header "Recording Complete!"

echo ""
echo -e "${CYAN}Files Generated:${NC}"
echo "  📝 Tape Script: $TAPE_FILE"
echo "  🎬 ASCII Recording: $ASCII_FILE"
if [[ -f "$MP4_FILE" ]]; then
  echo "  🎥 MP4 Video: $MP4_FILE"
fi
echo "  📋 Review Guide: $REVIEW_FILE"

echo ""
echo -e "${CYAN}Next Steps:${NC}"
echo "  1. Review the recording:"
echo "     vhs play $ASCII_FILE"
echo ""
echo "  2. Compare with specification:"
echo "     cat spec/$SPEC_NAME/spec.md"
echo ""
echo "  3. Check against review guide:"
echo "     cat $REVIEW_FILE"
echo ""
echo "  4. Generate GIF (optional):"
echo "     vhs record $TAPE_FILE --output $RECORDINGS_DIR/${FEATURE_NAME}.gif"

echo ""

# ============================================================================
# Restore Config (if it was backed up)
# ============================================================================

if [[ -f "$CONFIG_FILE.bak" ]]; then
  print_info "Restoring original config..."
  mv "$CONFIG_FILE.bak" "$CONFIG_FILE"
  print_success "Config restored"
fi

echo ""
print_success "All done!"

#!/bin/bash
#
# VHS Integration Test Runner for evm-cli
#
# This script runs all VHS tape files and reports results.
# Each tape tests specific acceptance criteria from the specs.
#
# Prerequisites:
# - VHS installed (https://github.com/charmbracelet/vhs)
# - Foundry installed (forge)
# - Anvil running on localhost:8545 (for connected tests)
# - cargo build --release completed
#
# Usage:
#   ./tests/run-vhs-tests.sh [--skip-anvil] [--only <pattern>]
#
# Options:
#   --skip-anvil  Skip anvil startup (useful if already running)
#   --only <pat>  Only run tapes matching pattern (e.g., "contracts-menu")

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TESTS_DIR="$SCRIPT_DIR/recordings"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse arguments
SKIP_ANVIL=false
ONLY_PATTERN=""
while [[ $# -gt 0 ]]; do
	case $1 in
	--skip-anvil)
		SKIP_ANVIL=true
		shift
		;;
	--only)
		ONLY_PATTERN="$2"
		shift 2
		;;
	*)
		echo "Unknown option: $1"
		exit 1
		;;
	esac
done

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}  evm-cli VHS Integration Tests${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v vhs &>/dev/null; then
	echo -e "${RED}ERROR: vhs is not installed${NC}"
	echo "Install with: go install github.com/charmbracelet/vhs@latest"
	exit 1
fi

if ! command -v forge &>/dev/null; then
	echo -e "${RED}ERROR: forge is not installed${NC}"
	echo "Install Foundry: curl -L https://foundry.paradigm.xyz | bash"
	exit 1
fi

# Build the project
echo -e "${YELLOW}Building evm-cli...${NC}"
cd "$PROJECT_DIR"
cargo build --release --quiet
echo -e "${GREEN}Build complete${NC}"

# Start anvil if needed
ANVIL_PID=""
if [ "$SKIP_ANVIL" = false ]; then
	echo -e "${YELLOW}Starting anvil...${NC}"
	# Kill any existing anvil
	pkill anvil 2>/dev/null || true
	sleep 1

	# Start anvil in background
	anvil --silent &
	ANVIL_PID=$!
	sleep 2

	if ps -p $ANVIL_PID >/dev/null; then
		echo -e "${GREEN}Anvil started (PID: $ANVIL_PID)${NC}"
	else
		echo -e "${RED}ERROR: Failed to start anvil${NC}"
		exit 1
	fi
fi

# Cleanup function
cleanup() {
	if [ -n "$ANVIL_PID" ]; then
		echo -e "${YELLOW}Stopping anvil...${NC}"
		kill $ANVIL_PID 2>/dev/null || true
	fi
}
trap cleanup EXIT

# Run tests
echo ""
echo -e "${YELLOW}Running VHS tests...${NC}"
echo ""

PASSED=0
FAILED=0
SKIPPED=0

# Find all tape files (excluding common/)
for tape in $(find "$TESTS_DIR" -name "*.tape" -not -path "*/common/*" | sort); do
	# Skip if doesn't match pattern
	if [ -n "$ONLY_PATTERN" ]; then
		if [[ ! "$tape" =~ $ONLY_PATTERN ]]; then
			((SKIPPED++))
			continue
		fi
	fi

	# Get relative path for display
	relative_path="${tape#$TESTS_DIR/}"

	echo -n "  Testing: $relative_path ... "

	# Run VHS and capture output
	if vhs "$tape" 2>&1 >/tmp/vhs-output.log; then
		echo -e "${GREEN}PASSED${NC}"
		((PASSED++))
	else
		echo -e "${RED}FAILED${NC}"
		((FAILED++))
		echo "    Output:"
		cat /tmp/vhs-output.log | sed 's/^/      /'
	fi
done

# Summary
echo ""
echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}  Test Summary${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""
echo -e "  ${GREEN}Passed:${NC}  $PASSED"
echo -e "  ${RED}Failed:${NC}  $FAILED"
if [ $SKIPPED -gt 0 ]; then
	echo -e "  ${YELLOW}Skipped:${NC} $SKIPPED"
fi
echo ""

if [ $FAILED -gt 0 ]; then
	echo -e "${RED}Some tests failed!${NC}"
	exit 1
else
	echo -e "${GREEN}All tests passed!${NC}"
	exit 0
fi

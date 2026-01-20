#!/bin/bash
# Test runner for evm-cli integration tests using VHS
# Run this script to execute all integration tests

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TESTS_DIR="$PROJECT_ROOT/tests"
BLOCKCHAIN_RPC="http://localhost:8545"
BUILD_DIR="$PROJECT_ROOT/target/release"
BINARY="$BUILD_DIR/evm-cli"

# Track results
PASSED=0
FAILED=0
TESTS=()

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}evm-cli Integration Test Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    if [ $status -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $message"
    else
        echo -e "${RED}✗${NC} $message"
    fi
}

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"
echo ""

if ! command_exists vhs; then
    echo -e "${RED}✗${NC} vhs not found"
    echo "  Install with: brew install charmbracelet/tap/vhs"
    exit 1
fi
print_status 0 "vhs found"

if ! command_exists solc; then
    echo -e "${RED}✗${NC} solc not found"
    echo "  Install with: sudo pacman -S solidity"
    exit 1
fi
print_status 0 "solc found"

# Check blockchain connectivity
if ! curl -s -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
    "$BLOCKCHAIN_RPC" >/dev/null 2>&1; then
    echo -e "${RED}✗${NC} Blockchain not accessible at $BLOCKCHAIN_RPC"
    echo "  Start anvil with: anvil"
    exit 1
fi
print_status 0 "Blockchain accessible"

echo ""

# Build project
echo -e "${YELLOW}Building project...${NC}"
cd "$PROJECT_ROOT"
if ! cargo build --release 2>&1 | tail -5; then
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi
print_status 0 "Build successful"
echo ""

# Run tests
echo -e "${YELLOW}Running integration tests...${NC}"
echo ""

cd "$TESTS_DIR"

for tape_file in *.tape; do
    # Skip non-test tapes (like demo.tape if in tests dir)
    if [[ ! "$tape_file" =~ ^[0-9]{2}- ]]; then
        continue
    fi

    test_name="${tape_file%.tape}"
    TESTS+=("$test_name")

    echo -n "Running $test_name... "

    # Run the test with timeout
    if timeout 60 vhs "$tape_file" >/dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}FAIL${NC}"
        ((FAILED++))
    fi
done

echo ""
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Test Results${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

echo "Total:  ${#TESTS[@]}"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

# Summary
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    echo ""
    exit 0
else
    echo -e "${RED}✗ $FAILED test(s) failed${NC}"
    echo ""
    exit 1
fi

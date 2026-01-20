#!/bin/bash
# VHS Demo Runner for evm-cli
# Records demo as GIF and MP4 using VHS instead of asciinema

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
DEMO_DIR="$PROJECT_ROOT/demo"
TAPE_FILE="$DEMO_DIR/demo.tape"
BLOCKCHAIN_RPC="http://localhost:8545"

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}evm-cli Demo Recording (VHS)${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v vhs >/dev/null 2>&1; then
	echo -e "${RED}✗${NC} vhs not found"
	echo "  Install with: brew install charmbracelet/tap/vhs"
	exit 1
fi
echo -e "${GREEN}✓${NC} vhs found"

if ! command -v solc >/dev/null 2>&1; then
	echo -e "${RED}✗${NC} solc not found"
	echo "  Install with: sudo pacman -S solidity"
	exit 1
fi
echo -e "${GREEN}✓${NC} solc found"

# Check blockchain
if ! curl -s -X POST -H "Content-Type: application/json" \
	--data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
	"$BLOCKCHAIN_RPC" >/dev/null 2>&1; then
	echo -e "${RED}✗${NC} Blockchain not accessible at $BLOCKCHAIN_RPC"
	echo "  Start anvil with: anvil"
	exit 1
fi
echo -e "${GREEN}✓${NC} Blockchain accessible"

echo ""

# Build project
echo -e "${YELLOW}Building project...${NC}"
cd "$PROJECT_ROOT"
if cargo build --release 2>&1 | grep -E "(Finished|Compiling)" | tail -1; then
	echo -e "${GREEN}✓${NC} Build successful"
else
	echo -e "${RED}✗${NC} Build failed"
	exit 1
fi

echo ""

# Record demo
echo -e "${YELLOW}Recording demo...${NC}"
echo "  Tape: $TAPE_FILE"
echo "  Outputs: demo.gif, demo.mp4"
echo ""

rm -f "$HOME/evm-cli/.evm-cli/config.json"
cd "$DEMO_DIR"

if vhs "$TAPE_FILE"; then
	echo ""
	echo -e "${GREEN}✓${NC} Recording successful!"
else
	echo -e "${RED}✗${NC} Recording failed"
	exit 1
fi

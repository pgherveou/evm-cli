# User Story: Load a Contract
#
# As a developer, I want to load a Solidity contract file into evm-cli
# so I can deploy and interact with it.
#
# Acceptance Criteria:
# - AC-CM-1: "Load new contract" appears at top of sidebar
# - AC-CM-9: Contract auto-expands when loaded
# - AC-CM-12: Enter on "Load new contract" opens file picker
# - AC-CM-13: File picker filters for .sol files
# - AC-CM-14: Tab autocompletes path in file picker

Output tests/recordings/user-stories/contract-management/01-load-contract.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s

# Reset state to start fresh
Ctrl+P
Type "Reset"
Enter
Sleep 0.5s
Show

# Step 1: Initial state - "Load new contract" option visible at top
Screenshot tests/recordings/user-stories/contract-management/01-load-01-initial.png

# Step 2: Press Enter to open file picker
Enter
Wait+Screen /Path to .sol file|Load Contract/
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/01-load-02-file-picker.png

# Step 3: Type the path to Demo.sol
Type "demo/Demo.sol"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/01-load-03-path-entered.png

# Step 4: Press Enter - contract loads and auto-expands
Enter
Wait+Screen /Demo/
Wait+Screen /Deploy new/
Sleep 0.5s
Screenshot tests/recordings/user-stories/contract-management/01-load-04-loaded.png

Ctrl+C

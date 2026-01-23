# User Story: Load a Solidity Contract
#
# As a developer, I want to load a Solidity contract file into evm-cli
# so I can deploy and interact with it.

Output tests/recordings/contracts-menu/load-contract.ascii

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

# STORY: Load a contract from file
# Step 1: Initial state - "Load new contract" option visible
Screenshot tests/recordings/contracts-menu/story-load-01-initial.png

# Step 2: Press Enter to open file picker
Enter
Wait+Screen /Path to .sol file|Load Contract/
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-load-02-file-picker.png

# Step 3: Type the path to Demo.sol
Type "demo/Demo.sol"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-load-03-path-entered.png

# Step 4: Press Enter - contract loads and auto-expands
Enter
Wait+Screen /Demo/
Wait+Screen /Deploy new/
Sleep 0.5s
Screenshot tests/recordings/contracts-menu/story-load-04-contract-loaded.png

Ctrl+C

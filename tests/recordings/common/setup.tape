# Common setup for all VHS tests
# This tape is sourced by other tests to establish a clean starting state
#
# Acceptance Criteria Tested:
# - AC-GS-1: Config file created on first run
# - AC-CM-9: Contract auto-expands when loaded

Set Width 1920
Set Height 900

# Start the app from the project directory
Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter

# Wait for app to load
Wait+Screen /Load new contract|Contracts/
Sleep 0.5s

# Clear any existing state via Ctrl+P > Reset
Ctrl+P
Wait+Screen /Commands/
Type "Reset"
Enter
Sleep 0.5s

# Load the demo contract
Enter
Wait+Screen /Path to .sol file|Load Contract/
Type "demo/Demo.sol"
Enter

# Wait for contract to appear and auto-expand
Wait+Screen /Demo/
Wait+Screen /Deploy new instance|Deploy new/
Sleep 0.5s
Show

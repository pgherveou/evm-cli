# Test: Load Rust Contract
# Verifies that Rust contracts can be loaded, built, and remembered with proper metadata

Output tests/recordings/rust-contract/load-rust-contract.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load contract/
Sleep 0.5s

# Reset state to start fresh
Ctrl+P
Type "Reset"
Enter
Sleep 0.5s
Show

# Step 1: Initial state
Screenshot tests/recordings/rust-contract/01-initial.png

# Step 2: Expand "Load new contract..." menu
Enter
Wait+Screen /Solidity/
Sleep 0.3s
Screenshot tests/recordings/rust-contract/02-load-menu-expanded.png

# Step 3: Navigate to "Rust (Cargo project)"
Down
Sleep 0.2s
Screenshot tests/recordings/rust-contract/03-rust-option-selected.png

# Step 4: Press Enter to open file picker for Rust project
Enter
Wait+Screen /Path to/
Sleep 0.3s
Screenshot tests/recordings/rust-contract/04-file-picker.png

# Step 5: Type full path to test Rust project directory
Type "/tmp/my-token"
Sleep 0.3s
Screenshot tests/recordings/rust-contract/05-path-entered.png

# Step 6: Press Enter to select directory and open Rust config popup
Enter
Wait+Screen /Bin Target/
Sleep 0.3s
Screenshot tests/recordings/rust-contract/06-rust-config.png

# Step 7: Accept defaults and build (Enter confirms)
Enter
Wait+Screen /Building|Compiling|MyToken/
Sleep 1s
Screenshot tests/recordings/rust-contract/07-building.png

# Wait for build to complete (may take a while)
Wait+Screen /MyToken/
Sleep 1s
Screenshot tests/recordings/rust-contract/08-contract-loaded.png

# Verify the contract shows with Rust marker
Ctrl+C

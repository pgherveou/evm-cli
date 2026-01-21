# evm-cli Demo - Shared initialization
# Clears state and loads examples/Demo.sol

# Start the app
Type "cd /home/pg/github/evm-cli && ./target/release/evm-cli"
Enter

# Wait for app to load
Wait+Screen /Load new contract|Load Contract/
Sleep 1s

# Enter the ctrl+p menu and select clear state
Ctrl+P
Sleep 0.5s
Down
Enter
Sleep 1s

# Step 1: Load contract - examples/Demo.sol
Enter
Sleep 1s
Type "examples/Demo.sol"
Sleep 0.5s
Enter
Sleep 1s

Set Width 1920
Set Height 900

# Start the app
Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release"
Enter

# Wait for app to load (skip compilation output)
Wait+Screen /Load new contract|Load Contract/
Sleep 0.5s

# Enter the ctrl+p menu and select clear state
Ctrl+P
Sleep 0.5s
Type "Reset"
Enter
Sleep 0.5s

# Step 1: Load contract - examples/Demo.sol
Enter
Type "demo/Demo.sol"
Enter
Sleep 0.5s
Show

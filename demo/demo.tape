# evm-cli Demo Recording
# Shows: Load contract → Deploy → Call functions

Output demo/demo.gif

Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 700
Set FontSize 14
Set Padding 20

# Start the app
Hide
Type "cd /home/pg/github/evm-cli && cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s

# Reset state to start fresh
Ctrl+P
Type "Reset"
Enter
Sleep 0.5s
Show

# === LOAD CONTRACT ===
Sleep 1s

# Open file picker
Enter
Wait+Screen /Path to .sol file/
Sleep 0.3s

# Type path
Type "demo/Demo.sol"
Sleep 0.5s
Enter

# Wait for contract to load and expand
Wait+Screen /Demo/
Wait+Screen /Deploy new instance/
Sleep 1s

# === DEPLOY CONTRACT ===
# Navigate to "Deploy new instance"
Type "j"
Sleep 0.3s

# Press Enter to deploy
Enter
Wait+Screen /_initial/
Sleep 0.3s

# Enter constructor parameter
Type "100"
Sleep 0.5s
Enter

# Wait for deployment
Wait+Screen /0x[0-9a-fA-F]/
Sleep 2s

# === CALL getCount ===
# Navigate down to getCount (first method after address)
Type "j"
Sleep 0.3s

# Call getCount
Enter
Wait+Screen /Result/
Sleep 2s

# === CALL increment ===
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s
Enter
Wait+Screen /increment/
Sleep 2s

# === CALL getCount again to show value changed ===
Type "kk"
Sleep 0.3s
Enter
Wait+Screen /Result/
Sleep 2s

# === SHOW COMMAND PALETTE ===
Ctrl+P
Sleep 1.5s
Escape
Sleep 1s

# End
Ctrl+C

# Test: Load Contract File
# Verifies that evm-cli can load a Solidity contract file

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output 01-load-contract.png

Require solc

# Start the app
Type "clear && ./target/release/evm-cli"
Enter
Sleep 2s

# Should see the file selection prompt
Wait /Choose a file/

# Type the contract path
Type "examples/Demo.sol"
Enter
Sleep 1.5s

# Verify it loaded successfully - should show action menu
Wait /Select an action/

# Verify contract details are shown
Wait /Demo/

Sleep 1s
Type "C-c"

# Test: Call Contract Functions
# Verifies that evm-cli can call both view and state-changing functions

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output 03-call-functions.png

Require solc

# Start the app
Type "clear && ./target/release/evm-cli"
Enter
Sleep 2s

# Load the contract
Wait /Choose a file/
Type "examples/Demo.sol"
Enter
Sleep 1.5s

# Deploy with initial value 50
Wait /Select an action/
Type "Down"
Enter
Sleep 0.5s
Type "50"
Enter
Sleep 2s

# Verify deployment
Wait /Deployment successful/

# Call getCount() view function
Type "Down"
Enter
Sleep 1.5s

# Should see the count value
Wait /50/

# Call increment() state function
Type "Down"
Enter
Sleep 1.5s

# Verify transaction hash is shown
Wait /Transaction/

# Call getCount() again to verify increment
Type "Up"
Enter
Sleep 1.5s

# Should see incremented value
Wait /51/

Sleep 1s
Type "C-c"

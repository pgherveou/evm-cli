# Test: Deploy Contract with Constructor Arguments
# Verifies that evm-cli can deploy a contract with constructor arguments

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output 02-deploy-with-args.png

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

# Select deploy action
Wait /Select an action/
Type "Down"
Enter
Sleep 0.5s

# Enter constructor argument
Type "100"
Enter
Sleep 2s

# Verify deployment was successful
Wait /Deployment successful/

# Verify contract address is shown
Wait /Contract deployed at/

Sleep 1s
Type "C-c"

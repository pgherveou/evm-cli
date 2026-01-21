# evm-cli Demo - Interactive Solidity Contract Deployment
# Replicates the exact sequence from record-demo.sh

# Configuration
Set Shell bash
# Set Theme Molokai
Set Width 1200
Set Height 800
Set TypingSpeed 50ms
# Set FontSize 16
# Set FontFamily "FiraCode Nerd Font"
Set Padding 20
Set Margin 10

Output demo.mp4
Output demo.ascii
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

# Step 2: Deploy contract with constructor argument
# Navigate Down to Deploy option
Down
Sleep 0.3s
Enter
Sleep 0.5s

# Enter constructor argument (42)
Type "42"
Enter
Sleep 2s

# Step 3: Call getCount() view function
# Navigate Down to getCount
Down
Sleep 0.3s
Enter
Sleep 1.5s

# Step 4: Call increment() state-changing function
# Navigate Down to increment
Down
Sleep 0.3s
Enter
Sleep 1.5s

# Step 5: Call getCount() again to verify increment
# Navigate Up back to getCount
Up
Sleep 0.3s
Enter
Sleep 1.5s

# Final pause for visual result
Sleep 2s

# Exit the app
Ctrl+C

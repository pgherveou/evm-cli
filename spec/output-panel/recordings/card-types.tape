# Output Panel: Card Types
# Demonstrates different card types (Transaction, Call, Log)

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output card-types.ascii
Output card-types.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Load a contract
Type "Enter"
Sleep 1s

Type "examples/Demo.sol"
Enter
Sleep 3s

# Switch to output
Type "Tab"
Sleep 0.5s

# Navigate through cards to see different types
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# If we deploy, transaction cards will appear
# Navigate back to contracts to deploy
Type "Tab"
Sleep 0.5s

# Navigate to deploy option
Type "l"
Sleep 0.3s
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# Press Enter on deploy (would show parameter popup)
Type "Enter"
Sleep 1s

# Should see parameter input, cancel for now
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

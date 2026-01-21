# Output Panel: Card Navigation
# Demonstrates navigating through output cards and viewing card details

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output card-navigation.ascii
Output card-navigation.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Load a contract to generate output
Type "Enter"
Sleep 1s

Type "examples/Demo.sol"
Enter
Sleep 3s

# Switch to output area with Tab
Type "Tab"
Sleep 0.5s

# Navigate through cards with j/k
Type "j"
Sleep 0.3s

Type "k"
Sleep 0.3s

# Press Enter or Space to show card actions/menu
Type "Enter"
Sleep 0.5s

# Navigate through action menu with arrow keys (if visible)
Type "Right"
Sleep 0.3s

Type "Left"
Sleep 0.3s

# Close menu with Escape
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

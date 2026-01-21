# Contracts Menu: Tree Navigation
# Demonstrates hierarchical tree structure and navigation

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output tree-navigation.ascii
Output tree-navigation.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Navigate down
Type "j"
Sleep 0.5s

# Navigate up
Type "k"
Sleep 0.5s

# Try to expand (should show that nothing to expand on load option)
Type "l"
Sleep 0.3s

# Navigate down to any existing contracts
Type "j"
Sleep 0.5s

# If there's a collapsed contract, expand it
Type "l"
Sleep 0.5s

# Navigate through expanded items
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# Collapse
Type "h"
Sleep 0.5s

# Exit
Type "C-c"

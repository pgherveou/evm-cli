# Command Palette: Basic Usage
# Demonstrates opening palette, searching, and executing commands

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output palette-basics.ascii
Output palette-basics.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Open command palette with Ctrl+P
Type "C-p"
Sleep 1s

# Navigate down
Type "j"
Sleep 0.3s

# Navigate up
Type "k"
Sleep 0.3s

# Type to search
Type "edit"
Sleep 0.5s

# Clear search with Escape
Type "C-c"
Sleep 0.5s

# Open palette again
Type "C-p"
Sleep 1s

# Navigate to a command
Type "j"
Sleep 0.3s

# Close palette without executing
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

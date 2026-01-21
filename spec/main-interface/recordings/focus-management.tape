# Main Interface: Focus Management
# Demonstrates Tab key switching between sidebar and output area

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output focus-management.ascii
Output focus-management.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Tab switches focus from sidebar (default) to output
Type "Tab"
Sleep 0.5s

# Tab switches back to sidebar
Type "Tab"
Sleep 0.5s

# Show navigation with focus on sidebar
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# Tab to output area
Type "Tab"
Sleep 0.5s

# Navigation in output area (when there are cards)
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# Tab back to sidebar
Type "Tab"
Sleep 0.5s

# Exit
Type "C-c"

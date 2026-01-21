# Output Panel: Card Navigation
# Demonstrates navigating through output cards with j/k keys

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
Sleep 4s

# Cancel the "Load new contract" prompt to avoid dialog
Escape
Sleep 0.5s

# Switch to output area with Tab to see any existing cards/output
Tab
Sleep 0.5s

# Demonstrate card navigation with j (down)
Type "j"
Sleep 0.3s

# Demonstrate navigation with k (up)
Type "k"
Sleep 0.3s

# Navigate down again with j
Type "j"
Sleep 0.3s

# Demonstrate scrolling with Down arrow (alternative to j)
Down
Sleep 0.3s

# Demonstrate scrolling with Up arrow (alternative to k)
Up
Sleep 0.3s

# Show that output area is focused
Type "j"
Sleep 0.5s

# Exit the app
Ctrl+C

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
Sleep 4s

# Cancel the "Load new contract" prompt
Escape
Sleep 0.5s

# Switch to output area
Tab
Sleep 0.5s

# Navigate through output with j (down)
Type "j"
Sleep 0.3s

# Navigate with k (up)
Type "k"
Sleep 0.3s

# Demonstrate scrolling with arrow keys
Down
Sleep 0.3s

Up
Sleep 0.3s

# Show j/k navigation again
Type "j"
Sleep 0.3s

Type "k"
Sleep 0.3s

# Exit
Ctrl+C

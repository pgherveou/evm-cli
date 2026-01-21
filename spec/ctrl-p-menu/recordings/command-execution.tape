# Command Palette: Command Execution
# Demonstrates executing various commands from the palette

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output command-execution.ascii
Output command-execution.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Open command palette
Type "C-p"
Sleep 1s

# Navigate to clear output command
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# Execute command
Type "Enter"
Sleep 1s

# Palette should close
# Open again to show another command
Type "C-p"
Sleep 1s

# Navigate to reset/clear state
Type "Down"
Sleep 0.3s

# Close without executing
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

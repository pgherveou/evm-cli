# Transaction & Call Popup: Parameter Input
# Demonstrates parameter input popup with validation

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output parameter-input.ascii
Output parameter-input.mp4

Require solc

Source spec/shared/load_demo.tape

# Show main interface with sidebar focus
Type "j"
Sleep 0.3s

Type "k"
Sleep 0.3s

# Switch to output area with Tab
Tab
Sleep 0.5s

# Navigate output with Tab to show focus switching
Tab
Sleep 0.5s

# Demo j/k navigation in sidebar
Type "j"
Sleep 0.3s

Type "k"
Sleep 0.3s

# Exit
Ctrl+C

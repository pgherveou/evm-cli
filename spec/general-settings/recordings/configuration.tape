# General Settings: Configuration Management
# Demonstrates config file handling and environment variable overrides

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output configuration.ascii
Output configuration.mp4

Require solc

Source spec/shared/load_demo.tape

# Open command palette to edit config
Type "C-p"
Sleep 1s

# Search for edit config (or navigate to it)
Type "edit"
Sleep 0.5s

# Execute edit config (would open editor)
Type "Enter"
Sleep 1s

# For this demo, we cancel to avoid opening editor
Type "C-c"
Sleep 0.5s

# Exit
Type "C-c"

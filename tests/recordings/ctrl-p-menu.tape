Output tests/recordings/ctrl-p-menu.ascii
Source ./tests/recordings/load_demo.tape

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

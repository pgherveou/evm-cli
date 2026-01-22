Output tests/recordings/general-settings-configuration.ascii
Source ./tests/recordings/load_demo.tape

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

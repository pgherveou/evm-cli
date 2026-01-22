Output tests/recordings/main-interface-focus-management.ascii
Source ./tests/recordings/load_demo.tape

# Tab switches focus from sidebar (default) to output
Tab
Sleep 0.5s

# Tab switches back to sidebar
Tab
Sleep 0.5s

# Show navigation with focus on sidebar
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# Tab to output area
Tab
Sleep 0.5s

# Navigation in output area (when there are cards)
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# Tab back to sidebar
Tab
Sleep 0.5s

# Exit
Ctrl+C

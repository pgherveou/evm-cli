Output tests/recordings/general-settings-keyboard-shortcuts.ascii
Source ./tests/recordings/load_demo.tape

# Demo Tab to switch focus
Tab
Sleep 0.5s

Tab
Sleep 0.5s

# Demo Ctrl+P to open command palette
Ctrl+P
Sleep 1s

# Close palette
Escape
Sleep 0.5s

# Demo navigation with vim keys (j/k/h/l)
Type "j"
Sleep 0.3s

Type "k"
Sleep 0.3s

Type "l"
Sleep 0.3s

Type "h"
Sleep 0.3s

# Demo arrow key navigation
Up
Sleep 0.3s

Down
Sleep 0.3s

# Exit with Ctrl+C
Ctrl+C

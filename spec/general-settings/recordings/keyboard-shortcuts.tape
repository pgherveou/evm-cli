# General Settings: Keyboard Shortcuts
# Demonstrates all major keyboard shortcuts

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output keyboard-shortcuts.ascii
Output keyboard-shortcuts.mp4

Require solc

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# Demo Tab to switch focus
Type "Tab"
Sleep 0.5s

Type "Tab"
Sleep 0.5s

# Demo Ctrl+P to open command palette
Type "C-p"
Sleep 1s

# Close palette
Type "Escape"
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

# Demo Delete key (should delete item, but confirm first)
# Skip actual deletion, just show the key works
Type "Delete"
Sleep 0.5s

# Exit with Ctrl+C
Type "C-c"

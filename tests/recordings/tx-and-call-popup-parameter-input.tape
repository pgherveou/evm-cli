Output tests/recordings/tx-and-call-popup-parameter-input.ascii
Source ./tests/recordings/load_demo.tape

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

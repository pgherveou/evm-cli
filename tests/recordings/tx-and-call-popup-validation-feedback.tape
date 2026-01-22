Output tests/recordings/tx-and-call-popup-validation-feedback.ascii
Source ./tests/recordings/load_demo.tape

# Navigate to deploy to show parameter popup
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# Press Enter on Deploy new instance
Type "Enter"
Sleep 1s

# Type invalid address (too short)
Type "0x123"
Sleep 0.5s

# Backspace to clear and fix
Type "C-u"
Sleep 0.3s

Type "0x742d35Cc6634C0532925a3b844Bc9e7595f7a3"
Sleep 0.3s

# Cancel
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

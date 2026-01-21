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

# Start the app
Type "cargo run --release"
Enter
Sleep 3s

# First, load a contract
Type "Enter"
Sleep 1s

# Type contract path for file picker
Type "examples/Demo.sol"
Enter
Sleep 2s

# Contract should load, navigate to deploy
Type "j"
Sleep 0.3s
Type "j"
Sleep 0.3s

# If we reach "Deploy new instance", press Enter to show parameter popup
Type "Enter"
Sleep 1s

# Popup should appear (if contract has constructor parameters)
# Type in a parameter value
Type "100"
Sleep 0.3s

# Tab to next field
Type "Tab"
Sleep 0.3s

# Type another parameter
Type "0x742d35Cc6634C0532925a3b844Bc9e7595f"
Sleep 0.3s

# Cancel with Escape instead of submitting
Type "Escape"
Sleep 0.5s

# Exit
Type "C-c"

# Test: Deployment Target Selection
#
# Acceptance Criteria Tested:
# - AC-PP-12: EVM/PVM selector shown for deploy operations
# - AC-PP-13: Left/Right switches between targets
# - AC-PP-14: Default target is EVM
# - AC-UN-24: Left/Right switches deployment target (EVM/PVM)

Output tests/recordings/parameter-popup/target-selection.ascii

Source ./tests/recordings/common/setup.tape

Wait+Screen /Demo/

# Navigate to Deploy new instance
Type "j"
Sleep 0.3s

# Open deploy popup
Enter
Wait+Screen /constructor|Target/
Sleep 0.3s

# Verify EVM/PVM selector exists and EVM is default (AC-PP-12, AC-PP-14)
Wait+Screen /EVM/
Screenshot tests/recordings/parameter-popup/target-01-default-evm.png

# Switch to PVM with Right arrow (AC-PP-13, AC-UN-24)
Right
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/target-02-pvm.png

# Switch back to EVM with Left arrow
Left
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/target-03-evm-again.png

# Cancel
Escape
Sleep 0.3s

# Exit
Ctrl+C

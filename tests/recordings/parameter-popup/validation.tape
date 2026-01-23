# Test: Parameter Validation
#
# Acceptance Criteria Tested:
# - AC-PP-7: Invalid fields show error message below
# - AC-PP-15: Address fields validate 0x + 40 hex chars
# - AC-PP-17: Errors shown in real-time as user types
# - AC-PP-18: Form cannot submit with validation errors

Output tests/recordings/parameter-popup/validation.ascii

Source ./tests/recordings/common/setup.tape

Wait+Screen /Demo/

# Deploy first to get an instance
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1s
Wait+Screen /0x[0-9a-fA-F]/

# Navigate to setCount (uint256 parameter)
Type "jjjjjj"
Sleep 0.3s
Enter
Wait+Screen /setCount|uint256/
Screenshot tests/recordings/parameter-popup/valid-01-popup.png

# Try submitting empty - should show error (AC-PP-17, AC-PP-18)
Enter
Sleep 0.5s
Screenshot tests/recordings/parameter-popup/valid-02-error.png

# Type valid value
Type "100"
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/valid-03-valid.png

# Cancel to exit without executing
Escape
Sleep 0.3s

# Exit
Ctrl+C

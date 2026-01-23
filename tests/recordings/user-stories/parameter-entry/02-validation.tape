# User Story: Validate Parameter Input
#
# As a developer, I want to see validation errors when I enter invalid
# parameters so I can correct my input before submitting.
#
# Acceptance Criteria:
# - AC-PP-7: Invalid fields show error message below
# - AC-PP-15: Address fields validate 0x + 40 hex chars
# - AC-PP-16: Numeric fields validate appropriate ranges
# - AC-PP-17: Errors shown in real-time as user types
# - AC-PP-18: Form cannot submit with validation errors

Output tests/recordings/user-stories/parameter-entry/02-validation.ascii

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
Screenshot tests/recordings/user-stories/parameter-entry/02-valid-01-popup.png

# Step 1: Try submitting empty - should show error
Enter
Sleep 0.5s
Screenshot tests/recordings/user-stories/parameter-entry/02-valid-02-empty-error.png

# Step 2: Type invalid value (letters)
Type "abc"
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/02-valid-03-invalid.png

# Step 3: Clear and type valid value
Backspace
Backspace
Backspace
Type "100"
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/02-valid-04-valid.png

# Cancel to exit without executing
Escape
Sleep 0.3s

Ctrl+C

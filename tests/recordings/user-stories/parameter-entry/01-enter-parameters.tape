# User Story: Enter Parameters for Contract Methods
#
# As a developer, I want to enter parameters when calling a contract method
# so I can test different inputs.
#
# Acceptance Criteria:
# - AC-PP-5: Each field shows parameter name and type
# - AC-PP-6: Current field has visible cursor
# - AC-PP-10: Enter submits form
# - AC-PP-11: Escape cancels and closes popup
# - AC-UN-23: Enter submits form

Output tests/recordings/user-stories/parameter-entry/01-enter-parameters.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy contract to get an instance with methods
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Navigate to setCount method (has uint256 parameter)
Type "jjjjjj"
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/01-params-01-select.png

# Step 2: Press Enter to open parameter popup
Enter
Wait+Screen /setCount|uint256/
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/01-params-02-popup.png

# Step 3: Enter a value for _count parameter
Type "100"
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/01-params-03-value.png

# Step 4: Press Escape to cancel (demonstrating cancel)
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/01-params-04-cancelled.png

# Step 5: Open again and submit this time
Enter
Wait+Screen /setCount|uint256/
Type "200"
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/parameter-entry/01-params-05-submitted.png

Ctrl+C

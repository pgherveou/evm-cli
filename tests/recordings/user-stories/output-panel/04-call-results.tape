# User Story: View Call Results
#
# As a developer, I want to call view methods and see the results
# in the output panel.
#
# Acceptance Criteria:
# - AC-OP-3: Call cards show function, parameters, result
# - AC-CM-7: Enter executes the selected action
# - AC-UN-9: Enter executes selected action

Output tests/recordings/user-stories/output-panel/04-call-results.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy a contract first
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "42"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Navigate to a view method (getCount)
Type "jjjj"
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/04-call-01-select-method.png

# Step 2: Press Enter to call the method
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/output-panel/04-call-02-result.png

# Step 3: Call increment (state-changing)
Type "k"
Sleep 0.3s
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/output-panel/04-call-03-increment.png

# Step 4: Call getCount again to see updated value
Type "j"
Sleep 0.3s
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/output-panel/04-call-04-new-value.png

Ctrl+C

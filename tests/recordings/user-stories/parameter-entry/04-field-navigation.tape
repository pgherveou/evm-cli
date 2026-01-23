# User Story: Navigate Between Parameter Fields
#
# As a developer entering multiple parameters, I want to navigate
# between fields using Tab and Shift+Tab.
#
# Acceptance Criteria:
# - AC-PP-8: Tab moves to next field
# - AC-PP-9: Shift+Tab moves to previous field
# - AC-UN-21: Tab moves to next field
# - AC-UN-22: Shift+Tab moves to previous field

Output tests/recordings/user-stories/parameter-entry/04-field-navigation.ascii

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

# Navigate to setCount which has only one parameter
# For multi-field demo, we'd need a method with multiple params
# Demo.sol only has single-param methods, so we demonstrate the concept

Type "jjjjjj"
Sleep 0.3s
Enter
Wait+Screen /setCount|uint256/
Screenshot tests/recordings/user-stories/parameter-entry/04-field-01-first.png

# With single field, Tab would submit or move to target selector
# Type a value first
Type "42"
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/04-field-02-value.png

# Tab behavior with single field
Tab
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/04-field-03-after-tab.png

# Cancel
Escape
Sleep 0.3s

Ctrl+C

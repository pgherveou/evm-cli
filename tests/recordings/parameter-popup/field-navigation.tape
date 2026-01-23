# User Story: Enter Parameters for a Contract Method
#
# As a developer, I want to enter parameters when calling a contract method
# so I can test different inputs.

Output tests/recordings/parameter-popup/field-navigation.ascii

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

# STORY: Call a method with parameters
# Step 1: Navigate to setCount method (has uint256 parameter)
Type "jjjjjj"
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/story-params-01-select-method.png

# Step 2: Press Enter to open parameter popup
Enter
Wait+Screen /setCount|uint256/
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/story-params-02-popup-opened.png

# Step 3: Enter a value for _count parameter
Type "100"
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/story-params-03-value-entered.png

# Step 4: Press Escape to cancel (instead of submitting)
Escape
Sleep 0.3s
Screenshot tests/recordings/parameter-popup/story-params-04-cancelled.png

Ctrl+C

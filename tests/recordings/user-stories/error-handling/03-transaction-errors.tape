# User Story: View Transaction Errors
#
# As a developer, when a transaction fails I want to see clear error
# messages so I can understand what went wrong.
#
# Acceptance Criteria:
# - AC-GS-14: Compilation errors displayed in output panel
# - AC-GS-15: Transaction revert reasons shown when available
# - AC-GS-16: Network errors reported in status bar

Output tests/recordings/user-stories/error-handling/03-transaction-errors.ascii

Source ./tests/recordings/common/setup.tape

Wait+Screen /Demo/

# Deploy an instance
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Contract deployed successfully
Screenshot tests/recordings/user-stories/error-handling/03-error-01-deployed.png

# Step 2: Load fibonacci contract which can revert
Type "kkkkk"
Type "k"
Enter
Wait+Screen /Path to .sol file|Load Contract/
Type "demo/fibonacci.sol"
Enter
Sleep 1s
Wait+Screen /Fibonacci/

# Step 3: Deploy Fibonacci
Type "j"
Enter
Wait+Screen /constructor|Enter Parameters/
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/
Screenshot tests/recordings/user-stories/error-handling/03-error-02-fib-deployed.png

# Step 4: Call fibonacci(0) which should revert
Type "jjj"
Sleep 0.3s
Enter
Wait+Screen /fibonacci|uint32/
Type "0"
Enter
Sleep 1.5s
Screenshot tests/recordings/user-stories/error-handling/03-error-03-reverted.png

Ctrl+C

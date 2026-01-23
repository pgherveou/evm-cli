# User Story: Select Deployment Target (EVM/PVM)
#
# As a developer deploying to a multi-VM chain, I want to choose between
# EVM and PVM targets for my deployment.
#
# Acceptance Criteria:
# - AC-PP-12: EVM/PVM selector shown for deploy operations
# - AC-PP-13: ←/→ switches between targets
# - AC-PP-14: Default target is EVM
# - AC-UN-24: ←/→ switches deployment target (EVM/PVM)
# - AC-CM-16: Parameter popup includes EVM/PVM target selection

Output tests/recordings/user-stories/parameter-entry/03-target-selection.ascii

Source ./tests/recordings/common/setup.tape

Wait+Screen /Demo/

# Navigate to Deploy new instance
Type "j"
Sleep 0.3s

# Step 1: Open deploy popup
Enter
Wait+Screen /constructor|Target/
Sleep 0.3s

# Step 2: Verify EVM/PVM selector exists and EVM is default
Wait+Screen /EVM/
Screenshot tests/recordings/user-stories/parameter-entry/03-target-01-evm-default.png

# Step 3: Switch to PVM with Right arrow
Right
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/03-target-02-pvm.png

# Step 4: Switch back to EVM with Left arrow
Left
Sleep 0.3s
Screenshot tests/recordings/user-stories/parameter-entry/03-target-03-evm-again.png

# Cancel
Escape
Sleep 0.3s

Ctrl+C

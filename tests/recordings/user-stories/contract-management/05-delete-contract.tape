# User Story: Delete a Contract
#
# As a developer, I want to remove an entire contract from the sidebar
# when I no longer need it.
#
# Acceptance Criteria:
# - AC-CM-20: Delete/Backspace removes item immediately (no confirmation)
# - AC-CM-21: Deleting contract removes all its instances
# - AC-CM-22: Deletion persists to config.json

Output tests/recordings/user-stories/contract-management/05-delete-contract.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy an instance first to show that deleting contract removes instances too
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Contract with deployed instance is visible
Screenshot tests/recordings/user-stories/contract-management/05-contract-01-initial.png

# Step 2: Navigate to select the Demo contract (parent node)
Type "kkkkk"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/05-contract-02-selected.png

# Step 3: Press Delete to remove the contract (and all instances)
Delete
Sleep 0.5s
Screenshot tests/recordings/user-stories/contract-management/05-contract-03-deleted.png

# Verify: Contract is gone, only "Load new contract" remains
Wait+Screen /Load new contract/

Ctrl+C

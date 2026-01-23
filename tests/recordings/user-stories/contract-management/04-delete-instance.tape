# User Story: Delete a Deployed Instance
#
# As a developer, I want to remove a deployed instance from the sidebar
# when I no longer need to interact with it.
#
# Acceptance Criteria:
# - AC-CM-20: Delete/Backspace removes item immediately (no confirmation)
# - AC-CM-22: Deletion persists to config.json

Output tests/recordings/user-stories/contract-management/04-delete-instance.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy an instance first so we have something to delete
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Instance is deployed and visible
Screenshot tests/recordings/user-stories/contract-management/04-delete-01-deployed.png

# Step 2: Navigate to select the deployed instance (the address line)
Type "jjjj"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/04-delete-02-selected.png

# Step 3: Press Backspace to delete the instance (immediate, no confirmation)
Backspace
Sleep 0.5s
Screenshot tests/recordings/user-stories/contract-management/04-delete-03-deleted.png

# Verify: Instance is removed, contract still exists with deploy/load options
Wait+Screen /Deploy new instance/

Ctrl+C

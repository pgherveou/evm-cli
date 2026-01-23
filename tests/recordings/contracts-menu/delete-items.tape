# User Story: Delete Contracts and Instances
#
# As a developer, I want to remove deployed instances or contracts
# from the sidebar when I no longer need them.

Output tests/recordings/contracts-menu/delete-items.ascii

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

# STORY: Delete a deployed instance
# Step 1: Instance is deployed and visible
Screenshot tests/recordings/contracts-menu/story-delete-01-instance-deployed.png

# Step 2: Navigate down to select the deployed instance
Type "jjjj"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-delete-02-instance-selected.png

# Step 3: Press Backspace to delete the instance (immediate, no confirmation)
Backspace
Sleep 0.5s
Screenshot tests/recordings/contracts-menu/story-delete-03-instance-deleted.png

# STORY: Delete the entire contract
# Step 4: Navigate up to select the Demo contract
Type "kk"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-delete-04-contract-selected.png

# Step 5: Press Delete to remove the contract
Delete
Sleep 0.5s
Screenshot tests/recordings/contracts-menu/story-delete-05-contract-deleted.png

Ctrl+C

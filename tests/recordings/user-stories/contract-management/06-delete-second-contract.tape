# User Story: Delete Second Contract
#
# As a developer with multiple contracts loaded, I want to delete any contract
# and have it fully removed including all its deployed instances.
#
# Acceptance Criteria:
# - AC-CM-20: Delete/Backspace removes item immediately (no confirmation)
# - AC-CM-21: Deleting contract removes all its instances
# - AC-CM-22: Deletion persists to config.json

Output tests/recordings/user-stories/contract-management/06-delete-second-contract.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Load a second contract to reproduce the bug
Wait+Screen /Demo/

# Step 1: First contract (Demo) is loaded and visible
Screenshot tests/recordings/user-stories/contract-management/06-second-01-first-loaded.png

# Step 2: Deploy an instance of the first contract
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/
Screenshot tests/recordings/user-stories/contract-management/06-second-02-first-deployed.png

# Step 3: Navigate to "Load new contract" and load a second contract
Type "kkkkk"
Sleep 0.3s
Type "k"
Sleep 0.3s
Enter
Wait+Screen /Path to .sol file|Load Contract/
Type "demo/fibonacci.sol"
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/contract-management/06-second-03-second-loaded.png

# Step 4: Deploy an instance of the second contract
Wait+Screen /Fibonacci/
Type "j"
Enter
Wait+Screen /constructor|Enter Parameters/
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/
Screenshot tests/recordings/user-stories/contract-management/06-second-04-second-deployed.png

# Step 5: Navigate UP to select the Fibonacci contract (3 ups from address at index 12 to Fibonacci at index 9)
Type "kkk"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/06-second-05-second-selected.png

# Step 6: Press Delete to remove the second contract
# BUG: Deployed address removed but contract remains
Delete
Sleep 0.5s
Screenshot tests/recordings/user-stories/contract-management/06-second-06-after-delete.png

# Step 7: Verify state - check if contract is fully removed or not
Sleep 0.5s
Screenshot tests/recordings/user-stories/contract-management/06-second-07-final-state.png

Ctrl+C

# User Story: Load an Existing Contract Instance
#
# As a developer, I want to load an already-deployed contract by its address
# so I can interact with it without redeploying.
#
# Acceptance Criteria:
# - AC-CM-11: Instance auto-expands when loaded by address
# - AC-CM-18: Enter on "Load existing..." prompts for address
# - AC-CM-19: Valid address adds instance to tree

Output tests/recordings/user-stories/contract-management/03-load-existing.ascii

Source ./tests/recordings/common/setup.tape

# Step 1: Deploy first to get a valid address
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/
Screenshot tests/recordings/user-stories/contract-management/03-existing-01-deployed.png

# Step 2: Navigate to "Load existing instance"
Type "jj"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/03-existing-02-select.png

# Step 3: Press Enter to open address prompt
Enter
Wait+Screen /Enter Address|address|0x/
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/03-existing-03-prompt.png

# Step 4: Enter a valid contract address (use deployed contract address pattern)
Type "0x5FbDB2315678afecb367f032d93F642f64180aa3"
Sleep 0.3s
Screenshot tests/recordings/user-stories/contract-management/03-existing-04-address.png

# Step 5: Press Enter - instance is loaded (appears as second instance)
Enter
Sleep 1s
Screenshot tests/recordings/user-stories/contract-management/03-existing-05-loaded.png

Ctrl+C

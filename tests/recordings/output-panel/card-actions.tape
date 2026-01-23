# User Story: Interact with Transaction Cards
#
# As a developer, after deploying a contract I want to inspect
# the transaction by copying its hash, viewing the receipt, or debugging.

Output tests/recordings/output-panel/card-actions.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy a contract to create a transaction card
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /Transaction|Success/

# STORY: Use card action shortcuts
# Step 1: Transaction card is visible in output panel
Screenshot tests/recordings/output-panel/story-actions-01-tx-card.png

# Step 2: Press 'c' to open copy menu
Type "c"
Sleep 0.5s
Wait+Screen /Copy/
Screenshot tests/recordings/output-panel/story-actions-02-copy-menu.png

# Step 3: Press Escape to close copy menu
Escape
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-actions-03-copy-cancelled.png

# Step 4: Press 'd' to open debug trace menu
Type "d"
Sleep 0.5s
Wait+Screen /Tracer|Call Tracer/
Screenshot tests/recordings/output-panel/story-actions-04-debug-menu.png

# Step 5: Press Escape to close debug menu
Escape
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-actions-05-debug-cancelled.png

Ctrl+C

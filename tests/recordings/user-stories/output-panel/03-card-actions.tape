# User Story: Interact with Transaction Cards
#
# As a developer, after deploying a contract I want to inspect
# the transaction by copying its hash, viewing the receipt, or debugging.
#
# Acceptance Criteria:
# - AC-OP-12: Transaction cards support r (View Receipt)
# - AC-OP-13: Transaction cards support d (Debug Trace)
# - AC-OP-14: Transaction cards support c (Copy)
# - AC-UN-14: r key triggers View Receipt on Transaction cards
# - AC-UN-15: d key triggers Debug Trace/Call
# - AC-UN-16: c key triggers Copy on Transaction cards

Output tests/recordings/user-stories/output-panel/03-card-actions.ascii

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

# Step 1: Transaction card is visible in output panel
Screenshot tests/recordings/user-stories/output-panel/03-actions-01-tx-card.png

# Step 2: Press 'c' to open copy menu
Type "c"
Sleep 0.5s
Wait+Screen /Copy/
Screenshot tests/recordings/user-stories/output-panel/03-actions-02-copy-menu.png

# Step 3: Press Escape to close copy menu
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/03-actions-03-copy-closed.png

# Step 4: Press 'd' to open debug trace menu
Type "d"
Sleep 0.5s
Wait+Screen /Tracer|Call Tracer/
Screenshot tests/recordings/user-stories/output-panel/03-actions-04-debug-menu.png

# Step 5: Press Escape to close debug menu
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/03-actions-05-debug-closed.png

# Step 6: Press 'r' to view receipt
Type "r"
Sleep 0.5s
Screenshot tests/recordings/user-stories/output-panel/03-actions-06-receipt.png

Ctrl+C

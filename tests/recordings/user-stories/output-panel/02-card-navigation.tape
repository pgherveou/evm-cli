# User Story: Navigate Output Cards
#
# As a developer, I want to navigate between output cards
# to review different transactions and call results.
#
# Acceptance Criteria:
# - AC-OP-9: j/k or arrows navigate between cards
# - AC-OP-10: Navigation wraps (last to first)
# - AC-OP-11: Selected card auto-scrolls into view
# - AC-UN-11: j/↓ selects next card
# - AC-UN-12: k/↑ selects previous card
# - AC-UN-13: Navigation wraps at boundaries

Output tests/recordings/user-stories/output-panel/02-card-navigation.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy a contract to create output cards
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /Transaction|Success/

# Step 1: Transaction card is visible (along with Connection card)
Screenshot tests/recordings/user-stories/output-panel/02-cards-01-initial.png

# Step 2: Press Tab to focus output panel
Tab
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/02-cards-02-focused.png

# Step 3: Press 'j' to select next card
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/02-cards-03-next.png

# Step 4: Press 'k' to go back to previous card
Type "k"
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/02-cards-04-prev.png

# Step 5: Navigation wraps - pressing 'j' repeatedly wraps to first card
Type "jjjjj"
Sleep 0.3s
Screenshot tests/recordings/user-stories/output-panel/02-cards-05-wrap.png

Ctrl+C

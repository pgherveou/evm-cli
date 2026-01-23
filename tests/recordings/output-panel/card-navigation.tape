# User Story: Navigate Output Cards
#
# As a developer, I want to navigate between output cards
# to review different transactions and call results.

Output tests/recordings/output-panel/card-navigation.ascii

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

# STORY: Navigate between cards
# Step 1: Transaction card is visible (along with Connection card)
Screenshot tests/recordings/output-panel/story-cards-01-initial.png

# Step 2: Press Tab to focus output panel
Tab
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-cards-02-output-focused.png

# Step 3: Press 'j' to select next card
Type "j"
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-cards-03-next-card.png

# Step 4: Press 'k' to go back to previous card
Type "k"
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-cards-04-prev-card.png

# Step 5: Navigation wraps - pressing 'j' repeatedly wraps to first card
Type "jjjjj"
Sleep 0.3s
Screenshot tests/recordings/output-panel/story-cards-05-wrapped.png

Ctrl+C

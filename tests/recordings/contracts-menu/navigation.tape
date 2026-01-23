# User Story: Navigate the Contract Sidebar
#
# As a developer, I want to navigate, expand, and collapse items
# in the contract sidebar using keyboard shortcuts.

Output tests/recordings/contracts-menu/navigation.ascii

Source ./tests/recordings/common/setup.tape

# STORY: Expand and collapse contracts
# Step 1: Contract is loaded and expanded
Wait+Screen /Deploy new/
Screenshot tests/recordings/contracts-menu/story-nav-01-expanded.png

# Step 2: Press 'h' to collapse the contract
Type "h"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-nav-02-collapsed.png

# Step 3: Press 'l' to expand it again
Type "l"
Sleep 0.3s
Wait+Screen /Deploy new/
Screenshot tests/recordings/contracts-menu/story-nav-03-expanded-again.png

# STORY: Navigate up and down
# Step 4: Press 'j' to move down to "Deploy new instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-nav-04-move-down.png

# Step 5: Press 'j' again to move to "Load existing instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-nav-05-move-down-again.png

# Step 6: Press 'k' to move back up
Type "k"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-nav-06-move-up.png

# STORY: Navigation does not wrap
# Step 7: Go to top and press 'k' - should stay at top
Type "kkkkkk"
Sleep 0.3s
Screenshot tests/recordings/contracts-menu/story-nav-07-at-top.png

Ctrl+C

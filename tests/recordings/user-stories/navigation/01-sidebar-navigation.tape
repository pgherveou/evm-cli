# User Story: Navigate the Contract Sidebar
#
# As a developer, I want to navigate, expand, and collapse items
# in the contract sidebar using keyboard shortcuts.
#
# Acceptance Criteria:
# - AC-CM-5: j/k or arrow keys navigate items
# - AC-CM-6: h/l or arrow keys collapse/expand nodes
# - AC-CM-8: Navigation does NOT wrap at boundaries
# - AC-UN-5: j/↓ moves to next item
# - AC-UN-6: k/↑ moves to previous item
# - AC-UN-7: h/← collapses expanded node
# - AC-UN-8: l/→ expands collapsed node

Output tests/recordings/user-stories/navigation/01-sidebar-navigation.ascii

Source ./tests/recordings/common/setup.tape

# Step 1: Contract is loaded and expanded (default state)
Wait+Screen /Deploy new/
Screenshot tests/recordings/user-stories/navigation/01-sidebar-01-expanded.png

# Step 2: Press 'h' to collapse the contract
Type "h"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/01-sidebar-02-collapsed.png

# Step 3: Press 'l' to expand it again
Type "l"
Sleep 0.3s
Wait+Screen /Deploy new/
Screenshot tests/recordings/user-stories/navigation/01-sidebar-03-expanded.png

# Step 4: Press 'j' to move down to "Deploy new instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/01-sidebar-04-down.png

# Step 5: Press 'j' again to move to "Load existing instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/01-sidebar-05-down-again.png

# Step 6: Press 'k' to move back up
Type "k"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/01-sidebar-06-up.png

# Step 7: Go to top and press 'k' - should stay at top (no wrapping)
Type "kkkkkk"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/01-sidebar-07-at-top.png

Ctrl+C

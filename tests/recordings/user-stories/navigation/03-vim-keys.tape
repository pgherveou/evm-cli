# User Story: Navigate with Vim-Style Keys
#
# As a developer familiar with vim, I want to use j/k/h/l keys
# to navigate the contract tree efficiently.
#
# Acceptance Criteria:
# - AC-UN-5: j/↓ moves to next item
# - AC-UN-6: k/↑ moves to previous item
# - AC-UN-7: h/← collapses expanded node
# - AC-UN-8: l/→ expands collapsed node
# - AC-CM-5: j/k or arrow keys navigate items
# - AC-CM-6: h/l or arrow keys collapse/expand nodes

Output tests/recordings/user-stories/navigation/03-vim-keys.ascii

Source ./tests/recordings/common/setup.tape

# Step 1: Initial state - Demo contract expanded
Wait+Screen /Demo/
Screenshot tests/recordings/user-stories/navigation/03-vim-01-initial.png

# Step 2: Press 'j' to move down to "Deploy new instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/03-vim-02-j.png

# Step 3: Press 'k' to move back up to "Demo"
Type "k"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/03-vim-03-k.png

# Step 4: Press 'h' to collapse the Demo contract
Type "h"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/03-vim-04-h.png

# Step 5: Press 'l' to expand the Demo contract again
Type "l"
Sleep 0.3s
Wait+Screen /Deploy new/
Screenshot tests/recordings/user-stories/navigation/03-vim-05-l.png

Ctrl+C

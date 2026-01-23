# User Story: Navigate with Vim-Style Keys
#
# As a developer familiar with vim, I want to use j/k/h/l keys
# to navigate the contract tree efficiently.

Output tests/recordings/navigation/vim-keys.ascii

Source ./tests/recordings/common/setup.tape

# STORY: Use vim keys to navigate the contract tree
# Step 1: Initial state - Demo contract expanded
Wait+Screen /Demo/
Screenshot tests/recordings/navigation/story-vim-01-initial.png

# Step 2: Press 'j' to move down to "Deploy new instance"
Type "j"
Sleep 0.3s
Screenshot tests/recordings/navigation/story-vim-02-j-down.png

# Step 3: Press 'k' to move back up to "Demo"
Type "k"
Sleep 0.3s
Screenshot tests/recordings/navigation/story-vim-03-k-up.png

# Step 4: Press 'h' to collapse the Demo contract
Type "h"
Sleep 0.3s
Screenshot tests/recordings/navigation/story-vim-04-h-collapse.png

# Step 5: Press 'l' to expand the Demo contract again
Type "l"
Sleep 0.3s
Wait+Screen /Deploy new/
Screenshot tests/recordings/navigation/story-vim-05-l-expand.png

Ctrl+C

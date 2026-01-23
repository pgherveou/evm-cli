# User Story: Navigate Commands in Palette
#
# As a developer, I want to navigate through commands using arrow keys
# or vim-style j/k keys.
#
# Acceptance Criteria:
# - AC-CP-6: Selected command highlighted with cyan background
# - AC-CP-11: j/↓ moves to next command
# - AC-CP-12: k/↑ moves to previous command
# - AC-CP-13: Navigation wraps at boundaries
# - AC-UN-18: j/↓ and k/↑ navigate filtered commands

Output tests/recordings/user-stories/command-palette/02-navigate-commands.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# Step 1: Open command palette
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-01-opened.png

# Step 2: Press Down arrow to move to second command
Down
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-02-down.png

# Step 3: Press 'j' to move down (vim style)
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-03-j.png

# Step 4: Press 'k' to go back up (vim style)
Type "k"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-04-k.png

# Step 5: Press Up arrow to go up
Up
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-05-up.png

# Step 6: Close with Escape (no action taken)
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/02-nav-06-closed.png

Ctrl+C

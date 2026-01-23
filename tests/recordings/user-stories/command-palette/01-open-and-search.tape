# User Story: Open and Search Command Palette
#
# As a developer, I want to open the command palette and filter commands
# by typing search terms so I can quickly find the command I need.
#
# Acceptance Criteria:
# - AC-CP-1: Ctrl+P opens command palette from anywhere
# - AC-CP-2: Search input automatically focused
# - AC-CP-3: First command pre-selected
# - AC-CP-7: Typing filters commands in real-time
# - AC-CP-8: Filter is case-insensitive
# - AC-CP-10: Backspace removes search characters
# - AC-UN-17: Typing filters commands in real-time

Output tests/recordings/user-stories/command-palette/01-open-and-search.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# Step 1: Open palette with Ctrl+P - all commands visible
Ctrl+P
Wait+Screen /Commands/
Wait+Screen /Edit config/
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/01-search-01-all.png

# Step 2: Type "edit" to filter commands
Type "edit"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/01-search-02-edit.png

# Step 3: Clear and type "log" (case insensitive)
Backspace
Backspace
Backspace
Backspace
Type "LOG"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/01-search-03-log.png

# Step 4: Clear search - shows all commands again
Backspace
Backspace
Backspace
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/01-search-04-cleared.png

# Step 5: Close with Escape
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/01-search-05-closed.png

Ctrl+C

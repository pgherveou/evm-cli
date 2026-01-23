# User Story: Toggle Debug Panel
#
# As a developer, I want to toggle the debug panel visibility
# to see or hide debug information.
#
# Acceptance Criteria:
# - AC-CP-5: All commands visible including "Toggle Debug"
# - AC-CP-14: Enter executes selected command

Output tests/recordings/user-stories/command-palette/04-toggle-debug.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# Step 1: Initial state - debug panel visible (if EVM_CLI_DEBUG=1)
Screenshot tests/recordings/user-stories/command-palette/04-debug-01-initial.png

# Step 2: Open palette and search for toggle
Ctrl+P
Wait+Screen /Commands/
Type "toggle"
Sleep 0.3s
Wait+Screen /Toggle Debug/
Screenshot tests/recordings/user-stories/command-palette/04-debug-02-found.png

# Step 3: Execute toggle
Enter
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/04-debug-03-toggled.png

# Step 4: Toggle again to restore
Ctrl+P
Wait+Screen /Commands/
Type "toggle"
Enter
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/04-debug-04-restored.png

Ctrl+C

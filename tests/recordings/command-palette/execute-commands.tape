# User Story: Command Palette Navigation and Execution
#
# As a user, I can open the command palette, navigate commands with arrow keys,
# filter by typing, and execute commands.
#
# Acceptance Criteria:
# - AC-CP-11: Down arrow moves to next command
# - AC-CP-12: Up arrow moves to previous command
# - AC-CP-14: Enter executes selected command
# - AC-CP-15: Escape closes palette without action
# - AC-CP-18: "Clear output" removes all output cards

Output tests/recordings/command-palette/execute-commands.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy a contract to create output cards
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1s
Wait+Screen /Transaction/

# STORY 1: Navigate commands with arrow keys
# Step 1: Open command palette
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/command-palette/story1-01-palette-opened.png

# Step 2: Press Down arrow to move to second command
Down
Sleep 0.3s
Screenshot tests/recordings/command-palette/story1-02-arrow-down.png

# Step 3: Press Down again to move to third command
Down
Sleep 0.3s
Screenshot tests/recordings/command-palette/story1-03-arrow-down-again.png

# Step 4: Press Up arrow to go back
Up
Sleep 0.3s
Screenshot tests/recordings/command-palette/story1-04-arrow-up.png

# Step 5: Close with Escape (no action taken)
Escape
Sleep 0.3s
Screenshot tests/recordings/command-palette/story1-05-escaped.png

# STORY 2: Filter and execute a command
# Step 1: Open palette and type to filter
Ctrl+P
Wait+Screen /Commands/
Type "clear out"
Sleep 0.3s
Screenshot tests/recordings/command-palette/story2-01-filtered.png

# Step 2: Execute the filtered command
Enter
Sleep 0.5s
Screenshot tests/recordings/command-palette/story2-02-output-cleared.png

# STORY 3: Toggle Debug panel
# Step 1: Open palette and search for toggle
Ctrl+P
Wait+Screen /Commands/
Type "toggle"
Sleep 0.3s
Wait+Screen /Toggle Debug/
Screenshot tests/recordings/command-palette/story3-01-toggle-found.png

# Step 2: Execute toggle (debug panel will disappear since it was ON)
Enter
Sleep 0.3s
Screenshot tests/recordings/command-palette/story3-02-debug-toggled.png

Ctrl+C

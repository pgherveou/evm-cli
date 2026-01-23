# User Story: Execute Commands from Palette
#
# As a developer, I want to execute commands like "Clear output",
# "Edit config", and "Reset" from the command palette.
#
# Acceptance Criteria:
# - AC-CP-14: Enter executes selected command
# - AC-CP-15: Escape closes palette without action
# - AC-CP-16: Focus returns to previous panel after close
# - AC-CP-18: "Clear output" removes all output cards
# - AC-CP-20: "Reset" clears all deployments from config
# - AC-UN-19: Enter executes selected command

Output tests/recordings/user-stories/command-palette/03-execute-command.ascii

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

# Step 1: Output panel has transaction card
Screenshot tests/recordings/user-stories/command-palette/03-exec-01-has-cards.png

# Step 2: Open palette and search for "clear"
Ctrl+P
Wait+Screen /Commands/
Type "clear out"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/03-exec-02-filtered.png

# Step 3: Execute the command with Enter
Enter
Sleep 0.5s
Screenshot tests/recordings/user-stories/command-palette/03-exec-03-cleared.png

# Step 4: Verify focus returned to previous panel
Screenshot tests/recordings/user-stories/command-palette/03-exec-04-focus.png

Ctrl+C

# User Story: Reset Application State
#
# As a developer, I want to reset the application state to clear
# all deployed contracts and start fresh.
#
# Acceptance Criteria:
# - AC-CP-20: "Reset" clears all deployments from config
# - AC-GS-8: Reset command clears deployments only
# - AC-GS-9: RPC and account settings preserved after reset

Output tests/recordings/user-stories/command-palette/05-reset-state.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Have contracts and deployments
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /0x[0-9a-fA-F]/

# Step 1: Contract is deployed
Screenshot tests/recordings/user-stories/command-palette/05-reset-01-deployed.png

# Step 2: Open palette and find Reset
Ctrl+P
Wait+Screen /Commands/
Type "Reset"
Sleep 0.3s
Screenshot tests/recordings/user-stories/command-palette/05-reset-02-found.png

# Step 3: Execute Reset
Enter
Sleep 0.5s
Screenshot tests/recordings/user-stories/command-palette/05-reset-03-after.png

# Step 4: Verify only "Load new contract" remains
Wait+Screen /Load new contract/
Screenshot tests/recordings/user-stories/command-palette/05-reset-04-clean.png

Ctrl+C

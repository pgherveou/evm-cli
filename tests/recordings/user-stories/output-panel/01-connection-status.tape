# User Story: View Connection Status
#
# As a developer, when I launch evm-cli I want to see my connection
# status, account address, and balance.
#
# Acceptance Criteria:
# - AC-OP-1: Connection card appears at startup
# - AC-OP-17: Shows "Connected" or "Disconnected" status
# - AC-OP-18: Shows account address and balance when connected
# - AC-MI-5: Status bar shows connection indicator

Output tests/recordings/user-stories/output-panel/01-connection-status.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 1s
Show

# Step 1: App launched - connection card visible in output panel
Wait+Screen /Connection/
Sleep 0.5s
Screenshot tests/recordings/user-stories/output-panel/01-connection-01-launched.png

# Step 2: Connection details visible - account, balance, chain ID
Wait+Screen /Account/
Wait+Screen /Balance/
Wait+Screen /Chain/
Screenshot tests/recordings/user-stories/output-panel/01-connection-02-details.png

# Step 3: Verify status bar shows connection indicator
Wait+Screen /Connected/
Screenshot tests/recordings/user-stories/output-panel/01-connection-03-status-bar.png

Ctrl+C

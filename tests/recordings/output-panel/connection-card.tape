# User Story: View Connection Status
#
# As a developer, when I launch evm-cli I want to see my connection
# status, account address, and balance.

Output tests/recordings/output-panel/connection-card.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 1s
Show

# STORY: Connection card shows status
# Step 1: App launched - connection card visible in output panel
Wait+Screen /Connection/
Sleep 0.5s
Screenshot tests/recordings/output-panel/story-connection-01-launched.png

# Step 2: Connection details visible - account, balance, chain ID
Wait+Screen /Account/
Wait+Screen /Balance/
Wait+Screen /Chain/
Screenshot tests/recordings/output-panel/story-connection-02-details.png

Ctrl+C

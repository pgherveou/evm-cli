# Test: Reconnect Command
#
# NOTE: Testing true disconnected mode requires stopping anvil which
# is complex in automated tests. This test verifies the Reconnect
# command works when already connected (no-op but tests the pathway).
#
# For AC-GS-10 and AC-OP-17 (disconnected mode), manual testing is required.
#
# Acceptance Criteria Tested:
# - AC-CP-6: Commands include "Reconnect"
# - Reconnect command is available and executable

Output tests/recordings/error-handling/disconnected-mode.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# Verify connected status
Wait+Screen /Connected/
Screenshot tests/recordings/error-handling/reconnect-01-connected.png

# Open command palette and find Reconnect
Ctrl+P
Wait+Screen /Commands/
Type "Reconnect"
Sleep 0.3s
Screenshot tests/recordings/error-handling/reconnect-02-command.png

# Execute Reconnect
Enter
Sleep 1s

# Should still be connected after reconnect
Wait+Screen /Connected/
Screenshot tests/recordings/error-handling/reconnect-03-still-connected.png

# Exit
Ctrl+C

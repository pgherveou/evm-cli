# Test: Small Terminal Warning
#
# NOTE: VHS requires minimum 120x120 pixel dimensions, so we cannot
# truly test the "small terminal" warning with VHS. This test instead
# verifies the app handles minimum viable dimensions gracefully.
#
# For AC-MI-10 (terminal < 80 chars warning), manual testing is required.
#
# Acceptance Criteria Tested:
# - AC-MI-11: Layout re-renders on terminal resize (partial)

Output tests/recordings/error-handling/small-terminal.ascii

# Use constrained but usable dimensions - tests layout at smaller size
Set Width 1280
Set Height 600

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract|Contracts/
Sleep 0.5s
Show

# Verify app renders at constrained size
Screenshot tests/recordings/error-handling/small-01-constrained.png

# Verify navigation still works
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# Verify Ctrl+P works at constrained size
Ctrl+P
Wait+Screen /Commands/
Screenshot tests/recordings/error-handling/small-02-ctrl-p.png
Escape
Sleep 0.3s

# Exit
Ctrl+C

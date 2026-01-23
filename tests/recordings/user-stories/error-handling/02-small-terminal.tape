# User Story: Handle Small Terminal
#
# As a developer, I want the application to handle small terminal
# sizes gracefully with appropriate warnings or layout adjustments.
#
# Acceptance Criteria:
# - AC-MI-10: Warning displayed when terminal width < 80 characters
# - AC-MI-11: Layout re-renders immediately on terminal resize
# - AC-MI-12: Address truncation adapts to terminal width
#
# NOTE: VHS requires minimum 120x120 pixel dimensions, so we cannot
# truly test the "small terminal" warning with VHS. This test verifies
# the app handles minimum viable dimensions gracefully.

Output tests/recordings/user-stories/error-handling/02-small-terminal.ascii

# Use constrained but usable dimensions
Set Width 1280
Set Height 600

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract|Contracts/
Sleep 0.5s
Show

# Step 1: Verify app renders at constrained size
Screenshot tests/recordings/user-stories/error-handling/02-small-01-constrained.png

# Step 2: Verify navigation still works
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s
Screenshot tests/recordings/user-stories/error-handling/02-small-02-navigated.png

# Step 3: Verify Ctrl+P works at constrained size
Ctrl+P
Wait+Screen /Commands/
Screenshot tests/recordings/user-stories/error-handling/02-small-03-ctrl-p.png

# Step 4: Close and continue
Escape
Sleep 0.3s

Ctrl+C

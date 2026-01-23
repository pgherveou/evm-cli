# User Story: Use Global Keyboard Shortcuts
#
# As a developer, I want Ctrl+P to open the command palette from anywhere
# and Escape to close popups/overlays.
#
# Acceptance Criteria:
# - AC-UN-2: Escape closes any open popup/overlay
# - AC-UN-3: Ctrl+P opens command palette from anywhere
# - AC-UN-4: Ctrl+C exits application immediately

Output tests/recordings/user-stories/navigation/04-global-shortcuts.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# Step 1: From sidebar - press Ctrl+P to open command palette
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/04-shortcuts-01-ctrl-p.png

# Step 2: Close palette with Escape
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/04-shortcuts-02-escape.png

# Step 3: Switch to output panel with Tab
Tab
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/04-shortcuts-03-output.png

# Step 4: From output panel - Ctrl+P still works
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/04-shortcuts-04-ctrl-p-output.png

# Step 5: Close palette
Escape
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/04-shortcuts-05-closed.png

Ctrl+C

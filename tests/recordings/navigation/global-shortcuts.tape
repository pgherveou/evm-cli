# User Story: Use Global Keyboard Shortcuts
#
# As a developer, I want Ctrl+P to open the command palette from anywhere
# and Ctrl+C to exit the application.

Output tests/recordings/navigation/global-shortcuts.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# STORY: Ctrl+P works from any panel
# Step 1: From sidebar - press Ctrl+P to open command palette
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/navigation/story-shortcuts-01-ctrl-p-sidebar.png

# Step 2: Close palette and switch to output panel
Escape
Sleep 0.3s
Tab
Sleep 0.3s
Screenshot tests/recordings/navigation/story-shortcuts-02-output-focused.png

# Step 3: From output panel - Ctrl+P still works
Ctrl+P
Wait+Screen /Commands/
Sleep 0.3s
Screenshot tests/recordings/navigation/story-shortcuts-03-ctrl-p-output.png

# Step 4: Close palette
Escape
Sleep 0.3s
Screenshot tests/recordings/navigation/story-shortcuts-04-closed.png

Ctrl+C

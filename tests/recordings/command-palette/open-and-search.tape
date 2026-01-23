# User Story: Command Palette Search and Filter
#
# As a user, I want to open the command palette and filter commands
# by typing search terms so I can quickly find the command I need.

Output tests/recordings/command-palette/open-and-search.ascii

Set Width 1920
Set Height 900

Hide
Type "cd /home/pg/github/evm-cli && EVM_CLI_DEBUG=1 cargo run --release 2>/dev/null"
Enter
Wait+Screen /Load new contract/
Sleep 0.5s
Show

# STORY: Search and filter commands
# Step 1: Open palette with Ctrl+P - all commands visible
Ctrl+P
Wait+Screen /Commands/
Wait+Screen /Edit config/
Sleep 0.3s
Screenshot tests/recordings/command-palette/story-search-01-palette-open.png

# Step 2: Type "edit" to filter commands
Type "edit"
Sleep 0.3s
Screenshot tests/recordings/command-palette/story-search-02-filter-edit.png

# Step 3: Clear and type "log" - matches multiple commands
Backspace
Backspace
Backspace
Backspace
Type "log"
Sleep 0.3s
Screenshot tests/recordings/command-palette/story-search-03-filter-log.png

# Step 4: Clear search - shows all commands again
Backspace
Backspace
Backspace
Sleep 0.3s
Screenshot tests/recordings/command-palette/story-search-04-cleared.png

# Step 5: Close with Escape
Escape
Sleep 0.3s
Screenshot tests/recordings/command-palette/story-search-05-closed.png

Ctrl+C

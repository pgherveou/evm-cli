# User Story: Switch Focus Between Panels
#
# As a developer, I want to switch focus between the sidebar and output panel
# using Tab so I can navigate different parts of the interface.

Output tests/recordings/navigation/focus-switching.ascii

Source ./tests/recordings/common/setup.tape

# SETUP: Deploy a contract so both panels have content
Wait+Screen /Demo/
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1.5s
Wait+Screen /Transaction/

# STORY: Switch focus with Tab
# Step 1: Initial state - focus is on sidebar (cyan border)
Screenshot tests/recordings/navigation/story-focus-01-sidebar-focused.png

# Step 2: Press Tab to move focus to output panel
Tab
Sleep 0.3s
Screenshot tests/recordings/navigation/story-focus-02-output-focused.png

# Step 3: Press Tab again to return to sidebar
Tab
Sleep 0.3s
Screenshot tests/recordings/navigation/story-focus-03-back-to-sidebar.png

# Step 4: Navigate in sidebar with j/k while focused
Type "j"
Sleep 0.3s
Screenshot tests/recordings/navigation/story-focus-04-sidebar-navigate.png

# Step 5: Switch to output and navigate cards
Tab
Sleep 0.3s
Type "j"
Sleep 0.3s
Screenshot tests/recordings/navigation/story-focus-05-output-navigate.png

Ctrl+C

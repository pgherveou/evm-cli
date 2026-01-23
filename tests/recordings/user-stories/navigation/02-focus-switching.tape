# User Story: Switch Focus Between Panels
#
# As a developer, I want to switch focus between the sidebar and output panel
# using Tab so I can navigate different parts of the interface.
#
# Acceptance Criteria:
# - AC-UN-1: Tab switches focus between sidebar and output panel
# - AC-MI-13: Tab key switches focus between sidebar and output panel
# - AC-MI-14: Current focus indicated by cyan highlighting

Output tests/recordings/user-stories/navigation/02-focus-switching.ascii

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

# Step 1: Initial state - focus is on sidebar (cyan border)
Screenshot tests/recordings/user-stories/navigation/02-focus-01-sidebar.png

# Step 2: Press Tab to move focus to output panel
Tab
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/02-focus-02-output.png

# Step 3: Press Tab again to return to sidebar
Tab
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/02-focus-03-sidebar-again.png

# Step 4: Navigate in sidebar with j/k while focused
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/02-focus-04-sidebar-nav.png

# Step 5: Switch to output and navigate cards
Tab
Sleep 0.3s
Type "j"
Sleep 0.3s
Screenshot tests/recordings/user-stories/navigation/02-focus-05-output-nav.png

Ctrl+C

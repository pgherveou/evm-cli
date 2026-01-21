# Main Interface: Layout and Zones
# Demonstrates the three-zone layout with sidebar, output area, and status bar

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output layout-and-zones.ascii
Output layout-and-zones.mp4

Require solc

Source spec/shared/load_demo.tape

# Should see main interface with three zones:
# - Sidebar (left) with contract tree
# - Output area (center/right)
# - Status bar (bottom)
Sleep 1s

# Test navigation to demonstrate layout awareness
Type "j"
Sleep 0.5s
Type "k"
Sleep 0.5s

# Exit
Type "C-c"

Output tests/recordings/main-interface-layout-and-zones.ascii
Source ./tests/recordings/load_demo.tape

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

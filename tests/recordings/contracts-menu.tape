Output tests/recordings/contracts-menu.ascii

Source ./tests/recordings/load_demo.tape

Wait+Screen /Demo/
Sleep 0.5s

# Collapse and expand contract
Type "h"
Sleep 0.5s
Type "l"
Sleep 0.5s

# Deploy new instance
Type "j"
Enter
Wait+Screen /constructor/
Type "1"
Enter
Sleep 1s
Wait+Screen /increment\(\)|getCount\(\)|setCount\(\)/
Sleep 0.5s

# Call a view method
Type "jjj"
Enter
Sleep 1s

# Call a state-changing method with parameters
Type "j"
Enter
Wait+Screen /setCount|uint256/
Type "5"
Enter
Sleep 1s

# Delete deployed instance
Type "kkkk"
Backspace
Sleep 0.5s

# Load existing instance
Enter
Wait+Screen /Enter Address|address|0x/
Type "0x0000000000000000000000000000000000000000"
Enter
Sleep 1s

# Delete contract
Type "kkkkkkkk"
Type "j"
Delete
Sleep 0.5s

# Exit
Type "C-c"

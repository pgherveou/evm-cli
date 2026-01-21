Output card-types.ascii
Source ./spec/shared/load_demo.tape

# Deploy a new instance, make a call and a transaction
Down
Sleep 0.3s
Type "42"
Enter
# Call getCount
Down
Enter
# Tx decrement
Down
Enter

# Switch to output area
Tab
Sleep 0.5s

# Navigate through output with j (down)
Type "j"
Sleep 0.3s

# Navigate with k (up)
Type "k"
Sleep 0.3s

# Demonstrate scrolling with arrow keys
Down
Sleep 0.3s

Up
Sleep 0.3s

# Exit
Ctrl+C

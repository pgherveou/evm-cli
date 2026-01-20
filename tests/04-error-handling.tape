# Test: Error Handling
# Verifies that evm-cli handles errors gracefully (e.g., invalid file)

Set Shell bash
Set Theme Molokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output 04-error-handling.png


# Start the app
Type "clear && ./target/release/evm-cli"
Enter
Sleep 2s

# Should see the file selection prompt
Wait /Choose a file/

# Try to load non-existent file
Type "nonexistent.sol"
Enter
Sleep 1.5s

# Should see an error message (or go back to prompt if it handles gracefully)
Wait /(not found|error|Choose a file)/

Sleep 1s
Type "C-c"

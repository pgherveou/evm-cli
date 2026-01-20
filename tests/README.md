# evm-cli Integration Tests with VHS

This directory contains integration tests for evm-cli using [VHS](https://github.com/charmbracelet/vhs) - a tool for recording terminal sessions as code.

## What is VHS?

VHS allows you to write terminal recordings as **declarative `.tape` files** instead of manual scripts. Each tape file:
- Simulates user interactions (typing, key presses)
- Waits for expected output patterns (assertions)
- Generates GIFs, MP4s, or PNG screenshots
- Runs deterministically in CI/CD pipelines

## Test Structure

```
tests/
├── 01-load-contract.tape          # Test: Load a contract file
├── 02-deploy-with-args.tape       # Test: Deploy with constructor args
├── 03-call-functions.tape         # Test: Call view and state functions
├── 04-error-handling.tape         # Test: Error handling
├── run-tests.sh                    # Test runner script
└── README.md                       # This file
```

## Prerequisites

### Install VHS

```bash
# macOS
brew install charmbracelet/tap/vhs

# Linux (Arch)
yay -S vhs

# Linux (generic, requires Go)
go install github.com/charmbracelet/vhs@latest
```

### Other Requirements

- **Solidity Compiler**: `solc`
- **Anvil**: Running local blockchain at `http://localhost:8545`
- **Rust**: Cargo build tools

```bash
# Start anvil in a separate terminal
anvil
```

## Running Tests

### Run All Tests

```bash
./tests/run-tests.sh
```

This will:
1. Check all prerequisites
2. Build the project
3. Run all tests (`*.tape` files prefixed with numbers)
4. Report results

### Run a Single Test

```bash
cd tests
vhs 01-load-contract.tape
```

### Run Tests with Output

By default, tests generate PNG screenshots. View them:

```bash
ls -la tests/*.png
open tests/01-load-contract.png  # macOS
xdg-open tests/01-load-contract.png  # Linux
```

## Understanding Tape File Syntax

### Configuration (top of file)

```tape
Set Shell bash                  # Which shell to use
Set Theme Monokai              # Color theme
Set Width 1200                 # Terminal width
Set Height 600                 # Terminal height
Set FontSize 14                # Font size in pixels
Set TypingSpeed 50ms           # Delay between keystrokes
Set Padding 20                 # Frame padding
```

### Input Simulation

```tape
Type "command"                 # Type text
Enter                          # Press enter key
Type "Down"                    # Arrow key down
Type "C-c"                     # Ctrl+C
Sleep 2s                       # Pause for 2 seconds
```

### Testing/Assertions

```tape
Wait /expected output/         # Wait for text (regex)
Wait+Screen /pattern/          # Wait for text visible on screen
Wait+Line /pattern/            # Wait for text on specific line
```

## Writing New Tests

1. **Create a `.tape` file** with a descriptive name (e.g., `05-new-feature.tape`)
2. **Start with configuration** - copy from an existing test
3. **Simulate user interactions**:
   ```tape
   Type "command"
   Enter
   Sleep 1s
   ```
4. **Add assertions** - wait for expected output:
   ```tape
   Wait /Expected: something/
   ```
5. **Test it locally**:
   ```bash
   cd tests
   vhs 05-new-feature.tape
   ```

### Example Test Template

```tape
# Test: My Feature
# Description of what this tests

Set Shell bash
Set Theme Monokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output my-feature.png

Require evm-cli

# Start the app
Type "clear && ./target/release/evm-cli"
Enter
Sleep 2s

# Your test steps here
Wait /prompt pattern/
Type "user input"
Enter
Sleep 1s

# Assertions - verify expected output
Wait /expected result/

Sleep 1s
Type "C-c"
```

## VHS Commands Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `Type` | Simulate typing | `Type "hello"` |
| `Enter` | Press enter key | `Enter` |
| `Sleep` | Pause execution | `Sleep 1s` |
| `Wait` | Wait for regex match | `Wait /success/` |
| `Wait+Screen` | Wait for visible text | `Wait+Screen /error/` |
| `Set FontSize` | Terminal font size | `Set FontSize 16` |
| `Set Theme` | Color theme | `Set Theme Monokai` |
| `Output` | Output file(s) | `Output test.gif` |
| `Require` | Check dependencies | `Require solc` |

## Debugging Tests

If a test fails:

1. **Check the PNG output** - Shows what was on screen:
   ```bash
   xdg-open tests/01-load-contract.png
   ```

2. **Increase sleep times** - App might be slow:
   ```tape
   Sleep 3s  # Instead of 1s
   ```

3. **Verify regex patterns** - Use simpler patterns:
   ```tape
   Wait /Demo/  # Generic, catches more
   Wait /Select an action/  # Specific
   ```

4. **Run with verbose output** (if vhs supports it):
   ```bash
   vhs --debug 01-load-contract.tape
   ```

## Continuous Integration

Add to your CI pipeline (GitHub Actions, GitLab CI, etc.):

```yaml
# Example GitHub Actions
- name: Run Integration Tests
  run: |
    # Start anvil in background
    anvil &
    sleep 2

    # Run tests
    ./tests/run-tests.sh
```

## Common Issues

### "vhs: command not found"
- Install VHS (see Prerequisites)
- Ensure `~/.cargo/bin` is in PATH if using `go install`

### "Blockchain not accessible"
- Start anvil: `anvil`
- Check it's running at `http://localhost:8545`

### Test hangs or times out
- Increase `Sleep` durations
- Simplify regex patterns in `Wait` commands
- Check app logs for errors

### PNG screenshots look wrong
- Adjust `Set Width` and `Set Height`
- Change font settings if text overlaps
- Use different `Set Theme`

## Resources

- [VHS GitHub](https://github.com/charmbracelet/vhs)
- [VHS Documentation](https://github.com/charmbracelet/vhs#documentation)
- [VHS Examples](https://github.com/charmbracelet/vhs/tree/main/examples)

## Contributing Tests

When adding new features to evm-cli:

1. Create a corresponding `.tape` file
2. Test it locally with: `vhs test-name.tape`
3. Verify PNG output looks correct
4. Add to version control
5. Run full suite: `./run-tests.sh`

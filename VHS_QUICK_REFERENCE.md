# VHS Quick Reference

Quick command reference for VHS tape files.

## Installation

```bash
brew install charmbracelet/tap/vhs      # macOS
yay -S vhs                              # Arch
go install github.com/charmbracelet/vhs@latest  # Generic
```

## Essential Commands

### Configuration
```tape
Set Shell bash                  # Shell to use
Set Theme Monokai              # Color theme
Set Width 1200                 # Width pixels
Set Height 600                 # Height pixels
Set FontSize 14                # Font size
Set TypingSpeed 50ms           # Keystroke delay
Set Padding 20                 # Frame padding
```

### Input
```tape
Type "text"                     # Type characters
Enter                           # Press Enter
Type "Up"                       # Arrow key up
Type "Down"                     # Arrow key down
Type "C-c"                      # Ctrl+C
Type "C-d"                      # Ctrl+D
Backspace                       # Delete character
Tab                             # Tab key
Space                           # Space key
```

### Timing & Output
```tape
Sleep 1s                        # Pause 1 second
Sleep 500ms                     # Pause 500ms
Screenshot                      # Take screenshot
Wait /pattern/                  # Wait for output (regex)
Wait+Screen /pattern/           # Wait for visible output
Wait+Line /pattern/             # Wait for line match
```

### Metadata
```tape
Output demo.gif                 # Generate GIF
Output demo.mp4                 # Generate MP4
Require program                 # Check dependency exists
Source other.tape               # Include another tape
```

## Common Patterns

### Simple Test
```tape
Type "command"
Enter
Sleep 1s
Wait /expected result/
```

### Multi-step Flow
```tape
Type "step 1"
Enter
Wait /step 1 done/

Type "step 2"
Enter
Wait /step 2 done/
```

### Number Verification
```tape
Wait /count: 42/                # Exact number
Wait /count: [0-9]+/            # Any number
```

### Error Handling
```tape
Type "invalid"
Enter
Wait /(error|failed)/           # Catch errors
```

## Running

```bash
# Run single tape
vhs my-test.tape

# Run all tests
./tests/run-tests.sh

# Record new tape
vhs record
```

## Files

- **Demo**: `demo/demo.tape`
- **Tests**: `tests/*.tape`
- **Guide**: `VHS_INTEGRATION_GUIDE.md` (detailed)
- **Runner**: `tests/run-tests.sh`

## Key Differences from asciinema

| Feature | VHS | asciinema |
|---------|-----|-----------|
| Input | Declarative `.tape` | Bash scripts/recording |
| Testing | Built-in `Wait` | No assertions |
| Setup | Simple script | Requires tmux |
| Output | GIF, MP4, PNG | `.cast` file |
| CI/CD | Native support | Requires wrapper |

## Testing Syntax

```tape
Wait /text/              # Regex match anywhere
Wait+Screen /text/       # Must be visible
Wait+Line /text/         # Must be on a line
```

Examples:
```tape
Wait /success/           # Any output with "success"
Wait /Tx: 0x[0-9a-f]+/  # Transaction hash
Wait /error|failed/      # Error OR failed
```

## Tips

- Use `Sleep` generously (1-2 seconds between steps)
- Keep regex patterns simple
- Name tests descriptively: `01-feature.tape`
- Add comments explaining test purpose
- Check PNG output if test fails
- Run `./tests/run-tests.sh` before committing

## Debugging

```bash
# View test screenshot
xdg-open tests/01-test.png

# Increase sleep times
Sleep 2s

# Simplify patterns
Wait /error/  # Instead of complex regex

# Check dependencies
which vhs solc anvil
```

## Next Steps

1. Record demo: `./demo/run-demo-vhs.sh`
2. Run tests: `./tests/run-tests.sh`
3. Write new test: Copy template, add steps
4. Debug: Check PNG, adjust timing/patterns

See `VHS_INTEGRATION_GUIDE.md` for details.

# VHS Integration Guide for evm-cli

This guide explains how to use VHS for both recording demos and running integration tests for evm-cli.

## Table of Contents

1. [Quick Start](#quick-start)
2. [VHS vs. Previous Approach](#vhs-vs-previous-approach)
3. [Setup & Prerequisites](#setup--prerequisites)
4. [Demo Recording](#demo-recording)
5. [Integration Testing](#integration-testing)
6. [Writing Custom Tests](#writing-custom-tests)
7. [CI/CD Integration](#cicd-integration)

---

## Quick Start

### 1. Install VHS

```bash
# macOS (using Homebrew)
brew install charmbracelet/tap/vhs

# Linux (Arch Linux)
yay -S vhs

# macOS/Linux (using Go)
go install github.com/charmbracelet/vhs@latest
```

### 2. Start Anvil

```bash
# Terminal 1
anvil
```

### 3. Record Demo

```bash
# Terminal 2
cd evm-cli
./demo/run-demo-vhs.sh
```

### 4. Run Tests

```bash
./tests/run-tests.sh
```

---

## VHS vs. Previous Approach

### Previous (asciinema + tmux bash script)

```bash
# ❌ Complex, imperative bash scripting
send_keys() {
    local keys="$1"
    local delay="$2"
    tmux send-keys -t "$DEMO_PANE" "$keys"
    sleep "$delay"
}

send_keys "examples/Demo.sol" 0.5
send_enter 1
# ... 300+ lines of timing-dependent code
```

**Problems:**
- Brittle timing dependencies
- Hard to read and maintain
- Not directly testable
- Tmux required
- Can't verify output programmatically

### New (VHS + declarative .tape files)

```tape
# ✓ Declarative, easy to read
Type "examples/Demo.sol"
Enter
Sleep 1.5s
Wait /Select an action/     # Assert expected output
```

**Benefits:**
- Declarative syntax (what, not how)
- Built-in output verification
- No tmux required
- Deterministic execution
- Perfect for CI/CD
- Generates GIFs, MP4s, PNGs
- Easy to debug visually

---

## Setup & Prerequisites

### System Requirements

```bash
# Install VHS
brew install charmbracelet/tap/vhs  # macOS
yay -S vhs                           # Arch Linux

# Install Solidity compiler
brew install solidity               # macOS
sudo pacman -S solidity             # Arch Linux

# Ensure Rust is installed
rustup update
```

### Running Anvil

All tests require a local blockchain at `http://localhost:8545`:

```bash
# Terminal 1 - Start anvil
anvil

# Default config:
# - Listens on 127.0.0.1:8545
# - Unlimited gas
# - Instant block production
```

---

## Demo Recording

### Files

- **Input**: `demo/demo.tape` - VHS tape file defining the demo
- **Outputs**:
  - `demo/demo.gif` - Animated GIF
  - `demo/demo.mp4` - MP4 video
- **Runner**: `demo/run-demo-vhs.sh` - Build and record script

### Record the Demo

```bash
./demo/run-demo-vhs.sh
```

This:
1. Checks prerequisites (vhs, solc, anvil)
2. Builds the project
3. Runs the demo.tape file
4. Generates demo.gif and demo.mp4

### View the Recording

```bash
open demo/demo.gif        # macOS
xdg-open demo/demo.gif    # Linux
```

### Customize the Demo

Edit `demo/demo.tape`:

```tape
Set Width 1200          # Terminal width
Set Height 800          # Terminal height
Set FontSize 16         # Font size in pixels
Set TypingSpeed 50ms    # Delay between keystrokes
Set Theme Monokai       # Color theme

Output demo.gif         # GIF output
Output demo.mp4         # MP4 output

# Your demo steps here
Type "command"
Enter
Sleep 1s
```

---

## Integration Testing

### Test Structure

```
tests/
├── 01-load-contract.tape       # Load contract file
├── 02-deploy-with-args.tape    # Deploy with constructor args
├── 03-call-functions.tape      # Call view and state functions
├── 04-error-handling.tape      # Error handling
├── run-tests.sh                # Test runner
└── README.md                   # Test documentation
```

### Run All Tests

```bash
./tests/run-tests.sh
```

Output:
```
================================
evm-cli Integration Test Suite
================================

Checking prerequisites...
✓ vhs found
✓ solc found
✓ Blockchain accessible

Building project...
✓ Build successful

Running integration tests...

Running 01-load-contract... PASS
Running 02-deploy-with-args... PASS
Running 03-call-functions... PASS
Running 04-error-handling... PASS

================================
Test Results
================================

Total:  4
Passed: 4
Failed: 0

✓ All tests passed!
```

### Run a Single Test

```bash
cd tests
vhs 01-load-contract.tape
```

### Test Output

Each test generates a PNG screenshot showing the final state:

```bash
# View test outputs
ls -la tests/*.png

# Open a screenshot
xdg-open tests/01-load-contract.png
```

---

## Writing Custom Tests

### Test Template

```tape
# Test: Feature Name
# Description of what this test validates

Set Shell bash
Set Theme Monokai
Set Width 1200
Set Height 600
Set TypingSpeed 50ms
Set FontSize 14

Output my-test.png      # Optional - generates PNG

Require evm-cli         # Check dependency exists

# Start application
Type "clear && ./target/release/evm-cli"
Enter
Sleep 2s

# Step 1: Input
Wait /Choose a file/    # Assert: wait for prompt
Type "examples/Demo.sol"
Enter
Sleep 1s

# Step 2: Verify
Wait /Select an action/ # Assert: verify action menu appeared

# Step 3: More actions
Type "Down"
Enter
Sleep 1s

# Step 4: Final assertion
Wait /Success/          # Assert: operation succeeded

# Cleanup
Sleep 1s
Type "C-c"
```

### Key Concepts

#### 1. Input Simulation

```tape
Type "text"             # Type text character by character
Enter                   # Press Enter key
Type "Down"             # Arrow keys: Up, Down, Left, Right
Type "C-c"              # Ctrl+C
Type "C-d"              # Ctrl+D
Tab                     # Tab key
Backspace               # Backspace key
Space                   # Space key
```

#### 2. Timing

```tape
Sleep 1s                # Pause 1 second
Sleep 500ms             # Pause 500 milliseconds
Sleep 2500ms            # Pause 2.5 seconds
```

#### 3. Assertions (Testing)

```tape
# Wait for text anywhere in output
Wait /pattern/

# Wait for text currently visible on screen
Wait+Screen /pattern/

# Wait for text on a specific line
Wait+Line /pattern/

# Regex examples
Wait /Deployment successful/        # Literal text
Wait /[0-9a-f]+/                    # Hex number
Wait /Transaction.*0x[0-9a-f]+/     # Transaction with hash
```

#### 4. Configuration

```tape
Set Shell bash                  # Shell: bash, zsh, fish
Set Theme Monokai              # Themes: Monokai, Dracula, etc.
Set Width 1200                 # Width in pixels
Set Height 600                 # Height in pixels
Set FontSize 14                # Font size (pixels)
Set FontFamily "Fira Code"     # Font family
Set TypingSpeed 50ms           # Delay per keystroke
Set Padding 20                 # Frame padding (pixels)
Set LineHeight 1.2             # Line spacing multiplier
```

### Testing Patterns

#### Pattern 1: Basic Output Verification

```tape
Type "command"
Enter
Wait /expected output/          # Test passes if output appears
```

#### Pattern 2: Multi-step Workflow

```tape
Type "step1"
Enter
Sleep 1s
Wait /step1 complete/

Type "step2"
Enter
Sleep 1s
Wait /step2 complete/
```

#### Pattern 3: Error Handling

```tape
Type "invalid input"
Enter
Sleep 1s
Wait /(error|failed|invalid)/   # Catch any error message
```

#### Pattern 4: Numeric Verification

```tape
# Verify specific number in output
Wait /count: 42/

# Verify any number
Wait /count: [0-9]+/
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install solc
        run: sudo apt-get install -y solc

      - name: Install VHS
        run: |
          curl -LO https://github.com/charmbracelet/vhs/releases/download/v0.6.0/vhs-linux-x86_64
          chmod +x vhs-linux-x86_64
          sudo mv vhs-linux-x86_64 /usr/local/bin/vhs

      - name: Start Anvil
        run: |
          curl -L https://foundry.paradigm.xyz | bash
          source ~/.bashrc
          anvil &
          sleep 2

      - name: Run Integration Tests
        run: ./tests/run-tests.sh
```

### GitLab CI Example

```yaml
integration_tests:
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y solc
    - curl -L https://github.com/charmbracelet/vhs/releases/download/v0.6.0/vhs-linux-x86_64 -o /usr/local/bin/vhs
    - chmod +x /usr/local/bin/vhs
  script:
    - anvil &
    - sleep 2
    - ./tests/run-tests.sh
```

### Local CI Simulation

```bash
# Simulate CI locally
./tests/run-tests.sh
```

---

## Debugging Tests

### Common Issues

#### 1. Test Hangs

```tape
# ❌ Too fast - app hasn't appeared yet
Type "command"
Wait /prompt/

# ✓ With delay
Type "command"
Enter
Sleep 2s        # Give app time to render
Wait /prompt/
```

#### 2. Flaky Tests

```tape
# ❌ Tight timing
Sleep 0.5s

# ✓ More conservative
Sleep 1.5s
```

#### 3. Pattern Not Matching

```tape
# ❌ Too specific
Wait /Deployment successful at contract address 0x.../

# ✓ More flexible
Wait /Deployment successful/
Wait /0x[0-9a-f]+/
```

#### 4. Screenshot Looks Wrong

```tape
# Adjust terminal size
Set Width 1200      # Wider
Set Height 600      # Taller

# Adjust font
Set FontSize 14     # Larger/smaller
Set FontFamily "monospace"  # Different font
```

### Debugging Workflow

1. **Look at PNG output**:
   ```bash
   xdg-open tests/01-load-contract.png
   ```

2. **Increase sleep times**:
   ```tape
   Sleep 2s  # Test if timing is the issue
   ```

3. **Simplify regex patterns**:
   ```tape
   Wait /error/  # Generic, catches more
   ```

4. **Run with verbose logging** (if available):
   ```bash
   vhs --verbose tests/01-load-contract.tape
   ```

---

## File Structure

```
evm-cli/
├── demo/
│   ├── demo.tape              # VHS demo file
│   ├── run-demo-vhs.sh       # Demo runner script
│   ├── demo.gif              # Generated demo GIF
│   ├── demo.mp4              # Generated demo MP4
│   ├── demo.cast             # (old) Asciinema recording
│   └── record-demo.sh        # (old) Bash-based recorder
│
├── tests/
│   ├── 01-load-contract.tape
│   ├── 02-deploy-with-args.tape
│   ├── 03-call-functions.tape
│   ├── 04-error-handling.tape
│   ├── run-tests.sh           # Test runner
│   └── README.md              # Test documentation
│
├── VHS_INTEGRATION_GUIDE.md   # This file
├── Cargo.toml
└── ...
```

---

## Best Practices

1. **Keep tests focused** - One test, one feature
2. **Use descriptive names** - `02-deploy-with-args.tape` not `test2.tape`
3. **Add comments** - Explain what's being tested
4. **Reuse common patterns** - Use a template for new tests
5. **Verify in CI** - Always test changes in CI pipeline
6. **Generate screenshots** - Helps debug visually
7. **Use realistic delays** - Don't make tests too fast
8. **Clear assertions** - Use specific Wait patterns

---

## Resources

- [VHS GitHub Repository](https://github.com/charmbracelet/vhs)
- [VHS Documentation](https://github.com/charmbracelet/vhs/blob/main/README.md)
- [VHS Examples](https://github.com/charmbracelet/vhs/tree/main/examples)
- [Anvil Documentation](https://book.getfoundry.sh/anvil/)

---

## Migration from asciinema

If you want to migrate existing asciinema recordings:

1. Record with VHS: `vhs record` (creates `.tape` file)
2. Edit the generated tape file for testing (add `Wait` commands)
3. Run: `vhs filename.tape`

Or write `.tape` files from scratch using the templates in `tests/`.

---

## Next Steps

1. ✅ Install VHS
2. ✅ Start Anvil
3. ✅ Run the demo: `./demo/run-demo-vhs.sh`
4. ✅ Run the tests: `./tests/run-tests.sh`
5. ✅ Write a custom test for your new feature
6. ✅ Add to CI/CD pipeline

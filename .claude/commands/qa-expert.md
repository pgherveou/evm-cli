# QA Expert - VHS Recording Guidelines

The QA Expert skill creates VHS recordings of specification demonstrations. This document defines what constitutes a valid recording.

## Valid Recording Format

### ASCII Recording Structure

A valid `.ascii` file should contain:

1. **Shell prompt and command execution**
   ```
   > cargo run --release
   [startup output/initialization]
   ```

2. **Application TUI rendering**
   - The full terminal UI should be visible
   - Layout zones (sidebar, output, status bar) should be clearly rendered
   - Components should show proper styling and structure

3. **User interactions captured**
   - Keyboard input commands
   - Navigation sequences
   - Menu selections
   - State changes

4. **Output showing feature behavior**
   - Feature being demonstrated works as expected
   - Visual feedback is present
   - Interactions produce expected results

### Example: Valid Recording (demo.ascii)

```
> cargo run --release
   Compiling evm-cli v0.1.0 (/home/pg/github/evm-cli)
    Finished `release` profile [optimized] target(s) in ...
     Running `target/release/evm-cli`

[Blank lines representing application startup]

┌ Contracts ───────────┐┌ Output ─────────────────────────────────────┐
│ + Load new contract  ││ ● Connected | Chain: 1 | Account: 0x...     │
│                      ││ Balance: 10000.000000 ETH                    │
└──────────────────────┘└──────────────────────────────────────────────┘
● Connected | Chain: 1 | Account: 0xf39fd6e... | Balance: 10000.000000

[User interaction - pressing 'j' to navigate]

[Output updates showing result of interaction]

[More interactions and state changes]
```

### Invalid Recording Indicators

A recording is **INVALID** if it contains:

❌ `bash: ./target/release/evm-cli: No such file or directory`
- Indicates the command failed to execute
- Build was not run before recording
- Use `cargo run --release` instead

❌ File ends immediately after startup without interactions
- No actual feature demonstration
- Application didn't run successfully
- Missing expected UI elements

❌ Empty lines only (no UI rendered)
- Application failed to start
- Terminal capture incomplete
- Build verification needed

❌ Error messages instead of TUI
- Application crashed or errored
- Prerequisites missing (solc, blockchain, etc.)
- Configuration issues

## Recording Prerequisites

Before creating a recording, ensure:

1. ✅ **Build is clean**
   ```bash
   cargo build --release
   ```

2. ✅ **All dependencies available**
   - `solc` installed for contract compilation
   - Blockchain accessible (anvil running for some specs)
   - VHS tool installed and working

3. ✅ **Tape file uses correct command**
   - ❌ Wrong: `Type "clear && ./target/release/evm-cli"`
   - ✅ Correct: `Type "cargo run --release"`
   - Or: `Type "cd /full/path && cargo run --release"`

## Tape File Best Practices

### Timing
- Use `Sleep` commands appropriately between interactions
- Wait for UI to render: minimum `Sleep 1s` after app startup
- Wait between interactions: `Sleep 0.3s-0.5s` for navigation
- Wait for operations: `Sleep 1s-2s` for transactions/async operations

### Commands
```vhs
# ✅ GOOD: Full path, proper timing
Type "cd /home/pg/github/evm-cli && cargo run --release"
Enter
Sleep 2s

# ✅ GOOD: Relative to working directory
Type "cargo run --release"
Enter
Sleep 2s

# ❌ BAD: Pre-built binary that may not exist
Type "./target/release/evm-cli"
Enter

# ❌ BAD: Assumes shell state/aliases
Type "evm-cli"
Enter
```

### Navigation
```vhs
# ✅ GOOD: Clear, deliberate navigation
Type "j"
Sleep 0.3s
Type "k"
Sleep 0.3s

# ❌ BAD: Too fast, might not capture state changes
Type "j"
Type "k"
```

## ASCII File Validation Checklist

When reviewing a recorded ASCII file, verify:

- [ ] Recording starts with a shell prompt (>)
- [ ] Command shows proper execution (not "No such file or directory")
- [ ] Application TUI is visible (box drawing characters, layout zones)
- [ ] Feature being demonstrated is shown
- [ ] User interactions are captured (j/k navigation, menu selections, etc.)
- [ ] Expected output/state changes are present
- [ ] Recording ends cleanly (app closes with Ctrl+C or naturally)
- [ ] No error messages or crashes in output

## Examples by Specification

### Main Interface (layout-and-zones)
✅ Should show:
- Three-zone layout (sidebar, output, status bar)
- Connection status indicator (● Connected)
- Chain info and account displayed
- Layout proportions correct

### Contracts Menu (tree-navigation)
✅ Should show:
- Sidebar with tree structure
- Navigation with j/k keys
- Node indicators (▾, ▸, ◇)
- Selection highlighting

### Output Panel (card-navigation)
✅ Should show:
- Cards displayed in output area
- Selection with cyan background
- Card content visible (hash, status, function, etc.)
- Navigation between cards with j/k

## Recording Recovery

If a recording is invalid:

1. **Verify build succeeded**
   ```bash
   cargo build --release
   ```

2. **Check tape file command**
   - Ensure it uses `cargo run --release`
   - Not a pre-built binary path

3. **Test prerequisites**
   ```bash
   which cargo
   which solc
   command -v vhs
   ```

4. **Re-record the spec**
   ```bash
   ./scripts/record-spec-tape.sh <spec-name> <feature-name>
   ```

5. **Verify ASCII output**
   - Check first 50 lines show proper UI
   - Verify interactions are captured
   - Ensure feature works as expected

## Reference Commands

```bash
# View recorded ASCII file
cat spec/*/recordings/*.ascii | head -100

# Play recording with VHS
vhs play spec/*/recordings/*.ascii

# View with less (preserves formatting)
less -R spec/*/recordings/*.ascii

# Search for specific content
grep -n "Connected" spec/*/recordings/*.ascii

# Check file size (large files = more content captured)
ls -lh spec/*/recordings/*.ascii
```

---

**Last Updated**: January 21, 2026
**Status**: Documentation for QA recording standards and validation

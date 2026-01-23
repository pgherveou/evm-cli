# Command Palette (Ctrl+P) Specification

## Overview

The Command Palette is a centered overlay menu providing quick access to application-wide commands and settings. Accessed via `Ctrl+P`, it features a search input with real-time filtering and categorized command grouping.

## User Interface

### Mockup

```
               ┌ Commands (Ctrl+P) ──────────────────────────┐
               │> █                                          │
               │─────────────────────────────────────────────│
               │> Edit config - Open config file in $EDITOR  │
               │  Clear output - Clear the output area       │
               │  Open Logs - Open application log file      │
               │  Clear Logs - Delete the application log    │
               │  Reconnect - Retry connection to RPC server │
               │  Toggle Debug [OFF] - Toggle debug panel    │
               │  Reset - Clear all saved state              │
               │  Quit - Exit the application [Ctrl+C]       │
               └─────────────────────────────────────────────┘
```

### Layout

- **Title:** "Commands" (top-left)
- **Close hint:** `esc` (top-right)
- **Search input:** Prominent text input field
- **Command groups:** Categorized sections (Suggested, Settings, Help, etc.)
- **Selected item:** Highlighted with cyan background
- **Width:** ~60-80% of terminal width (centered)
- **Height:** Dynamic based on commands shown (max ~80% of terminal height)

### Search Behavior

- **Real-time filtering:** Commands matching search term highlighted
- **Fuzzy matching:** Optional support for fuzzy search (future)
- **Exact matching:** Commands with exact match prioritized
- **Case-insensitive:** Search ignores case
- **Clear search:** Backspace to delete, or Escape clears and closes

**No Results:**
When search filter doesn't match any commands:
- Empty list shown (no commands visible)
- No "No results" message displayed
- User can backspace to modify search or Escape to close

## Available Commands

Commands available in the palette (in order):

| # | Command | Shortcut | Action |
|---|---------|----------|--------|
| 1 | Edit config | (none) | Open `.evm-cli/config.json` in `$EDITOR` |
| 2 | Clear output | (none) | Clear all cards from output panel |
| 3 | Open Logs | (none) | Open `~/.evm-cli/output.log` in `$EDITOR` |
| 4 | Clear Logs | (none) | Delete the application log file |
| 5 | Reconnect | (none) | Retry connection to RPC server |
| 6 | Toggle Debug | (none) | Toggle debug panel visibility (shows key/action/focus info) |
| 7 | Reset | (none) | Clear all deployment addresses from config (keeps RPC/account settings) |
| 8 | Quit | Ctrl+C | Exit the application |


## Command Execution

### Workflow

1. User presses `Ctrl+P` → Palette opens with focus on search field
2. User types to filter commands (optional)
3. User navigates with `↑`/`↓` or `j`/`k`
4. User presses `Enter` to execute selected command
5. Command executes and palette closes

### Execution Context

Commands execute with context based on current application state:

#### Edit Config
- Opens `~/.evm-cli/config.json` in `$EDITOR`
- User can edit and save
- On close: Config reloaded if changed

#### Clear Output
- Removes all cards from the output panel
- Sidebar tree remains unchanged
- Output area returns to empty state

#### Open Logs
- Opens `~/.evm-cli/output.log` in `$EDITOR`
- Displays application debug logs
- Useful for troubleshooting connection issues or errors

#### Reset
- Clears all deployment addresses from `~/.evm-cli/config.json`
- Preserves RPC URL and account settings
- All instances removed from sidebar tree
- No confirmation dialog

#### Quit
- Closes the application
- Equivalent to Ctrl+C

---

## Visual States

### Open State

```
               ┌ Commands (Ctrl+P) ──────────────────────────┐
               │> █                                          │
               │─────────────────────────────────────────────│
               │> Edit config - Open config file in $EDITOR  │
               │  Clear output - Clear the output area       │
               │  Open Logs - Open application log file      │
               │  Reconnect - Retry connection to RPC server │
               │  Reset - Clear all saved state              │
               │  Quit - Exit the application [Ctrl+C]       │
               └─────────────────────────────────────────────┘
```

### With Search Filter

User types "edit":
```
               ┌ Commands (Ctrl+P) ──────────────────────────┐
               │> edit█                                      │
               │─────────────────────────────────────────────│
               │> Edit config - Open config file in $EDITOR  │
               │                                             │
               └─────────────────────────────────────────────┘
```

### Selection Navigation

User presses down arrow:
```
               ┌ Commands (Ctrl+P) ──────────────────────────┐
               │> █                                          │
               │─────────────────────────────────────────────│
               │  Edit config - Open config file in $EDITOR  │
               │> Clear output - Clear the output area       │
               │  Open Logs - Open application log file      │
               │  Reconnect - Retry connection to RPC server │
               │  Reset - Clear all saved state              │
               │  Quit - Exit the application [Ctrl+C]       │
               └─────────────────────────────────────────────┘
```

## Focus Management

### Opening Palette
- Search input automatically focused when palette opens
- Sidebar/output area loses focus temporarily
- All keyboard input directed to palette

### Closing Palette
- `Escape` key closes palette
- `Enter` executes command and closes
- `Tab` alternative close method
- On close: Previous focus (sidebar/output) restored

---

## Acceptance Criteria

### Opening
- **AC-CP-1**: Ctrl+P opens command palette from anywhere
- **AC-CP-2**: Search input automatically focused
- **AC-CP-3**: First command pre-selected

### Display
- **AC-CP-4**: Palette centered on screen
- **AC-CP-5**: All 7 commands visible (Edit config, Clear output, Open Logs, Clear Logs, Reconnect, Reset, Quit)
- **AC-CP-6**: Selected command highlighted with cyan background

### Search
- **AC-CP-7**: Typing filters commands in real-time
- **AC-CP-8**: Filter is case-insensitive
- **AC-CP-9**: Empty results show no commands (blank list)
- **AC-CP-10**: Backspace removes search characters

### Navigation
- **AC-CP-11**: j/↓ moves to next command
- **AC-CP-12**: k/↑ moves to previous command
- **AC-CP-13**: Navigation wraps at boundaries
- **AC-CP-14**: Enter executes selected command

### Closing
- **AC-CP-15**: Escape closes palette without action
- **AC-CP-16**: Focus returns to previous panel after close

### Commands
- **AC-CP-17**: "Edit config" opens config.json in $EDITOR
- **AC-CP-18**: "Clear output" removes all output cards
- **AC-CP-19**: "Open Logs" opens log file in $EDITOR
- **AC-CP-20**: "Reset" clears all deployments from config
- **AC-CP-21**: "Quit" exits application


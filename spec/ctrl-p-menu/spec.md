# Command Palette (Ctrl+P) Specification

## Overview

The Command Palette is a centered overlay menu providing quick access to application-wide commands and settings. Accessed via `Ctrl+P`, it features a search input with real-time filtering and categorized command grouping.

---

## User Interface

### Mockup

```
               ┌ Commands (Ctrl+P) ──────────────────────────┐
               │> █                                          │
               │─────────────────────────────────────────────│
               │> Edit config - Open config file in $EDITOR  │
               │  Reset - Clear all saved state              │
               │  Clear output - Clear the output area       │
               │  Quit - Exit the application [Ctrl+C]       │
               │                                             │
               │                                             │
               │                                             │
               │                                             │
               │                                             │
               │                                             │
               │                                             │
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

---

## Components

### Header
- **Title:** "Commands"
- **Close indicator:** "esc" in top-right corner
- Spanning horizontal rule beneath title

### Search Input

- **Placeholder:** Optional hint text
- **Active state:** Cursor visible, blinking
- **Width:** Full width of palette
- **Real-time filtering:** Commands filtered as user types
- **Case-insensitive:** Matches both upper and lowercase

**Visual:**
```
Search
█
```

### Command Groups

Commands organized into logical categories:

1. **Suggested** - Frequently used or context-appropriate commands
2. **Settings** - Configuration and preferences
3. **Help** - Documentation and shortcuts
4. **(others)** - Additional categories as needed

Each group shows:
- **Group label:** Displayed above items (e.g., "Suggested")
- **Items:** Indented beneath label
- **Separator:** Blank line between groups

### Command Items

Each command item displays:
- **Indicator:** `▌` for selected item, ` ` (space) for others
- **Command name:** Descriptive label
- **Optional shortcut:** Keyboard shortcut (e.g., "Ctrl+S")
- **Optional description:** Brief explanation (future enhancement)

**Visual:**
```
▌Load Contract
 Load Existing Instance
 Clear State
```

---

## Navigation

### Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `k` | Select previous command |
| `↓` / `j` | Select next command |
| `Enter` | Execute selected command |
| `Escape` | Close palette |
| Type any character | Filter commands |
| `Backspace` | Delete search character |
| `Ctrl+U` | Clear search (optional) |
| `Tab` | Close palette (alternative) |

### Search Behavior

- **Real-time filtering:** Commands matching search term highlighted
- **Fuzzy matching:** Optional support for fuzzy search (future)
- **Exact matching:** Commands with exact match prioritized
- **Case-insensitive:** Search ignores case
- **Clear search:** Backspace to delete, or Escape clears and closes

**Example:**
```
User types: "load c"

Suggested
▌Load Contract
 Load Existing Instance

(Settings commands hidden as they don't match)
```

**No Results:**
When search filter doesn't match any commands:
- Empty list shown (no commands visible)
- No "No results" message displayed
- User can backspace to modify search or Escape to close

---

## Available Commands

Commands available in the palette:

| Command | Shortcut | Action | Category |
|---------|----------|--------|----------|
| Load Contract | Ctrl+O | Open file picker with autocomplete to load `.sol` file | Suggested |
| Open Config | (none) | Open `.evm-cli/config.json` in `$EDITOR` | Settings |
| Clear State | (none) | Clear all deployment addresses from config (keeps RPC/account settings) | Settings |
| Clear Output | Ctrl+L | Clear all cards from output panel | Settings |
| Quit | Ctrl+C | Exit the application | Settings |

**Not in Command Palette (Sidebar Only):**
- Deploy New Instance
- Load Existing Instance

These actions appear only in the sidebar tree under each contract.

---

## Command Execution

### Workflow

1. User presses `Ctrl+P` → Palette opens with focus on search field
2. User types to filter commands (optional)
3. User navigates with `↑`/`↓` or `j`/`k`
4. User presses `Enter` to execute selected command
5. Command executes and palette closes

### Execution Context

Commands execute with context based on current application state:

#### Load Contract
- Opens file picker dialog
- Filters for `.sol` files in current directory
- Supports autocomplete/search
- On selection: Contract loaded, compiled, added to sidebar

#### Open Config
- Opens `.evm-cli/config.json` in `$EDITOR`
- User can edit and save
- On close: Config reloaded if changed

#### Clear State
- Clears all deployment addresses from `.evm-cli/config.json`
- Preserves RPC URL and account settings
- All instances removed from sidebar tree
- No confirmation dialog

#### Clear Output
- Removes all cards from the output panel
- Sidebar tree remains unchanged
- Output area returns to empty state

#### Quit
- Saves any pending state to config
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
               │  Reset - Clear all saved state              │
               │  Clear output - Clear the output area       │
               │  Quit - Exit the application [Ctrl+C]       │
               │                                             │
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
               │> Reset - Clear all saved state              │
               │  Clear output - Clear the output area       │
               │  Quit - Exit the application [Ctrl+C]       │
               │                                             │
               └─────────────────────────────────────────────┘
```

---

## Error Handling

### Invalid Command Execution

If a command fails to execute:

**Visual:**
```
Command failed: Connection lost
Retry? (y/n)
```

- Error message shown in palette
- Optional retry prompt
- Palette remains open

### Empty Search Results

If no commands match search term:

**Visual:**
```
                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │ xyz█                                        │
                    │                                             │
                    │ No commands found                           │
                    │                                             │
                    └─────────────────────────────────────────────┘
```

---

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

## Accessibility

- Full keyboard-driven interface (no mouse)
- Clear command descriptions
- Real-time search feedback
- Visual selection indicator
- Organized command grouping
- Tab navigation integration
- Help command for reference

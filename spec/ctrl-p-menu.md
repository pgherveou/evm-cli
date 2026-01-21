# Command Palette (Ctrl+P) Specification

## Overview

The Command Palette is a centered overlay menu providing quick access to application-wide commands and settings. Accessed via `Ctrl+P`, it features a search input with real-time filtering and categorized command grouping.

---

## User Interface

### Mockup

TODO: revise mockup with recording
::<E::T>
```

                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │ █                                           │
                    │                                             │
                    │ Suggested                                   │
                    │ ▌Load Contract                              │ ← selected
                    │  Load Existing Instance                     │
                    │  Clear State                                │
                    │                                             │
                    │ Settings                                    │
                    │  Open Config                                │
                    │  Change RPC URL                             │
                    │  Change Account                             │
                    │                                             │
                    │ Help                                        │
                    │  Show Keyboard Shortcuts                    │
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

---

## Available Commands

### Suggested Commands

These are context-sensitive and frequently used:

| Command | Shortcut | Action | Context |
|---------|----------|--------|---------|
| Load Contract | Ctrl+O | Open file picker to load new `.sol` file | Always available |
| Load Existing Instance | (none) | Prompt for address to load deployed instance | When contract selected |
| Deploy New Instance | (none) | Show deployment parameter form for selected contract | When contract selected |
| Clear State | (none) | Clear all deployments and reset application state | When deployments exist |

### Settings Commands

Configuration and preferences:

| Command | Shortcut | Action |
|---------|----------|--------|
| Open Config | (none) | Open `.evm-cli/config.json` in `$EDITOR` |
| Change RPC URL | (none) | Prompt to change Ethereum RPC endpoint |
| Change Account | (none) | Prompt to change active account/address |
| Change Private Key | (none) | Prompt to change signing key |

### Help Commands

Documentation and reference:

| Command | Shortcut | Action |
|---------|----------|--------|
| Show Keyboard Shortcuts | (none) | Display full keyboard shortcut reference |
| About evm-cli | (none) | Show version and basic info |
| Open Documentation | (none) | Open README in pager |

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

#### Deploy New Instance
- Only available when contract selected
- Shows parameter input popup for constructor args
- On submission: Transaction sent, instance deployed

#### Load Existing Instance
- Only available when contract selected
- Prompts for address input in popup
- On submission: Instance loaded and added to sidebar

#### Clear State
- Only available when deployments exist
- Confirmation: "Clear all deployments? (y/n)"
- On confirmation: All state cleared, sidebar reset

#### Open Config
- Opens `.evm-cli/config.json` in `$EDITOR`
- User can edit and save
- On close: Config reloaded if changed

#### Change RPC URL
- Shows text input popup for new RPC URL
- On submission: RPC endpoint updated
- Validation: URL must be valid HTTP/HTTPS
- On success: Status bar shows new endpoint

#### Change Account
- Shows text input popup for new account address
- On submission: Account updated
- Validation: Address must be valid (0x + 40 hex chars)
- On success: Status bar shows new account and balance

---

## Visual States

### Open State

```
                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │ █                                           │
                    │                                             │
                    │ Suggested                                   │
                    │ ▌Load Contract                              │
                    │  Load Existing Instance                     │
                    │  Clear State                                │
                    │                                             │
                    │ Settings                                    │
                    │  Open Config                                │
                    │  Change RPC URL                             │
                    │                                             │
                    └─────────────────────────────────────────────┘
```

### With Search Filter

User types "load":
```
                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │ load█                                       │
                    │                                             │
                    │ Suggested                                   │
                    │ ▌Load Contract                              │
                    │  Load Existing Instance                     │
                    │                                             │
                    └─────────────────────────────────────────────┘
```

### Selection Navigation

User presses down arrow:
```
                    ┌─────────────────────────────────────────────┐
                    │ Commands                               esc  │
                    │                                             │
                    │ Search                                      │
                    │ load█                                       │
                    │                                             │
                    │ Suggested                                   │
                    │  Load Contract                              │
                    │ ▌Load Existing Instance                     │
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

# Spacing Guidelines

This document defines the spacing system used for margins, padding, and gaps throughout the EVM CLI.

---

## Terminal-Based Spacing System

Since this application runs in terminal environments, spacing is measured in characters (horizontal) and lines (vertical) rather than pixels.

### Horizontal Spacing Units
- **1 space**: Single character width, minimal gap
- **2 spaces**: Small gap, default for indentation
- **4 spaces**: Medium gap, secondary indentation level
- **8+ spaces**: Large gap, major visual separation (rare)

### Vertical Spacing Units
- **0 lines**: No gap (consecutive lines)
- **1 line**: Single blank line, default section separator
- **2 lines**: Significant gap, section boundary
- **3+ lines**: Major visual separation (rare)

---

## Sidebar (Tree Navigation) Spacing

### Vertical Spacing
- **Between items**: 0 lines (consecutive lines)
- **Between groups**: 1 line (blank line between logical groups)
- **Top/bottom padding**: 0-1 line
- **Rationale**: Compact display shows many items

### Horizontal Spacing
- **Per indentation level**: 2 spaces
- **Left margin**: 1 space from sidebar edge
- **Right margin**: 1 space from sidebar edge
- **Total sidebar padding**: 1 space left + right

**Example**:
```
+ Load new contract
▾ Counter.sol
  ◇ Deploy new instance
  ◇ Load existing...
  ▾ 0x1234...
    ├ method1()
    └ method2()
```

### Tree Hierarchy Indentation
- **Level 0**: 0 spaces (e.g., `+ Load new contract`)
- **Level 1**: 0 spaces (e.g., `▾ Counter.sol`)
- **Level 2**: 2 spaces (e.g., `  ◇ Deploy new instance`)
- **Level 3**: 4 spaces (e.g., `    ├ method1()`)
- **Level 4+**: Add 2 spaces per level

**Rationale**: Each indentation level clearly shows hierarchy; max ~4 levels before readability suffers

---

## Output Panel (Cards) Spacing

### Horizontal Spacing
- **Left margin**: 1 space from panel edge
- **Right margin**: 1 space from panel edge
- **Content padding**: 1 space inside card borders
- **Total card padding**: 1 space on all sides

### Vertical Spacing
- **Between cards**: 1 line (blank line between cards)
- **Within card**: 0 lines (consecutive content lines)
- **Card header**: 0-1 lines above first content
- **Card footer actions**: 1 line below content

**Example**:
```
┌─ Transaction: transfer(address,uint256) ┐
│ Status: Success ✓                        │
│ Gas Used: 21000                          │
│ Block: 12345678                          │
│                                          │
│ ◇ View Receipt (r)  ◇ Debug Trace (d)   │
└──────────────────────────────────────────┘

┌─ Call: balanceOf(address) ────────────── ┐
│ Result: 1000000000000000000 Wei          │
│                                          │
│ ◇ Copy Result (c)   ◇ View as JSON (j)  │
└──────────────────────────────────────────┘
```

### Card Internal Structure
- **Label + value pairs**: 0 lines between pairs
- **Section grouping**: 1 line between logical sections
- **Status line**: 1 line above/below if separated

---

## Modals and Popups Spacing

### Outer Spacing (Modal to terminal edge)
- **Horizontal margin**: ~10-20% of terminal width on each side (centers modal)
- **Vertical margin**: Centered vertically
- **Minimum margin**: 1 space all around modal

### Inner Spacing (Modal content)
- **Title padding**: 1 space left/right, 0 lines top
- **Content padding**: 1 space left/right, 1 line top/bottom
- **Field spacing**: 1 line between form fields
- **Footer padding**: 1 space left/right, 1 line top

### Field Layout (Parameter Input)
```
┌─ Method Signature ──────────── esc ┐
│                                    │
│ fieldName (type):                  │
│ inputValue█                        │
│                                    │
│ nextField (type):                  │
│ █                                  │
│                                    │
│ Instructions for navigation        │
└────────────────────────────────────┘
```

**Spacing**:
- 1 blank line before first field
- 1 blank line between fields
- 1 blank line before footer

---

## Status Bar Spacing

### Width Distribution
```
● Connected | Chain: 1 | Account: 0xabc... | Balance: 10.5 ETH
├─ indicator
              ├─ separator (` | `)
                              └─ content sections
```

**Element spacing**:
- **Between sections**: ` | ` (space-pipe-space)
- **Within sections**: Space-separated values
- **No left margin**: Status bar starts at column 0
- **No right margin**: Extends to terminal width

**Calculation**:
- Indicator: 2 characters (`● `)
- Each separator: 3 characters (` | `)
- Content variables: Fit remaining width

---

## Keyboard Shortcut Display Spacing

### Action Menu Footer
```
◇ Copy Result (c)    ◇ View as JSON (j)    ◇ Copy Call Data (d)
```

**Spacing**:
- **Between actions**: 4 spaces (` ` + 3 spaces)
- **After action text**: 1 space before parenthesis
- **Diamond prefix**: `◇ ` (diamond + space)
- **Parenthetical shortcut**: `(key)` immediately after action name

**Example breakdown**:
```
◇ Copy Result (c)
↑ Diamond + space (2 chars)
              ↑ Action name + space + shortcut (15+ chars)
                           ↑ 4 spaces to next action
```

---

## List and Menu Spacing

### Command Palette Layout
```
Suggested
▌Load Contract
 Other Command
 Another Option

Settings
 Setting 1
 Setting 2
```

**Spacing**:
- **Group header**: No leading spaces (column 0)
- **Group items**: 1 space indent
- **Selected indicator**: `▌` prefix (takes 1 space)
- **Unselected indicator**: 1 space (no indicator shown)
- **Between groups**: 1 blank line

---

## Text and Content Spacing

### Long Lines Wrapping
- **Wrap point**: Terminal width (e.g., 80 characters)
- **Continuation**: Wrapped lines maintain alignment
- **Indent continuation**: Optional 2-space indent for clarity

### Address/Hash Display
- **Truncation pattern**: `0xabc...def` (first + `...` + last 8 chars)
- **Space before/after**: Surrounded by spaces if inline
- **Full display**: Show complete in focused/detail view

### Labels and Values
```
Key: value
Longer Key: longer value that may wrap
  to next line if needed
Another Key: value
```

**Spacing**:
- **Label to colon**: No space (`Label:`)
- **Colon to value**: 1 space (`: `)
- **Wrapping indent**: 2 spaces for continuation

---

## Separator and Border Spacing

### Box Drawing
```
┌─────────────────────────┐
│ Content with padding   │
└─────────────────────────┘
```

**Characters used**:
- `┌` `┐` `└` `┘` = Corners
- `─` = Horizontal line (extends full width)
- `│` = Vertical line (extends full height)
- `├` `┤` `┬` `┴` `┼` = Intersections (used in trees)

**Width calculation**:
- `┌` + `─` repeated + `┐` = Title line
- Each horizontal line must match width exactly
- No extra spaces at line ends

### Separating Lines
- **Within section**: `────────` (8+ dashes)
- **Between major sections**: Blank line + separator line
- **Minimal separator**: At least width of content above

---

## Auto-Expansion Spacing Impact

When items auto-expand, spacing should:
- **Maintain visual hierarchy**: Indentation continues same pattern
- **Group related items**: Deployed methods stay together
- **Preserve alignment**: Other items shift but maintain alignment

**Example**:
```
Before:
▸ Counter.sol

After expanding (auto):
▾ Counter.sol
  ◇ Deploy new instance
  ◇ Load existing...
  ▾ 0x1234...
    ├ method1()
    └ method2()
```

---

## Responsive Spacing

### Terminal Size Adaptation
- **Wide terminals (120+ chars)**: Use natural spacing, no compression
- **Standard (80-120 chars)**: Current spacing rules apply
- **Narrow (60-80 chars)**: May truncate or wrap more aggressively
- **Very narrow (<60 chars)**: Alert shown, app pauses

### Sidebar Width Adjustment
- **Wide**: Sidebar can expand to 30-35% width
- **Standard**: 25-30% width
- **Narrow**: Reduced to ~20% width
- **Indentation preserved**: Spacing rules don't change with width

---

## Accessibility Spacing

### Click/Selection Target Spacing (Terminal Equivalent)
- **Between items**: 1 line minimum (allows selection)
- **No clustering**: Items well-separated enough for accurate selection
- **Clear focus**: Selected item spacing doesn't compress

### Reading Comfort
- **Line height**: Adequate spacing aids readability
- **Column separation**: Clear gaps between columns
- **Section grouping**: Spacing groups related information

---

## Best Practices

1. **Consistency**: Apply same spacing rules across all components
2. **Hierarchy**: More space = less related content
3. **Alignment**: Use indentation to show relationships
4. **Breathing room**: Adequate spacing prevents crowding
5. **Terminal awareness**: Account for minimum/maximum terminal widths
6. **Visual balance**: Symmetrical padding creates professional appearance
7. **Scannability**: Spacing aids quick visual scanning

---

## Quick Reference

| Element | Horizontal | Vertical | Example |
|---------|-----------|----------|---------|
| Tree level | 2 spaces/level | 0 lines | `  ├ item` |
| Sidebar margin | 1 space | 0-1 line | ` ▾ Contract ` |
| Card spacing | 1 space padding | 1 line between | Card box |
| Group separator | 0 spaces | 1 line | Between groups |
| Status bar | Sections w/ ` \| ` | Single line | Full width |
| Field spacing | 1 space indent | 1 line between | Form layout |
| List indent | 1 space | 0 lines | ` Item` |

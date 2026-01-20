# PRD: Output Card Feature

## Introduction/Overview

The Output Card Feature transforms the current flat, linear output display into an interactive, navigable card-based interface. Each output (transaction, call, or log) is presented as a selectable card that users can navigate through and interact with. This enables deeper exploration and debugging capabilities without cluttering the initial view—similar to how social media apps present content in a thread format.

**Problem Statement:** Currently, outputs are displayed linearly and provide limited interactivity. Users cannot easily replay transactions with different tracers or view transaction receipts without manual re-execution. By presenting outputs as interactive cards, users gain immediate access to debugging and inspection tools.

---

## Goals

1. **Enable Card Navigation** - Users can fluently navigate between output cards using keyboard shortcuts (j/k or arrow keys)
2. **Provide Context-Specific Actions** - Each card type (transaction, call, log) offers relevant interaction options
3. **Support Advanced Debugging** - Users can replay transactions/calls with different tracers and configurations without leaving the interface
4. **Integrate with User's Workflow** - Results are displayed in the user's configured $EDITOR, maintaining consistency with existing tools

---

## User Stories

### US-1: Navigate Between Output Cards
**Description:** As a user, I want to navigate through multiple output cards using j/k or arrow keys so that I can quickly browse through all outputs.

**Acceptance Criteria:**
- Pressing 'j' or down arrow moves to the next card (with visual indication)
- Pressing 'k' or up arrow moves to the previous card (with visual indication)
- Navigation wraps around (last card → first card and vice versa)
- Current card is visually highlighted/active
- Navigation works with any combination of card types (transaction, call, log)

---

### US-2: View and Select Transaction Cards
**Description:** As a user, I want to select a transaction card and see available options (receipt, debug trace) so that I can inspect transaction details.

**Acceptance Criteria:**
- Transaction cards display: transaction hash, status (success/failed), function called, gas used
- Pressing Enter/Space on a transaction card opens a menu with options
- Options are clearly labeled: "View Receipt" and "Debug Trace"
- Menu can be dismissed by pressing Escape
- Selection persists until an action is taken or menu is dismissed

---

### US-3: View Transaction Receipt
**Description:** As a user, I want to view the transaction receipt by selecting "View Receipt" so that I can inspect transaction execution details.

**Acceptance Criteria:**
- Selecting "View Receipt" retrieves the full transaction receipt via eth_getTransactionReceipt
- Receipt data is formatted as JSON
- Output is opened in $EDITOR
- User can edit/save the output in $EDITOR if desired
- Returns to card view after $EDITOR closes

---

### US-4: Replay Transaction with Debug Tracer
**Description:** As a user, I want to replay a transaction with a selected debug tracer (call, prestate, oplog) so that I can analyze execution flow with different perspectives.

**Acceptance Criteria:**
- Selecting "Debug Trace" shows a submenu with tracer options: "Call Tracer", "Prestate Tracer", "Oplog Tracer"
- After selecting a tracer, a configuration menu appears showing available options for that tracer
- Configuration options match Geth's built-in tracer specifications (http://geth.ethereum.org/docs/developers/evm-tracing/built-in-tracers)
- User can modify configuration values (e.g., onlyTopCall: true/false, diffMode: true/false)
- After confirming config, debug_traceTransaction is executed with selected tracer and config
- Results are formatted as JSON and opened in $EDITOR
- Returns to card view after $EDITOR closes

---

### US-5: View and Select Call Cards
**Description:** As a user, I want to select a call card and view the option to replay it so that I can debug the call execution.

**Acceptance Criteria:**
- Call cards display: function signature, to address, value sent, data
- Pressing Enter/Space on a call card opens a menu
- Menu shows single option: "Debug Call"
- Selection and menu behavior matches transaction cards

---

### US-6: Replay Call with Debug
**Description:** As a user, I want to replay a call with debug_traceCall so that I can inspect the execution trace.

**Acceptance Criteria:**
- Selecting "Debug Call" executes debug_traceCall for that specific call
- Call tracer is used with standard configuration
- Results are formatted as JSON and opened in $EDITOR
- Returns to card view after $EDITOR closes

---

### US-7: View Log Cards
**Description:** As a user, I want to see simple log outputs (e.g., "Compilation succeeded") displayed as non-interactive cards so that I can view all output types in the card interface.

**Acceptance Criteria:**
- Log cards display the log message text
- Log cards are navigable (j/k keys work) but non-interactive (pressing Enter has no effect)
- Visual styling differs from transaction/call cards to indicate they are non-interactive

---

## Functional Requirements

**FR-1:** The system shall render all outputs as selectable cards in a vertical list format.

**FR-2:** Cards shall remain in memory and maintain state (current selection) during the user's session.

**FR-3:** The system shall support three card types: Transaction, Call, and Log.

**FR-4:** Navigation via j/k and arrow keys shall move selection between cards in order (with wrapping).

**FR-5:** Selecting a card (Enter/Space) shall display context-specific action menu.

**FR-6:** For Transaction cards: Menu shall offer "View Receipt" and "Debug Trace" options.

**FR-7:** For Call cards: Menu shall offer "Debug Call" option.

**FR-8:** For Log cards: Selection shall have no effect (non-interactive).

**FR-9:** "View Receipt" action shall call eth_getTransactionReceipt and display result in $EDITOR.

**FR-10:** "Debug Trace" action shall present submenu with tracer options: Call Tracer, Prestate Tracer, Oplog Tracer.

**FR-11:** After tracer selection, system shall present configurable options for that tracer based on Geth's specification.

**FR-12:** User shall be able to modify tracer configuration before execution.

**FR-13:** Upon configuration confirmation, debug_traceTransaction shall be executed with selected tracer and config.

**FR-14:** "Debug Call" action shall execute debug_traceCall with call tracer.

**FR-15:** All RPC call results (receipt, trace) shall be formatted as JSON and opened in $EDITOR.

**FR-16:** After $EDITOR closes, user shall return to card view with current card selection preserved.

**FR-17:** Pressing Escape shall close any open menu and return to card navigation view.

---

## Non-Goals

- Persistent storage of card history across sessions
- Sharing or exporting card collections
- Custom card styling or theming
- Batch operations on multiple cards
- Integration with other tools besides $EDITOR output display
- Advanced filtering or search of cards
- Real-time updates to card data
- Network connectivity display or status indicators

---

## Design/Technical Considerations

### UI Layout
- Cards displayed in a vertical scrollable list
- Current/active card highlighted (e.g., different background color, border, or indicator)
- Card content is right-aligned or padded to prevent text cutoff
- Menus are displayed as inline pop-ups or overlays relative to the selected card

### Card Format Examples

**Transaction Card:**
```
TX: 0x1a2b3c... | Success | swap(USDC→ETH) | Gas: 125,450 / 500,000
```

**Call Card:**
```
CALL: 0xabc123...→0xdef456... | balanceOf(0x...) | Value: 0 ETH
```

**Log Card:**
```
✓ Compilation succeeded
```

### Integration Points
- Must integrate with existing output system (capture outputs before display)
- RPC calls (eth_getTransactionReceipt, debug_traceTransaction, debug_traceCall) must use existing provider/client connection
- $EDITOR integration must respect user's EDITOR environment variable

### Technical Constraints
- All tracer configurations must match Geth's built-in tracer API
- JSON formatting must be readable and properly indented
- Menu navigation must be responsive (no network delays while navigating between cards)
- RPC calls may take time; consider adding status indicators (e.g., "Loading...")

---

## Success Metrics

1. **Adoption:** % of users who interact with at least one card action in their first session
2. **Engagement:** Average number of card interactions per session
3. **Efficiency:** Reduction in time to access transaction receipt or debug trace (compared to manual re-execution)
4. **User Feedback:** Positive feedback on card-based interface vs. linear output in user surveys

---

## Open Questions

1. Should card selection/history be preserved if user returns to the output view?
2. Should there be a visual indicator showing current position (e.g., "Card 3 of 10")?
3. For Call cards, should we allow configuration of the call tracer, or always use defaults?
4. Should failed transactions show different styling or additional debug options?
5. Should we support keyboard shortcuts for direct action selection (e.g., 'r' for receipt, 'd' for debug)?

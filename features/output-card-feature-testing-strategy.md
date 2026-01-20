# Testing Strategy: Output Card Feature

## Executive Summary

This document outlines a comprehensive testing strategy for the **Output Card Feature** using agentic AI as a core feedback mechanism. By combining Rust's `ratatui-testlib` for integration testing with Claude's generative and analytical capabilities, we create an autonomous feedback loop that catches issues early, validates user interactions, and ensures high-quality UX before deployment.

**Key Approach**: Unit tests for state transitions → Integration tests with AI-validated scenarios → AI-powered UX feedback loop

---

## 1. Testing Architecture Overview

### Three-Layer Testing Stack

```
┌─────────────────────────────────────────┐
│  Layer 3: AI-Powered UX Validation      │
│  (Claude Computer Use + Vision LLM)     │
│  - Screenshot analysis                  │
│  - UX feedback and improvements         │
│  - Edge case discovery                  │
└─────────────────────────────────────────┘
                    ▲
                    │
┌─────────────────────────────────────────┐
│  Layer 2: Integration Testing           │
│  (ratatui-testlib + Tokio)              │
│  - Full workflow simulation             │
│  - Keyboard navigation                  │
│  - Multi-step interactions              │
│  - State validation                     │
└─────────────────────────────────────────┘
                    ▲
                    │
┌─────────────────────────────────────────┐
│  Layer 1: Unit Testing                  │
│  (Ratatui TestBackend)                  │
│  - Card rendering                       │
│  - State transitions                    │
│  - Menu logic                           │
└─────────────────────────────────────────┘
```

### Test Automation Workflow

```
Developer writes code
         │
         ▼
┌─────────────────┐
│ Local Unit      │ ◄─── Developer runs: cargo test --lib
│ Tests Pass      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Git Push        │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│ GitHub Actions CI Pipeline              │
│ 1. Build                                │
│ 2. Run unit tests                       │
│ 3. Run integration tests                │
└────────┬────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│ Claude Code AI Testing (MCP)            │
│ 1. Generate test scenarios              │
│ 2. Execute via ratatui-testlib          │
│ 3. Validate outputs                     │
│ 4. Report issues                        │
└────────┬────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────┐
│ AI UX Analysis                          │
│ 1. Take screenshots                     │
│ 2. Analyze for usability issues         │
│ 3. Check consistency                    │
│ 4. Generate improvement suggestions     │
└────────┬────────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ Test Report     │
│ Generated       │
└─────────────────┘
```

---

## 2. Layer 1: Unit Testing

### Setup: Add Test Dependencies

```toml
# Cargo.toml - [dev-dependencies]
[dev-dependencies]
ratatui-testlib = "0.1"
tokio = { version = "1", features = ["rt", "macros", "sync"] }
assert_matches = "1.5"
insta = { version = "1.36", features = ["redactions"] }
tempfile = "3.8"
```

### 2.1 Card State Transitions

**File**: `src/tui/widgets/card/tests.rs`

```rust
#[cfg(test)]
mod card_tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_transaction_card_rendering() {
        // Test that transaction card renders correctly
        let card = OutputCard::Transaction {
            hash: "0x1a2b3c...".to_string(),
            status: "Success".to_string(),
            function: "swap".to_string(),
            gas_used: 125450,
        };

        let backend = TestBackend::new(80, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            let widget = CardWidget::new(&card);
            f.render_widget(widget, f.area());
        }).unwrap();

        let buffer = terminal.backend().buffer.clone();
        // Verify card content is present
        assert!(buffer.as_str().contains("0x1a2b3c"));
        assert!(buffer.as_str().contains("Success"));
    }

    #[test]
    fn test_log_card_rendering() {
        let card = OutputCard::Log {
            message: "Compilation succeeded".to_string(),
        };

        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            let widget = CardWidget::new(&card);
            f.render_widget(widget, f.area());
        }).unwrap();

        let buffer = terminal.backend().buffer.clone();
        assert!(buffer.as_str().contains("Compilation succeeded"));
    }
}
```

### 2.2 Card Navigation State Machine

**File**: `src/tui/widgets/card/navigation_tests.rs`

```rust
#[cfg(test)]
mod navigation_tests {
    use assert_matches::assert_matches;

    #[test]
    fn test_navigation_next_card() {
        let mut state = CardNavigationState::new(3);
        assert_eq!(state.current(), 0);

        state.next();
        assert_eq!(state.current(), 1);

        state.next();
        assert_eq!(state.current(), 2);
    }

    #[test]
    fn test_navigation_wraps_around() {
        let mut state = CardNavigationState::new(3);
        state.set_current(2);

        state.next();
        assert_eq!(state.current(), 0, "Should wrap to start");
    }

    #[test]
    fn test_navigation_prev_card() {
        let mut state = CardNavigationState::new(3);
        state.set_current(1);

        state.prev();
        assert_eq!(state.current(), 0);
    }

    #[test]
    fn test_navigation_prev_wraps() {
        let mut state = CardNavigationState::new(3);
        state.set_current(0);

        state.prev();
        assert_eq!(state.current(), 2, "Should wrap to end");
    }
}
```

### 2.3 Menu State Logic

**File**: `src/tui/widgets/card/menu_tests.rs`

```rust
#[cfg(test)]
mod menu_tests {
    use assert_matches::assert_matches;

    #[test]
    fn test_transaction_menu_options() {
        let card = OutputCard::Transaction {
            hash: "0x123".to_string(),
            status: "Success".to_string(),
            function: "transfer".to_string(),
            gas_used: 21000,
        };

        let menu = CardMenu::from_card(&card);

        assert_matches!(menu.options(), [
            CardMenuOption::ViewReceipt,
            CardMenuOption::DebugTrace,
        ]);
    }

    #[test]
    fn test_call_menu_options() {
        let card = OutputCard::Call {
            from: "0xabc".to_string(),
            to: "0xdef".to_string(),
            function: "balanceOf".to_string(),
            value: "0".to_string(),
        };

        let menu = CardMenu::from_card(&card);

        assert_matches!(menu.options(), [
            CardMenuOption::DebugCall,
        ]);
    }

    #[test]
    fn test_log_menu_noninteractive() {
        let card = OutputCard::Log {
            message: "Event happened".to_string(),
        };

        let menu = CardMenu::from_card(&card);
        assert!(menu.options().is_empty(), "Log cards should have no menu options");
    }
}
```

### 2.4 Tracer Configuration

**File**: `src/tui/widgets/card/tracer_config_tests.rs`

```rust
#[cfg(test)]
mod tracer_config_tests {
    #[test]
    fn test_call_tracer_default_config() {
        let config = TracerConfig::call_tracer();
        assert_eq!(config.tracer_type, TracerType::CallTracer);
        assert!(config.get_option("onlyTopCall").is_none());
    }

    #[test]
    fn test_prestate_tracer_default_config() {
        let config = TracerConfig::prestate_tracer();
        assert_eq!(config.tracer_type, TracerType::PrestateTracer);
    }

    #[test]
    fn test_oplog_tracer_default_config() {
        let config = TracerConfig::oplog_tracer();
        assert_eq!(config.tracer_type, TracerType::OplogTracer);
    }

    #[test]
    fn test_config_serialization_to_json() {
        let mut config = TracerConfig::call_tracer();
        config.set_option("onlyTopCall", true);

        let json = config.to_json().unwrap();
        assert!(json.contains("\"onlyTopCall\":true"));
    }
}
```

### 2.5 Snapshot Tests for UI Rendering

**File**: `src/tui/widgets/card/snapshot_tests.rs`

```rust
#[cfg(test)]
mod snapshot_tests {
    use insta::assert_snapshot;

    #[test]
    fn test_transaction_card_snapshot() {
        let card = OutputCard::Transaction {
            hash: "0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d".to_string(),
            status: "Success".to_string(),
            function: "swap(USDC→ETH)".to_string(),
            gas_used: 125450,
        };

        let backend = TestBackend::new(80, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            let widget = CardWidget::new(&card);
            f.render_widget(widget, f.area());
        }).unwrap();

        let output = terminal.backend().buffer.as_str().to_string();
        assert_snapshot!(output);
    }
}
```

---

## 3. Layer 2: Integration Testing

### 3.1 Test Harness Setup

**File**: `tests/integration/card_harness.rs`

```rust
use ratatui_testlib::TestHarness;
use tokio::sync::mpsc;

pub struct CardTestHarness {
    harness: TestHarness,
    tx: mpsc::UnboundedSender<InputEvent>,
}

impl CardTestHarness {
    pub async fn new(width: u16, height: u16) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let harness = TestHarness::new(width, height);

        Self { harness, tx }
    }

    pub async fn send_key(&self, key: KeyCode) {
        self.tx.send(InputEvent::Key(KeyEvent {
            code: key,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })).unwrap();
    }

    pub async fn wait_for_render(&mut self) {
        self.harness.wait_for(Duration::from_millis(100)).await;
    }

    pub fn get_screen_text(&self) -> String {
        self.harness.get_screen_text()
    }

    pub fn screen_contains(&self, text: &str) -> bool {
        self.get_screen_text().contains(text)
    }
}
```

### 3.2 Card Navigation Integration Test

**File**: `tests/integration/card_navigation_tests.rs`

```rust
#[tokio::test]
async fn test_navigate_through_cards() {
    let mut harness = CardTestHarness::new(80, 20).await;

    // Setup: Display 3 cards (transaction, call, log)
    harness.setup_cards(vec![
        OutputCard::Transaction { /* ... */ },
        OutputCard::Call { /* ... */ },
        OutputCard::Log { /* ... */ },
    ]).await;

    // Initial state: First card selected
    assert!(harness.screen_contains("TX: 0x1a2b3c"));
    harness.wait_for_render().await;

    // Navigate to second card with 'j'
    harness.send_key(KeyCode::Char('j')).await;
    harness.wait_for_render().await;
    assert!(harness.screen_contains("CALL: 0xabc123"));

    // Navigate to third card with down arrow
    harness.send_key(KeyCode::Down).await;
    harness.wait_for_render().await;
    assert!(harness.screen_contains("Compilation succeeded"));

    // Wrap around with 'j'
    harness.send_key(KeyCode::Char('j')).await;
    harness.wait_for_render().await;
    assert!(harness.screen_contains("TX: 0x1a2b3c"), "Should wrap to first card");
}

#[tokio::test]
async fn test_navigate_backwards() {
    let mut harness = CardTestHarness::new(80, 20).await;
    harness.setup_cards(vec![/* 3 cards */]).await;

    harness.send_key(KeyCode::Char('j')).await;
    harness.wait_for_render().await;

    // Navigate backward with 'k'
    harness.send_key(KeyCode::Char('k')).await;
    harness.wait_for_render().await;
    assert!(harness.screen_contains("TX: 0x1a2b3c"), "Should return to first card");
}
```

### 3.3 Transaction Card Actions

**File**: `tests/integration/transaction_card_tests.rs`

```rust
#[tokio::test]
async fn test_view_transaction_receipt() {
    let mut harness = CardTestHarness::new(80, 20).await;

    harness.setup_cards(vec![
        OutputCard::Transaction {
            hash: "0x123abc".to_string(),
            status: "Success".to_string(),
            function: "swap".to_string(),
            gas_used: 100000,
        },
    ]).await;

    // Select card and open menu with Enter
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Menu should appear with options
    assert!(harness.screen_contains("View Receipt"));
    assert!(harness.screen_contains("Debug Trace"));

    // Select "View Receipt" with arrow/Enter
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Should invoke eth_getTransactionReceipt and open in $EDITOR
    // (Test verifies RPC call was made with correct parameters)
    let rpc_calls = harness.get_rpc_calls().await;
    assert!(rpc_calls.iter().any(|c| {
        c.method == "eth_getTransactionReceipt" &&
        c.params.contains("0x123abc")
    }));
}

#[tokio::test]
async fn test_debug_trace_with_call_tracer() {
    let mut harness = CardTestHarness::new(80, 20).await;

    harness.setup_cards(vec![
        OutputCard::Transaction {
            hash: "0x456def".to_string(),
            status: "Success".to_string(),
            function: "transfer".to_string(),
            gas_used: 50000,
        },
    ]).await;

    // Open menu
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Select "Debug Trace"
    harness.send_key(KeyCode::Down).await;
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Tracer selection menu should appear
    assert!(harness.screen_contains("Call Tracer"));
    assert!(harness.screen_contains("Prestate Tracer"));
    assert!(harness.screen_contains("Oplog Tracer"));

    // Select Call Tracer
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Config menu should appear
    assert!(harness.screen_contains("Config") || harness.screen_contains("Options"));

    // Confirm and execute
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Verify RPC call
    let rpc_calls = harness.get_rpc_calls().await;
    assert!(rpc_calls.iter().any(|c| {
        c.method == "debug_traceTransaction"
    }));
}
```

### 3.4 Call Card Actions

**File**: `tests/integration/call_card_tests.rs`

```rust
#[tokio::test]
async fn test_debug_call_workflow() {
    let mut harness = CardTestHarness::new(80, 20).await;

    harness.setup_cards(vec![
        OutputCard::Call {
            from: "0xabc123".to_string(),
            to: "0xdef456".to_string(),
            function: "balanceOf(0x...)".to_string(),
            value: "0 ETH".to_string(),
        },
    ]).await;

    // Select and open menu
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Should show only "Debug Call" option
    assert!(harness.screen_contains("Debug Call"));
    assert!(!harness.screen_contains("View Receipt"));

    // Select and execute
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Verify RPC call
    let rpc_calls = harness.get_rpc_calls().await;
    assert!(rpc_calls.iter().any(|c| {
        c.method == "debug_traceCall"
    }));
}
```

### 3.5 Edge Cases

**File**: `tests/integration/edge_cases_tests.rs`

```rust
#[tokio::test]
async fn test_menu_dismiss_with_escape() {
    let mut harness = CardTestHarness::new(80, 20).await;
    harness.setup_cards(vec![/* 1 transaction card */]).await;

    // Open menu
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;
    assert!(harness.screen_contains("View Receipt"));

    // Close menu with Escape
    harness.send_key(KeyCode::Esc).await;
    harness.wait_for_render().await;
    assert!(!harness.screen_contains("View Receipt"));
}

#[tokio::test]
async fn test_log_card_noninteractive() {
    let mut harness = CardTestHarness::new(80, 20).await;
    harness.setup_cards(vec![
        OutputCard::Log {
            message: "Compilation succeeded".to_string(),
        },
    ]).await;

    // Try to select log card
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // No menu should appear
    assert!(!harness.screen_contains("Menu"));
    assert!(!harness.screen_contains("Debug"));
}

#[tokio::test]
async fn test_long_transaction_hash_truncation() {
    let mut harness = CardTestHarness::new(40, 5).await; // Small terminal

    harness.setup_cards(vec![
        OutputCard::Transaction {
            hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            status: "Success".to_string(),
            function: "veryLongFunctionNameThatShouldTruncate".to_string(),
            gas_used: 999999999,
        },
    ]).await;

    harness.wait_for_render().await;

    // Should render without text overflow
    let output = harness.get_screen_text();
    assert!(!output.contains("panic"));
    assert!(!output.contains("overflow"));
}
```

---

## 4. Layer 3: AI-Powered Testing

### 4.1 Setup Claude Code MCP Server

**File**: `mcp/card_testing.rs`

```rust
use claude_sdk::types::{Tool, ToolInput};

pub struct CardTestingMcpServer {
    test_harness: CardTestHarness,
}

impl CardTestingMcpServer {
    pub async fn handle_tool(&self, tool: &str, input: &ToolInput) -> String {
        match tool {
            "run_card_test_scenario" => {
                self.run_test_scenario(input).await
            },
            "get_ui_state" => {
                self.get_ui_state().await
            },
            "analyze_test_failure" => {
                self.analyze_failure(input).await
            },
            "get_card_list" => {
                self.get_cards().await
            },
            _ => "Unknown tool".to_string(),
        }
    }

    async fn run_test_scenario(&self, input: &ToolInput) -> String {
        // Execute a test scenario described by Claude
        // Example: "Simulate user navigating to 2nd card and viewing receipt"
        // Returns: Test result (pass/fail) and screen output
    }

    async fn get_ui_state(&self) -> String {
        // Return current screen state as text
        format!("Screen:\n{}", self.test_harness.get_screen_text())
    }
}
```

### 4.2 AI Test Scenario Generation Prompt

**File**: `prompts/test_scenario_generation.txt`

```
You are an expert QA engineer testing a Rust TUI application for EVM contract interaction.

The application displays transaction, call, and log outputs as interactive cards.
Users can navigate cards with j/k keys, select with Enter, and perform actions.

Card Types:
1. Transaction: Shows hash, status, function, gas. Actions: View Receipt, Debug Trace
2. Call: Shows from/to addresses, function, value. Action: Debug Call
3. Log: Shows message. Non-interactive.

Tracer Configuration:
- Call Tracer: Available options (onlyTopCall, etc.)
- Prestate Tracer: Available options
- Oplog Tracer: Available options

Generate 20 comprehensive test scenarios for the Output Card Feature.

For EACH scenario provide:
1. Scenario Name (descriptive)
2. Setup (initial state)
3. User Actions (exact key sequence)
4. Expected Behavior
5. Validation Criteria
6. Edge Case Considerations

Include scenarios for:
- Happy paths (user succeeds at task)
- Input variations (different card types, configurations)
- Error conditions (network failures, invalid states)
- UI edge cases (small terminals, long names)
- Keyboard navigation (all key combinations)
- Menu interactions (navigation, selection, dismissal)

Format as structured test cases that can be executed by a test harness.
```

### 4.3 AI-Generated Test Validation

**File**: `tests/ai_generated_tests.rs`

```rust
// This file is AUTO-GENERATED by Claude
// DO NOT EDIT MANUALLY
// Run: claude-code --generate-ai-tests output-card-feature

#[tokio::test]
async fn ai_test_navigate_and_view_receipt() {
    let mut harness = CardTestHarness::new(80, 20).await;

    // AI-Generated: Scenario - Navigate to transaction and view receipt
    harness.setup_cards(vec![/* ... */]).await;

    // Step 1: Verify first card is displayed
    assert!(harness.screen_contains("TX:"));

    // Step 2: Navigate to second card
    harness.send_key(KeyCode::Char('j')).await;
    harness.wait_for_render().await;

    // Step 3: Select card
    harness.send_key(KeyCode::Enter).await;
    harness.wait_for_render().await;

    // Step 4: Verify menu appeared
    assert!(harness.screen_contains("Receipt") || harness.screen_contains("Debug"));

    // AI Validation: RPC call should be for eth_getTransactionReceipt
    let calls = harness.get_rpc_calls().await;
    assert!(!calls.is_empty());
}

#[tokio::test]
async fn ai_test_tracer_configuration_persistence() {
    // AI-Generated: Scenario - User configures tracer and executes multiple times

    let mut harness = CardTestHarness::new(80, 20).await;

    // First execution with call tracer
    harness.select_tracer("call").await;
    harness.configure_tracer("onlyTopCall", true).await;
    harness.execute_trace().await;

    let first_call = harness.get_last_rpc_call().await;
    assert!(first_call.params.contains("\"onlyTopCall\":true"));

    // Second execution should remember configuration
    harness.execute_trace().await;
    let second_call = harness.get_last_rpc_call().await;
    assert!(second_call.params.contains("\"onlyTopCall\":true"));
}

// ... 18 more AI-generated test cases ...
```

### 4.4 AI UX Analysis and Feedback

**File**: `scripts/ai-ux-analysis.sh`

```bash
#!/bin/bash
# Invoke Claude to analyze UI/UX of output card feature

cargo build --release

# Take screenshots of different scenarios
evm_cli_screenshot_scenario "card_navigation" > /tmp/nav.png
evm_cli_screenshot_scenario "menu_open" > /tmp/menu.png
evm_cli_screenshot_scenario "tracer_config" > /tmp/config.png

# Invoke Claude for analysis
claude-code --analyze-ux \
  --screenshots /tmp/nav.png,/tmp/menu.png,/tmp/config.png \
  --context "Output Card Feature for EVM CLI" \
  --report-file /tmp/ux_feedback.md

# Display results
echo "UX Analysis Report:"
cat /tmp/ux_feedback.md
```

### 4.5 Autonomous Testing Loop

**File**: `scripts/autonomous-test-loop.sh`

```bash
#!/bin/bash
# Autonomous testing loop: test → report → fix → retest

set -e

echo "🤖 Starting Autonomous Test Loop..."

# Phase 1: Run all tests
echo "📋 Phase 1: Running automated tests..."
cargo test --release 2>&1 | tee /tmp/test_results.txt

if grep -q "test result: FAILED" /tmp/test_results.txt; then
    echo "❌ Tests failed. Invoking AI analysis..."

    # Phase 2: AI analyzes failures and generates fixes
    echo "🧠 Phase 2: AI analyzing test failures..."
    claude-code \
      --analyze-failures /tmp/test_results.txt \
      --codebase ./src \
      --generate-fixes \
      --output /tmp/fixes.patch

    echo "✅ AI generated fixes:"
    cat /tmp/fixes.patch

    # Phase 3: Apply fixes and retest
    echo "🔧 Phase 3: Applying fixes and retesting..."
    git apply /tmp/fixes.patch
    cargo test --release

    # Phase 4: Validate fix
    echo "✔️  Phase 4: Validating fixes..."
    claude-code \
      --validate-fix \
      --original /tmp/test_results.txt \
      --after-fix <(cargo test --release 2>&1)
fi

echo "✅ Autonomous test loop completed successfully!"
```

---

## 5. Test Case Matrix

### Comprehensive Test Coverage Map

| Feature | Unit Tests | Integration Tests | AI-Generated Tests | UX Analysis |
|---------|-----------|-------------------|-------------------|-------------|
| **Card Navigation** | ✅ 4 tests | ✅ 3 tests | ✅ 5 scenarios | ✅ Discoverability |
| **Transaction Actions** | ✅ 3 tests | ✅ 4 tests | ✅ 6 scenarios | ✅ Clarity |
| **Call Actions** | ✅ 2 tests | ✅ 3 tests | ✅ 4 scenarios | ✅ Efficiency |
| **Log Display** | ✅ 2 tests | ✅ 2 tests | ✅ 2 scenarios | ✅ Readability |
| **Menu Interactions** | ✅ 3 tests | ✅ 4 tests | ✅ 5 scenarios | ✅ Accessibility |
| **Tracer Config** | ✅ 4 tests | ✅ 3 tests | ✅ 4 scenarios | ✅ Consistency |
| **Error Handling** | ✅ 3 tests | ✅ 3 tests | ✅ 6 scenarios | ✅ Messages |
| **Edge Cases** | ✅ 5 tests | ✅ 4 tests | ✅ 8 scenarios | ✅ Robustness |
| **TOTAL** | **26 tests** | **26 tests** | **40+ scenarios** | **Continuous** |

---

## 6. Implementation Checklist

### Phase 1: Foundation (Week 1)

- [ ] Add test dependencies to `Cargo.toml`
- [ ] Create test directory structure
  - [ ] `tests/integration/`
  - [ ] `src/tui/widgets/card/tests.rs`
  - [ ] `mcp/card_testing.rs`
- [ ] Write 26 unit tests (all pass locally)
- [ ] Document unit test patterns in wiki

### Phase 2: Integration Testing (Week 2)

- [ ] Implement `CardTestHarness`
- [ ] Write 26 integration tests
- [ ] Setup snapshot testing with `insta`
- [ ] Test all major workflows
- [ ] Verify edge cases

### Phase 3: AI Integration (Week 3)

- [ ] Create MCP server for card testing
- [ ] Write test scenario generation prompt
- [ ] Generate first batch of AI test cases
- [ ] Validate AI-generated tests run correctly
- [ ] Setup GitHub Actions CI integration

### Phase 4: Feedback Loop (Week 4)

- [ ] Implement autonomous test loop script
- [ ] Add UX analysis via Claude Computer Use
- [ ] Create test reporting dashboard
- [ ] Document best practices for AI testing
- [ ] Package as reusable component

### Phase 5: Optimization (Ongoing)

- [ ] Monitor test execution time
- [ ] Optimize slow tests
- [ ] Expand AI test case library
- [ ] Refine prompts based on results
- [ ] Share patterns with team

---

## 7. Key Metrics and Success Criteria

### Testing Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| **Unit Test Coverage** | >85% | - | Pending |
| **Integration Test Pass Rate** | 100% | - | Pending |
| **AI Test Case Acceptance** | >80% | - | Pending |
| **Test Execution Time** | <5 min | - | Pending |
| **UX Issue Discovery** | 5+ per cycle | - | Pending |

### Feature Quality Metrics

| Metric | Target |
|--------|--------|
| **Keyboard Navigation Coverage** | 100% |
| **Menu Option Validation** | 100% |
| **RPC Call Accuracy** | 100% |
| **Error Message Clarity** | >90% |
| **User Task Success Rate** | >95% |

---

## 8. Tools and Commands Reference

### Running Tests Locally

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific test
cargo test test_navigate_through_cards -- --nocapture

# Run with output capturing
cargo test -- --nocapture --test-threads=1

# Generate and review snapshots
cargo insta test --review
```

### Claude Code Integration

```bash
# Generate AI tests
claude-code --generate-ai-tests output-card-feature

# Run AI-powered UX analysis
claude-code --analyze-ux output-card-feature

# Start autonomous test loop
./scripts/autonomous-test-loop.sh

# Validate a specific fix
claude-code --validate-fix ./features/fix.patch
```

### CI/CD Pipeline

```bash
# GitHub Actions will automatically:
1. Build the project
2. Run all unit tests
3. Run all integration tests
4. Generate AI test scenarios
5. Execute AI-generated tests
6. Generate UX analysis report
7. Post results as PR comment
```

---

## 9. Troubleshooting & Common Issues

### Test Hangs or Times Out

**Cause**: Terminal rendering deadlock in test harness

**Solution**:
```rust
// Add timeout to test
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_with_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(10),
        run_test_scenario()
    ).await;

    assert!(result.is_ok(), "Test timed out");
}
```

### RPC Calls Not Captured

**Cause**: Mock provider not properly configured

**Solution**: Ensure test harness initializes mock provider before setup_cards()

```rust
harness.setup_mock_provider().await;
harness.setup_cards(vec![/*...*/]).await;
```

### Snapshot Differences on Different Terminals

**Cause**: Terminal width/height affects rendering

**Solution**: Always use fixed dimensions in tests
```rust
let harness = CardTestHarness::new(80, 24).await; // Standard dimensions
```

---

## 10. Future Enhancements

- [ ] Visual regression testing with pixel-perfect comparisons
- [ ] Performance benchmarking for large card sets
- [ ] Multi-user interaction simulation (concurrent cards)
- [ ] Accessibility testing automation (WCAG compliance)
- [ ] Voice command testing (for future accessibility feature)
- [ ] Mobile terminal emulation testing (iPhone SSH clients)

---

## Summary

This testing strategy combines:
- **Robust unit testing** for isolated components
- **Integration testing** for complete workflows
- **AI-powered test generation** for comprehensive scenario coverage
- **Autonomous feedback loops** for rapid iteration
- **UX validation** via vision analysis

Expected outcome: High-quality, thoroughly-tested output-card-feature with confidence in keyboard navigation, menu interactions, and RPC correctness.


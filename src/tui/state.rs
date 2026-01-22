use crate::cards::{Card, CopyOption, TracerConfig, TracerType};
use crate::compile::BytecodeTarget;
use crate::tui::widgets::PathSuggestion;
use alloy::json_abi::Param;
use alloy::primitives::Address;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    Output,
    CommandPalette,
}

#[derive(Debug, Clone, Default)]
pub struct FieldState {
    pub value: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PopupState {
    None,
    CommandPalette {
        query: String,
        selected: usize,
    },
    ParameterPopup {
        method_name: String,
        params: Vec<Param>,
        fields: Vec<FieldState>,
        current: usize,
        /// Some for deploy operations (to select EVM/PVM), None for calls
        bytecode_target: Option<BytecodeTarget>,
    },
    ContractSelector {
        contracts: Vec<String>,
        selected: usize,
    },
    FilePicker {
        path: String,
        error: Option<String>,
    },
    AddressInput {
        address: String,
        error: Option<String>,
    },
    TracerMenu {
        card_index: usize,
        tracers: Vec<TracerType>,
        selected: usize,
    },
    TracerConfig {
        card_index: usize,
        config: TracerConfig,
        current: usize,
    },
    CopyMenu {
        card_index: usize,
        options: Vec<CopyOption>,
        selected: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Default)]
pub struct SidebarState {
    pub selected: usize,
    pub scroll_offset: usize,
    /// Expanded contracts: (path, contract_name) pairs
    pub expanded_contracts: std::collections::HashSet<(PathBuf, String)>,
    pub expanded_instances: std::collections::HashSet<Address>,
}

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub style: OutputStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputStyle {
    Normal,
    Success,
    Error,
    Info,
    Waiting,
    Highlight,
    Separator,
}

#[derive(Debug, Clone, Default)]
pub struct OutputState {
    pub lines: Vec<OutputLine>,
    pub scroll_offset: usize,
}

impl OutputState {
    pub fn push(&mut self, text: impl Into<String>, style: OutputStyle) {
        self.lines.push(OutputLine {
            text: text.into(),
            style,
        });
    }

    pub fn push_normal(&mut self, text: impl Into<String>) {
        self.push(text, OutputStyle::Normal);
    }

    pub fn push_success(&mut self, text: impl Into<String>) {
        self.push(text, OutputStyle::Success);
    }

    pub fn push_error(&mut self, text: impl Into<String>) {
        self.push(text, OutputStyle::Error);
    }

    pub fn push_info(&mut self, text: impl Into<String>) {
        self.push(text, OutputStyle::Info);
    }

    pub fn push_separator(&mut self) {
        self.push("─".repeat(60), OutputStyle::Separator);
    }

    pub fn scroll_to_bottom(&mut self) {
        if self.lines.len() > 20 {
            self.scroll_offset = self.lines.len().saturating_sub(20);
        }
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.scroll_offset = 0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct CardState {
    #[allow(clippy::vec_init_then_push)]
    pub cards: Vec<Card>,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl CardState {
    /// Calculate the scroll offset needed to keep the selected card visible
    /// Returns the new scroll offset
    pub fn calculate_scroll_offset(&self, viewport_height: usize) -> usize {
        if self.cards.is_empty() {
            return 0;
        }

        // Calculate card line positions
        let mut card_positions = Vec::new();
        let mut total_lines = 0;

        for (i, card) in self.cards.iter().enumerate() {
            let start = total_lines;
            let is_selected = i == self.selected_index;
            // Estimate card height (header + content + actions only for selected + spacing)
            let card_height = match card {
                Card::Transaction {
                    gas_used,
                    contract_address,
                    ..
                } => {
                    let base = 5; // header + hash + status + function + contract_name
                    let addr_line = if contract_address.is_some() { 1 } else { 0 };
                    let gas_line = if gas_used.is_some() { 1 } else { 0 };
                    // Actions only rendered for selected interactive cards
                    let actions = if is_selected && card.is_interactive() {
                        2
                    } else {
                        0
                    };
                    base + addr_line + gas_line + actions + 2 // +2 for border line + blank line spacing
                }
                Card::Call { .. } => {
                    let base = 6; // header + function + to + from + empty + result
                    let actions = if is_selected && card.is_interactive() {
                        2
                    } else {
                        0
                    };
                    base + actions + 2 // +2 for border line + blank line spacing
                }
                Card::Log { message } => {
                    // header + message lines + border + blank spacing
                    let message_lines = message.lines().count();
                    1 + message_lines + 1 + 1
                }
                Card::Connection { error, .. } => {
                    // header + connected/disconnected + account + balance (optional) + chain_id (optional) + error (optional) + border + spacing
                    let base = 4; // header + status + account + border
                    let error_line = if error.is_some() { 1 } else { 0 };
                    base + 2 + error_line + 1 // +2 for balance and chain_id, +1 for spacing
                }
            };
            total_lines += card_height;
            card_positions.push((start, total_lines));
        }

        // Get selected card position
        if self.selected_index >= card_positions.len() {
            return self.scroll_offset;
        }

        let (card_start, card_end) = card_positions[self.selected_index];
        let current_offset = self.scroll_offset;
        let viewport_end = current_offset + viewport_height;

        // Check if card is fully visible
        if card_start >= current_offset && card_end <= viewport_end {
            // Card is fully visible, no scroll needed
            return current_offset;
        }

        // Card is not fully visible, calculate new offset
        // Try to center the card in the viewport
        let card_height = card_end - card_start;

        if card_height >= viewport_height {
            // Card is taller than viewport, show from start
            card_start
        } else {
            // Center the card
            let center_offset = card_start.saturating_sub(viewport_height / 2);
            center_offset.min(total_lines.saturating_sub(viewport_height))
        }
    }
}

pub struct AppState {
    pub focus: Focus,
    pub popup: PopupState,
    pub sidebar: SidebarState,
    pub output: OutputState,
    pub cards: CardState,
    pub connection: ConnectionStatus,
    pub chain_id: Option<u64>,
    pub account: Option<Address>,
    pub balance: Option<String>,
    pub connection_error: Option<String>,
    pub file_picker_suggestions: Vec<PathSuggestion>,
    pub file_picker_selected_idx: usize,
    pub terminal_size: (u16, u16),
    pub terminal_too_small: bool,
    pub output_area_height: u16,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: Focus::Sidebar,
            popup: PopupState::None,
            sidebar: SidebarState::default(),
            output: OutputState::default(),
            cards: CardState::default(),
            connection: ConnectionStatus::Disconnected,
            chain_id: None,
            account: None,
            balance: None,
            connection_error: None,
            file_picker_suggestions: Vec::new(),
            file_picker_selected_idx: 0,
            terminal_size: (80, 24),
            terminal_too_small: false,
            output_area_height: 20,
        }
    }
}

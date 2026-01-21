use crate::cards::{Card, CardAction, TracerType, TracerConfig};
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
    CardMenu {
        card_index: usize,
        actions: Vec<CardAction>,
        selected: usize,
    },
    TracerMenu {
        card_index: usize,
        tracers: Vec<TracerType>,
        selected: usize,
    },
    TracerConfig {
        card_index: usize,
        config: TracerConfig,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct SidebarState {
    pub selected: usize,
    pub scroll_offset: usize,
    pub expanded_contracts: std::collections::HashSet<PathBuf>,
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

#[derive(Debug, Clone)]
#[derive(Default)]
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

pub struct CardState {
    pub cards: Vec<Card>,
    pub selected_index: usize,
}

impl Default for CardState {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            selected_index: 0,
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
    pub file_picker_suggestions: Vec<PathSuggestion>,
    pub file_picker_selected_idx: usize,
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
            file_picker_suggestions: Vec::new(),
            file_picker_selected_idx: 0,
        }
    }
}

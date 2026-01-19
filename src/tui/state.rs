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

pub struct AppState {
    pub focus: Focus,
    pub popup: PopupState,
    pub sidebar: SidebarState,
    pub output: OutputState,
    pub connection: ConnectionStatus,
    pub chain_id: Option<u64>,
    pub account: Option<Address>,
    pub balance: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            focus: Focus::Sidebar,
            popup: PopupState::None,
            sidebar: SidebarState::default(),
            output: OutputState::default(),
            connection: ConnectionStatus::Disconnected,
            chain_id: None,
            account: None,
            balance: None,
        }
    }
}

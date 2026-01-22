use alloy::primitives::{Address, TxHash};
use std::fmt;

/// Represents a single output card that can be displayed and interacted with
#[derive(Clone, Debug)]
pub enum Card {
    Transaction {
        hash: TxHash,
        status: TransactionStatus,
        function_name: String,
        gas_used: Option<String>,
        contract_name: String,
        contract_address: Option<Address>,
        error_message: Option<String>,
    },
    Call {
        from: Address,
        to: Address,
        function_signature: String,
        result: String,
    },
    Log {
        message: String,
    },
    Connection {
        connected: bool,
        account: Address,
        balance: Option<String>,
        chain_id: Option<u64>,
        error: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionStatus {
    Success,
    Failed,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Success => write!(f, "Success"),
            TransactionStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl Card {
    /// Determine if this card is interactive (has a menu)
    pub fn is_interactive(&self) -> bool {
        matches!(self, Card::Transaction { .. } | Card::Call { .. })
    }
}

/// Menu options available for cards
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardAction {
    ViewReceipt,
    DebugTrace,
    DebugCall,
    Copy,
}

impl fmt::Display for CardAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardAction::ViewReceipt => write!(f, "View Receipt"),
            CardAction::DebugTrace => write!(f, "Debug Trace"),
            CardAction::DebugCall => write!(f, "Debug Call"),
            CardAction::Copy => write!(f, "Copy"),
        }
    }
}

/// Get available actions for a card type
pub fn get_card_actions(card: &Card) -> Vec<CardAction> {
    match card {
        Card::Transaction { .. } => vec![
            CardAction::Copy,
            CardAction::ViewReceipt,
            CardAction::DebugTrace,
        ],
        Card::Call { .. } => vec![CardAction::DebugCall],
        Card::Log { .. } => vec![],
        Card::Connection { .. } => vec![],
    }
}

/// Copy options for transaction cards
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyOption {
    Hash,
    Address,
}

impl fmt::Display for CopyOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyOption::Hash => write!(f, "Transaction Hash"),
            CopyOption::Address => write!(f, "Contract Address"),
        }
    }
}

/// Get copy options for a transaction card
pub fn get_copy_options(card: &Card) -> Vec<CopyOption> {
    match card {
        Card::Transaction {
            contract_address,
            function_name,
            ..
        } => {
            // If it's a deployment (has contract_address and function starts with "Deploy")
            if contract_address.is_some() && function_name.starts_with("Deploy") {
                vec![CopyOption::Hash, CopyOption::Address]
            } else {
                vec![CopyOption::Hash]
            }
        }
        _ => vec![],
    }
}

/// Debug tracer type options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TracerType {
    Call,
    Prestate,
    Execution,
}

impl fmt::Display for TracerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TracerType::Call => write!(f, "Call Tracer"),
            TracerType::Prestate => write!(f, "Prestate Tracer"),
            TracerType::Execution => write!(f, "Execution Tracer"),
        }
    }
}

pub fn get_tracer_types() -> Vec<TracerType> {
    vec![
        TracerType::Call,
        TracerType::Prestate,
        TracerType::Execution,
    ]
}

/// Configuration options for tracers
#[derive(Clone, Debug)]
pub struct TracerConfig {
    pub tracer_type: TracerType,
    // Call tracer options
    pub with_logs: bool,
    pub only_top_call: bool,
    // Prestate tracer options
    pub diff_mode: bool,
    // Execution tracer options
    pub enable_memory: bool,
    pub disable_stack: bool,
    pub disable_storage: bool,
}

impl Default for TracerConfig {
    fn default() -> Self {
        Self {
            tracer_type: TracerType::Call,
            with_logs: true,
            only_top_call: false,
            diff_mode: false,
            enable_memory: false,
            disable_stack: false,
            disable_storage: false,
        }
    }
}

impl TracerConfig {
    /// Convert to JSON-RPC TracerConfig format matching Polkadot SDK API
    /// - For callTracer and prestateTracer: {"tracer": "...", "tracerConfig": {...}}
    /// - For executionTracer: config is inlined without "tracer" field (it's the default)
    pub fn to_json(&self) -> serde_json::Value {
        match self.tracer_type {
            TracerType::Call => {
                serde_json::json!({
                    "tracer": "callTracer",
                    "tracerConfig": {
                        "withLogs": self.with_logs,
                        "onlyTopCall": self.only_top_call
                    }
                })
            }
            TracerType::Prestate => {
                serde_json::json!({
                    "tracer": "prestateTracer",
                    "tracerConfig": {
                        "diffMode": self.diff_mode
                    }
                })
            }
            TracerType::Execution => {
                // ExecutionTracer is the default - config is inlined without "tracer" field
                serde_json::json!({
                    "enableMemory": self.enable_memory,
                    "disableStack": self.disable_stack,
                    "disableStorage": self.disable_storage
                })
            }
        }
    }

    pub fn tracer_name(&self) -> &'static str {
        match self.tracer_type {
            TracerType::Call => "callTracer",
            TracerType::Prestate => "prestateTracer",
            TracerType::Execution => "executionTracer",
        }
    }
}

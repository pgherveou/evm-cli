use alloy::primitives::{Address, TxHash};
use alloy::json_abi::Function;
use std::fmt;

/// Represents a single output card that can be displayed and interacted with
#[derive(Clone, Debug)]
pub enum Card {
    Transaction {
        hash: TxHash,
        status: TransactionStatus,
        function_name: String,
        gas_used: Option<String>,
    },
    Call {
        from: Address,
        to: Address,
        function_signature: String,
        value: String,
    },
    Log {
        message: String,
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
    /// Get a one-line display representation of the card
    pub fn display_line(&self) -> String {
        match self {
            Card::Transaction {
                hash,
                status,
                function_name,
                gas_used,
            } => {
                let hash_str = format!("{:?}", hash).chars().take(10).collect::<String>();
                let status_str = match status {
                    TransactionStatus::Success => "✓ Success",
                    TransactionStatus::Failed => "✗ Failed",
                };
                let gas_str = gas_used
                    .as_ref()
                    .map(|g| format!(" | Gas: {}", g))
                    .unwrap_or_default();
                format!(
                    "TX: {} | {} | {} {}",
                    hash_str, status_str, function_name, gas_str
                )
            }
            Card::Call {
                from,
                to,
                function_signature,
                value,
            } => {
                let from_str = format!("{:?}", from).chars().take(8).collect::<String>();
                let to_str = format!("{:?}", to).chars().take(8).collect::<String>();
                format!(
                    "CALL: {}→{} | {} | Value: {}",
                    from_str, to_str, function_signature, value
                )
            }
            Card::Log { message } => {
                format!("✓ {}", message)
            }
        }
    }

    /// Determine if this card is interactive (has a menu)
    pub fn is_interactive(&self) -> bool {
        matches!(self, Card::Transaction { .. } | Card::Call { .. })
    }

    /// Get the type name for styling purposes
    pub fn type_name(&self) -> &'static str {
        match self {
            Card::Transaction { .. } => "Transaction",
            Card::Call { .. } => "Call",
            Card::Log { .. } => "Log",
        }
    }
}

/// Menu options available for cards
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardAction {
    ViewReceipt,
    DebugTrace,
    DebugCall,
}

impl fmt::Display for CardAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardAction::ViewReceipt => write!(f, "View Receipt"),
            CardAction::DebugTrace => write!(f, "Debug Trace"),
            CardAction::DebugCall => write!(f, "Debug Call"),
        }
    }
}

/// Get available actions for a card type
pub fn get_card_actions(card: &Card) -> Vec<CardAction> {
    match card {
        Card::Transaction { .. } => vec![CardAction::ViewReceipt, CardAction::DebugTrace],
        Card::Call { .. } => vec![CardAction::DebugCall],
        Card::Log { .. } => vec![],
    }
}

/// Debug tracer type options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TracerType {
    Call,
    Prestate,
    Oplog,
}

impl fmt::Display for TracerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TracerType::Call => write!(f, "Call Tracer"),
            TracerType::Prestate => write!(f, "Prestate Tracer"),
            TracerType::Oplog => write!(f, "Oplog Tracer"),
        }
    }
}

pub fn get_tracer_types() -> Vec<TracerType> {
    vec![TracerType::Call, TracerType::Prestate, TracerType::Oplog]
}

/// Configuration options for tracers (subset of Geth tracer config)
#[derive(Clone, Debug)]
pub struct TracerConfig {
    pub tracer_type: TracerType,
    pub only_top_call: bool,
    pub diff_mode: bool,
}

impl Default for TracerConfig {
    fn default() -> Self {
        Self {
            tracer_type: TracerType::Call,
            only_top_call: false,
            diff_mode: false,
        }
    }
}

impl TracerConfig {
    pub fn to_json(&self) -> serde_json::Value {
        let mut config = serde_json::json!({});

        match self.tracer_type {
            TracerType::Call => {
                config["onlyTopCall"] = serde_json::json!(self.only_top_call);
            }
            TracerType::Prestate => {
                config["diffMode"] = serde_json::json!(self.diff_mode);
            }
            TracerType::Oplog => {
                // Oplog doesn't have standard config options
            }
        }

        config
    }

    pub fn tracer_name(&self) -> &'static str {
        match self.tracer_type {
            TracerType::Call => "callTracer",
            TracerType::Prestate => "prestateTracer",
            TracerType::Oplog => "oplogTracer",
        }
    }
}

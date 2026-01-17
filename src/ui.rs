use alloy::primitives::Address;
use crossterm::style::Stylize;
use std::io::{self, Write};
use std::path::Path;

pub struct StatusBar {
    contract_name: Option<String>,
    contract_path: Option<String>,
    address: Option<Address>,
    chain_id: Option<u64>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            contract_name: None,
            contract_path: None,
            address: None,
            chain_id: None,
        }
    }

    pub fn set_contract(&mut self, name: &str, path: &Path) {
        self.contract_name = Some(name.to_string());
        self.contract_path = Some(path.to_string_lossy().to_string());
    }

    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address);
    }

    pub fn set_chain_id(&mut self, chain_id: u64) {
        self.chain_id = Some(chain_id);
    }

    pub fn clear_address(&mut self) {
        self.address = None;
    }

    pub fn render(&self) -> String {
        let mut parts = Vec::new();

        if let Some(name) = &self.contract_name {
            parts.push(format!("Contract: {}", name.clone().cyan()));
        }

        if let Some(addr) = &self.address {
            parts.push(format!("Address: {}", format_address(*addr).green()));
        } else {
            parts.push("Address: (not deployed)".dark_yellow().to_string());
        }

        if let Some(chain_id) = self.chain_id {
            parts.push(format!("Chain: {}", chain_id.to_string().magenta()));
        }

        parts.join(" | ")
    }

    pub fn footer_lines(&self) -> Vec<String> {
        vec![
            "─".repeat(65).dark_grey().to_string(),
            self.render(),
            String::new(), // margin
            format!(
                "  {} methods    {} context",
                "/".cyan().bold(),
                "@".cyan().bold()
            ),
        ]
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

pub fn format_address(addr: Address) -> String {
    format!("{:?}", addr)
}

pub fn print_result(label: &str, value: &str) {
    println!();
    println!("{}: {}", label.green().bold(), value);
    println!();
}

pub fn print_error(message: &str) {
    println!();
    println!("{}: {}", "Error".red().bold(), message);
    println!();
}

pub fn print_info(message: &str) {
    println!("{}", message.dark_grey());
}

pub fn print_tx_hash(hash: &str) {
    println!();
    println!("{}: {}", "Transaction".green().bold(), hash.yellow());
}

pub fn print_method_call(call: &str) {
    println!(">>> {} <<<", call.cyan());
    println!();
}

pub fn print_waiting(message: &str) {
    print!("{} ", message.dark_yellow());
    io::stdout().flush().ok();
}

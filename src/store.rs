use alloy::primitives::Address;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const STORE_FILE: &str = ".evm-cli";

/// Configuration settings stored in the .evm-cli file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// RPC URL for the Ethereum node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeploymentStore {
    /// Configuration settings
    #[serde(default)]
    pub config: Config,
    /// Deployments: map of sol file path -> list of deployed addresses
    #[serde(default)]
    deployments: HashMap<String, Vec<String>>,
    /// Last used contract path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_contract: Option<String>,
    /// Last used address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_address: Option<String>,
}

impl DeploymentStore {
    pub fn load() -> Result<Self> {
        let path = Self::store_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::store_path();
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize deployment store")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write {}", path.display()))?;

        Ok(())
    }

    pub fn get_deployments(&self, sol_path: &Path) -> Vec<Address> {
        let key = Self::path_key(sol_path);
        self.deployments
            .get(&key)
            .map(|addrs| {
                addrs
                    .iter()
                    .filter_map(|s| s.parse().ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn add_deployment(&mut self, sol_path: &Path, address: Address) {
        let key = Self::path_key(sol_path);
        let addr_str = format!("{:?}", address);

        let addrs = self.deployments.entry(key).or_default();
        if !addrs.contains(&addr_str) {
            addrs.push(addr_str);
        }
    }

    pub fn all_contracts(&self) -> Vec<PathBuf> {
        self.deployments.keys().map(PathBuf::from).collect()
    }

    fn store_path() -> PathBuf {
        PathBuf::from(STORE_FILE)
    }

    fn path_key(sol_path: &Path) -> String {
        sol_path
            .canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| sol_path.to_string_lossy().to_string())
    }

    pub fn set_last_contract(&mut self, path: &Path) {
        self.last_contract = Some(Self::path_key(path));
    }

    pub fn set_last_address(&mut self, address: Address) {
        self.last_address = Some(format!("{:?}", address));
    }

    pub fn get_last_contract(&self) -> Option<PathBuf> {
        self.last_contract.as_ref().map(PathBuf::from)
    }

    pub fn get_last_address(&self) -> Option<Address> {
        self.last_address.as_ref().and_then(|s| s.parse().ok())
    }

    pub fn clear_last(&mut self) {
        self.last_contract = None;
        self.last_address = None;
    }
}

pub fn select_address(addresses: &[Address], allow_new: bool) -> Result<Option<Address>> {
    if addresses.is_empty() && !allow_new {
        anyhow::bail!("No deployed addresses available");
    }

    let mut options: Vec<String> = addresses
        .iter()
        .map(|a| format!("{:?}", a))
        .collect();

    if allow_new {
        options.insert(0, "Deploy new contract".to_string());
    }
    options.push("Enter address manually".to_string());

    let selection = inquire::Select::new("Select contract address:", options.clone())
        .prompt()
        .context("Failed to select address")?;

    if selection == "Deploy new contract" {
        return Ok(None);
    }

    if selection == "Enter address manually" {
        let addr_str = inquire::Text::new("Enter contract address:")
            .with_placeholder("0x...")
            .prompt()
            .context("Failed to get address")?;

        let address: Address = addr_str
            .parse()
            .context("Invalid address format")?;

        return Ok(Some(address));
    }

    // Parse selected address
    let address: Address = selection
        .parse()
        .context("Failed to parse selected address")?;

    Ok(Some(address))
}

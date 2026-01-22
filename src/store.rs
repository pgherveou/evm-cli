use alloy::primitives::Address;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const STORE_DIR: &str = ".evm-cli";
const STORE_FILE: &str = "config.json";

/// Get the default global config directory (~/.evm-cli)
fn default_store_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(STORE_DIR)
}

/// Get the default global config path (~/.evm-cli/config.json)
fn default_store_path() -> PathBuf {
    default_store_dir().join(STORE_FILE)
}

// Default values for local development.
// These are well-known test credentials used by local Ethereum dev nodes (e.g., Anvil, Hardhat).
// NEVER use these in production - they are publicly known and have no real value.
pub const DEFAULT_RPC_URL: &str = "http://localhost:8545";
pub const DEFAULT_ADDRESS: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
pub const DEFAULT_PRIVATE_KEY: &str =
    "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133";

/// Configuration settings stored in the .evm-cli/config.json file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// RPC URL for the Ethereum node
    #[serde(default = "default_rpc_url")]
    pub rpc_url: String,
    /// Account address
    #[serde(default = "default_address")]
    pub address: String,
    /// Private key for signing transactions
    #[serde(default = "default_private_key")]
    pub private_key: String,
}

fn default_rpc_url() -> String {
    DEFAULT_RPC_URL.to_string()
}

fn default_address() -> String {
    DEFAULT_ADDRESS.to_string()
}

fn default_private_key() -> String {
    DEFAULT_PRIVATE_KEY.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: default_rpc_url(),
            address: default_address(),
            private_key: default_private_key(),
        }
    }
}

/// A contract identifier consisting of file path and contract name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractId {
    pub path: PathBuf,
    pub name: String,
}

impl ContractId {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self { path, name }
    }

    /// Create a storage key in the format "path:ContractName"
    fn to_key(&self) -> String {
        let canonical_path = self
            .path
            .canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| self.path.to_string_lossy().to_string());
        format!("{canonical_path}:{}", self.name)
    }

    /// Parse a storage key back into a ContractId
    fn from_key(key: &str) -> Option<Self> {
        // Find the last colon that separates path from contract name
        // Contract names don't contain colons, but paths might (e.g., Windows C:\)
        let last_colon = key.rfind(':')?;
        let path_str = &key[..last_colon];
        let name = &key[last_colon + 1..];

        if name.is_empty() {
            return None;
        }

        Some(Self {
            path: PathBuf::from(path_str),
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStore {
    /// Configuration settings
    #[serde(default)]
    pub config: Config,
    /// Deployments: map of "path:ContractName" -> list of deployed addresses
    /// Empty array means contract is saved but not yet deployed
    #[serde(default)]
    deployments: HashMap<String, Vec<String>>,
    /// Path to the config file (not serialized)
    #[serde(skip)]
    config_file_path: PathBuf,
}

impl Default for DeploymentStore {
    fn default() -> Self {
        Self {
            config: Config::default(),
            deployments: HashMap::new(),
            config_file_path: default_store_path(),
        }
    }
}

impl DeploymentStore {
    /// Load from the default global config path (~/.evm-cli/config.json)
    pub fn load() -> Result<Self> {
        Self::load_from(None)
    }

    /// Load from a specific path, or the default if None
    pub fn load_from(custom_path: Option<PathBuf>) -> Result<Self> {
        let path = custom_path.unwrap_or_else(default_store_path);

        let store = if !path.exists() {
            // Create directory and default config file
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create directory {}", parent.display())
                    })?;
                }
            }
            let store = Self {
                config: Config::default(),
                deployments: HashMap::new(),
                config_file_path: path.clone(),
            };
            // Save the default config
            let content = serde_json::to_string_pretty(&store)
                .context("Failed to serialize default config")?;
            fs::write(&path, content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            store
        } else {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let mut store: Self = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse {}", path.display()))?;
            store.config_file_path = path;
            store
        };

        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }

        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize deployment store")?;

        fs::write(&self.config_file_path, content)
            .with_context(|| format!("Failed to write {}", self.config_file_path.display()))?;

        Ok(())
    }

    /// Get the path to the config file being used
    pub fn config_path(&self) -> &PathBuf {
        &self.config_file_path
    }

    pub fn get_deployments(&self, contract_id: &ContractId) -> Vec<Address> {
        let key = contract_id.to_key();
        self.deployments
            .get(&key)
            .map(|addrs| addrs.iter().filter_map(|s| s.parse().ok()).collect())
            .unwrap_or_default()
    }

    pub fn add_deployment(&mut self, contract_id: &ContractId, address: Address) {
        let key = contract_id.to_key();
        let addr_str = format!("{address:?}");

        let addrs = self.deployments.entry(key).or_default();
        if !addrs.contains(&addr_str) {
            addrs.push(addr_str);
        }
    }

    /// Remove a specific deployment address from a contract
    /// Note: The contract entry is preserved even if all deployments are removed
    pub fn remove_deployment(&mut self, contract_id: &ContractId, address: Address) -> bool {
        let key = contract_id.to_key();
        let addr_str = format!("{address:?}");

        if let Some(addrs) = self.deployments.get_mut(&key) {
            if let Some(pos) = addrs.iter().position(|a| a == &addr_str) {
                addrs.remove(pos);
                // Keep the contract entry even if no deployments left
                // User must explicitly delete the contract to remove it
                return true;
            }
        }
        false
    }

    /// Remove all deployments for a contract
    pub fn remove_contract(&mut self, contract_id: &ContractId) -> bool {
        let key = contract_id.to_key();
        self.deployments.remove(&key).is_some()
    }

    /// Returns all contracts as ContractId
    pub fn all_contracts(&self) -> Vec<ContractId> {
        self.deployments
            .keys()
            .filter_map(|key| ContractId::from_key(key))
            .collect()
    }

    /// Ensure a contract exists in the store (with empty deployments if not yet deployed)
    pub fn ensure_contract(&mut self, contract_id: &ContractId) {
        let key = contract_id.to_key();
        self.deployments.entry(key).or_default();
    }

    /// Clear all deployments
    pub fn clear(&mut self) {
        self.deployments.clear();
    }
}

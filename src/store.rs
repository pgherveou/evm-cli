use alloy::primitives::Address;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const STORE_DIR: &str = ".evm-cli";
const STORE_FILE: &str = "config.json";

// Default values per PRD
pub const DEFAULT_RPC_URL: &str = "http://localhost:8545";
pub const DEFAULT_ADDRESS: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
pub const DEFAULT_PRIVATE_KEY: &str = "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133";

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeploymentStore {
    /// Configuration settings
    #[serde(default)]
    pub config: Config,
    /// Deployments: map of sol file path -> list of deployed addresses
    #[serde(default)]
    deployments: HashMap<String, Vec<String>>,
}

impl DeploymentStore {
    pub fn load() -> Result<Self> {
        let path = Self::store_path();
        if !path.exists() {
            // Return default with default config
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::store_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create directory {}", dir.display()))?;
        }

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

    /// Remove a specific deployment address from a contract
    pub fn remove_deployment(&mut self, sol_path: &Path, address: Address) -> bool {
        let key = Self::path_key(sol_path);
        let addr_str = format!("{:?}", address);

        if let Some(addrs) = self.deployments.get_mut(&key) {
            if let Some(pos) = addrs.iter().position(|a| a == &addr_str) {
                addrs.remove(pos);
                // Remove the contract entry if no deployments left
                if addrs.is_empty() {
                    self.deployments.remove(&key);
                }
                return true;
            }
        }
        false
    }

    /// Remove all deployments for a contract
    pub fn remove_contract(&mut self, sol_path: &Path) -> bool {
        let key = Self::path_key(sol_path);
        self.deployments.remove(&key).is_some()
    }

    pub fn all_contracts(&self) -> Vec<PathBuf> {
        self.deployments.keys().map(PathBuf::from).collect()
    }

    /// Clear all deployments
    pub fn clear(&mut self) {
        self.deployments.clear();
    }

    fn store_dir() -> PathBuf {
        PathBuf::from(STORE_DIR)
    }

    /// Get the path to the config file
    pub fn config_path() -> PathBuf {
        PathBuf::from(STORE_DIR).join(STORE_FILE)
    }

    fn store_path() -> PathBuf {
        Self::config_path()
    }

    fn path_key(sol_path: &Path) -> String {
        sol_path
            .canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| sol_path.to_string_lossy().to_string())
    }
}

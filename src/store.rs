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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Helper to create a test store with a temporary directory
    fn create_test_store() -> (DeploymentStore, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let store = DeploymentStore::load_from(Some(config_path)).unwrap();
        (store, temp_dir)
    }

    #[test]
    fn test_default_config_values() {
        let config = Config::default();
        assert_eq!(config.rpc_url, DEFAULT_RPC_URL);
        assert_eq!(config.address, DEFAULT_ADDRESS);
        assert_eq!(config.private_key, DEFAULT_PRIVATE_KEY);
    }

    #[test]
    fn test_contract_id_to_key_and_back() {
        let contract_id =
            ContractId::new(PathBuf::from("/tmp/Test.sol"), "TestContract".to_string());
        let key = contract_id.to_key();

        // Key should contain path and name separated by colon
        assert!(key.contains("Test.sol"));
        assert!(key.ends_with(":TestContract"));

        // Parse back
        let parsed = ContractId::from_key(&key).unwrap();
        assert_eq!(parsed.name, "TestContract");
    }

    #[test]
    fn test_contract_id_from_key_with_windows_path() {
        // Windows paths have colons, so we need to handle them correctly
        let key = "C:\\Users\\test\\Contract.sol:MyContract";
        let parsed = ContractId::from_key(key).unwrap();
        assert_eq!(parsed.name, "MyContract");
        assert_eq!(parsed.path, PathBuf::from("C:\\Users\\test\\Contract.sol"));
    }

    #[test]
    fn test_contract_id_from_key_invalid() {
        // No colon
        assert!(ContractId::from_key("invalid").is_none());
        // Empty name
        assert!(ContractId::from_key("/path/to/file:").is_none());
    }

    #[test]
    fn test_store_load_creates_default_config() {
        let (store, _temp_dir) = create_test_store();

        assert_eq!(store.config.rpc_url, DEFAULT_RPC_URL);
        assert_eq!(store.config.address, DEFAULT_ADDRESS);
        assert!(store.all_contracts().is_empty());
    }

    #[test]
    fn test_store_save_and_reload() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Create and modify store
        {
            let mut store = DeploymentStore::load_from(Some(config_path.clone())).unwrap();
            store.config.rpc_url = "http://custom:8545".to_string();

            let contract_id =
                ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
            store.ensure_contract(&contract_id);
            store.save().unwrap();
        }

        // Reload and verify
        let store = DeploymentStore::load_from(Some(config_path)).unwrap();
        assert_eq!(store.config.rpc_url, "http://custom:8545");
        assert_eq!(store.all_contracts().len(), 1);
    }

    #[test]
    fn test_add_and_get_deployment() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
        let address: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();

        // Initially empty
        assert!(store.get_deployments(&contract_id).is_empty());

        // Add deployment
        store.add_deployment(&contract_id, address);

        // Verify
        let deployments = store.get_deployments(&contract_id);
        assert_eq!(deployments.len(), 1);
        assert_eq!(deployments[0], address);
    }

    #[test]
    fn test_add_duplicate_deployment_ignored() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
        let address: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();

        store.add_deployment(&contract_id, address);
        store.add_deployment(&contract_id, address); // Duplicate

        let deployments = store.get_deployments(&contract_id);
        assert_eq!(deployments.len(), 1);
    }

    #[test]
    fn test_remove_deployment() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
        let address1: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();
        let address2: Address = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
            .parse()
            .unwrap();

        store.add_deployment(&contract_id, address1);
        store.add_deployment(&contract_id, address2);
        assert_eq!(store.get_deployments(&contract_id).len(), 2);

        // Remove one
        let removed = store.remove_deployment(&contract_id, address1);
        assert!(removed);

        let deployments = store.get_deployments(&contract_id);
        assert_eq!(deployments.len(), 1);
        assert_eq!(deployments[0], address2);

        // Contract entry still exists (even with one deployment)
        assert_eq!(store.all_contracts().len(), 1);
    }

    #[test]
    fn test_remove_nonexistent_deployment() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
        let address: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();

        let removed = store.remove_deployment(&contract_id, address);
        assert!(!removed);
    }

    #[test]
    fn test_remove_contract() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());
        let address: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();

        store.add_deployment(&contract_id, address);
        assert_eq!(store.all_contracts().len(), 1);

        let removed = store.remove_contract(&contract_id);
        assert!(removed);
        assert!(store.all_contracts().is_empty());
    }

    #[test]
    fn test_ensure_contract_creates_empty_entry() {
        let (mut store, _temp_dir) = create_test_store();
        let contract_id = ContractId::new(PathBuf::from("/test/Contract.sol"), "Test".to_string());

        store.ensure_contract(&contract_id);

        // Contract exists but has no deployments
        assert_eq!(store.all_contracts().len(), 1);
        assert!(store.get_deployments(&contract_id).is_empty());
    }

    #[test]
    fn test_clear_removes_all_deployments() {
        let (mut store, _temp_dir) = create_test_store();

        let contract1 = ContractId::new(PathBuf::from("/test/A.sol"), "A".to_string());
        let contract2 = ContractId::new(PathBuf::from("/test/B.sol"), "B".to_string());
        let address: Address = "0x1234567890123456789012345678901234567890"
            .parse()
            .unwrap();

        store.add_deployment(&contract1, address);
        store.add_deployment(&contract2, address);
        assert_eq!(store.all_contracts().len(), 2);

        store.clear();
        assert!(store.all_contracts().is_empty());
    }

    #[test]
    fn test_all_contracts_returns_all_entries() {
        let (mut store, _temp_dir) = create_test_store();

        let contract1 = ContractId::new(PathBuf::from("/test/A.sol"), "ContractA".to_string());
        let contract2 = ContractId::new(PathBuf::from("/test/B.sol"), "ContractB".to_string());

        store.ensure_contract(&contract1);
        store.ensure_contract(&contract2);

        let contracts = store.all_contracts();
        assert_eq!(contracts.len(), 2);

        let names: Vec<_> = contracts.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"ContractA"));
        assert!(names.contains(&"ContractB"));
    }

    #[test]
    fn test_config_path_returns_correct_path() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("custom_config.json");
        let store = DeploymentStore::load_from(Some(config_path.clone())).unwrap();

        assert_eq!(store.config_path(), &config_path);
    }
}

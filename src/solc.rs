use alloy::json_abi::JsonAbi;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CompiledContract {
    pub name: String,
    pub abi: JsonAbi,
    pub bytecode: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct SolcOutput {
    /// Map from "filename:ContractName" to contract data
    contracts: HashMap<String, SolcContract>,
    #[serde(default)]
    errors: Vec<SolcError>,
}

#[derive(Debug, Deserialize)]
struct SolcContract {
    abi: serde_json::Value,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct SolcError {
    severity: String,
    message: String,
    #[serde(rename = "formattedMessage")]
    formatted_message: Option<String>,
}

/// Compile Solidity file, optionally using pre-compiled bytecode
pub fn compile_solidity(sol_path: &Path) -> Result<Vec<CompiledContract>> {
    compile_solidity_with_bytecode(sol_path, None)
}

/// Compile Solidity file with optional pre-compiled bytecode override
pub fn compile_solidity_with_bytecode(
    sol_path: &Path,
    bytecode_path: Option<&Path>,
) -> Result<Vec<CompiledContract>> {
    let sol_path = sol_path
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", sol_path.display()))?;

    if !sol_path.exists() {
        bail!("Solidity file not found: {}", sol_path.display());
    }

    // Load external bytecode if provided
    let external_bytecode = if let Some(bc_path) = bytecode_path {
        let bytecode = std::fs::read(bc_path)
            .with_context(|| format!("Failed to read bytecode from {}", bc_path.display()))?;
        log::info!("Using pre-compiled bytecode from {}", bc_path.display());
        Some(bytecode)
    } else {
        None
    };

    let output = Command::new("solc")
        .arg("--combined-json")
        .arg("abi,bin")
        .arg(&sol_path)
        .output()
        .context("Failed to execute solc. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("solc compilation failed:\n{}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let solc_output: SolcOutput =
        serde_json::from_str(&stdout).context("Failed to parse solc output")?;

    // Check for errors (not warnings)
    let errors: Vec<_> = solc_output
        .errors
        .iter()
        .filter(|e| e.severity == "error")
        .collect();

    if !errors.is_empty() {
        let messages: Vec<_> = errors
            .iter()
            .map(|e| e.formatted_message.as_deref().unwrap_or(&e.message))
            .collect();
        bail!("Solidity compilation errors:\n{}", messages.join("\n"));
    }

    let mut contracts = Vec::new();

    for (full_key, contract) in solc_output.contracts {
        // full_key is "filename:ContractName" - extract just the contract name
        let contract_name = full_key
            .rsplit(':')
            .next()
            .unwrap_or(&full_key)
            .to_string();

        let abi: JsonAbi = serde_json::from_value(contract.abi.clone())
            .with_context(|| format!("Failed to parse ABI for {}", contract_name))?;

        // Use external bytecode if provided, otherwise use solc output
        let bytecode = external_bytecode.clone().map_or_else(
            || {
                hex::decode(&contract.bin)
                    .with_context(|| format!("Failed to decode bytecode for {}", contract_name))
            },
            Ok,
        )?;

        contracts.push(CompiledContract {
            name: contract_name,
            abi,
            bytecode,
        });
    }

    if contracts.is_empty() {
        bail!("No contracts found in {}", sol_path.display());
    }

    Ok(contracts)
}

pub fn select_contract(contracts: Vec<CompiledContract>) -> Result<CompiledContract> {
    if contracts.len() == 1 {
        return Ok(contracts.into_iter().next().unwrap());
    }

    let options: Vec<String> = contracts.iter().map(|c| c.name.clone()).collect();

    let selection = inquire::Select::new("Select contract to interact with:", options)
        .prompt()
        .context("Failed to select contract")?;

    contracts
        .into_iter()
        .find(|c| c.name == selection)
        .ok_or_else(|| anyhow::anyhow!("Contract not found"))
}

use alloy::json_abi::JsonAbi;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

/// Target bytecode format for compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BytecodeTarget {
    #[default]
    Evm,
    Pvm,
}

impl BytecodeTarget {
    /// Get the output directory name for this target
    pub fn output_dir(&self) -> &'static str {
        match self {
            BytecodeTarget::Evm => "out-evm",
            BytecodeTarget::Pvm => "out-pvm",
        }
    }

    /// Toggle to the other target
    pub fn toggle(&self) -> Self {
        match self {
            BytecodeTarget::Evm => BytecodeTarget::Pvm,
            BytecodeTarget::Pvm => BytecodeTarget::Evm,
        }
    }
}

impl std::fmt::Display for BytecodeTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeTarget::Evm => write!(f, "EVM"),
            BytecodeTarget::Pvm => write!(f, "PVM"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompiledContract {
    pub name: String,
    pub abi: JsonAbi,
    pub bytecode: Vec<u8>,
}

/// Forge artifact JSON structure
#[derive(Debug, Deserialize)]
struct ForgeArtifact {
    abi: serde_json::Value,
    bytecode: ForgeBytecode,
}

#[derive(Debug, Deserialize)]
struct ForgeBytecode {
    object: String,
}

/// PVM bytecode magic bytes (ASCII "PVM\0")
const PVM_MAGIC: [u8; 4] = [0x50, 0x56, 0x4d, 0x00];

/// Load contract ABI from forge build artifacts (EVM target).
/// This does a quick build to get the ABI without full bytecode validation.
pub fn load_contract_abi(sol_path: &Path) -> Result<Vec<(String, JsonAbi)>> {
    let sol_path = sol_path
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", sol_path.display()))?;

    if !sol_path.exists() {
        bail!("Solidity file not found: {}", sol_path.display());
    }

    // Run forge build for EVM to get ABIs
    let parent_dir = sol_path.parent().unwrap_or(Path::new("."));
    let output_dir = parent_dir.join(BytecodeTarget::Evm.output_dir());

    let build_cmd = run_forge_build(&sol_path, BytecodeTarget::Evm)?;

    // Find and parse artifacts
    let filename = sol_path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .to_string_lossy();

    let artifact_dir = output_dir.join(format!("{filename}.sol"));

    if !artifact_dir.exists() {
        let detailed_error = format!(
            "No artifacts found at {}.\n\nCommand executed:\n  {}\n\nThe build may have succeeded but produced no artifacts. Check if the contract is valid.",
            artifact_dir.display(),
            build_cmd
        );
        log::error!("{detailed_error}");

        // Simplified error for UI
        bail!("No artifacts found. The build may have succeeded but produced no artifacts. Check ~/.evm-cli/output.log for details.");
    }

    let mut contracts = Vec::new();

    for entry in std::fs::read_dir(&artifact_dir).with_context(|| {
        format!(
            "Failed to read artifact directory: {}",
            artifact_dir.display()
        )
    })? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let contract_name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();

            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read artifact: {}", path.display()))?;

            let artifact: ForgeArtifact = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse artifact: {}", path.display()))?;

            let abi: JsonAbi = serde_json::from_value(artifact.abi)
                .with_context(|| format!("Failed to parse ABI for {contract_name}"))?;

            contracts.push((contract_name, abi));
        }
    }

    if contracts.is_empty() {
        bail!("No contracts found in {}", sol_path.display());
    }

    Ok(contracts)
}

/// Compile contract for a specific bytecode target using forge.
pub fn compile_contract(
    sol_path: &Path,
    contract_name: &str,
    target: BytecodeTarget,
) -> Result<CompiledContract> {
    let sol_path = sol_path
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", sol_path.display()))?;

    // Run forge build
    let build_cmd = run_forge_build(&sol_path, target)?;

    // Find artifact
    let artifact_path = get_artifact_path(&sol_path, contract_name, target);

    if !artifact_path.exists() {
        let detailed_error = format!(
            "Contract artifact not found: {}.\n\nCommand executed:\n  {}\n\nThe contract name '{}' may not match any contract in the file.",
            artifact_path.display(),
            build_cmd,
            contract_name
        );
        log::error!("{detailed_error}");

        // Simplified error for UI
        bail!("Contract '{contract_name}' not found in artifacts. The name may not match. Check ~/.evm-cli/output.log for details.");
    }

    let content = std::fs::read_to_string(&artifact_path)
        .with_context(|| format!("Failed to read artifact: {}", artifact_path.display()))?;

    let artifact: ForgeArtifact = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse artifact: {}", artifact_path.display()))?;

    let abi: JsonAbi = serde_json::from_value(artifact.abi)
        .with_context(|| format!("Failed to parse ABI for {contract_name}"))?;

    // Decode bytecode
    let bytecode_hex = artifact
        .bytecode
        .object
        .strip_prefix("0x")
        .unwrap_or(&artifact.bytecode.object);
    let bytecode = hex::decode(bytecode_hex)
        .with_context(|| format!("Failed to decode bytecode for {contract_name}"))?;

    if bytecode.is_empty() {
        bail!("Empty bytecode for {contract_name}. This may be an interface or abstract contract.");
    }

    // Validate PVM magic if targeting PVM
    if target == BytecodeTarget::Pvm && (bytecode.len() < 4 || bytecode[..4] != PVM_MAGIC) {
        bail!("Invalid PVM bytecode: missing magic bytes. Ensure resolc is installed and working.");
    }

    Ok(CompiledContract {
        name: contract_name.to_string(),
        abi,
        bytecode,
    })
}

/// Run forge build for a specific target
fn run_forge_build(sol_path: &Path, target: BytecodeTarget) -> Result<String> {
    let parent_dir = sol_path.parent().unwrap_or(Path::new("."));
    let output_dir = parent_dir.join(target.output_dir());

    let mut cmd = Command::new("forge");
    cmd.arg("build")
        .arg("-o")
        .arg(&output_dir)
        .arg(sol_path)
        .current_dir(parent_dir);

    if target == BytecodeTarget::Pvm {
        cmd.arg("--resolc-compile");
    }

    // Format command for display
    let cmd_display = format!(
        "cd {} && forge build -o {} {}{}",
        parent_dir.display(),
        output_dir.display(),
        sol_path.display(),
        if target == BytecodeTarget::Pvm {
            " --resolc-compile"
        } else {
            ""
        }
    );

    let output = cmd
        .output()
        .context("Failed to execute forge. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = if stderr.is_empty() { stdout } else { stderr };

        // Log detailed error
        let detailed_error =
            format!("forge build failed.\n\nCommand:\n  {cmd_display}\n\nOutput:\n{combined}");
        log::error!("{detailed_error}");
        log::info!("Command: {cmd_display}");

        // Simplified error for UI - show first line of error
        let first_error_line = combined
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("Unknown error");
        bail!(
            "forge build failed: {first_error_line}\n\nCheck ~/.evm-cli/output.log for full details."
        );
    }

    // Log successful builds too
    log::info!("Command: {cmd_display}");

    Ok(cmd_display)
}

/// Get the artifact path for a compiled contract
fn get_artifact_path(
    sol_path: &Path,
    contract_name: &str,
    target: BytecodeTarget,
) -> std::path::PathBuf {
    let parent_dir = sol_path.parent().unwrap_or(Path::new("."));
    let filename = sol_path.file_stem().unwrap_or_default().to_string_lossy();

    parent_dir
        .join(target.output_dir())
        .join(format!("{filename}.sol"))
        .join(format!("{contract_name}.json"))
}

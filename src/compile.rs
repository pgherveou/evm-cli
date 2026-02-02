use alloy::json_abi::JsonAbi;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ContractSource {
    Solidity {
        sol_path: PathBuf,
    },
    Rust {
        project_path: PathBuf,
        sol_path: PathBuf,
        bin_target: String,
        release_mode: bool,
    },
}

impl ContractSource {
    pub fn is_rust(&self) -> bool {
        matches!(self, ContractSource::Rust { .. })
    }

    #[allow(dead_code)]
    pub fn sol_path(&self) -> &PathBuf {
        match self {
            ContractSource::Solidity { sol_path } => sol_path,
            ContractSource::Rust { sol_path, .. } => sol_path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompiledContract {
    pub name: String,
    pub abi: JsonAbi,
    pub bytecode: Vec<u8>,
    pub source: Option<ContractSource>,
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
        source: None,
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

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
    bin: Option<Vec<CargoBin>>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CargoBin {
    name: String,
    #[allow(dead_code)]
    path: Option<String>,
}

pub fn parse_cargo_bin_targets(project_path: &Path) -> Result<Vec<String>> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        bail!("Cargo.toml not found in {}", project_path.display());
    }

    let content = std::fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;

    let cargo: CargoToml = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", cargo_toml_path.display()))?;

    let mut targets = Vec::new();

    if let Some(bins) = cargo.bin {
        for bin in bins {
            targets.push(bin.name);
        }
    }

    if targets.is_empty() {
        if let Some(package) = cargo.package {
            let main_rs = project_path.join("src").join("main.rs");
            let bin_rs = project_path
                .join("src")
                .join(format!("{}.rs", package.name));
            if main_rs.exists() || bin_rs.exists() {
                targets.push(package.name);
            }
        }
    }

    if targets.is_empty() {
        bail!("No binary targets found in Cargo.toml");
    }

    Ok(targets)
}

pub fn find_sol_files(project_path: &Path) -> Result<Vec<PathBuf>> {
    let mut sol_files = Vec::new();

    for entry in std::fs::read_dir(project_path)
        .with_context(|| format!("Failed to read directory {}", project_path.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("sol") {
            sol_files.push(path);
        }
    }

    let src_path = project_path.join("src");
    if src_path.exists() {
        for entry in std::fs::read_dir(&src_path)
            .with_context(|| format!("Failed to read directory {}", src_path.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("sol") {
                sol_files.push(path);
            }
        }
    }

    if sol_files.is_empty() {
        bail!("No Solidity interface files (.sol) found in project");
    }

    Ok(sol_files)
}

pub fn match_sol_to_bin(bin_name: &str, sol_files: &[PathBuf]) -> Option<usize> {
    let bin_pascal = to_pascal_case(bin_name);

    for (i, sol_path) in sol_files.iter().enumerate() {
        if let Some(stem) = sol_path.file_stem().and_then(|s| s.to_str()) {
            if stem.eq_ignore_ascii_case(&bin_pascal) {
                return Some(i);
            }
        }
    }

    for (i, sol_path) in sol_files.iter().enumerate() {
        if let Some(stem) = sol_path.file_stem().and_then(|s| s.to_str()) {
            let stem_lower = stem.to_lowercase().replace(['_', '-'], "");
            let bin_lower = bin_name.to_lowercase().replace(['_', '-'], "");
            if stem_lower == bin_lower {
                return Some(i);
            }
        }
    }

    None
}

fn to_pascal_case(s: &str) -> String {
    s.split(['-', '_'])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[allow(dead_code)]
pub struct RustBuildResult {
    pub command: String,
    pub pvm_path: PathBuf,
}

pub fn build_rust_contract(
    project_path: &Path,
    bin_target: &str,
    release_mode: bool,
) -> Result<RustBuildResult> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--bin").arg(bin_target);

    if release_mode {
        cmd.arg("--release");
    }

    cmd.current_dir(project_path);

    let cmd_display = format!(
        "cd {} && cargo build{} --bin {}",
        project_path.display(),
        if release_mode { " --release" } else { "" },
        bin_target
    );

    log::info!("Running: {}", cmd_display);

    let output = cmd
        .output()
        .context("Failed to execute cargo. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = if stderr.is_empty() {
            stdout.to_string()
        } else {
            stderr.to_string()
        };

        log::error!("cargo build failed:\n{}", combined);
        bail!("cargo build failed:\n{}", combined);
    }

    let pvm_path = if release_mode {
        project_path
            .join("target")
            .join(format!("{}.release.polkavm", bin_target))
    } else {
        project_path
            .join("target")
            .join(format!("{}.debug.polkavm", bin_target))
    };

    if !pvm_path.exists() {
        let alt_paths = [
            project_path
                .join("target")
                .join(format!("{}.polkavm", bin_target)),
            project_path
                .join("target")
                .join(format!("{}.release.polkavm", bin_target)),
            project_path
                .join("target")
                .join(format!("{}.debug.polkavm", bin_target)),
        ];
        for alt_path in &alt_paths {
            if alt_path.exists() {
                return Ok(RustBuildResult {
                    command: cmd_display,
                    pvm_path: alt_path.clone(),
                });
            }
        }
        bail!(
            "PVM bytecode not found at expected path: {}",
            pvm_path.display()
        );
    }

    Ok(RustBuildResult {
        command: cmd_display,
        pvm_path,
    })
}

pub fn compile_rust_contract(
    project_path: &Path,
    bin_target: &str,
    sol_path: &Path,
    release_mode: bool,
) -> Result<(CompiledContract, RustBuildResult)> {
    let build_result = build_rust_contract(project_path, bin_target, release_mode)?;

    let bytecode = std::fs::read(&build_result.pvm_path).with_context(|| {
        format!(
            "Failed to read PVM bytecode from {}",
            build_result.pvm_path.display()
        )
    })?;

    if bytecode.len() < 4 || bytecode[..4] != PVM_MAGIC {
        bail!("Invalid PVM bytecode: missing magic bytes");
    }

    let (name, abi) = compile_solidity_abi(project_path, sol_path)?;

    let contract = CompiledContract {
        name,
        abi,
        bytecode,
        source: Some(ContractSource::Rust {
            project_path: project_path.to_path_buf(),
            sol_path: sol_path.to_path_buf(),
            bin_target: bin_target.to_string(),
            release_mode,
        }),
    };

    Ok((contract, build_result))
}

/// Compile Solidity interface to get ABI, storing artifacts in project's target/abi/ folder.
fn compile_solidity_abi(project_path: &Path, sol_path: &Path) -> Result<(String, JsonAbi)> {
    let sol_path = sol_path
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", sol_path.display()))?;

    let output_dir = project_path.join("target").join("abi");

    let mut cmd = Command::new("forge");
    cmd.arg("build")
        .arg("-o")
        .arg(&output_dir)
        .arg(&sol_path)
        .current_dir(project_path);

    let cmd_display = format!(
        "cd {} && forge build -o {} {}",
        project_path.display(),
        output_dir.display(),
        sol_path.display()
    );

    log::info!("Running: {}", cmd_display);

    let output = cmd
        .output()
        .context("Failed to execute forge. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("forge build failed:\n{}", stderr);
        bail!("forge build failed:\n{}", stderr);
    }

    let filename = sol_path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .to_string_lossy();

    let artifact_dir = output_dir.join(format!("{filename}.sol"));

    if !artifact_dir.exists() {
        bail!("No artifacts found at {}", artifact_dir.display());
    }

    for entry in std::fs::read_dir(&artifact_dir)? {
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

            return Ok((contract_name, abi));
        }
    }

    bail!("No contracts found in {}", sol_path.display())
}

/// Load ABI from a Rust project's target/abi/ folder (for stored contracts).
pub fn load_rust_contract_abi(
    project_path: &Path,
    sol_path: &Path,
) -> Result<Vec<(String, JsonAbi)>> {
    let sol_path = sol_path
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", sol_path.display()))?;

    let filename = sol_path
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .to_string_lossy();

    let artifact_dir = project_path
        .join("target")
        .join("abi")
        .join(format!("{filename}.sol"));

    if !artifact_dir.exists() {
        let (name, abi) = compile_solidity_abi(project_path, &sol_path)?;
        return Ok(vec![(name, abi)]);
    }

    let mut contracts = Vec::new();

    for entry in std::fs::read_dir(&artifact_dir)? {
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
        bail!("No contracts found in {}", artifact_dir.display());
    }

    Ok(contracts)
}

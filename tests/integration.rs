use std::path::PathBuf;
use std::process::Command;

fn forge_available() -> bool {
    Command::new("forge")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn demo_sol_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("demo/Demo.sol")
}

mod compile_tests {
    use super::*;

    #[test]
    fn test_load_contract_abi() {
        if !forge_available() {
            eprintln!("Skipping test: forge not available");
            return;
        }

        let path = demo_sol_path();
        assert!(path.exists(), "Demo.sol should exist at {:?}", path);

        let content = std::fs::read_to_string(&path).expect("Failed to read Demo.sol");
        assert!(
            content.contains("contract Demo"),
            "Should contain Demo contract"
        );
        assert!(
            content.contains("function getCount"),
            "Should have getCount function"
        );
        assert!(
            content.contains("function increment"),
            "Should have increment function"
        );
    }

    #[test]
    fn test_forge_build_demo() {
        if !forge_available() {
            eprintln!("Skipping test: forge not available");
            return;
        }

        let demo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("demo");
        let demo_sol = demo_dir.join("Demo.sol");

        let output = Command::new("forge")
            .arg("build")
            .arg("-o")
            .arg(demo_dir.join("out-evm"))
            .arg(&demo_sol)
            .current_dir(&demo_dir)
            .output()
            .expect("Failed to execute forge");

        assert!(
            output.status.success(),
            "forge build should succeed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let artifact_path = demo_dir.join("out-evm/Demo.sol/Demo.json");
        assert!(
            artifact_path.exists(),
            "Artifact should be created at {:?}",
            artifact_path
        );

        let content = std::fs::read_to_string(&artifact_path).expect("Failed to read artifact");
        let artifact: serde_json::Value =
            serde_json::from_str(&content).expect("Artifact should be valid JSON");

        let abi = artifact.get("abi").expect("Should have abi field");
        let abi_str = abi.to_string();
        assert!(abi_str.contains("getCount"), "ABI should have getCount");
        assert!(abi_str.contains("increment"), "ABI should have increment");
        assert!(abi_str.contains("decrement"), "ABI should have decrement");
        assert!(abi_str.contains("setCount"), "ABI should have setCount");

        let bytecode = artifact
            .get("bytecode")
            .and_then(|b| b.get("object"))
            .and_then(|o| o.as_str())
            .expect("Should have bytecode.object");
        assert!(
            bytecode.starts_with("0x") && bytecode.len() > 10,
            "Should have valid bytecode"
        );
    }

    #[test]
    fn test_demo_contract_structure() {
        let path = demo_sol_path();
        let content = std::fs::read_to_string(&path).expect("Failed to read Demo.sol");

        assert!(content.contains("pragma solidity"), "Should have pragma");
        assert!(
            content.contains("constructor(uint256 _initial)"),
            "Should have constructor with _initial param"
        );
        assert!(
            content.contains("event Incremented"),
            "Should have Incremented event"
        );
        assert!(
            content.contains("event Decremented"),
            "Should have Decremented event"
        );
    }
}

mod store_tests {
    use std::path::PathBuf;

    #[test]
    fn test_config_directory() {
        let home = std::env::var("HOME").expect("HOME should be set");
        let config_path = PathBuf::from(home).join(".evm-cli");

        assert!(
            config_path.to_str().is_some(),
            "Config path should be valid UTF-8"
        );
    }
}

use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn prompt_sol_path() -> Result<PathBuf> {
    let path = inquire::Text::new("Path to .sol file:")
        .with_placeholder("./contracts/MyContract.sol")
        .with_validator(|s: &str| {
            let path = PathBuf::from(s);
            if path.exists() {
                Ok(inquire::validator::Validation::Valid)
            } else {
                Ok(inquire::validator::Validation::Invalid(
                    "File does not exist".into(),
                ))
            }
        })
        .prompt()
        .context("Failed to get contract path")?;

    Ok(PathBuf::from(path))
}

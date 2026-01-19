mod app;
mod method_list;
mod prompts;
mod provider;
mod solc;
mod store;
mod tui;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "evm-cli")]
#[command(about = "Interactive CLI for deploying and interacting with Solidity contracts")]
#[command(version)]
struct Args {
    /// Path to a .sol file to load on startup
    #[arg(short, long)]
    contract: Option<PathBuf>,

    /// Path to pre-compiled bytecode file (e.g., .polkavm)
    #[arg(short, long)]
    bytecode: Option<PathBuf>,

    /// Contract address to interact with
    #[arg(short, long)]
    address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (ignore if missing)
    let _ = dotenvy::dotenv();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    let args = Args::parse();

    // Load store first (for config)
    let store = store::DeploymentStore::load()?;

    // Create provider (uses config for RPC URL fallback)
    let (provider, signer) = provider::create_provider(&store.config).await?;

    let signer_address = signer.address();

    // Get balance
    let balance = provider::get_balance(&provider, signer_address).await?;

    // Create app with the already-loaded store
    let mut app = app::App::new(provider, store);
    app.initialize().await?;
    app.set_account_info(signer_address, format_ether(balance));

    // Push initial info to output
    app.state.output.push_success(format!("Connected with account: {:?}", signer_address));
    app.state.output.push_info(format!("Balance: {} ETH", format_ether(balance)));
    app.state.output.push_separator();

    // Handle initial contract loading from args
    if let Some(path) = args.contract.filter(|p| p.exists()) {
        app.state.output.push_info(format!("Loading {}...", path.display()));
        match solc::compile_solidity_with_bytecode(&path, args.bytecode.as_deref()) {
            Ok(contracts) => {
                // Select first contract if multiple, or the only one
                if let Some(contract) = contracts.into_iter().next() {
                    app.state.output.push_success(format!("Loaded contract: {}", contract.name));
                    app.set_contract(contract, path.clone());

                    // Handle address from args
                    if let Some(addr) = args.address.as_ref().and_then(|s| s.parse().ok()) {
                        app.set_address(addr);
                    }
                }
            }
            Err(e) => app.state.output.push_error(format!("Failed to load contract: {}", e)),
        }
    }

    // Run interactive mode
    app.run_interactive().await?;

    Ok(())
}

fn format_ether(wei: alloy::primitives::U256) -> String {
    const DECIMALS: usize = 18;
    const DISPLAY_DECIMALS: usize = 6;

    let wei_str = wei.to_string();
    let len = wei_str.len();

    let (int_part, frac_part) = if len <= DECIMALS {
        let padded = format!("{:0>width$}", wei_str, width = DECIMALS);
        ("0".to_string(), padded)
    } else {
        let split = len - DECIMALS;
        (wei_str[..split].to_string(), wei_str[split..].to_string())
    };

    let frac_display = &frac_part[..DISPLAY_DECIMALS.min(frac_part.len())];
    format!("{}.{}", int_part, frac_display)
}

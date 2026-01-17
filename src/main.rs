mod app;
mod context_menu;
mod filter_ui;
mod method_list;
mod prompts;
mod provider;
mod solc;
mod store;
mod ui;

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

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    let args = Args::parse();

    // Load store first (for config)
    let store = store::DeploymentStore::load()?;

    // Create provider (uses config for RPC URL fallback)
    let (provider, signer) = provider::create_provider(&store.config).await?;

    let signer_address = signer.address();
    println!("Connected with account: {:?}", signer_address);

    // Get balance
    let balance = provider::get_balance(&provider, signer_address).await?;
    println!(
        "Balance: {} ETH",
        format_ether(balance)
    );
    println!();

    // Create app with the already-loaded store
    let mut app = app::App::new(provider, store);
    app.initialize().await?;

    // Handle initial contract loading from args or last saved state
    let contract_path = args.contract
        .or_else(|| app.store.get_last_contract());

    if let Some(path) = contract_path {
        if path.exists() {
            println!("Loading {}...", path.display());
            match solc::compile_solidity_with_bytecode(&path, args.bytecode.as_deref()) {
                Ok(contracts) => {
                    if let Ok(contract) = solc::select_contract(contracts) {
                        println!("Loaded contract: {}", contract.name);
                        app.set_contract(contract, path.clone());

                        // Handle address from args or last saved
                        let address = args.address
                            .as_ref()
                            .and_then(|s| s.parse().ok())
                            .or_else(|| app.store.get_last_address());

                        if let Some(addr) = address {
                            app.set_address(addr);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to load contract: {}", e);
                }
            }
        }
    }

    // Run interactive mode
    app.run_interactive().await?;

    Ok(())
}

fn format_ether(wei: alloy::primitives::U256) -> String {
    // Simple conversion: divide by 10^18
    let wei_str = wei.to_string();
    let len = wei_str.len();

    if len <= 18 {
        let padding = "0".repeat(18 - len);
        let padded = format!("{}{}", padding, wei_str);
        let (int_part, frac_part) = ("0", padded.as_str());
        format!("{}.{}", int_part, &frac_part[..6])
    } else {
        let split_point = len - 18;
        let int_part = &wei_str[..split_point];
        let frac_part = &wei_str[split_point..];
        format!("{}.{}", int_part, &frac_part[..6.min(frac_part.len())])
    }
}

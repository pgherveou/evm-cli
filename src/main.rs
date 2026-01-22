mod app;
mod cards;
mod compile;
mod logger;
mod method_list;
mod prompts;
mod provider;
mod store;
mod tui;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// An interactive CLI for deploying and interacting with Solidity contracts
#[derive(Parser, Debug)]
#[command(name = "evm-cli", version, about)]
struct Args {
    /// Path to config file (default: ~/.evm-cli/config.json)
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logger to write to ~/.evm-cli/output.log
    logger::init()?;

    // Load store from specified path or default (~/.evm-cli/config.json)
    let store = store::DeploymentStore::load_from(args.config)?;

    // Create provider (no connection test - will be done asynchronously)
    let (provider, signer) = provider::create_provider(&store.config)?;
    let signer_address = signer.address();

    // Create app with the already-loaded store and signer address
    let mut app = app::App::new(provider, store, signer_address);

    // Try initial connection (non-blocking failure)
    app.try_connect().await;

    // Create connection card (will be updated by polling if disconnected)
    app.add_connection_card();

    app.run_interactive().await?;

    Ok(())
}

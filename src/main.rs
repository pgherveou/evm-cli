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

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if present (ignore if missing)
    let _ = dotenvy::dotenv();

    // Initialize logger to write to ~/.evm-cli/output.log
    logger::init()?;

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
    if let Some(home) = std::env::var_os("HOME") {
        let log_path = std::path::PathBuf::from(home).join(".evm-cli/output.log");
        app.state.output.push_info(format!("Logs: {}", log_path.display()));
    }
    app.state.output.push_separator();

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

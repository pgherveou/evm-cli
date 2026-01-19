mod app;
mod compile;
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

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .format_target(false)
        .init();

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

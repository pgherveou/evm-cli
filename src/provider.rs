use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use anyhow::{Context, Result};

use crate::store::Config;

/// Create provider and signer using configuration from the config file.
pub fn create_provider(
    config: &Config,
) -> Result<(impl Provider + Clone + 'static, PrivateKeySigner)> {
    log::info!("Using RPC URL: {}", config.rpc_url);

    let private_key = config
        .private_key
        .strip_prefix("0x")
        .unwrap_or(&config.private_key);

    let signer: PrivateKeySigner = private_key.parse().context("Failed to parse private key")?;

    let wallet = EthereumWallet::from(signer.clone());

    let url: url::Url = config.rpc_url.parse().context("Failed to parse RPC URL")?;

    let provider = ProviderBuilder::new().wallet(wallet).connect_http(url);

    Ok((provider, signer))
}

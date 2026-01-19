use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use anyhow::{Context, Result};
use std::env;

use crate::store::Config;

pub async fn create_provider(
    config: &Config,
) -> Result<(impl Provider + Clone + 'static, PrivateKeySigner)> {
    // Priority: 1. Environment variable, 2. Config file (which has defaults)
    let rpc_url = env::var("ETH_RPC_URL")
        .unwrap_or_else(|_| config.rpc_url.clone());

    log::info!("Using RPC URL: {}", rpc_url);

    // Priority: 1. Environment variable, 2. Config file (which has defaults)
    let private_key = env::var("PRIVATE_KEY")
        .unwrap_or_else(|_| config.private_key.clone());

    let private_key = private_key.strip_prefix("0x").unwrap_or(&private_key);

    let signer: PrivateKeySigner = private_key
        .parse()
        .context("Failed to parse private key")?;

    let wallet = EthereumWallet::from(signer.clone());

    let url: url::Url = rpc_url.parse().context("Failed to parse RPC URL")?;

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect_http(url);

    // Test connection
    let chain_id = provider
        .get_chain_id()
        .await
        .context("Failed to connect to RPC endpoint")?;
    log::info!("Connected to chain ID: {}", chain_id);

    Ok((provider, signer))
}

pub async fn get_balance<P: Provider>(provider: &P, address: Address) -> Result<U256> {
    provider
        .get_balance(address)
        .await
        .context("Failed to get balance")
}

use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use anyhow::{bail, Context, Result};
use std::env;

use crate::store::Config;

const DEFAULT_RPC_URL: &str = "http://localhost:8545";

pub async fn create_provider(
    config: &Config,
) -> Result<(impl Provider + Clone + 'static, PrivateKeySigner)> {
    // Priority: 1. Environment variable, 2. Config file, 3. Default
    let rpc_url = env::var("ETH_RPC_URL")
        .ok()
        .or_else(|| config.rpc_url.clone())
        .unwrap_or_else(|| DEFAULT_RPC_URL.to_string());

    log::info!("Using RPC URL: {}", rpc_url);

    let private_key = env::var("PRIVATE_KEY").context(
        "PRIVATE_KEY environment variable not set. \
         Please set it to your private key (with or without 0x prefix).",
    )?;

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
    match provider.get_chain_id().await {
        Ok(chain_id) => {
            log::info!("Connected to chain ID: {}", chain_id);
        }
        Err(e) => {
            bail!("Failed to connect to RPC endpoint: {}", e);
        }
    }

    Ok((provider, signer))
}

pub async fn get_balance<P: Provider>(provider: &P, address: Address) -> Result<U256> {
    provider
        .get_balance(address)
        .await
        .context("Failed to get balance")
}

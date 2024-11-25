use alloy::{
    network::EthereumWallet,
    primitives::{Address, B256},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub provider: ProviderConfig,
    pub contract: ContractConfig,
    pub transfer: TransferConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    url: String,
    pk: String,
}

#[derive(Debug, Deserialize)]
pub struct ContractConfig {
    pub erc20: Address,
}

#[derive(Debug, Deserialize)]
pub struct TransferConfig {
    pub to: Address,
    pub amount: Decimal,
}

pub fn read_config<T: serde::de::DeserializeOwned>(
    path: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    use std::fs;

    let config_contents = fs::read_to_string(path)?;
    let cfg: T = toml::from_str(&config_contents)?;
    Ok(cfg)
}

pub fn read_provider_info(
    cfg: ProviderConfig,
) -> Result<(Url, Address, EthereumWallet), Box<dyn std::error::Error>> {
    use alloy::hex;

    let rpc_url = Url::parse(cfg.url.as_str())?;
    let private_key = PrivateKeySigner::from_bytes(&B256::from_slice(
        &hex::decode(cfg.pk.as_str())?,
    ))?;
    let address = private_key.address();
    let wallet = EthereumWallet::from(private_key);
    Ok((rpc_url, address, wallet))
}

//! Example of using the HTTP provider using the `on_http` method.

use alloy::providers::{Provider, ProviderBuilder};
use test_optimism::config::{read_config, read_provider_info, Config};
use test_optimism::{to_ether, to_wei};
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: Config = read_config("./config.toml")?;
    let (rpc_url, from, wallet) = read_provider_info(cfg.provider)?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    let erc20 = felix_contracts::ierc20::IERC20::new(
        cfg.contract.erc20,
        provider.clone(),
    );

    let to = cfg.transfer.to;
    let amount = to_wei(cfg.transfer.amount);

    for _i in 0..3 {
        let tx = erc20.transfer(to, amount).send().await?;
        let start_time = Instant::now();
        let tx = tx.watch().await?;
        let receipt = provider.get_transaction_receipt(tx).await?.unwrap();
        let elapsed = start_time.elapsed().as_millis();
        let gas_fee = receipt.gas_used * receipt.effective_gas_price;
        println!(
            "tx: {}\nblock_number: {}\ngas_fee: {}, gas_used: {}, gas_price: {}\nelapsed :{}",
            tx,
            receipt.block_number.unwrap(),
            gas_fee, receipt.gas_used, receipt.effective_gas_price,
            elapsed,
        );
    }

    let to_balance = erc20.balanceOf(cfg.transfer.to).call().await?._0;
    let from_balance = erc20.balanceOf(from).call().await?._0;
    println!(
        "\tfrom_balance: {}\n\tto_balance: {}",
        to_ether(from_balance),
        to_ether(to_balance)
    );
    Ok(())
}

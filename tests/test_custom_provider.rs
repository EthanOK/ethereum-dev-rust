mod common;
use alloy::{eips::BlockNumberOrTag, primitives::utils::format_ether, providers::Provider};
use common::{configs, constants};
use eyre::Result;

#[tokio::test]
async fn test_get_chain_id() -> Result<()> {
    let rpc_url = configs::get_rpc_url();
    let provider = configs::get_custom_provider(&rpc_url)?;
    let chain_id = provider.get_chain_id().await?;
    println!("chain_id: {}", chain_id);
    Ok(())
}

#[tokio::test]
async fn test_get_block_number() -> Result<()> {
    let rpc_url = configs::get_rpc_url();
    let provider = configs::get_custom_provider(&rpc_url)?;
    let block_number = provider.get_block_number().await?;
    println!("block_number: {}", block_number);
    Ok(())
}

#[tokio::test]
async fn test_get_balance() -> Result<()> {
    let rpc_url = configs::get_rpc_url();
    let provider = configs::get_custom_provider(&rpc_url)?;
    let address = constants::ZERO_ADDRESS;
    let balance = provider.get_balance(address).await?;
    println!("ZERO_ADDRESS balance: {} ether", format_ether(balance));
    Ok(())
}

#[tokio::test]
async fn test_get_dyn_provider() -> Result<()> {
    let rpc_url = configs::get_rpc_url();
    let provider = configs::get_provider(&rpc_url).await?;
    let transactions_count =
        provider.get_block_transaction_count_by_number(BlockNumberOrTag::Latest).await?;
    println!("transactions count: {}", transactions_count.unwrap());
    Ok(())
}

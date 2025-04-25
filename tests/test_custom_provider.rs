mod common;
use alloy::primitives::utils::format_ether;
use eyre::Result;

#[tokio::test]
async fn test_get_chain_id() -> Result<()> {
    let rpc_url = common::configs::get_rpc_url();
    let provider = common::configs::get_custom_provider(&rpc_url)?;
    let chain_id = provider.get_chain_id().await?;
    println!("chain_id: {}", chain_id);
    Ok(())
}

#[tokio::test]
async fn test_get_block_number() -> Result<()> {
    let rpc_url = common::configs::get_rpc_url();
    let provider = common::configs::get_custom_provider(&rpc_url)?;
    let block_number = provider.get_block_number().await?;
    println!("block_number: {}", block_number);
    Ok(())
}

#[tokio::test]
async fn test_get_balance() -> Result<()> {
    let rpc_url = common::configs::get_rpc_url();
    let provider = common::configs::get_custom_provider(&rpc_url)?;
    let address = common::constants::ZERO_ADDRESS;
    let balance = provider.get_balance(address).await?;
    println!("ZERO_ADDRESS balance: {} ether", format_ether(balance));
    Ok(())
}

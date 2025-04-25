mod common;
use alloy::{
    primitives::utils::{format_units, parse_ether},
    signers::local::PrivateKeySigner,
};
use common::{configs, constants};
use ethereum_dev::token::ERC20;
use eyre::{Ok, Result};

#[tokio::test]
async fn test_erc20_only_read() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();

    let provider = configs::get_provider(&rpc_url).await?;

    let erc20 = ERC20::new(constants::YGIO_ADDRESS, provider);

    let token_name = erc20.name().await?;
    let token_symbol = erc20.symbol().await?;
    let token_decimals = erc20.decimals().await?;
    let balance_token = erc20.balance_of(constants::ETHAN_ADDRESS).await?;
    let total_supply_token = erc20.total_supply().await?;

    println!("Token name: {token_name}");
    println!("{} total supply: {}", token_symbol, total_supply_token);
    println!("ethan's {} balance: {}", token_symbol, format_units(balance_token, token_decimals)?);

    Ok(())
}

#[tokio::test]
async fn test_transfer_erc20() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();
    let signer: PrivateKeySigner = configs::get_private_key().parse()?;

    let _from = signer.address();

    let to = constants::BOB_ADDRESS;

    let value = parse_ether("1")?;

    let provider = configs::get_provider_signer_fork(&rpc_url, signer).await?;

    let erc20 = ERC20::new(constants::YGIO_ADDRESS, provider);

    let balance_before = erc20.balance_of(to).await?;

    let tx_hash = erc20.transfer(to, value).await?;
    println!("tx_hash: {}", tx_hash);

    let balance_after = erc20.balance_of(to).await?;

    assert_eq!(balance_after - balance_before, value);

    Ok(())
}

#[tokio::test]
async fn test_transfer_from_erc20() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();
    let signer: PrivateKeySigner = configs::get_private_key().parse()?;

    let from = signer.address();

    let to = constants::BOB_ADDRESS;

    let value = parse_ether("1")?;

    let provider = configs::get_provider_signer_fork(&rpc_url, signer).await?;

    let erc20 = ERC20::new(constants::YGIO_ADDRESS, provider);

    let balance_before = erc20.balance_of(to).await?;

    // approve
    erc20.approve(from, value).await?;

    let tx_hash = erc20.transfer_from(from, to, value).await?;
    println!("tx_hash: {}", tx_hash);

    let balance_after = erc20.balance_of(to).await?;

    assert_eq!(balance_after - balance_before, value);

    Ok(())
}

mod common;

use alloy::{
    primitives::utils::{format_ether, parse_ether},
    providers::{CallItemBuilder, Provider},
};
use common::{configs, constants};
use ethereum_dev::IWETH9::IWETH9Instance;
use eyre::Result;

#[tokio::test]
async fn test_multicall_only_read() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();
    let ethan = constants::ETHAN_ADDRESS;
    let provider = configs::get_provider(&rpc_url).await?;
    let weth = IWETH9Instance::new(constants::WETH_ADDRESS_SEPOLIA, provider.clone());

    let multicall = provider
        .multicall()
        .add(weth.totalSupply())
        .add(weth.balanceOf(ethan))
        .get_eth_balance(ethan);

    let (total_supply, balance_weth, balance_eth) = multicall.aggregate().await?;

    println!("weth total_supply: {}", format_ether(total_supply));
    println!("ethan balance_weth: {}", format_ether(balance_weth));
    println!("ethan balance_eth: {}", format_ether(balance_eth));

    Ok(())
}

#[tokio::test]
async fn test_multicall() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();

    let ethan = constants::ETHAN_ADDRESS;
    let amount = parse_ether("2.5")?;

    let provider = configs::get_provider_fork(&rpc_url).await?;

    let weth = IWETH9Instance::new(constants::WETH_ADDRESS_SEPOLIA, provider.clone());

    let transfer_call = CallItemBuilder::new(weth.transfer(ethan, amount)).allow_failure(true);

    let deposit_call = CallItemBuilder::new(weth.deposit()).value(amount).allow_failure(true);

    let multicall = provider
        .multicall()
        .add(weth.balanceOf(ethan))
        .add_call(transfer_call.clone())
        .add_call(deposit_call)
        .add_call(transfer_call)
        .add(weth.balanceOf(ethan));

    let multicall_results = multicall.aggregate3_value().await?;
    // println!("multicall_results: {:?}", multicall_results);
    let (balance_before, transfer_result_failure, deposit_result, transfer_result, balance_after) =
        multicall_results;

    println!("balance_before: {}", format_ether(balance_before.clone()?));
    println!("transfer_result: {:?}", transfer_result_failure);
    println!("deposit_result: {:?}", deposit_result.is_ok());
    println!("transfer_result: {:?}", transfer_result);
    println!("balance_after: {}", format_ether(balance_after.clone()?));

    assert_eq!(amount, balance_after? - balance_before?);

    Ok(())
}

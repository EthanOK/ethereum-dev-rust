mod utils;

use std::env;

use alloy::{
    primitives::{
        address,
        utils::{format_ether, parse_units},
        U256,
    },
    sol,
};
use dotenv::dotenv;
use eyre::Result;
use utils::CustomProvider;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // a == b
    let a = U256::from(100_u32);

    let b = U256::from(100_u32);

    assert_eq!(a, b);

    let num = U256::from(100_u32);

    let num_u128 = num.to::<u128>();
    assert_eq!(num_u128, 100_u128);

    let num_u64 = num.to::<u64>();
    assert_eq!(num_u64, 100_u64);

    let num_string = num.to_string();
    assert_eq!(num_string, "100");

    let a = U256::from_str_radix("2a", 16)?;

    println!("a: {}", a);

    let amount_ether = "1.1";
    let amount_wei = parse_units(amount_ether, "ether")?;
    println!("{} ether = {} wei", amount_ether, amount_wei.to_string());

    println!("Hello, Alloy!");

    let rpc_url = env::var("RPC_URL").map_err(|_| eyre::eyre!("RPC_URL not found in .env file"))?;

    let provider = CustomProvider::new(&rpc_url)?;

    let latest_block = provider.get_block_number().await?;
    let chain_id = provider.get_chain_id().await?;

    let vitalik = address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = provider.get_balance(vitalik).await?;

    println!("Latest block number: {latest_block}");
    println!("Chain ID: {chain_id}");

    println!("Vitalik's balance: {} ether", format_ether(balance));

    Ok(())
}

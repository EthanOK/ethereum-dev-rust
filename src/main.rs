mod token;
mod utils;

use alloy::{
    primitives::{
        address,
        utils::{format_ether, format_units, parse_units},
        Bytes, U256,
    },
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;
use token::{CustomERC20, WETH9};
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
    let rpc_url_sepolia = env::var("RPC_URL_SEPOLIA")
        .map_err(|_| eyre::eyre!("RPC_URL_SEPOLIA not found in .env file"))?;
    let private_key =
        env::var("PRIVATE_KEY").map_err(|_| eyre::eyre!("PRIVATE_KEY not found in .env file"))?;

    let provider = CustomProvider::new(&rpc_url)?;

    let latest_block = provider.get_block_number().await?;
    let chain_id = provider.get_chain_id().await?;

    let vitalik = address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let balance = provider.get_balance(vitalik).await?;

    println!("Latest block number: {latest_block}");
    println!("Chain ID: {chain_id}");

    println!("Vitalik's balance: {} ether", format_ether(balance));

    // TODO: Transfer ETH
    println!("Transfer ETH");
    let bob = address!("0x53188E798f2657576c9de8905478F46ac2f24b67");
    let amount = parse_units("0.01", "ether")?.into();
    let input_data = Bytes::default();
    // let input_data = Bytes::from("hello bob");

    // TODO: new_with_signer or new_with_signer_fork
    let provider = CustomProvider::new_with_signer_fork(&rpc_url_sepolia, private_key.parse()?)?;
    let tx_hash = provider.send_transaction(bob, amount, input_data).await?;
    println!("Transaction hash: {tx_hash}");

    // TODO: Read Call ERC20 contract
    println!("Call ERC20 contract");

    let alice = address!("0x6278A1E803A76796a3A1f7F6344fE874ebfe94B2");

    let ygio = address!("0x5Bb9dE881543594D17a7Df91D62459024c4EEf02");

    let provider = ProviderBuilder::new().connect(&rpc_url_sepolia).await?;
    // let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

    let erc20 = CustomERC20::new(ygio, provider);

    let token_name = erc20.name().await?;
    let token_symbol = erc20.symbol().await?;
    let token_decimals = erc20.decimals().await?;
    let balance_token = erc20.balance_of(alice).await?;
    let total_supply_token = erc20.total_supply().await?;

    println!("Token name: {token_name}");
    println!("{} total supply: {}", token_symbol, total_supply_token);
    println!("alice's {} balance: {}", token_symbol, format_units(balance_token, token_decimals)?);

    // TODO: Write Call ERC20 contract
    println!("Write Call ERC20 contract");

    let signer: PrivateKeySigner = private_key.parse()?;

    // let provider_signer = ProviderBuilder::new().wallet(signer).connect(&rpc_url_sepolia).await?;
    let provider_signer_fork = ProviderBuilder::new()
        .wallet(signer)
        .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url_sepolia))?;
    let erc20 = CustomERC20::new(ygio, &provider_signer_fork);

    let amount = parse_units("0.1", "ether")?.into();
    let tx_hash = erc20.transfer(alice, amount).await?;
    println!("ERC20 Transfer tx hash: {tx_hash}");

    let weth_sepolia = address!("0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14");

    let weth9 = WETH9::new(weth_sepolia, &provider_signer_fork);
    let balance = weth9.balance_of(alice).await?;
    println!("alice WETH9 balance: {balance}");
    println!("deposit");
    let tx_hash = weth9.deposit(amount).await?;
    println!("WETH9 deposit tx hash: {tx_hash}");
    let balance = weth9.balance_of(alice).await?;
    println!("alice WETH9 balance: {balance}");
    println!("withdraw");
    let tx_hash = weth9.withdraw(amount).await?;
    println!("WETH9 withdraw tx hash: {tx_hash}");
    let balance = weth9.balance_of(alice).await?;
    println!("alice WETH9 balance: {balance}");

    Ok(())
}

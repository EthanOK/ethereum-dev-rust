//! Example of subscribing and listening for specific contract events by `WebSocket` subscription.

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
};
use eyre::Result;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // Create the provider.
    let alchemy_api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", alchemy_api_key);
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Create a filter to watch for UNI token transfers.
    let uniswap_token_address = address!("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984");

    let filter =
        Filter::new().address(uniswap_token_address).from_block(BlockNumberOrTag::Number(22352699));

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        println!("Uniswap token logs: {log:?}");
    }

    Ok(())
}

use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol_types::SolEvent,
};
use ethereum_dev::{
    IERC20::{Approval, Transfer},
    IERC721,
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

    let transfer_signature = Transfer::SIGNATURE;
    let approve_signature = Approval::SIGNATURE;

    // TODO: Add more topics.
    let events = vec![transfer_signature, approve_signature];

    let filter =
        Filter::new().events(events.into_iter()).from_block(BlockNumberOrTag::Number(22352699));

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let topics_len = log.topics().len();
        let topic0 = log.topics()[0];

        match topic0 {
            topic0 if topic0 == Transfer::SIGNATURE_HASH => {
                if topics_len == 3 {
                    let (_topic0, from, to) = Transfer::decode_topics(log.topics())?;
                    let (value,) = Transfer::abi_decode_data(&log.data().data)?;
                    println!("Contract Address: {:?}", log.address());
                    println!("ERC20 Transfer: {from} -> {to} value:{}", value);
                } else if topics_len == 4 {
                    let (_topic0, from, to, token_id) =
                        IERC721::Transfer::decode_topics(log.topics())?;
                    println!("Contract Address: {:?}", log.address());
                    println!("ERC721 Transfer: {from} -> {to} token_id:{}", token_id);
                }
            }
            topic0 if topic0 == Approval::SIGNATURE_HASH => {
                if topics_len == 3 {
                    let (_topic0, owner, spender) = Approval::decode_topics(log.topics())?;
                    let (value,) = Approval::abi_decode_data(&log.data().data)?;
                    println!("Contract Address: {:?}", log.address());
                    println!("ERC20 Approval: {owner} -> {spender} value:{}", value);
                } else if topics_len == 4 {
                    let (_topic0, owner, spender, token_id) =
                        IERC721::Approval::decode_topics(log.topics())?;
                    println!("Contract Address: {:?}", log.address());
                    println!("ERC721 Approval: {owner} -> {spender} token_id:{}", token_id);
                }
            }
            _ => {
                println!("No Data");
            }
        }
    }

    Ok(())
}

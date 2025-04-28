use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol_types::SolEvent,
};
use ethereum_dev::{
    erc20_transfer_model::ActiveModel,
    erc721_transfer_model, get_mysql_connection_env, handle_erc20_transfer_event,
    handle_erc721_transfer_event, update_config_map_value, FILTER_START_BLOCK_NUMBER,
    IERC20::{Approval, Transfer},
    IERC721,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use sea_orm::ActiveValue::{NotSet, Set};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // Create the provider.
    let alchemy_api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
    let rpc_url = format!("wss://eth-mainnet.g.alchemy.com/v2/{}", alchemy_api_key);
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let db = get_mysql_connection_env().await?;

    let transfer_signature = Transfer::SIGNATURE;
    let approve_signature = Approval::SIGNATURE;

    // TODO: Add more topics.
    let events = vec![transfer_signature, approve_signature];

    let filter = Filter::new().events(events.into_iter()).from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    let mut current_block_number: Option<u64> = None;

    while let Some(log) = stream.next().await {
        let topics_len = log.topics().len();
        let topic0 = log.topics()[0];
        let block_number = log.block_number.unwrap();
        let timestamp = chrono::Utc::now().timestamp() as u64;

        if current_block_number.is_none() {
            current_block_number = Some(block_number);
        } else if current_block_number.is_some() && block_number != current_block_number.unwrap() {
            println!("此区块已完成: {}", current_block_number.unwrap());
            let _ = update_config_map_value(
                FILTER_START_BLOCK_NUMBER,
                current_block_number.unwrap().to_string().as_str(),
                db.clone(),
            )
            .await?;
            println!("---------------------------------------------------------------------------");
            current_block_number = Some(block_number);
        }

        match topic0 {
            topic0 if topic0 == Transfer::SIGNATURE_HASH => {
                if topics_len == 3 {
                    let (_topic0, from, to) = Transfer::decode_topics(log.topics())?;
                    let (value,) = Transfer::abi_decode_data(&log.data().data)?;

                    let active_model = ActiveModel {
                        id: NotSet,
                        token: Set(log.address().to_string()),
                        from: Set(from.to_string()),
                        to: Set(to.to_string()),
                        value: Set(value.to_string()),
                        block_number: Set(log.block_number.unwrap()),
                        timestamp: Set(log.block_timestamp.unwrap_or_else(|| timestamp)),
                        tx_hash: Set(log.transaction_hash.unwrap().to_string()),
                        created_at: NotSet,
                        updated_at: NotSet,
                    };
                    // TODO: handle erc20 transfer event
                    handle_erc20_transfer_event(active_model, db.clone()).await?;
                } else if topics_len == 4 {
                    let (_topic0, from, to, token_id) =
                        IERC721::Transfer::decode_topics(log.topics())?;
                    println!("Contract Address: {:?}", log.address());
                    println!("ERC721 Transfer: {from} -> {to} token_id:{}", token_id);

                    let active_model = erc721_transfer_model::ActiveModel {
                        id: NotSet,
                        token: Set(log.address().to_string()),
                        from: Set(from.to_string()),
                        to: Set(to.to_string()),
                        token_id: Set(token_id.to_string()),
                        block_number: Set(log.block_number.unwrap()),
                        timestamp: Set(log.block_timestamp.unwrap_or_else(|| timestamp)),
                        tx_hash: Set(log.transaction_hash.unwrap().to_string()),
                        created_at: NotSet,
                        updated_at: NotSet,
                    };
                    // TODO: handle erc721 transfer event
                    handle_erc721_transfer_event(active_model, db.clone()).await?;
                }
            }
            topic0 if topic0 == Approval::SIGNATURE_HASH => {
                if topics_len == 3 {
                    // let (_topic0, owner, spender) = Approval::decode_topics(log.topics())?;
                    // let (value,) = Approval::abi_decode_data(&log.data().data)?;
                    // println!("Contract Address: {:?}", log.address());
                    // println!("ERC20 Approval: {owner} -> {spender} value:{}", value);
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

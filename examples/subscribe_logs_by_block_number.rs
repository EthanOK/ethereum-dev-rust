use std::collections::HashMap;

use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder, WsConnect},
    sol_types::SolEvent,
};
use ethereum_dev::{
    filter_logs_by_block_number, get_config_map_value, get_mysql_connection_env, handle_log,
    FILTER_START_BLOCK_NUMBER,
    IERC20::{Approval, Transfer},
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // Create the provider.
    let alchemy_api_key = std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY not set");
    let rpc_url = format!("wss://eth-sepolia.g.alchemy.com/v2/{}", alchemy_api_key);
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let db = get_mysql_connection_env().await?;

    let transfer_signature = Transfer::SIGNATURE;
    let approve_signature = Approval::SIGNATURE;

    // TODO: Add more topics.
    let events = vec![transfer_signature, approve_signature];

    // let filter = Filter::new().events(events.into_iter()).from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    // let sub = provider.subscribe_logs(&filter).await?;

    let lastest_block_number = provider.get_block_number().await?;
    let datebase_block_number =
        get_config_map_value(FILTER_START_BLOCK_NUMBER, db.clone()).await?.parse::<u64>()?;

    if datebase_block_number < lastest_block_number {
        println!(
            "First: Please handle the logs from {} to {}.",
            datebase_block_number, lastest_block_number
        );

        let logs = filter_logs_by_block_number(
            provider.clone(),
            events.into_iter(),
            lastest_block_number - 10,
            lastest_block_number,
        )
        .await?;

        let mut block_timestamps: HashMap<u64, u64> = HashMap::new();

        for log in logs {
            let block_number = log.block_number.unwrap();
            if log.block_number.is_some() && !block_timestamps.contains_key(&block_number) {
                let block_timestamp = provider
                    .clone()
                    .get_block_by_number(BlockNumberOrTag::Number(block_number))
                    .await?
                    .unwrap()
                    .header
                    .timestamp;
                block_timestamps.insert(log.block_number.unwrap(), block_timestamp);
            }

            if block_timestamps.contains_key(&(block_number - 1)) {
                // remove the block_timestamp from the map
                block_timestamps.remove(&(block_number - 1));
            }

            let block_timestamp = block_timestamps.get(&block_number).unwrap();

            handle_log(db.clone(), log, *block_timestamp).await?;
        }
    }

    Ok(())
}

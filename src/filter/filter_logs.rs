use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::{Filter, Log};
use eyre::Result;

pub async fn filter_logs_by_block_number(
    provider: impl Provider,
    events: impl IntoIterator<Item = impl AsRef<[u8]>>,
    start_block_number: u64,
    end_block_number: u64,
) -> Result<Vec<Log>> {
    if end_block_number - start_block_number > 2000 {
        return Err(eyre::eyre!("block number range is too large"));
    }
    let filter =
        Filter::new().from_block(start_block_number).to_block(end_block_number).events(events);
    let logs = provider.get_logs(&filter).await?;

    Ok(logs)
}

pub async fn filter_logs_by_block_number_at_address(
    provider: impl Provider,
    address: Address,
    events: impl IntoIterator<Item = impl AsRef<[u8]>>,
    start_block_number: u64,
    end_block_number: u64,
) -> Result<Vec<Log>> {
    let filter = Filter::new()
        .address(address)
        .from_block(start_block_number)
        .to_block(end_block_number)
        .events(events);
    let logs = provider.get_logs(&filter).await?;

    Ok(logs)
}

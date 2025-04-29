use alloy::rpc::types::Log;
use eyre::Result;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DatabaseConnection,
};

use crate::{
    erc20_transfer_model::ActiveModel,
    erc721_transfer_model,
    IERC20::{Approval, Transfer},
    IERC721,
};
use alloy::sol_types::SolEvent;

use super::{handle_erc20_transfer_event, handle_erc721_transfer_event};

pub async fn handle_log(db: DatabaseConnection, log: Log, block_timestamp: u64) -> Result<()> {
    let topics_len = log.topics().len();
    let topic0 = log.topics()[0];
    // let timestamp = chrono::Utc::now().timestamp() as u64;

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
                    // timestamp: Set(log.block_timestamp.unwrap_or_else(|| timestamp)),
                    timestamp: Set(block_timestamp),
                    tx_hash: Set(log.transaction_hash.unwrap().to_string()),
                    index: Set(log.log_index.unwrap()),
                    created_at: NotSet,
                    updated_at: NotSet,
                };
                // TODO: handle erc20 transfer event
                handle_erc20_transfer_event(active_model, db.clone()).await?;
            } else if topics_len == 4 {
                let (_topic0, from, to, token_id) = IERC721::Transfer::decode_topics(log.topics())?;
                println!("Contract Address: {:?}", log.address());
                println!("ERC721 Transfer: {from} -> {to} token_id: {}", token_id);
                println!("Transaction Hash: {:?}", log.transaction_hash.unwrap());

                let active_model = erc721_transfer_model::ActiveModel {
                    id: NotSet,
                    token: Set(log.address().to_string()),
                    from: Set(from.to_string()),
                    to: Set(to.to_string()),
                    token_id: Set(token_id.to_string()),
                    block_number: Set(log.block_number.unwrap()),
                    // timestamp: Set(log.block_timestamp.unwrap_or_else(|| timestamp)),
                    timestamp: Set(block_timestamp),
                    tx_hash: Set(log.transaction_hash.unwrap().to_string()),
                    index: Set(log.log_index.unwrap()),
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
    Ok(())
}

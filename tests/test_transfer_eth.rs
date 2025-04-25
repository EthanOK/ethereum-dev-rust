mod common;
use alloy::{
    primitives::{utils::parse_ether, Bytes},
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::test]
async fn test_transfer_eth() -> Result<()> {
    let rpc_url = common::configs::get_rpc_url_sepolia();
    let private_key: PrivateKeySigner = common::configs::get_private_key().parse()?;
    let provider = common::configs::get_custom_provider_signer_fork(&rpc_url, private_key)?;

    let to = common::constants::ZERO_ADDRESS;
    let value = parse_ether("0.1")?;
    let input_data = Bytes::from("burn ether");

    let tx_hash = provider.send_transaction(to, value, input_data).await?;
    println!("tx_hash: {}", tx_hash);
    Ok(())
}

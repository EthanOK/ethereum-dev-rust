mod common;

use common::{configs, constants};
use ethereum_dev::ERC721;
use eyre::{Ok, Result};

#[tokio::test]
async fn test_erc721_only_read() -> Result<()> {
    let rpc_url = configs::get_rpc_url_sepolia();

    let provider = configs::get_provider(&rpc_url).await?;

    let erc721 = ERC721::new(constants::YGME_ADDRESS, provider);

    let token_name = erc721.name().await?;
    let balance_token = erc721.balance_of(constants::ETHAN_ADDRESS).await?;

    println!("NFT Name: {token_name}");
    println!("NFT Balance: {balance_token}");

    Ok(())
}

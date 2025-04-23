use alloy::{
    eips::BlockId,
    primitives::{Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Block, TransactionInput, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use eyre::Result;

pub struct CustomProvider {
    provider: Box<dyn Provider>,
}

#[allow(dead_code)]
impl CustomProvider {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let rpc_url = rpc_url.parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        Ok(Self { provider: Box::new(provider) })
    }
    pub fn new_with_signer(rpc_url: &str, signer: PrivateKeySigner) -> Result<Self> {
        let rpc_url = rpc_url.parse()?;
        let provider = ProviderBuilder::new().wallet(signer).on_http(rpc_url);
        Ok(Self { provider: Box::new(provider) })
    }

    pub fn new_with_signer_fork(rpc_url: &str, signer: PrivateKeySigner) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
        Ok(Self { provider: Box::new(provider) })
    }

    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number)
    }

    pub async fn get_chain_id(&self) -> Result<u64> {
        let chain_id = self.provider.get_chain_id().await?;
        Ok(chain_id)
    }

    pub async fn get_balance(&self, address: Address) -> Result<U256> {
        let balance = self.provider.get_balance(address).await?;
        Ok(balance)
    }

    pub async fn get_block(&self) -> Result<Option<Block>> {
        let block = self.provider.get_block(BlockId::latest()).await?;
        Ok(block)
    }

    pub async fn send_transaction(
        &self,
        to: Address,
        value: U256,
        input_data: Bytes,
    ) -> Result<String> {
        let tx = TransactionRequest::default()
            .to(to)
            .value(value)
            .input(TransactionInput::both(input_data));

        let tx = self.provider.send_transaction(tx).await?;
        let transaction_receipt = tx.get_receipt().await?;

        println!(
            "from: {}  to: {}",
            transaction_receipt.from,
            transaction_receipt.to.unwrap_or_default()
        );

        let transaction_hash = transaction_receipt.transaction_hash;

        Ok(transaction_hash.to_string())
    }
}

// src/block/block.rs
use alloy::{
    eips::BlockId, primitives::{Address, U256}, providers::{Provider, ProviderBuilder}, rpc::types::Block
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
        Ok(Self {
            provider: Box::new(provider),
        })
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
}

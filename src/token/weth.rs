use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol,
};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "./abi/IWETH9.json"
);

pub struct WETH9<P: Provider> {
    weth9: IWETH9::IWETH9Instance<P>,
}

#[allow(dead_code)]
impl<P: Provider> WETH9<P> {
    pub fn new(address: Address, provider: P) -> Self {
        Self { weth9: IWETH9::IWETH9Instance::new(address, provider) }
    }
    pub async fn deposit(&self, value: U256) -> Result<String> {
        let transaction_hash = self.weth9.deposit().value(value).send().await?.watch().await?;

        Ok(transaction_hash.to_string())
    }

    pub async fn withdraw(&self, value: U256) -> Result<String> {
        let transaction_hash = self.weth9.withdraw(value).send().await?.watch().await?;

        Ok(transaction_hash.to_string())
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        let balance = self.weth9.balanceOf(owner).call().await?;

        Ok(balance)
    }
}

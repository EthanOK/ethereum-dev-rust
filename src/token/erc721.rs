use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol,
};
use eyre::Result;

// Generate the contract bindings for the ERC721 interface.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC721,
    "./abi/IERC721.json"
);

// Create a custom ERC721 contract wrapper.
pub struct ERC721<P: Provider> {
    erc721: IERC721::IERC721Instance<P>,
}

#[allow(dead_code)]
impl<P: Provider> ERC721<P> {
    pub fn new(address: Address, provider: P) -> Self {
        let erc721 = IERC721::new(address, provider);
        Self { erc721 }
    }

    pub async fn name(&self) -> Result<String> {
        let name = self.erc721.name().call().await?;
        Ok(name)
    }
    pub async fn symbol(&self) -> Result<String> {
        let symbol = self.erc721.symbol().call().await?;
        Ok(symbol)
    }

    pub async fn token_uri(&self, token_id: U256) -> Result<String> {
        let token_uri = self.erc721.tokenURI(token_id).call().await?;
        Ok(token_uri)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        let balance = self.erc721.balanceOf(owner).call().await?;
        Ok(balance)
    }

    pub async fn owner_of(&self, token_id: U256) -> Result<Address> {
        let owner = self.erc721.ownerOf(token_id).call().await?;
        Ok(owner)
    }

    pub async fn approve(&self, spender: Address, token_id: U256) -> Result<String> {
        let approve_tx = self.erc721.approve(spender, token_id).send().await?;
        let approve = approve_tx.get_receipt().await?;
        let tx_hash = approve.transaction_hash;
        Ok(tx_hash.to_string())
    }

    pub async fn set_approval_for_all(&self, operator: Address, approved: bool) -> Result<String> {
        let tx_hash =
            self.erc721.setApprovalForAll(operator, approved).send().await?.watch().await?;
        Ok(tx_hash.to_string())
    }

    pub async fn transfer_from(&self, from: Address, to: Address, value: U256) -> Result<String> {
        let tx_hash = self.erc721.transferFrom(from, to, value).send().await?.watch().await?;
        Ok(tx_hash.to_string())
    }
}

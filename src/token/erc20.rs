use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol,
};
use eyre::Result;

// Generate the contract bindings for the ERC20 interface.
sol! {
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)]
   contract ERC20 {
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);

        function name() external view returns (string memory);
        function symbol() external view returns (string memory);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address owner) public view returns (uint256);
        function allowance(address owner, address spender) external view returns (uint256);

        function transfer(address to, uint256 value) external returns (bool);
        function approve(address spender, uint256 value) external returns (bool);
        function transferFrom(address from, address to, uint256 value) external returns (bool);
   }
}

// Create a custom ERC20 contract wrapper.
pub struct CustomERC20<P: Provider> {
    erc20: ERC20::ERC20Instance<P>,
}

#[allow(dead_code)]
impl<P: Provider> CustomERC20<P> {
    pub fn new(address: Address, provider: P) -> Self {
        let erc20 = ERC20::new(address, provider);
        Self { erc20 }
    }

    pub async fn name(&self) -> Result<String> {
        let name = self.erc20.name().call().await?;
        Ok(name)
    }
    pub async fn symbol(&self) -> Result<String> {
        let symbol = self.erc20.symbol().call().await?;
        Ok(symbol)
    }
    pub async fn decimals(&self) -> Result<u8> {
        let decimals = self.erc20.decimals().call().await?;
        Ok(decimals)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        let balance = self.erc20.balanceOf(owner).call().await?;
        Ok(balance)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        let total_supply = self.erc20.totalSupply().call().await?;
        Ok(total_supply)
    }
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        let allowance = self.erc20.allowance(owner, spender).call().await?;
        Ok(allowance)
    }

    pub async fn transfer(&self, to: Address, value: U256) -> Result<String> {
        let transfer_tx = self.erc20.transfer(to, value).send().await?;
        let transfer = transfer_tx.get_receipt().await?;
        let tx_hash = transfer.transaction_hash;
        Ok(tx_hash.to_string())
    }
}

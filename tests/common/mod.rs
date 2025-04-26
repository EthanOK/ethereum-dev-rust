#[allow(dead_code)]
pub mod constants {
    use alloy::primitives::{address, Address};

    pub const ZERO_ADDRESS: Address = address!("0x0000000000000000000000000000000000000000");
    pub const YGIO_ADDRESS: Address = address!("0x5Bb9dE881543594D17a7Df91D62459024c4EEf02");
    pub const WETH_ADDRESS: Address = address!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    pub const USDC_ADDRESS: Address = address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    pub const USDT_ADDRESS: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");

    pub const ETHAN_ADDRESS: Address = address!("0x6278A1E803A76796a3A1f7F6344fE874ebfe94B2");
    pub const BOB_ADDRESS: Address = address!("0x53188E798f2657576c9de8905478F46ac2f24b67");
}

#[allow(dead_code)]
pub mod configs {
    use alloy::{
        providers::{DynProvider, Provider, ProviderBuilder},
        signers::local::PrivateKeySigner,
    };
    use dotenv::dotenv;
    use ethereum_dev::utils::custom_provider::CustomProvider;
    use eyre::Result;
    use std::env;

    pub fn get_custom_provider(rpc_url: &str) -> Result<CustomProvider> {
        let custom_provider = CustomProvider::new(rpc_url)?;
        Ok(custom_provider)
    }

    pub fn get_custom_provider_signer(
        rpc_url: &str,
        signer: PrivateKeySigner,
    ) -> Result<CustomProvider> {
        let custom_provider = CustomProvider::new_with_signer(rpc_url, signer)?;
        Ok(custom_provider)
    }

    pub fn get_custom_provider_signer_fork(
        rpc_url: &str,
        signer: PrivateKeySigner,
    ) -> Result<CustomProvider> {
        let custom_provider = CustomProvider::new_with_signer_fork(rpc_url, signer)?;
        Ok(custom_provider)
    }

    pub fn get_rpc_url() -> String {
        dotenv().ok();
        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
        rpc_url
    }

    pub fn get_rpc_url_sepolia() -> String {
        dotenv().ok();
        let rpc_url = env::var("RPC_URL_SEPOLIA").expect("RPC_URL_SEPOLIA must be set");
        rpc_url
    }
    pub fn get_private_key() -> String {
        dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
        private_key
    }

    pub async fn get_provider(rpc_url: &str) -> Result<DynProvider> {
        let provider = ProviderBuilder::new().connect(rpc_url).await?;
        let dyn_provider = provider.erased();
        Ok(dyn_provider)
    }

    pub async fn get_provider_signer(
        rpc_url: &str,
        signer: PrivateKeySigner,
    ) -> Result<DynProvider> {
        let provider = ProviderBuilder::new().wallet(signer).connect(rpc_url).await?;
        let dyn_provider = provider.erased();
        Ok(dyn_provider)
    }

    pub async fn get_provider_signer_fork(
        rpc_url: &str,
        signer: PrivateKeySigner,
    ) -> Result<DynProvider> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url))?;
        let dyn_provider = provider.erased();
        Ok(dyn_provider)
    }
}

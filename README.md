# ethereum rust client

docs: https://alloy.rs

## New Provider

```rust
use alloy::providers::{Provider, ProviderBuilder};
```

- 1. new provider only read from blockchain

```rust
let provider = ProviderBuilder::new().connect(rpc_url_sepolia).await?;
```

- 2. new provider can read and write to blockchain with private key

```rust
let signer: PrivateKeySigner = "0x12345544".parse()?;

let provider_signer = ProviderBuilder::new().wallet(signer).connect(rpc_url_sepolia).await?;
```

- 3. new fork provider can read and write to blockchain with private key with anvil

set `node-bindings`

```
alloy = { version = "0.14.0", features = ["full","node-bindings"] }
```

```rust
let signer: PrivateKeySigner = "0x12345544".parse()?;

let provider_signer_fork = ProviderBuilder::new()
    .wallet(signer)
    .on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url_sepolia))?;
```

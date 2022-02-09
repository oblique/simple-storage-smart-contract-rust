use anyhow::{Context, Result};
use ethers::abi::Abi;
use ethers::prelude::*;
use std::env;
use std::sync::Arc;
use std::time::Duration;

abigen!(SimpleStorage, "$OUT_DIR/SimpleStorage.abi");

static SIMPLESTORAGE_BYTECODE: Lazy<Bytes> = Lazy::new(|| {
    let hex_str = include_str!(concat!(env!("OUT_DIR"), "/SimpleStorage.bin"));
    let bin = hex::decode(hex_str).expect("Invalid bytecode");
    bin.into()
});

async fn deploy_contract<M>(
    abi: Abi,
    bytecode: Bytes,
    client: Arc<M>,
) -> Result<Contract<M>>
where
    M: Middleware + 'static,
{
    let contract = ContractFactory::new(abi, bytecode, client)
        .deploy(())?
        .legacy()
        .send()
        .await?;

    Ok(contract)
}

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = env::var("WALLET_PRIV_KEY")
        .unwrap_or_default()
        .trim_start_matches("0x")
        .parse::<LocalWallet>()
        .context("Failed to parse wallet private key")?;

    let endpoint =
        env::var("NETWORK_URL").context("NETWORK_URL is not specified")?;
    let provider = Provider::<Http>::try_from(endpoint)
        .context("Failed to create network provider")?
        .interval(Duration::from_millis(10));

    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let contract = deploy_contract(
        SIMPLESTORAGE_ABI.clone(),
        SIMPLESTORAGE_BYTECODE.clone(),
        client.clone(),
    )
    .await?;

    println!("contract address: {:?}", contract.address());
    let simple_storage = SimpleStorage::new(contract.address(), client.clone());

    let data = simple_storage.get_data().legacy().call().await?;
    println!("data before set: {}", data);

    simple_storage.set_data(44557.into()).legacy().send().await?;

    let data = simple_storage.get_data().legacy().call().await?;
    println!("data after set: {}", data);

    Ok(())
}

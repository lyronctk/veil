use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction, utils};
use std::sync::Arc;
mod watchtower;
use eyre::Result;
use std::env;
use watchtower::Watchtower;

const HTTP_PROVIDER: &str = "https://mainnet.infura.io/v3/24cbf7ab0f8c4621ab876e6b67b68a3d";

#[tokio::main]
async fn main() -> Result<()> {
    // Abi Generator
    // Abigen::new("Watchtower", "./abi.json")?
    //     .generate()?
    //     .write_to_file("watchtower.rs")?;

    // Setup
    let private_key = env::var("HACKLODGE_PRIVATE_KEY")?;
    let wallet = private_key.parse::<LocalWallet>()?;
    let address = "0xA808B22ffd2c472aD1278088F16D4010E6a54D5F".parse::<Address>()?;
    let provider = Provider::<Http>::try_from(HTTP_PROVIDER)?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Generate calldata
    let contract = Watchtower::new(address, client.clone());
    let tx = contract.approve(address, U256::from(100));
    let tx = tx.tx.as_eip1559_ref().unwrap();
    let data = tx.data.as_ref().unwrap().clone();

    // Presign transactions
    for nonce in 0..1000 {
        let tx: TypedTransaction = TransactionRequest::new()
            .to(address)
            .value(1000)
            .from(address)
            .nonce(nonce)
            .data(data.clone())
            .into();
        let signature = client.signer().sign_transaction_sync(&tx);
        let raw_tx = tx.rlp_signed(&signature);
        let rlp = utils::serialize(&raw_tx);
        // broadcast later
    }

    Ok(())
}

/*
problem: can't presign gas price ?
*/

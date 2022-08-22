use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
use eyre::Result;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

mod watchtower;
use watchtower::Watchtower;

mod erc20;
use erc20::ERC20;

const GAS_STEP: usize = 10;
const MIN_GAS: usize = 10;
const MAX_GAS: usize = 100;
const HTTP_PROVIDER: &str = "https://mainnet.infura.io/v3/24cbf7ab0f8c4621ab876e6b67b68a3d";
const ERC20_ADDRESSES: [&str; 3] = [
    "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
    "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9",
    "0xA808B22ffd2c472aD1278088F16D4010E6a54D5F",
];

#[tokio::main]
async fn main() -> Result<()> {
    // Abigen::new("Watchtower", "./abi.json")?
    //     .generate()?
    //     .write_to_file("watchtower.rs")?;

    // Abigen::new("ERC20", "./erc20.json")?
    //     .generate()?
    //     .write_to_file("erc20.rs")?;

    // Setup
    let address = env::var("HACKLODGE_ADDRESS")?.parse::<Address>()?;
    let private_key = env::var("HACKLODGE_PRIVATE_KEY")?;
    let wallet = private_key.parse::<LocalWallet>()?;
    let provider = Provider::<Http>::try_from(HTTP_PROVIDER)?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Generate calldata
    let contract = Watchtower::new(address, client.clone());
    let tx = contract.approve(address, U256::from(100));
    let tx = tx.tx.as_eip1559_ref().unwrap();
    let data = tx.data.as_ref().unwrap().clone();

    // Presign rescue transactions
    let mut buffer = File::create("presigned.csv")?;
    for nonce in 0..1000 {
        for gas_price in (MIN_GAS..MAX_GAS).step_by(GAS_STEP) {
            let tx: TypedTransaction = TransactionRequest::new()
                .to(address)
                .value(1000)
                .from(address)
                .nonce(nonce)
                .data(data.clone())
                .gas_price(gas_price)
                .into();
            let signature = client.signer().sign_transaction_sync(&tx);
            let raw_tx = tx.rlp_signed(&signature);
            let rlp = serde_json::to_string(&raw_tx)?;
            buffer.write(format!("{},{},{}\n", rlp, nonce, gas_price).as_bytes())?;
        }
    }
    buffer.flush()?;

    // Presign approve transactions
    let mut buffer = File::create("approves.csv")?;
    ERC20_ADDRESSES.map(|s| {
        let erc20_address = s.parse::<Address>().unwrap();
        let contract = ERC20::new(erc20_address, client.clone());
        let tx = contract.approve(address, U256::max_value());
        let tx = tx.tx.as_eip1559_ref().unwrap();
        let data = tx.data.as_ref().unwrap().clone();

        let tx: TypedTransaction = TransactionRequest::new()
            .to(erc20_address)
            .from(address)
            .data(data)
            .into();

        let signature = client.signer().sign_transaction_sync(&tx);
        let raw_tx = tx.rlp_signed(&signature);
        let rlp = serde_json::to_string(&raw_tx).unwrap();
        buffer.write(format!("{}\n", rlp).as_bytes()).unwrap();
    });
    buffer.flush()?;

    Ok(())
}

/*
problem: can't presign gas price ?
*/

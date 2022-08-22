use clap::Parser;
use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

mod watchtower;
use watchtower::Watchtower;

mod erc20;
use erc20::ERC20;

const ERC20_ADDRESSES: [&str; 3] = [
    "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", // Uniswap
    "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", // Aave
    "0xA808B22ffd2c472aD1278088F16D4010E6a54D5F", // ReFi
];

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(long)]
    private_key: String,
    #[clap(long)]
    backup_address: String,
    #[clap(long)]
    contract_address: String,
    #[clap(long)]
    min_gas: usize,
    #[clap(long)]
    max_gas: usize,
    #[clap(long)]
    gas_step: usize,
    #[clap(long)]
    nonce: usize,
    #[clap(long)]
    output_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Abigen::new("Watchtower", "./abi.json")?
    //     .generate()?
    //     .write_to_file("watchtower.rs")?;

    // Abigen::new("ERC20", "./erc20.json")?
    //     .generate()?
    //     .write_to_file("erc20.rs")?;

    // Setup
    let args = Arguments::parse();
    let private_key = args.private_key;
    let wallet = private_key.parse::<LocalWallet>()?;
    let user_address: Address = wallet.address();
    let backup_address = args.backup_address.parse::<Address>()?;
    let contract_address = args.contract_address.parse::<Address>()?;
    let start_nonce = args.nonce;
    let output_path = args.output_path;

    let min_gas = args.min_gas;
    let max_gas = args.max_gas;
    let gas_step = args.gas_step;

    let provider = Provider::<Http>::try_from("http://x.com")?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Generate calldata
    let contract = Watchtower::new(contract_address, client.clone());
    let tx = contract.rescue_assets(
        ERC20_ADDRESSES
            .map(|s| s.parse::<Address>().unwrap())
            .into(),
        backup_address,
    );
    let tx = tx.tx.as_eip1559_ref().unwrap();
    let data = tx.data.as_ref().unwrap().clone();

    // Presign rescue transactions
    let mut buffer = File::create(output_path)?;
    for nonce in start_nonce..(start_nonce + 1000) {
        for gas_price in (min_gas..max_gas).step_by(gas_step) {
            let tx: TypedTransaction = TransactionRequest::new()
                .to(backup_address)
                .from(user_address)
                .nonce(nonce)
                .data(data.clone())
                .gas_price(gas_price)
                .into();
            let signature = client.signer().sign_transaction_sync(&tx);
            let raw_tx = tx.rlp_signed(&signature);
            let rlp = serde_json::to_string(&raw_tx)?;
            buffer.write(format!("rescue,{},{},{}\n", rlp, nonce, gas_price).as_bytes())?;
        }
    }

    // Presign approve transactions
    let mut offset: usize = 0;
    ERC20_ADDRESSES.map(|s| {
        let erc20_address = s.parse::<Address>().unwrap();
        let contract = ERC20::new(erc20_address, client.clone());
        let tx = contract.approve(contract_address, U256::max_value());
        let tx = tx.tx.as_eip1559_ref().unwrap();
        let data = tx.data.as_ref().unwrap().clone();

        let tx: TypedTransaction = TransactionRequest::new()
            .to(erc20_address)
            .from(user_address)
            .data(data)
            .nonce(start_nonce + offset)
            .into();

        let signature = client.signer().sign_transaction_sync(&tx);
        let raw_tx = tx.rlp_signed(&signature);
        let rlp = serde_json::to_string(&raw_tx).unwrap();
        buffer
            .write(format!("approve,{},,\n", rlp).as_bytes())
            .unwrap();
        offset += 1;
    });
    buffer.flush()?;

    Ok(())
}

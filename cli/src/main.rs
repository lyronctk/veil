use clap::Parser;
use ethers::types::transaction::eip2930::AccessList;
use ethers::{prelude::*, types::transaction::eip2718::TypedTransaction};
use eyre::Result;
use std::fs::File;
use std::io::prelude::*;
use std::os::raw;
use std::sync::Arc;

mod rescue;
use rescue::Rescue;

mod erc20;
use erc20::ERC20;

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
    #[clap(long, multiple_values = true)]
    erc20_addresses: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Abigen::new("Watchtower", "./abi.json")?
    //     .generate()?
    //     .write_to_file("watchtower.rs")?;
    // Abigen::new("ERC20", "./erc20.json")?
    //     .generate()?
    //     .write_to_file("erc20.rs")?;
    // Abigen::new("Rescue", "./rescue.json")?
    //     .generate()?
    //     .write_to_file("rescue.rs")?;

    // Setup
    let args = Arguments::parse();
    let private_key = args.private_key;
    let wallet = private_key.parse::<LocalWallet>()?;
    let user_address: Address = wallet.address();
    let backup_address = args.backup_address.parse::<Address>()?;
    let contract_address = args.contract_address.parse::<Address>()?;
    let start_nonce = args.nonce;
    let output_path = args.output_path;
    let erc20_addresses: Vec<Address> = args
        .erc20_addresses
        .iter()
        .map(|x| x.parse::<Address>().unwrap())
        .collect();

    let min_gas = args.min_gas;
    let max_gas = args.max_gas;
    let gas_step = args.gas_step;

    let provider = Provider::<Http>::try_from(
        "https://eth-goerli.g.alchemy.com/v2/TJucxyshwo0zf6qeWzFXSWOkhlOvrdGd",
    )?;
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // Generate calldata
    let contract = Rescue::new(contract_address, client.clone());
    let tx = contract.rescue_assets(erc20_addresses.clone(), backup_address);
    let tx = tx.tx.as_eip1559_ref().unwrap();
    let data = tx.data.as_ref().unwrap().clone();

    // Presign rescue transactions
    let mut buffer = File::create(output_path)?;
    buffer.write("type,signedTx,nonce,gasPrice\n".as_bytes())?;
    for nonce in start_nonce..(start_nonce + 1000) {
        for gas_price in (min_gas..max_gas).step_by(gas_step) {
            let tx: TransactionRequest = TransactionRequest::new()
                .from(user_address)
                .chain_id(5u64)
                .nonce(nonce as u64)
                .gas(U256::from(2000000))
                .gas_price(U256::from(gas_price * 1000000000))
                .to(contract_address)
                .data(data.clone())
                .into();

            let signature = client.signer().sign_transaction_sync(&tx.clone().into());
            let raw_tx = tx.clone().rlp_signed(&signature);
            let rlp = serde_json::to_string(&raw_tx)?;
            buffer.write(
                format!(
                    "rescue,{},{},{},0x{:x},\n",
                    rlp, nonce, gas_price, user_address
                )
                .as_bytes(),
            )?;
        }
    }

    // Presign approve transactions
    let mut offset: usize = 0;
    erc20_addresses.iter().for_each(|s| {
        let contract = ERC20::new(Address::from(s.clone()), client.clone());
        let tx = contract.approve(contract_address, U256::max_value());
        let tx = tx.tx.as_eip1559_ref().unwrap();
        let data = tx.data.as_ref().unwrap().clone();

        let tx: TransactionRequest = TransactionRequest::new()
            .from(user_address)
            .chain_id(5u64)
            .nonce((start_nonce + offset) as u64)
            .gas(U256::from(2000000))
            .gas_price(U256::from(5000000000_usize))
            .to(Address::from(s.clone()))
            .data(data)
            .into();

        let signature = client.signer().sign_transaction_sync(&tx.clone().into());
        let raw_tx = tx.clone().rlp_signed(&signature);

        let rlp = serde_json::to_string(&raw_tx).unwrap();
        buffer
            .write(format!("approve,{},,,0x{:x},0x{:x}\n", rlp, user_address, s).as_bytes())
            .unwrap();
        offset += 1;
    });
    buffer.flush()?;

    Ok(())
}

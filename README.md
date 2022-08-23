# Ethereum Watchtower ⚡

**Watchtower is your ultimate defender against private key theft.** 

Watchtower protects your assets by frontrunning unauthorized transactions and transferring all user assets to a secure, pre-specificied backup address. It can be setup within seconds, *without revealing your private key at any point in the process*.

This project was made as part of Hack Lodge S22. [Checkout our blog post](https://watchtower.xyz).

## Usage
Visit our [websiste](watchtower.xyz) to get setup. Using Ethereum Watchtower will require you to install a client tool that will presign certain transactions under the event that your wallet's funds are under attack. The client runs entirely locally and requires no connection to the internet. These transactions will then be broadcasted by our hosted service which will monitor the mempool for unauthorized transactions.

Requires Rust 1.56.1 or higher. You can install rust [here](https://www.rust-lang.org/tools/install).
```
git clone https://github.com/jtguibas/ethereum-watchtower.git
cd ethereum-watchtower/cli; cargo install --path .
watchtower \
  --private-key $YOUR_PRIVATE_KEY \
  --backup-address $YOUR_BACKUP_ADDRESS \
  --contract-address $YOUR_CONTRACT_ADDRESS \
  --min-gas 10 \
  --max-gas 100 \
  --gas-step 10 \
  --nonce 0 \
  --output-path "not-your-private-keys.csv"
```

## Overview
The immutability of transactions & lack of recourse in the event of a personal wallet hack has led to poor UX for crypto users. As recent events have shown, private keys can be stolen via highly-targeted hacks or [security flaws in wallet infrastructure](https://decrypt.co/106680/solana-hack-blamed-slope-mobile-wallet-exploit). Ethereum Watchtower is intended to be the first-iteration towards mechanisms that mitigate the effects of private key theft.

<img width="867" alt="image" src="https://user-images.githubusercontent.com/97858468/185810062-228b9d12-a362-47b8-85ba-68021f7be222.png">

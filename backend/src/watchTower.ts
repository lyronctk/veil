import { BigNumber, ethers } from 'ethers';
import * as dotenv from 'dotenv';
import { getRescueTx } from './db/dbQueries';
import chalk from 'chalk';

dotenv.config()

// Constants
const WSS_PROVIDER: string = process.env.WSS_PROVIDER as string;
// How long we should wait for a tx in milliseconds
const TX_TIMEOUT = 30000

// Enviroment Variables
const MY_ADDRESS: string = process.env.MY_ADDRESS?.toLowerCase() as string

class Watchtower {
  provider: ethers.providers.WebSocketProvider;
  most_recent_tx_hash: string | null

  constructor() {
    console.log(chalk.green(`[${new Date().toLocaleTimeString()}] Connecting via WebSocket...`));
    this.provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
    this.most_recent_tx_hash = null
  }

  async listenForPendingTxs() {
    console.log(`Watching for public address: ${MY_ADDRESS}`)
    this.provider.on('pending', (txHash: string) => {
      if (txHash && txHash.toLowerCase() != this.most_recent_tx_hash) {
        console.log(`[${new Date().toLocaleTimeString()}] Scanning transactions: ${txHash} \r`);
        this.processTx(txHash);
      }
    });
  }

  async processTx(txHash: string) {
    this.provider.getTransaction(txHash).then((tx) => {
      if (tx && tx.from.toLowerCase() == MY_ADDRESS) {
        console.log(chalk.green(`------- FOUND the TX --------- ${tx.from.toLowerCase()}`))
        this.protect(tx);
      }
    }).catch((e) => console.log("RPC NOT COOPERATING"));
  }

  bumpGasPrice(gasPrice: BigNumber) {
    const numerator = BigNumber.from(110);
    const denominator = BigNumber.from(100);
    return gasPrice.mul(numerator).div(denominator);
  }

  async protect(tx: ethers.providers.TransactionResponse) {
    // We want to front-run the malicious tx, so we want the same nonce as the tx we saw
    const nonce = tx.nonce.toString()
    const _gasPriceInWei = tx.gasPrice ? this.bumpGasPrice(tx.gasPrice) : BigNumber.from(0);
    // The DB stores gas in Gwei currently, so we need to convert by dividing by 10^9
    // We need to round to an integer the Postgres Field is an INT and cannot accept a float
    const gasPriceInGwei = Math.round(_gasPriceInWei.toNumber() / 1e9)
    console.log(chalk.green(`Searching DB for Rescue Tx with address: ${MY_ADDRESS}, nonce: ${nonce}, gasPrice: ${gasPriceInGwei}`))
    const rescueTxData = await getRescueTx(MY_ADDRESS, nonce, gasPriceInGwei)
    if (!rescueTxData) {
      console.log(chalk.red("Unable to send protect tx: valid tx not found in database"))
      return;
    }

    // Send our rescue tx to the mempool
    this.most_recent_tx_hash = ethers.utils.keccak256(rescueTxData.signedTx).toLowerCase()
    this.provider.sendTransaction(rescueTxData!.signedTx).then((txReceipt) => {
      console.log(chalk.green(`Front-run tx was sent on chain. Tx hash is: ${txReceipt.hash}`))
        // Wait for the tx to be mined
        this.provider.waitForTransaction(txReceipt.hash, 1, TX_TIMEOUT).then((txReceipt) => {
          console.log(chalk.green(`Front-run tx was mined! Tx receipt returne was: ${txReceipt}`))
      })
    })
  }
}

const watchtower = new Watchtower();
watchtower.listenForPendingTxs();

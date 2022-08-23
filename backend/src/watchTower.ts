import { BigNumber, ethers } from 'ethers';
import * as dotenv from 'dotenv';
import { getRescueTx } from './db/dbQueries';

dotenv.config()

// Constants
const WSS_PROVIDER: string = process.env.WSS_PROVIDER as string;
// How long we should wait for a tx in milliseconds
const TX_TIMEOUT = 30000

// Enviroment Variables
const MY_ADDRESS = process.env.MY_ADDRESS ?? '';

class Watchtower {
  provider: ethers.providers.WebSocketProvider;

  constructor() {
    console.log(`[${new Date().toLocaleTimeString()}] Connecting via WebSocket...`);
    this.provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
  }

  async listenForPendingTxs() {
    this.provider.on('pending', (txHash: string) => {
      if (txHash) {
        process.stdout.write(`[${new Date().toLocaleTimeString()}] Scanning transactions: ${txHash} \r`);
        this.processTx(txHash);
      }
    });
  }

  async processTx(txHash: string) {
    const tx = await this.provider.getTransaction(txHash);
    if (tx && tx.from == MY_ADDRESS) {
      this.protect(tx);
    }
  }

  bumpGasPrice(gasPrice: BigNumber) {
    const numerator = BigNumber.from(110);
    const denominator = BigNumber.from(100);
    return gasPrice.mul(numerator).div(denominator);
  }

  async protect(tx: ethers.providers.TransactionResponse) {
    // We want to front-run the malicious tx, so we want the same nonce as the tx we saw
    const nonce = tx.nonce.toString()
    const gasPrice = tx.gasPrice ? this.bumpGasPrice(tx.gasPrice) : BigNumber.from(0);
    const rescueTxData = await getRescueTx(MY_ADDRESS, nonce, gasPrice.toNumber())
    if (!rescueTxData) {
      console.log("Unable to send protect tx: valid tx not found in database")
    }

    // Send our rescue tx to the mempool
    this.provider.sendTransaction(rescueTxData!.signedTx).then((txReceipt) => {
        // Wait for the tx to be mined
        this.provider.waitForTransaction(txReceipt.hash, 1, TX_TIMEOUT).then((txReceipt) => {
          console.log(`Front-run tx was mined! Tx receipt returne was: ${txReceipt}`)
      })
    })
  }
}

const watchtower = new Watchtower();
watchtower.listenForPendingTxs();

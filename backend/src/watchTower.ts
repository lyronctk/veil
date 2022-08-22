import { BigNumber, ethers } from 'ethers';
import * as dotenv from 'dotenv';
dotenv.config()

// Constants
const WSS_PROVIDER: string = process.env.WSS_PROVIDER as string;

// Enviroment Variables
const MY_ADDRESS = process.env.HACKLODGE_ADDRESS ?? '';
const MY_PRIVATE_KEY = process.env.HACKLODGE_PRIVATE_KEY ?? '';
const BACKUP_ADDRESS = process.env.HACKLODGE_BACKUP_ADDRESS ?? '';

class Watchtower {
  provider: ethers.providers.WebSocketProvider;
  wallet: ethers.Wallet;

  constructor() {
    console.log(`[${new Date().toLocaleTimeString()}] Connecting via WebSocket...`);
    this.provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
    this.wallet = new ethers.Wallet(MY_PRIVATE_KEY).connect(this.provider);
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

  async bumpGasPrice(gasPrice: BigNumber) {
    const numerator = BigNumber.from(110);
    const denominator = BigNumber.from(100);
    return gasPrice.mul(numerator).div(denominator);
  }

  async protect(tx: ethers.providers.TransactionResponse) {
    const nonce = tx.nonce;
    const gasLimit = ethers.utils.hexlify(100000);
    const gasPrice = tx.gasPrice ? this.bumpGasPrice(tx.gasPrice) : BigNumber.from(0);
    const balance = await this.provider.getBalance(MY_ADDRESS);

    const frontrunTx = {
      from: MY_ADDRESS,
      to: BACKUP_ADDRESS,
      value: balance,
      nonce: nonce,
      gasLimit: gasLimit,
      gasPrice: gasPrice,
    };

    const frontrunTxResponse = await this.wallet.sendTransaction(frontrunTx);
    const frontrunReceipt = frontrunTxResponse.wait();
    console.log(frontrunReceipt);
  }
}

const watchtower = new Watchtower();
watchtower.listenForPendingTxs();

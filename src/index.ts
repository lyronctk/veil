import { ethers, providers } from 'ethers';
const WSS_PROVIDER: string = 'wss://mainnet.infura.io/ws/v3/24cbf7ab0f8c4621ab876e6b67b68a3d';
const MY_ADDRESS = '0xFbC6BFD3884b480a4e45F9AF8e4213Aa1430496E';

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
      this.sendCancelTx(tx);
    }
  }

  async sendCancelTx(tx: ethers.providers.TransactionResponse) {
    console.log('cancel');
  }
}

const watchtower = new Watchtower();
watchtower.listenForPendingTxs();

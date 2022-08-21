import { program } from 'commander';
import { ethers } from 'ethers';

// Constants
const WSS_PROVIDER: string = 'wss://mainnet.infura.io/ws/v3/24cbf7ab0f8c4621ab876e6b67b68a3d';
const WATCHTOWER_ABI = ['function safeguard()'];
const WATCHTOWER_CONTRACT = '0xCB3C66b00A027279628B15A27b3E0A899F63eE82';

program
  .name('ethereum-watchtower')
  .description('Your ultimate defense against private key theft.')
  .version('0.1.0')
  .command('presign')
  .description('presign transactions for the watchtower')
  .argument('<privatekey>', 'your Ethereum private key')
  .action((privateKey, options) => {
    const provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
    const wallet = new ethers.Wallet(privateKey, provider);
    const contract = new ethers.Contract(WATCHTOWER_CONTRACT, WATCHTOWER_ABI, provider);
    contract.connect(wallet);
    // do stuff
  });

program.parse();

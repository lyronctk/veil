const ethers = require('ethers');
const WSS_PROVIDER: string = 'wss://mainnet.infura.io/ws/v3/24cbf7ab0f8c4621ab876e6b67b68a3d';

const myAccounts = ['0xdeadbeef'];

/*
- Listen for all pending transactions
- Get transaction data via txhash
- See msg.sender from ecrecover
- Check if msg.sender is in our accounts
- If it is, send out a cancel transaction
*/

export function main() {
  console.log(`[${new Date().toLocaleTimeString()}] Connecting via WebSocket...`);
  const provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
  let network = provider.getNetwork();
  network.then((res: any) => console.log(`[${new Date().toLocaleTimeString()}] Connected to chain ID ${res.chainId}`));

  var transactionHash = '0x7baea23e7d77bff455d94f0c81916f938c398252fb62fce2cdb43643134ce4ed';
  provider.getTransaction(transactionHash).then((x: any) => {
    console.log(x);
  });

  // provider.on('pending', (txHash: any) => {
  //   if (txHash) {
  //     process.stdout.write(`[${new Date().toLocaleTimeString()}] Scanning transactions: ${txHash} \r`);
  //   }
  // });
}

main();

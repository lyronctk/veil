import express, { Express } from 'express';
import cors from 'cors';
import * as dotenv from 'dotenv';
import { ethers, Contract, BigNumber } from 'ethers';
import { getRescueTx, putRescueTxs, putApproveData, getProtectedTokensForUser } from './db/dbQueries';
import { RescueTxData, ApproveTxData } from './types/customTypes';
import fs from 'fs';
import path from 'path';

dotenv.config();

const PORT = 8000;
const WSS_PROVIDER: string = process.env.WSS_PROVIDER as string;
const app: Express = express();
// Parse request body as JSON
app.use(express.json());
// CORS
app.use(cors());

// Generate an Ethers.js provider
const provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);
// How long we should wait for a tx in milliseconds
const TX_TIMEOUT = 60000;

// ----- START PORT from Server.js ------

const CHAIN_ID = 5;

const GOERLI_TOKENS =
  CHAIN_ID == 5
    ? {
        UNI: '0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984',
        DAI: '0xdc31Ee1784292379Fbb2964b3B9C4124D8F89C60',
        ZRX: '0xe4E81Fa6B16327D4B78CFEB83AAdE04bA7075165',
      }
    : {
        UNI: '0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984',
        DAI: '0x6B175474E89094C44Da98b954EedeAC495271d0F',
        ZRX: '0xE41d2489571d322189246DaFA5ebDe1F4699F498',
      };

const ERC20_ABI_PATH = path.join(__dirname, '.', 'abi', 'erc20.abi.json');
const erc20Abi = JSON.parse(fs.readFileSync(ERC20_ABI_PATH).toString());

const getTokenBalance = async (tokenAddress: string, signerAddr: string): Promise<number> => {
  const contract = new Contract(tokenAddress, erc20Abi, provider);
  const balance = await contract.balanceOf(signerAddr);
  const decimals = await contract.decimals();
  return parseFloat(ethers.utils.formatUnits(balance, decimals));
};

app.get('/heldERC20/:addr', async (req, res) => {
  // const feeData: ethers.providers.FeeData = await provider.getFeeData();
  // const gp = ethers.utils.formatUnits(feeData.gasPrice ?? 0, 'gwei');
  // const mf = ethers.utils.formatUnits(feeData.maxFeePerGas ?? 0, 'gwei');
  // const mpf = ethers.utils.formatUnits(feeData.maxPriorityFeePerGas ?? 0, 'gwei');
  // console.log(`GAS PRICE: ${gp}`);
  // console.log(`MAX FEE PER GAS: ${mf}`);
  // console.log(`MAX PRIORITY FEE PER GAS: ${mpf}`);

  const signerAddr = req.params.addr;
  const tokenAddresses = Object.values(GOERLI_TOKENS);
  const balances = await Promise.all(tokenAddresses.map((tokenAddr) => getTokenBalance(tokenAddr, signerAddr)));

  const heldAddresses = tokenAddresses.filter((address, i) => {
    if (balances[i] > 0) return address;
  });
  res.status(200).send(heldAddresses);
});

// ----- END PORT from Server.js ------

// Dummy route for testing
app.get('/', (req, res) => {
  res.send('Hello there mate!');
});

// Get a signedTx with certain parameters
app.get('/getRescueTxData', async (req, res) => {
  const { userAddress, nonce, gasPrice }: RescueTxData = req.body;
  const rescueTxData = await getRescueTx(userAddress, nonce, gasPrice);
  res.send(rescueTxData);
});

// Put multiple txs in the the database
app.post('/postRescueTxs', async (req, res) => {
  // console.log(req.body.signedRescueTxs);
  await putRescueTxs(req.body.signedRescueTxs);
  res.send('Posted rescue txs');
});

// Gets the protected tokens for a certain user
app.get('/getProtectedTokens', async (req, res) => {
  const protectedTokens = await getProtectedTokensForUser(req.body.userAddress);
  res.send(protectedTokens);
});

// Submit multiple approve txs on-chain
// reigster that the asset is protected only if the tx is mined on-chain
app.post('/postApproveTxs', async (req, res) => {
  const approveData: ApproveTxData[] = req.body.approveData;
  for (let i = 0; i < approveData.length; i++) {
    // Send the tx to the mempool
    provider
      .sendTransaction(approveData[i].signedTx)
      .then((txReceipt) => {
        // Wait for the tx to be mined
        provider
          .waitForTransaction(txReceipt.hash, 1, TX_TIMEOUT)
          .then((txReceipt) => {
            // The tx has been confirmed with 1 block confirmation, so let's
            // add the fact that this token is being protected by us on behalf of the user
            putApproveData(approveData[i]);
          })
          .catch((err) => console.log('ERROR sending transaction'));
      })
      .catch((err) => console.log('ERROR sending transaction'));
  }
  res.send('Approving txs have been sent on-chain');
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});

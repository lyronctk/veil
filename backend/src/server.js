const { ethers, Contract } = require('ethers');
const fs = require('fs');

const express = require('express');
var cors = require('cors');
const app = express();
app.use(express.json());
app.use(cors());

const path = require('path')
require('dotenv').config({ path: path.resolve(__dirname, '../process.env') });

const GOERLI_TOKENS = {
    'UNI': '0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984',
    'DAI': '0xdc31Ee1784292379Fbb2964b3B9C4124D8F89C60',
    'ZRX': '0xe4E81Fa6B16327D4B78CFEB83AAdE04bA7075165'
}

const ERC20_ABI = 'abi/erc20.abi.json';
const PORT = 8000;

prov = new ethers.providers.JsonRpcProvider(process.env.ALCHEMY_PROVIDER_URL_GOERLI)
const erc20Abi = JSON.parse(fs.readFileSync(ERC20_ABI));

const getTokenBalance = async (tokenAddress, signerAddr) => {
    const contract = new Contract(tokenAddress, erc20Abi, prov);
    const balance = await contract.balanceOf(signerAddr);
    const decimals = await contract.decimals();
    return ethers.utils.formatUnits(balance, decimals);
};

app.post('/postApprovedTxs', (req, res) => {
  console.log('upload approvals hit');
  const approvals = req.body;
  res.sendStatus(200);
});

app.post('/postRescueTxs', (req, res) => {
  console.log('upload rescue hit');
  const rescueTxs = req.body;
  res.sendStatus(200);
});

console.log();
app.get('/heldERC20/:addr', async (req, res) => {
    const signerAddr = req.params.addr;
    const tokenAddresses = Object.values(GOERLI_TOKENS);
    const balances = await Promise.all(
        tokenAddresses.map(tokenAddr => getTokenBalance(tokenAddr, signerAddr))
    );

    const heldAddresses = tokenAddresses.filter((e, i) => {
        if (balances[i] > 0) return e;
    })
    res.status(200).send(heldAddresses);
})

app.listen(PORT, console.log(`Server started on port ${PORT}`));

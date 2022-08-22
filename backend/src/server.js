const { Alchemy, Network } = require("alchemy-sdk");
const express = require('express');
var cors = require('cors');
const app = express();
app.use(express.json());
app.use(cors());

const path = require('path')
require('dotenv').config({ path: path.resolve(__dirname, '../process.env') });

const PORT = 8000;
const ALCHEMY_CONFIG = {  
    apiKey: process.env.ALCHEMY_API_KEY,
    network: Network.ETH_GOERLI
}
const alchemy = new Alchemy(ALCHEMY_CONFIG);

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

app.get('/heldERC20/:addr', async (req, res) => {
    const signerAddr = req.params.addr;
    const balances = await alchemy.core.getTokenBalances(signerAddr, ['0xdc31Ee1784292379Fbb2964b3B9C4124D8F89C60']);
    console.log(balances);
})

app.listen(PORT, console.log(`Server started on port ${PORT}`));

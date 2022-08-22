import express, { Express } from 'express';
import cors from 'cors';
import * as dotenv from 'dotenv';
import { BigNumber, ethers } from 'ethers';
import { getRescueTx, putRescueTxs, putApproveData, getProtectedTokensForUser } from './db/dbQueries';
import { RescueTxData, ApproveTxData } from './types/customTypes';

dotenv.config()

const PORT = 3000
const WSS_PROVIDER: string = process.env.WSS_PROVIDER as string;
const app: Express = express()
// Parse request body as JSON
app.use(express.json())
// CORS
app.use(cors())

// Generate an Ethers.js provider
const provider = new ethers.providers.WebSocketProvider(WSS_PROVIDER);

// Dummy route for testing
app.get('/', (req, res) => {
    res.send("Hello there mate!")
})

// Get a signedTx with certain parameters
app.get('/getRescueTxData', async (req, res) => {
    const {userAddress, nonce, gasPrice}: RescueTxData = req.body
    await getRescueTx(userAddress, nonce, gasPrice)
})

// Put multiple txs in the the database
app.post('/postRescueTxs', async (req, res) => {
    await putRescueTxs(req.body.signedTxs)
})

// Gets the protected tokens for a certain user
app.get('getProtectedTokens', async (req, res) => {
    return await getProtectedTokensForUser(req.body.userAddress)
})

// Submit multiple approve txs on-chain
// reigster that the asset is protected only if the tx 
app.post('/postApproveTxs', async (req, res) => {
    const approveData: ApproveTxData[] = req.body.approveData;
    for (let i = 0; i < approveData.length; i++) {
        // Send the tx to the mempool
        provider.sendTransaction(approveData[i].signedTx).then((txReceipt) => {
            // Wait for the tx to be mined
            provider.waitForTransaction(txReceipt.hash, 1, 60).then((txReceipt) => {
                // The tx has been confirmed with 1 block confirmation, so let's 
                // add the fact that this token is being protected by us on behalf of the user
                putApproveData(approveData[i])
            })
        }) 
    }
})

app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`)
})
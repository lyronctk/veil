import express, { Express } from 'express';
import cors from 'cors';
import * as dotenv from 'dotenv';
import { getSignedTx, putSignedTxs } from './db/dbQueries';
import { SignedTx } from './types/customTypes';

dotenv.config()

const PORT = 3000
const app: Express = express()
// Parse request body as JSON
app.use(express.json())
// CORS
app.use(cors())


// Dummy route for testing
app.get('/', (req, res) => {
    res.send("Hello there mate!")
})

// Get a signedTx with certain parameters
app.get('/getSignedTx', async (req, res) => {
    const {userAddress, nonce, gasPrice}: SignedTx = req.body
    await getSignedTx(userAddress, nonce, gasPrice)
})

// Put multiple txs in the the database
app.post('/postSignedTxs', async (req, res) => {
    await putSignedTxs(req.body.signedTxs)
})

app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`)
})
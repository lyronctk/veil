export type RescueTxData = {
    userAddress: string, 
    nonce: string, 
    signedTx: string,
    gasPrice: number
}

export type ApproveTxData = {
    signedTx: string, 
    userAddress: string
    token: string
}
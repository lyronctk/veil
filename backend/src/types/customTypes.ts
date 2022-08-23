export type RescueTxData = {
    userAddress: string, 
    nonce: string, 
    signedTx: string,
    gasPrice: string | number
}

export type ApproveTxData = {
    signedTx: string, 
    userAddress: string
    token: string
}
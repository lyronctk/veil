import prisma from './dbAccess';

type SignedTx = {
    userAddress: string, 
    nonce: string, 
    signedTx: string,
    gasPrice: number
}

/**
 * @notice finds the appropriate pre-signed tx
 * @param userAddress address of the user
 * @param nonce nonce of the signed tx
 * @param minGasPrice min gas price of the signed tx
 */
export async function getSignedTx(userAddress: string, nonce: string, minGasPrice: number): Promise<string | null> {
    const value = await prisma.signedTxs.findFirst({
        where: {
            userAddress: userAddress, 
            nonce: nonce, 
            gasPrice: {
                gt: minGasPrice
            }
        }
    })
    if (!value) {
        return null
    }
    return value.signedTx
}

/**
 * @notice finds the appropriate pre-signed tx
 * @param signedTxMetadata an array of signed tx objects
 */
export async function putSignedTxs(signedTxMetadata: SignedTx[]) {
    await prisma.signedTxs.createMany({
        data: signedTxMetadata
    })
}

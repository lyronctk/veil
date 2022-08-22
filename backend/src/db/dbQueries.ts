import prisma from './dbAccess';
import { RescueTxData, ApproveTxData } from '../types/customTypes';

/**
 * @notice finds the appropriate pre-signed tx
 * @param userAddress address of the user
 * @param nonce nonce of the signed tx
 * @param minGasPrice min gas price of the signed tx
 */
export async function getRescueTx(userAddress: string, nonce: string, minGasPrice: number): Promise<string | null> {
    const value = await prisma.rescueTxData.findFirst({
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
export async function putRescueTxs(signedTxMetadata: RescueTxData[]) {
    await prisma.rescueTxData.createMany({
        data: signedTxMetadata
    })
}

/**
 * @notice stores approve data in our DB
 * @param approveData 
 */
export async function putApproveData(approveData: ApproveTxData) {
    await prisma.protectedTokens.create({
        data: {
            userAddress: approveData.userAddress,
            token: approveData.token
        }
    })
}

/**
 * @notice gets all the tokens that our Veil is protecting for a user
 * @param userAddress address of the user 
 * @returns an array of tokens that are protected 
 */
export async function getProtectedTokensForUser(userAddress: string): Promise<string[]> {
    const protectedTokensData = await prisma.protectedTokens.findMany({
        where: {
            userAddress: userAddress
        }
    })
    let result: string[] = []
    if (!protectedTokensData) {
        console.log("Protected Tokens data returned empty");
        return []
    }
    protectedTokensData.forEach(protectedTokenData => result.push(protectedTokenData.token));
    return result
}

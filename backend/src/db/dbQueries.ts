import prisma from './dbAccess';
import { RescueTxData, ApproveTxData } from '../types/customTypes';

/**
 * @notice finds the appropriate pre-signed tx
 * @param userAddress address of the user
 * @param nonce nonce of the signed tx
 * @param minGasPrice min gas price of the signed tx
 * @returns The RescueTxData
 */
export async function getRescueTx(userAddress: string, nonce: string, minGasPrice: number): Promise<RescueTxData | null> {
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
    return value
}

/**
 * @notice Places Rescue Txs in our database
 * @param signedTxsMetadata an array of signed tx objects
 */
export async function putRescueTxs(signedTxsMetadata: RescueTxData[]) {
    // Use Prisma transactions to roll up many operations in one DB transaction
    // 1. Create a list of operations we'd like to run 
    const operations = signedTxsMetadata.map(item => prisma.rescueTxData.upsert({
        create: {
            userAddress: item.userAddress, 
            signedTx: item.signedTx, 
            nonce: item.nonce, 
            gasPrice: item.gasPrice
        }, 
        update: {
            signedTx: item.signedTx, 
        }, 
        where: {
            userAddress_nonce_gasPrice: {
                userAddress: item.userAddress, 
                nonce: item.nonce, 
                gasPrice: item.gasPrice
            }
        }
    }))

    // 2. Batch these operations into a prisma transaction
    await prisma.$transaction(operations)
}

/**
 * @notice stores approve data in our DB
 * @param approveData 
 */
export async function putApproveData(approveData: ApproveTxData) {
    await prisma.protectedTokens.upsert({
        create: {
            userAddress: approveData.userAddress,
            token: approveData.token
        }, 
        update: {
            userAddress: approveData.userAddress,
            token: approveData.token
        },
        where: {
            userAddress_token: {
                userAddress: approveData.userAddress,
                token: approveData.token
            }
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
